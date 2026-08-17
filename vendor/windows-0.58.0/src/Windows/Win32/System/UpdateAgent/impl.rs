#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdates_Impl: Sized + super::Com::IDispatch_Impl {
    fn DetectNow(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn ShowSettingsDialog(&self) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableService(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdates {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdates_Vtbl
    where
        Identity: IAutomaticUpdates_Impl,
    {
        unsafe extern "system" fn DetectNow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdates_Impl::DetectNow(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdates_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdates_Impl::Resume(this).into()
        }
        unsafe extern "system" fn ShowSettingsDialog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdates_Impl::ShowSettingsDialog(this).into()
        }
        unsafe extern "system" fn Settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdates_Impl::Settings(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdates_Impl::ServiceEnabled(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdates_Impl::EnableService(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DetectNow: DetectNow::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            ServiceEnabled: ServiceEnabled::<Identity, OFFSET>,
            EnableService: EnableService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdates2_Impl: Sized + IAutomaticUpdates_Impl {
    fn Results(&self) -> windows_core::Result<IAutomaticUpdatesResults>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdates2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdates2_Vtbl
    where
        Identity: IAutomaticUpdates2_Impl,
    {
        unsafe extern "system" fn Results<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdates2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdates2_Impl::Results(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IAutomaticUpdates_Vtbl::new::<Identity, OFFSET>(), Results: Results::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdates as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesResults_Impl: Sized + super::Com::IDispatch_Impl {
    fn LastSearchSuccessDate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn LastInstallationSuccessDate(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdatesResults {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdatesResults_Vtbl
    where
        Identity: IAutomaticUpdatesResults_Impl,
    {
        unsafe extern "system" fn LastSearchSuccessDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesResults_Impl::LastSearchSuccessDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesResults_Impl::LastInstallationSuccessDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LastSearchSuccessDate: LastSearchSuccessDate::<Identity, OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Required(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::Result<()>;
    fn ScheduledInstallationTime(&self) -> windows_core::Result<i32>;
    fn SetScheduledInstallationTime(&self, value: i32) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdatesSettings_Vtbl
    where
        Identity: IAutomaticUpdatesSettings_Impl,
    {
        unsafe extern "system" fn NotificationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings_Impl::NotificationLevel(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings_Impl::SetNotificationLevel(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Required<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings_Impl::Required(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings_Impl::ScheduledInstallationDay(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings_Impl::SetScheduledInstallationDay(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ScheduledInstallationTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings_Impl::ScheduledInstallationTime(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings_Impl::SetScheduledInstallationTime(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings_Impl::Save(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NotificationLevel: NotificationLevel::<Identity, OFFSET>,
            SetNotificationLevel: SetNotificationLevel::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Required: Required::<Identity, OFFSET>,
            ScheduledInstallationDay: ScheduledInstallationDay::<Identity, OFFSET>,
            SetScheduledInstallationDay: SetScheduledInstallationDay::<Identity, OFFSET>,
            ScheduledInstallationTime: ScheduledInstallationTime::<Identity, OFFSET>,
            SetScheduledInstallationTime: SetScheduledInstallationTime::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings2_Impl: Sized + IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludeRecommendedUpdates(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdatesSettings2_Vtbl
    where
        Identity: IAutomaticUpdatesSettings2_Impl,
    {
        unsafe extern "system" fn IncludeRecommendedUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings2_Impl::IncludeRecommendedUpdates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings2_Impl::SetIncludeRecommendedUpdates(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CheckPermission<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings2_Impl::CheckPermission(this, core::mem::transmute_copy(&usertype), core::mem::transmute_copy(&permissiontype)) {
                Ok(ok__) => {
                    userhaspermission.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IAutomaticUpdatesSettings_Vtbl::new::<Identity, OFFSET>(),
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Identity, OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Identity, OFFSET>,
            CheckPermission: CheckPermission::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings3_Impl: Sized + IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNonAdministratorsElevated(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFeaturedUpdatesEnabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAutomaticUpdatesSettings3_Vtbl
    where
        Identity: IAutomaticUpdatesSettings3_Impl,
    {
        unsafe extern "system" fn NonAdministratorsElevated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings3_Impl::NonAdministratorsElevated(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings3_Impl::SetNonAdministratorsElevated(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAutomaticUpdatesSettings3_Impl::FeaturedUpdatesEnabled(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IAutomaticUpdatesSettings3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAutomaticUpdatesSettings3_Impl::SetFeaturedUpdatesEnabled(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, OFFSET>(),
            NonAdministratorsElevated: NonAdministratorsElevated::<Identity, OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Identity, OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Identity, OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategory_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CategoryID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Children(&self) -> windows_core::Result<ICategoryCollection>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Image(&self) -> windows_core::Result<IImageInformation>;
    fn Order(&self) -> windows_core::Result<i32>;
    fn Parent(&self) -> windows_core::Result<ICategory>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICategory {}
#[cfg(feature = "Win32_System_Com")]
impl ICategory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICategory_Vtbl
    where
        Identity: ICategory_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Name(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CategoryID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::CategoryID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Children(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Description(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Image(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Order<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Order(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Parent(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Type(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategory_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            CategoryID: CategoryID::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            Order: Order::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategory as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategoryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<ICategory>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICategoryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICategoryCollection_Vtbl
    where
        Identity: ICategoryCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategoryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategoryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICategoryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategoryCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICategoryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICategoryCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategoryCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: Option<&IDownloadJob>, callbackargs: Option<&IDownloadCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadCompletedCallback_Vtbl
    where
        Identity: IDownloadCompletedCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadCompletedCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadCompletedCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&downloadjob), windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadCompletedCallbackArgs_Vtbl
    where
        Identity: IDownloadCompletedCallbackArgs_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IDownloadProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadJob {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadJob_Vtbl
    where
        Identity: IDownloadJob_Impl,
    {
        unsafe extern "system" fn AsyncState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadJob_Impl::AsyncState(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadJob_Impl::IsCompleted(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadJob_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadJob_Impl::CleanUp(this).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadJob_Impl::GetProgress(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadJob_Impl::RequestAbort(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadJob as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateIndex(&self) -> windows_core::Result<i32>;
    fn PercentComplete(&self) -> windows_core::Result<i32>;
    fn TotalBytesDownloaded(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn TotalBytesToDownload(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadProgress {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadProgress_Vtbl
    where
        Identity: IDownloadProgress_Impl,
    {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::CurrentUpdateBytesDownloaded(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::CurrentUpdateBytesToDownload(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::CurrentUpdateIndex(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::PercentComplete(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::TotalBytesDownloaded(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::TotalBytesToDownload(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPhase) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::CurrentUpdateDownloadPhase(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDownloadProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgress_Impl::CurrentUpdatePercentComplete(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentUpdateBytesDownloaded: CurrentUpdateBytesDownloaded::<Identity, OFFSET>,
            CurrentUpdateBytesToDownload: CurrentUpdateBytesToDownload::<Identity, OFFSET>,
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, OFFSET>,
            PercentComplete: PercentComplete::<Identity, OFFSET>,
            TotalBytesDownloaded: TotalBytesDownloaded::<Identity, OFFSET>,
            TotalBytesToDownload: TotalBytesToDownload::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
            CurrentUpdateDownloadPhase: CurrentUpdateDownloadPhase::<Identity, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgress as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: Option<&IDownloadJob>, callbackargs: Option<&IDownloadProgressChangedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadProgressChangedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadProgressChangedCallback_Vtbl
    where
        Identity: IDownloadProgressChangedCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadProgressChangedCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDownloadProgressChangedCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&downloadjob), windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> windows_core::Result<IDownloadProgress>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadProgressChangedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadProgressChangedCallbackArgs_Vtbl
    where
        Identity: IDownloadProgressChangedCallbackArgs_Impl,
    {
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadProgressChangedCallbackArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadProgressChangedCallbackArgs_Impl::Progress(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Progress: Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDownloadResult {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDownloadResult_Vtbl
    where
        Identity: IDownloadResult_Impl,
    {
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDownloadResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadResult_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: IDownloadResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadResult_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDownloadResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDownloadResult_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            HResult: HResult::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImageInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn AltText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Width(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImageInformation {}
#[cfg(feature = "Win32_System_Com")]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageInformation_Vtbl
    where
        Identity: IImageInformation_Impl,
    {
        unsafe extern "system" fn AltText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IImageInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageInformation_Impl::AltText(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageInformation_Impl::Height(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IImageInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageInformation_Impl::Source(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageInformation_Impl::Width(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AltText: AltText::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Source: Source::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageInformation as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationAgent_Impl: Sized + super::Com::IDispatch_Impl {
    fn RecordInstallationResult(&self, installationresultcookie: &windows_core::BSTR, hresult: i32, extendedreportingdata: Option<&IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationAgent {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationAgent_Vtbl
    where
        Identity: IInstallationAgent_Impl,
    {
        unsafe extern "system" fn RecordInstallationResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationresultcookie: core::mem::MaybeUninit<windows_core::BSTR>, hresult: i32, extendedreportingdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationAgent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInstallationAgent_Impl::RecordInstallationResult(this, core::mem::transmute(&installationresultcookie), core::mem::transmute_copy(&hresult), windows_core::from_raw_borrowed(&extendedreportingdata)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), RecordInstallationResult: RecordInstallationResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationAgent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationBehavior_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanRequestUserInput(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Impact(&self) -> windows_core::Result<InstallationImpact>;
    fn RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationBehavior {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationBehavior_Vtbl
    where
        Identity: IInstallationBehavior_Impl,
    {
        unsafe extern "system" fn CanRequestUserInput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInstallationBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationBehavior_Impl::CanRequestUserInput(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Impact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationImpact) -> windows_core::HRESULT
        where
            Identity: IInstallationBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationBehavior_Impl::Impact(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootBehavior<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> windows_core::HRESULT
        where
            Identity: IInstallationBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationBehavior_Impl::RebootBehavior(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInstallationBehavior_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationBehavior_Impl::RequiresNetworkConnectivity(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CanRequestUserInput: CanRequestUserInput::<Identity, OFFSET>,
            Impact: Impact::<Identity, OFFSET>,
            RebootBehavior: RebootBehavior::<Identity, OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationBehavior as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: Option<&IInstallationJob>, callbackargs: Option<&IInstallationCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationCompletedCallback_Vtbl
    where
        Identity: IInstallationCompletedCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationCompletedCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInstallationCompletedCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&installationjob), windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationCompletedCallbackArgs_Vtbl
    where
        Identity: IInstallationCompletedCallbackArgs_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IInstallationProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationJob {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationJob_Vtbl
    where
        Identity: IInstallationJob_Impl,
    {
        unsafe extern "system" fn AsyncState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationJob_Impl::AsyncState(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationJob_Impl::IsCompleted(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationJob_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInstallationJob_Impl::CleanUp(this).into()
        }
        unsafe extern "system" fn GetProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationJob_Impl::GetProgress(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInstallationJob_Impl::RequestAbort(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationJob as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateIndex(&self) -> windows_core::Result<i32>;
    fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32>;
    fn PercentComplete(&self) -> windows_core::Result<i32>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationProgress {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationProgress_Vtbl
    where
        Identity: IInstallationProgress_Impl,
    {
        unsafe extern "system" fn CurrentUpdateIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInstallationProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationProgress_Impl::CurrentUpdateIndex(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInstallationProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationProgress_Impl::CurrentUpdatePercentComplete(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInstallationProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationProgress_Impl::PercentComplete(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationProgress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationProgress_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, OFFSET>,
            PercentComplete: PercentComplete::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgress as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: Option<&IInstallationJob>, callbackargs: Option<&IInstallationProgressChangedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationProgressChangedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationProgressChangedCallback_Vtbl
    where
        Identity: IInstallationProgressChangedCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationProgressChangedCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInstallationProgressChangedCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&installationjob), windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> windows_core::Result<IInstallationProgress>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationProgressChangedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationProgressChangedCallbackArgs_Vtbl
    where
        Identity: IInstallationProgressChangedCallbackArgs_Impl,
    {
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationProgressChangedCallbackArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationProgressChangedCallbackArgs_Impl::Progress(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Progress: Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInstallationResult {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInstallationResult_Vtbl
    where
        Identity: IInstallationResult_Impl,
    {
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationResult_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationResult_Impl::RebootRequired(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: IInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationResult_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInstallationResult_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            HResult: HResult::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInvalidProductLicenseException_Impl: Sized + IUpdateException_Impl {
    fn Product(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IInvalidProductLicenseException {}
#[cfg(feature = "Win32_System_Com")]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInvalidProductLicenseException_Vtbl
    where
        Identity: IInvalidProductLicenseException_Impl,
    {
        unsafe extern "system" fn Product<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IInvalidProductLicenseException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInvalidProductLicenseException_Impl::Product(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateException_Vtbl::new::<Identity, OFFSET>(), Product: Product::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateException as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallback_Impl: Sized {
    fn Invoke(&self, searchjob: Option<&ISearchJob>, callbackargs: Option<&ISearchCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCompletedCallback_Vtbl
    where
        Identity: ISearchCompletedCallback_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCompletedCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCompletedCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&searchjob), windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCompletedCallbackArgs_Vtbl
    where
        Identity: ISearchCompletedCallbackArgs_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchJob {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchJob_Vtbl
    where
        Identity: ISearchJob_Impl,
    {
        unsafe extern "system" fn AsyncState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISearchJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchJob_Impl::AsyncState(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchJob_Impl::IsCompleted(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchJob_Impl::CleanUp(this).into()
        }
        unsafe extern "system" fn RequestAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchJob_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchJob_Impl::RequestAbort(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchJob as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn RootCategories(&self) -> windows_core::Result<ICategoryCollection>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchResult {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchResult_Vtbl
    where
        Identity: ISearchResult_Impl,
    {
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: ISearchResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchResult_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchResult_Impl::RootCategories(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchResult_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Warnings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchResult_Impl::Warnings(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ResultCode: ResultCode::<Identity, OFFSET>,
            RootCategories: RootCategories::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            Warnings: Warnings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStringCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn put_Item(&self, index: i32, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, value: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IStringCollection>;
    fn Insert(&self, index: i32, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStringCollection_Vtbl
    where
        Identity: IStringCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStringCollection_Impl::put_Item(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::Add(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStringCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringCollection_Impl::Copy(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStringCollection_Impl::Insert(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT
        where
            Identity: IStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStringCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            put_Item: put_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISystemInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISystemInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemInformation_Vtbl
    where
        Identity: ISystemInformation_Impl,
    {
        unsafe extern "system" fn OemHardwareSupportLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISystemInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemInformation_Impl::OemHardwareSupportLink(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISystemInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISystemInformation_Impl::RebootRequired(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OemHardwareSupportLink: OemHardwareSupportLink::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemInformation as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate_Impl: Sized + super::Com::IDispatch_Impl {
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AutoSelectOnWebSites(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BundledUpdates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CanRequireSource(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Categories(&self) -> windows_core::Result<ICategoryCollection>;
    fn Deadline(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EulaAccepted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EulaText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HandlerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Identity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn Image(&self) -> windows_core::Result<IImageInformation>;
    fn InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn IsBeta(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsDownloaded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsInstalled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsMandatory(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUninstallable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Languages(&self) -> windows_core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(&self) -> windows_core::Result<f64>;
    fn MaxDownloadSize(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MinDownloadSize(&self) -> windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MoreInfoUrls(&self) -> windows_core::Result<IStringCollection>;
    fn MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RecommendedCpuSpeed(&self) -> windows_core::Result<i32>;
    fn RecommendedHardDiskSpace(&self) -> windows_core::Result<i32>;
    fn RecommendedMemory(&self) -> windows_core::Result<i32>;
    fn ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection>;
    fn SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection>;
    fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<UpdateType>;
    fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn KBArticleIDs(&self) -> windows_core::Result<IStringCollection>;
    fn AcceptEula(&self) -> windows_core::Result<()>;
    fn DeploymentAction(&self) -> windows_core::Result<DeploymentAction>;
    fn CopyFromCache(&self, path: &windows_core::BSTR, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DownloadPriority(&self) -> windows_core::Result<DownloadPriority>;
    fn DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdate {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdate_Vtbl
    where
        Identity: IUpdate_Impl,
    {
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Title(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::AutoSelectOnWebSites(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::BundledUpdates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequireSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::CanRequireSource(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Categories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Categories(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Deadline(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::DeltaCompressedContentAvailable(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::DeltaCompressedContentPreferred(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Description(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaAccepted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::EulaAccepted(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::EulaText(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandlerID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::HandlerID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Identity(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Image(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallationBehavior<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::InstallationBehavior(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBeta<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsBeta(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsDownloaded(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsHidden(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdate_Impl::SetIsHidden(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsInstalled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsInstalled(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMandatory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsMandatory(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::IsUninstallable(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Languages(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::LastDeploymentChangeTime(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::MaxDownloadSize(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDownloadSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::MinDownloadSize(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::MoreInfoUrls(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsrcSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::MsrcSeverity(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::RecommendedCpuSpeed(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::RecommendedHardDiskSpace(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedMemory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::RecommendedMemory(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::ReleaseNotes(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::SecurityBulletinIDs(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::SupersededUpdateIDs(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::SupportUrl(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateType) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::Type(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::UninstallationNotes(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::UninstallationBehavior(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::UninstallationSteps(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KBArticleIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::KBArticleIDs(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEula<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdate_Impl::AcceptEula(this).into()
        }
        unsafe extern "system" fn DeploymentAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DeploymentAction) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::DeploymentAction(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdate_Impl::CopyFromCache(this, core::mem::transmute(&path), core::mem::transmute_copy(&toextractcabfiles)).into()
        }
        unsafe extern "system" fn DownloadPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::DownloadPriority(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadContents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate_Impl::DownloadContents(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Title: Title::<Identity, OFFSET>,
            AutoSelectOnWebSites: AutoSelectOnWebSites::<Identity, OFFSET>,
            BundledUpdates: BundledUpdates::<Identity, OFFSET>,
            CanRequireSource: CanRequireSource::<Identity, OFFSET>,
            Categories: Categories::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            DeltaCompressedContentAvailable: DeltaCompressedContentAvailable::<Identity, OFFSET>,
            DeltaCompressedContentPreferred: DeltaCompressedContentPreferred::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            EulaAccepted: EulaAccepted::<Identity, OFFSET>,
            EulaText: EulaText::<Identity, OFFSET>,
            HandlerID: HandlerID::<Identity, OFFSET>,
            Identity: Identity::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            InstallationBehavior: InstallationBehavior::<Identity, OFFSET>,
            IsBeta: IsBeta::<Identity, OFFSET>,
            IsDownloaded: IsDownloaded::<Identity, OFFSET>,
            IsHidden: IsHidden::<Identity, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, OFFSET>,
            IsInstalled: IsInstalled::<Identity, OFFSET>,
            IsMandatory: IsMandatory::<Identity, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, OFFSET>,
            Languages: Languages::<Identity, OFFSET>,
            LastDeploymentChangeTime: LastDeploymentChangeTime::<Identity, OFFSET>,
            MaxDownloadSize: MaxDownloadSize::<Identity, OFFSET>,
            MinDownloadSize: MinDownloadSize::<Identity, OFFSET>,
            MoreInfoUrls: MoreInfoUrls::<Identity, OFFSET>,
            MsrcSeverity: MsrcSeverity::<Identity, OFFSET>,
            RecommendedCpuSpeed: RecommendedCpuSpeed::<Identity, OFFSET>,
            RecommendedHardDiskSpace: RecommendedHardDiskSpace::<Identity, OFFSET>,
            RecommendedMemory: RecommendedMemory::<Identity, OFFSET>,
            ReleaseNotes: ReleaseNotes::<Identity, OFFSET>,
            SecurityBulletinIDs: SecurityBulletinIDs::<Identity, OFFSET>,
            SupersededUpdateIDs: SupersededUpdateIDs::<Identity, OFFSET>,
            SupportUrl: SupportUrl::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, OFFSET>,
            UninstallationBehavior: UninstallationBehavior::<Identity, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, OFFSET>,
            KBArticleIDs: KBArticleIDs::<Identity, OFFSET>,
            AcceptEula: AcceptEula::<Identity, OFFSET>,
            DeploymentAction: DeploymentAction::<Identity, OFFSET>,
            CopyFromCache: CopyFromCache::<Identity, OFFSET>,
            DownloadPriority: DownloadPriority::<Identity, OFFSET>,
            DownloadContents: DownloadContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate2_Impl: Sized + IUpdate_Impl {
    fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: Option<&IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdate2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdate2_Vtbl
    where
        Identity: IUpdate2_Impl,
    {
        unsafe extern "system" fn RebootRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate2_Impl::RebootRequired(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate2_Impl::IsPresent(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate2_Impl::CveIDs(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiles: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdate2_Impl::CopyToCache(this, windows_core::from_raw_borrowed(&pfiles)).into()
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            IsPresent: IsPresent::<Identity, OFFSET>,
            CveIDs: CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate3_Impl: Sized + IUpdate2_Impl {
    fn BrowseOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdate3 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdate3_Vtbl
    where
        Identity: IUpdate3_Impl,
    {
        unsafe extern "system" fn BrowseOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate3_Impl::BrowseOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdate2_Vtbl::new::<Identity, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate4_Impl: Sized + IUpdate3_Impl {
    fn PerUser(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdate4 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdate4_Vtbl
    where
        Identity: IUpdate4_Impl,
    {
        unsafe extern "system" fn PerUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdate4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate4_Impl::PerUser(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdate3_Vtbl::new::<Identity, OFFSET>(), PerUser: PerUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate5_Impl: Sized + IUpdate4_Impl {
    fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdate5 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdate5_Vtbl
    where
        Identity: IUpdate5_Impl,
    {
        unsafe extern "system" fn AutoSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT
        where
            Identity: IUpdate5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate5_Impl::AutoSelection(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT
        where
            Identity: IUpdate5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdate5_Impl::AutoDownload(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdate4_Vtbl::new::<Identity, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, OFFSET>,
            AutoDownload: AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate5 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<IUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IUpdate>;
    fn put_Item(&self, index: i32, value: Option<&IUpdate>) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, value: Option<&IUpdate>) -> windows_core::Result<i32>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IUpdateCollection>;
    fn Insert(&self, index: i32, value: Option<&IUpdate>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateCollection_Vtbl
    where
        Identity: IUpdateCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateCollection_Impl::put_Item(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::Add(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateCollection_Impl::Copy(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateCollection_Impl::Insert(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT
        where
            Identity: IUpdateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            put_Item: put_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateDownloadContent {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateDownloadContent_Vtbl
    where
        Identity: IUpdateDownloadContent_Impl,
    {
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadContent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadContent_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), DownloadUrl: DownloadUrl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContent2_Impl: Sized + IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateDownloadContent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateDownloadContent2_Vtbl
    where
        Identity: IUpdateDownloadContent2_Impl,
    {
        unsafe extern "system" fn IsDeltaCompressedContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadContent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadContent2_Impl::IsDeltaCompressedContent(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateDownloadContent_Vtbl::new::<Identity, OFFSET>(), IsDeltaCompressedContent: IsDeltaCompressedContent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateDownloadContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContentCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateDownloadContent>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateDownloadContentCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateDownloadContentCollection_Vtbl
    where
        Identity: IUpdateDownloadContentCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadContentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadContentCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadContentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadContentCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadContentCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadContentCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateDownloadResult {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateDownloadResult_Vtbl
    where
        Identity: IUpdateDownloadResult_Impl,
    {
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadResult_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloadResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloadResult_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), HResult: HResult::<Identity, OFFSET>, ResultCode: ResultCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloader_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsForced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<DownloadPriority>;
    fn SetPriority(&self, value: DownloadPriority) -> windows_core::Result<()>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: Option<&IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginDownload(&self, onprogresschanged: Option<&windows_core::IUnknown>, oncompleted: Option<&windows_core::IUnknown>, state: &windows_core::VARIANT) -> windows_core::Result<IDownloadJob>;
    fn Download(&self) -> windows_core::Result<IDownloadResult>;
    fn EndDownload(&self, value: Option<&IDownloadJob>) -> windows_core::Result<IDownloadResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateDownloader {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateDownloader_Vtbl
    where
        Identity: IUpdateDownloader_Impl,
    {
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateDownloader_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::IsForced(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateDownloader_Impl::SetIsForced(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::Priority(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: DownloadPriority) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateDownloader_Impl::SetPriority(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateDownloader_Impl::SetUpdates(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BeginDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: core::mem::MaybeUninit<windows_core::VARIANT>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::BeginDownload(this, windows_core::from_raw_borrowed(&onprogresschanged), windows_core::from_raw_borrowed(&oncompleted), core::mem::transmute(&state)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::Download(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateDownloader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateDownloader_Impl::EndDownload(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IsForced: IsForced::<Identity, OFFSET>,
            SetIsForced: SetIsForced::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            SetUpdates: SetUpdates::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            EndDownload: EndDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloader as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateException_Impl: Sized + super::Com::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HResult(&self) -> windows_core::Result<i32>;
    fn Context(&self) -> windows_core::Result<UpdateExceptionContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateException {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateException_Vtbl
    where
        Identity: IUpdateException_Impl,
    {
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateException_Impl::Message(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateException_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateExceptionContext) -> windows_core::HRESULT
        where
            Identity: IUpdateException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateException_Impl::Context(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Message: Message::<Identity, OFFSET>,
            HResult: HResult::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateException as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateExceptionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateException>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateExceptionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateExceptionCollection_Vtbl
    where
        Identity: IUpdateExceptionCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateExceptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateExceptionCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateExceptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateExceptionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateExceptionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateExceptionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Operation(&self) -> windows_core::Result<UpdateOperation>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn HResult(&self) -> windows_core::Result<i32>;
    fn Date(&self) -> windows_core::Result<f64>;
    fn UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UnmappedResultCode(&self) -> windows_core::Result<i32>;
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateHistoryEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateHistoryEntry_Vtbl
    where
        Identity: IUpdateHistoryEntry_Impl,
    {
        unsafe extern "system" fn Operation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateOperation) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::Operation(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::Date(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::UpdateIdentity(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::Title(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::Description(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::UnmappedResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::ServerSelection(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::ServiceID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::UninstallationSteps(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::UninstallationNotes(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry_Impl::SupportUrl(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Operation: Operation::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            HResult: HResult::<Identity, OFFSET>,
            Date: Date::<Identity, OFFSET>,
            UpdateIdentity: UpdateIdentity::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            UnmappedResultCode: UnmappedResultCode::<Identity, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            ServerSelection: ServerSelection::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, OFFSET>,
            SupportUrl: SupportUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntry2_Impl: Sized + IUpdateHistoryEntry_Impl {
    fn Categories(&self) -> windows_core::Result<ICategoryCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateHistoryEntry2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateHistoryEntry2_Vtbl
    where
        Identity: IUpdateHistoryEntry2_Impl,
    {
        unsafe extern "system" fn Categories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntry2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntry2_Impl::Categories(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateHistoryEntry_Vtbl::new::<Identity, OFFSET>(), Categories: Categories::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateHistoryEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateHistoryEntryCollection_Vtbl
    where
        Identity: IUpdateHistoryEntryCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntryCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateHistoryEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateHistoryEntryCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateIdentity_Impl: Sized + super::Com::IDispatch_Impl {
    fn RevisionNumber(&self) -> windows_core::Result<i32>;
    fn UpdateID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateIdentity {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateIdentity_Vtbl
    where
        Identity: IUpdateIdentity_Impl,
    {
        unsafe extern "system" fn RevisionNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateIdentity_Impl::RevisionNumber(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateIdentity_Impl::UpdateID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RevisionNumber: RevisionNumber::<Identity, OFFSET>,
            UpdateID: UpdateID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateIdentity as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateInstallationResult {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateInstallationResult_Vtbl
    where
        Identity: IUpdateInstallationResult_Impl,
    {
        unsafe extern "system" fn HResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstallationResult_Impl::HResult(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstallationResult_Impl::RebootRequired(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT
        where
            Identity: IUpdateInstallationResult_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstallationResult_Impl::ResultCode(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            HResult: HResult::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsForced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn SetParentHwnd(&self, value: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn SetParentWindow(&self, value: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: Option<&IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginInstall(&self, onprogresschanged: Option<&windows_core::IUnknown>, oncompleted: Option<&windows_core::IUnknown>, state: &windows_core::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn BeginUninstall(&self, onprogresschanged: Option<&windows_core::IUnknown>, oncompleted: Option<&windows_core::IUnknown>, state: &windows_core::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn EndInstall(&self, value: Option<&IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn EndUninstall(&self, value: Option<&IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn Install(&self) -> windows_core::Result<IInstallationResult>;
    fn RunWizard(&self, dialogtitle: &windows_core::BSTR) -> windows_core::Result<IInstallationResult>;
    fn IsBusy(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Uninstall(&self) -> windows_core::Result<IInstallationResult>;
    fn AllowSourcePrompts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowSourcePrompts(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateInstaller {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateInstaller_Vtbl
    where
        Identity: IUpdateInstaller_Impl,
    {
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::IsForced(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetIsForced(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::ParentHwnd(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetParentHwnd(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetParentWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetParentWindow(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::ParentWindow(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::Updates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetUpdates(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: core::mem::MaybeUninit<windows_core::VARIANT>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::BeginInstall(this, windows_core::from_raw_borrowed(&onprogresschanged), windows_core::from_raw_borrowed(&oncompleted), core::mem::transmute(&state)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: core::mem::MaybeUninit<windows_core::VARIANT>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::BeginUninstall(this, windows_core::from_raw_borrowed(&onprogresschanged), windows_core::from_raw_borrowed(&oncompleted), core::mem::transmute(&state)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::EndInstall(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUninstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::EndUninstall(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::Install(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWizard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dialogtitle: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::RunWizard(this, core::mem::transmute(&dialogtitle)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBusy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::IsBusy(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::Uninstall(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSourcePrompts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::AllowSourcePrompts(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller_Impl::SetAllowSourcePrompts(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller_Impl::RebootRequiredBeforeInstallation(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IsForced: IsForced::<Identity, OFFSET>,
            SetIsForced: SetIsForced::<Identity, OFFSET>,
            ParentHwnd: ParentHwnd::<Identity, OFFSET>,
            SetParentHwnd: SetParentHwnd::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            SetUpdates: SetUpdates::<Identity, OFFSET>,
            BeginInstall: BeginInstall::<Identity, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, OFFSET>,
            EndInstall: EndInstall::<Identity, OFFSET>,
            EndUninstall: EndUninstall::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            RunWizard: RunWizard::<Identity, OFFSET>,
            IsBusy: IsBusy::<Identity, OFFSET>,
            Uninstall: Uninstall::<Identity, OFFSET>,
            AllowSourcePrompts: AllowSourcePrompts::<Identity, OFFSET>,
            SetAllowSourcePrompts: SetAllowSourcePrompts::<Identity, OFFSET>,
            RebootRequiredBeforeInstallation: RebootRequiredBeforeInstallation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller2_Impl: Sized + IUpdateInstaller_Impl {
    fn ForceQuiet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetForceQuiet(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateInstaller2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateInstaller2_Vtbl
    where
        Identity: IUpdateInstaller2_Impl,
    {
        unsafe extern "system" fn ForceQuiet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller2_Impl::ForceQuiet(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceQuiet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller2_Impl::SetForceQuiet(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateInstaller_Vtbl::new::<Identity, OFFSET>(),
            ForceQuiet: ForceQuiet::<Identity, OFFSET>,
            SetForceQuiet: SetForceQuiet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller3_Impl: Sized + IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttemptCloseAppsIfNecessary(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateInstaller3 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateInstaller3_Vtbl
    where
        Identity: IUpdateInstaller3_Impl,
    {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateInstaller3_Impl::AttemptCloseAppsIfNecessary(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller3_Impl::SetAttemptCloseAppsIfNecessary(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateInstaller2_Vtbl::new::<Identity, OFFSET>(),
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Identity, OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<IUpdateInstaller2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller4_Impl: Sized + IUpdateInstaller3_Impl {
    fn Commit(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateInstaller4 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateInstaller4_Vtbl
    where
        Identity: IUpdateInstaller4_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IUpdateInstaller4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateInstaller4_Impl::Commit(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: IUpdateInstaller3_Vtbl::new::<Identity, OFFSET>(), Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<IUpdateInstaller2 as windows_core::Interface>::IID || iid == &<IUpdateInstaller3 as windows_core::Interface>::IID
    }
}
pub trait IUpdateLockdown_Impl: Sized {
    fn LockDown(&self, flags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUpdateLockdown {}
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateLockdown_Vtbl
    where
        Identity: IUpdateLockdown_Impl,
    {
        unsafe extern "system" fn LockDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT
        where
            Identity: IUpdateLockdown_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateLockdown_Impl::LockDown(this, core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LockDown: LockDown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateLockdown as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCanAutomaticallyUpgradeService(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludePotentiallySupersededUpdates(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn SetServerSelection(&self, value: ServerSelection) -> windows_core::Result<()>;
    fn BeginSearch(&self, criteria: &windows_core::BSTR, oncompleted: Option<&windows_core::IUnknown>, state: &windows_core::VARIANT) -> windows_core::Result<ISearchJob>;
    fn EndSearch(&self, searchjob: Option<&ISearchJob>) -> windows_core::Result<ISearchResult>;
    fn EscapeString(&self, unescaped: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn QueryHistory(&self, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&self, criteria: &windows_core::BSTR) -> windows_core::Result<ISearchResult>;
    fn Online(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOnline(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetTotalHistoryCount(&self) -> windows_core::Result<i32>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSearcher {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSearcher_Vtbl
    where
        Identity: IUpdateSearcher_Impl,
    {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::CanAutomaticallyUpgradeService(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetCanAutomaticallyUpgradeService(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::IncludePotentiallySupersededUpdates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetIncludePotentiallySupersededUpdates(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ServerSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::ServerSelection(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ServerSelection) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetServerSelection(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BeginSearch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: core::mem::MaybeUninit<windows_core::BSTR>, oncompleted: *mut core::ffi::c_void, state: core::mem::MaybeUninit<windows_core::VARIANT>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::BeginSearch(this, core::mem::transmute(&criteria), windows_core::from_raw_borrowed(&oncompleted), core::mem::transmute(&state)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSearch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchjob: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::EndSearch(this, windows_core::from_raw_borrowed(&searchjob)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unescaped: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::EscapeString(this, core::mem::transmute(&unescaped)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: i32, count: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::QueryHistory(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Search<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::Search(this, core::mem::transmute(&criteria)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::Online(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetOnline(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::GetTotalHistoryCount(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher_Impl::ServiceID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher_Impl::SetServiceID(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CanAutomaticallyUpgradeService: CanAutomaticallyUpgradeService::<Identity, OFFSET>,
            SetCanAutomaticallyUpgradeService: SetCanAutomaticallyUpgradeService::<Identity, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IncludePotentiallySupersededUpdates: IncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            SetIncludePotentiallySupersededUpdates: SetIncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            ServerSelection: ServerSelection::<Identity, OFFSET>,
            SetServerSelection: SetServerSelection::<Identity, OFFSET>,
            BeginSearch: BeginSearch::<Identity, OFFSET>,
            EndSearch: EndSearch::<Identity, OFFSET>,
            EscapeString: EscapeString::<Identity, OFFSET>,
            QueryHistory: QueryHistory::<Identity, OFFSET>,
            Search: Search::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            SetOnline: SetOnline::<Identity, OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            SetServiceID: SetServiceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher2_Impl: Sized + IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIgnoreDownloadPriority(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSearcher2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSearcher2_Vtbl
    where
        Identity: IUpdateSearcher2_Impl,
    {
        unsafe extern "system" fn IgnoreDownloadPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher2_Impl::IgnoreDownloadPriority(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher2_Impl::SetIgnoreDownloadPriority(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateSearcher_Vtbl::new::<Identity, OFFSET>(),
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Identity, OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher3_Impl: Sized + IUpdateSearcher2_Impl {
    fn SearchScope(&self) -> windows_core::Result<SearchScope>;
    fn SetSearchScope(&self, value: SearchScope) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSearcher3 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSearcher3_Vtbl
    where
        Identity: IUpdateSearcher3_Impl,
    {
        unsafe extern "system" fn SearchScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut SearchScope) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSearcher3_Impl::SearchScope(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SearchScope) -> windows_core::HRESULT
        where
            Identity: IUpdateSearcher3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSearcher3_Impl::SetSearchScope(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateSearcher2_Vtbl::new::<Identity, OFFSET>(),
            SearchScope: SearchScope::<Identity, OFFSET>,
            SetSearchScope: SetSearchScope::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<IUpdateSearcher2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateService_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ContentValidationCert(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ExpirationDate(&self) -> windows_core::Result<f64>;
    fn IsManaged(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsRegisteredWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IssueDate(&self) -> windows_core::Result<f64>;
    fn OffersWindowsUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectUrls(&self) -> windows_core::Result<IStringCollection>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsScanPackageService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CanRegisterWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateService {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateService_Vtbl
    where
        Identity: IUpdateService_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::Name(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentValidationCert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::ContentValidationCert(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::ExpirationDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManaged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::IsManaged(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::IsRegisteredWithAU(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::IssueDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::OffersWindowsUpdates(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectUrls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::RedirectUrls(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::ServiceID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanPackageService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::IsScanPackageService(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::CanRegisterWithAU(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::ServiceUrl(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupPrefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService_Impl::SetupPrefix(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            ContentValidationCert: ContentValidationCert::<Identity, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, OFFSET>,
            IsManaged: IsManaged::<Identity, OFFSET>,
            IsRegisteredWithAU: IsRegisteredWithAU::<Identity, OFFSET>,
            IssueDate: IssueDate::<Identity, OFFSET>,
            OffersWindowsUpdates: OffersWindowsUpdates::<Identity, OFFSET>,
            RedirectUrls: RedirectUrls::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            IsScanPackageService: IsScanPackageService::<Identity, OFFSET>,
            CanRegisterWithAU: CanRegisterWithAU::<Identity, OFFSET>,
            ServiceUrl: ServiceUrl::<Identity, OFFSET>,
            SetupPrefix: SetupPrefix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateService2_Impl: Sized + IUpdateService_Impl {
    fn IsDefaultAUService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateService2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateService2_Vtbl
    where
        Identity: IUpdateService2_Impl,
    {
        unsafe extern "system" fn IsDefaultAUService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateService2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateService2_Impl::IsDefaultAUService(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateService_Vtbl::new::<Identity, OFFSET>(), IsDefaultAUService: IsDefaultAUService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateService>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateServiceCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateServiceCollection_Vtbl
    where
        Identity: IUpdateServiceCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Services(&self) -> windows_core::Result<IUpdateServiceCollection>;
    fn AddService(&self, serviceid: &windows_core::BSTR, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveService(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnregisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddScanPackageService(&self, servicename: &windows_core::BSTR, scanfilelocation: &windows_core::BSTR, flags: i32) -> windows_core::Result<IUpdateService>;
    fn SetOption(&self, optionname: &windows_core::BSTR, optionvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateServiceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateServiceManager_Vtbl
    where
        Identity: IUpdateServiceManager_Impl,
    {
        unsafe extern "system" fn Services<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager_Impl::Services(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>, authorizationcabpath: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager_Impl::AddService(this, core::mem::transmute(&serviceid), core::mem::transmute(&authorizationcabpath)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateServiceManager_Impl::RegisterServiceWithAU(this, core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn RemoveService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateServiceManager_Impl::RemoveService(this, core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateServiceManager_Impl::UnregisterServiceWithAU(this, core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn AddScanPackageService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>, scanfilelocation: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager_Impl::AddScanPackageService(this, core::mem::transmute(&servicename), core::mem::transmute(&scanfilelocation), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionname: core::mem::MaybeUninit<windows_core::BSTR>, optionvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateServiceManager_Impl::SetOption(this, core::mem::transmute(&optionname), core::mem::transmute(&optionvalue)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Services: Services::<Identity, OFFSET>,
            AddService: AddService::<Identity, OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Identity, OFFSET>,
            RemoveService: RemoveService::<Identity, OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Identity, OFFSET>,
            AddScanPackageService: AddScanPackageService::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManager2_Impl: Sized + IUpdateServiceManager_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryServiceRegistration(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
    fn AddService2(&self, serviceid: &windows_core::BSTR, flags: i32, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateServiceManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateServiceManager2_Vtbl
    where
        Identity: IUpdateServiceManager2_Impl,
    {
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager2_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateServiceManager2_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn QueryServiceRegistration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager2_Impl::QueryServiceRegistration(this, core::mem::transmute(&serviceid)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, authorizationcabpath: core::mem::MaybeUninit<windows_core::BSTR>, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceManager2_Impl::AddService2(this, core::mem::transmute(&serviceid), core::mem::transmute_copy(&flags), core::mem::transmute(&authorizationcabpath)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdateServiceManager_Vtbl::new::<Identity, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, OFFSET>,
            AddService2: AddService2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateServiceManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceRegistration_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Service(&self) -> windows_core::Result<IUpdateService2>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateServiceRegistration {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateServiceRegistration_Vtbl
    where
        Identity: IUpdateServiceRegistration_Impl,
    {
        unsafe extern "system" fn RegistrationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceRegistration_Impl::RegistrationState(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceRegistration_Impl::ServiceID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceRegistration_Impl::IsPendingRegistrationWithAU(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateServiceRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateServiceRegistration_Impl::Service(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RegistrationState: RegistrationState::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Identity, OFFSET>,
            Service: Service::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WebProxy(&self) -> windows_core::Result<IWebProxy>;
    fn SetWebProxy(&self, value: Option<&IWebProxy>) -> windows_core::Result<()>;
    fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSession {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSession_Vtbl
    where
        Identity: IUpdateSession_Impl,
    {
        unsafe extern "system" fn ClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::ClientApplicationID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSession_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::WebProxy(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSession_Impl::SetWebProxy(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn CreateUpdateSearcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::CreateUpdateSearcher(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::CreateUpdateDownloader(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession_Impl::CreateUpdateInstaller(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            WebProxy: WebProxy::<Identity, OFFSET>,
            SetWebProxy: SetWebProxy::<Identity, OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Identity, OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Identity, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession2_Impl: Sized + IUpdateSession_Impl {
    fn UserLocale(&self) -> windows_core::Result<u32>;
    fn SetUserLocale(&self, lcid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSession2 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSession2_Vtbl
    where
        Identity: IUpdateSession2_Impl,
    {
        unsafe extern "system" fn UserLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUpdateSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession2_Impl::UserLocale(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: IUpdateSession2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUpdateSession2_Impl::SetUserLocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        Self {
            base__: IUpdateSession_Vtbl::new::<Identity, OFFSET>(),
            UserLocale: UserLocale::<Identity, OFFSET>,
            SetUserLocale: SetUserLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession3_Impl: Sized + IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&self) -> windows_core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&self, criteria: &windows_core::BSTR, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUpdateSession3 {}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUpdateSession3_Vtbl
    where
        Identity: IUpdateSession3_Impl,
    {
        unsafe extern "system" fn CreateUpdateServiceManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession3_Impl::CreateUpdateServiceManager(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: core::mem::MaybeUninit<windows_core::BSTR>, startindex: i32, count: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUpdateSession3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUpdateSession3_Impl::QueryHistory(this, core::mem::transmute(&criteria), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdateSession2_Vtbl::new::<Identity, OFFSET>(),
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Identity, OFFSET>,
            QueryHistory: QueryHistory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSession as windows_core::Interface>::IID || iid == &<IUpdateSession2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebProxy_Impl: Sized + super::Com::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAddress(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BypassList(&self) -> windows_core::Result<IStringCollection>;
    fn SetBypassList(&self, value: Option<&IStringCollection>) -> windows_core::Result<()>;
    fn BypassProxyOnLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBypassProxyOnLocal(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUserName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentials(&self, parentwindow: Option<&windows_core::IUnknown>, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentialsFromHwnd(&self, parentwindow: super::super::Foundation::HWND, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AutoDetect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoDetect(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWebProxy {}
#[cfg(feature = "Win32_System_Com")]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWebProxy_Vtbl
    where
        Identity: IWebProxy_Impl,
    {
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::Address(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetAddress(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BypassList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::BypassList(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetBypassList(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BypassProxyOnLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::BypassProxyOnLocal(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetBypassProxyOnLocal(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::UserName(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetUserName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetPassword(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PromptForCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwindow: *mut core::ffi::c_void, title: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::PromptForCredentials(this, windows_core::from_raw_borrowed(&parentwindow), core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::PromptForCredentialsFromHwnd(this, core::mem::transmute_copy(&parentwindow), core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn AutoDetect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebProxy_Impl::AutoDetect(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWebProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebProxy_Impl::SetAutoDetect(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            SetAddress: SetAddress::<Identity, OFFSET>,
            BypassList: BypassList::<Identity, OFFSET>,
            SetBypassList: SetBypassList::<Identity, OFFSET>,
            BypassProxyOnLocal: BypassProxyOnLocal::<Identity, OFFSET>,
            SetBypassProxyOnLocal: SetBypassProxyOnLocal::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            UserName: UserName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            PromptForCredentials: PromptForCredentials::<Identity, OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Identity, OFFSET>,
            AutoDetect: AutoDetect::<Identity, OFFSET>,
            SetAutoDetect: SetAutoDetect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebProxy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate_Impl: Sized + IUpdate_Impl {
    fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverVerDate(&self) -> windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> windows_core::Result<i32>;
    fn DeviceStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdate {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdate_Vtbl
    where
        Identity: IWindowsDriverUpdate_Impl,
    {
        unsafe extern "system" fn DriverClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverClass(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverHardwareID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverManufacturer(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverModel(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverProvider(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DriverVerDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DeviceProblemNumber(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate_Impl::DeviceStatus(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, OFFSET>(),
            DriverClass: DriverClass::<Identity, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, OFFSET>,
            DriverModel: DriverModel::<Identity, OFFSET>,
            DriverProvider: DriverProvider::<Identity, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate2_Impl: Sized + IWindowsDriverUpdate_Impl {
    fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: Option<&IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdate2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdate2_Vtbl
    where
        Identity: IWindowsDriverUpdate2_Impl,
    {
        unsafe extern "system" fn RebootRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate2_Impl::RebootRequired(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate2_Impl::IsPresent(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate2_Impl::CveIDs(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiles: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsDriverUpdate2_Impl::CopyToCache(this, windows_core::from_raw_borrowed(&pfiles)).into()
        }
        Self {
            base__: IWindowsDriverUpdate_Vtbl::new::<Identity, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            IsPresent: IsPresent::<Identity, OFFSET>,
            CveIDs: CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate3_Impl: Sized + IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdate3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdate3_Vtbl
    where
        Identity: IWindowsDriverUpdate3_Impl,
    {
        unsafe extern "system" fn BrowseOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate3_Impl::BrowseOnly(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWindowsDriverUpdate2_Vtbl::new::<Identity, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate4_Impl: Sized + IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdate4 {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdate4_Vtbl
    where
        Identity: IWindowsDriverUpdate4_Impl,
    {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate4_Impl::WindowsDriverUpdateEntries(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate4_Impl::PerUser(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWindowsDriverUpdate3_Vtbl::new::<Identity, OFFSET>(),
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Identity, OFFSET>,
            PerUser: PerUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate5_Impl: Sized + IWindowsDriverUpdate4_Impl {
    fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdate5 {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdate5_Vtbl
    where
        Identity: IWindowsDriverUpdate5_Impl,
    {
        unsafe extern "system" fn AutoSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate5_Impl::AutoSelection(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdate5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdate5_Impl::AutoDownload(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWindowsDriverUpdate4_Vtbl::new::<Identity, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, OFFSET>,
            AutoDownload: AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverVerDate(&self) -> windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> windows_core::Result<i32>;
    fn DeviceStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdateEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdateEntry_Vtbl
    where
        Identity: IWindowsDriverUpdateEntry_Impl,
    {
        unsafe extern "system" fn DriverClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverClass(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverHardwareID(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverManufacturer(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverModel(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverProvider(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DriverVerDate(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DeviceProblemNumber(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntry_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntry_Impl::DeviceStatus(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DriverClass: DriverClass::<Identity, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, OFFSET>,
            DriverModel: DriverModel::<Identity, OFFSET>,
            DriverProvider: DriverProvider::<Identity, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsDriverUpdateEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsDriverUpdateEntryCollection_Vtbl
    where
        Identity: IWindowsDriverUpdateEntryCollection_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntryCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsDriverUpdateEntryCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsDriverUpdateEntryCollection_Impl::Count(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsUpdateAgentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetInfo(&self, varinfoidentifier: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsUpdateAgentInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsUpdateAgentInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsUpdateAgentInfo_Vtbl
    where
        Identity: IWindowsUpdateAgentInfo_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinfoidentifier: core::mem::MaybeUninit<windows_core::VARIANT>, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWindowsUpdateAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsUpdateAgentInfo_Impl::GetInfo(this, core::mem::transmute(&varinfoidentifier)) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetInfo: GetInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsUpdateAgentInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
