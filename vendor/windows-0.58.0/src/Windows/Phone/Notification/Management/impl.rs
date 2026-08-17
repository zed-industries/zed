pub trait IAccessoryNotificationTriggerDetails_Impl: Sized {
    fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn AppId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&self) -> windows_core::Result<bool>;
    fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
impl IAccessoryNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccessoryNotificationTriggerDetails_Vtbl
    where
        Identity: IAccessoryNotificationTriggerDetails_Impl,
    {
        unsafe extern "system" fn TimeCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccessoryNotificationTriggerDetails_Impl::TimeCreated(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccessoryNotificationTriggerDetails_Impl::AppDisplayName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccessoryNotificationTriggerDetails_Impl::AppId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessoryNotificationType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AccessoryNotificationType) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccessoryNotificationTriggerDetails_Impl::AccessoryNotificationType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartedProcessing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAccessoryNotificationTriggerDetails_Impl::StartedProcessing(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartedProcessing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IAccessoryNotificationTriggerDetails_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccessoryNotificationTriggerDetails_Impl::SetStartedProcessing(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessoryNotificationTriggerDetails, OFFSET>(),
            TimeCreated: TimeCreated::<Identity, OFFSET>,
            AppDisplayName: AppDisplayName::<Identity, OFFSET>,
            AppId: AppId::<Identity, OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Identity, OFFSET>,
            StartedProcessing: StartedProcessing::<Identity, OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessoryNotificationTriggerDetails as windows_core::Interface>::IID
    }
}
