#[cfg(feature = "System")]
pub trait IGameController_Impl: Sized {
    fn HeadsetConnected(&self, value: Option<&super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetConnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn HeadsetDisconnected(&self, value: Option<&super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn UserChanged(&self, value: Option<&super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn Headset(&self) -> windows_core::Result<Headset>;
    fn IsWireless(&self) -> windows_core::Result<bool>;
    fn User(&self) -> windows_core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IGameController {
    const NAME: &'static str = "Windows.Gaming.Input.IGameController";
}
#[cfg(feature = "System")]
impl IGameController_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameController_Vtbl
    where
        Identity: IGameController_Impl,
    {
        unsafe extern "system" fn HeadsetConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::HeadsetConnected(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameController_Impl::RemoveHeadsetConnected(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn HeadsetDisconnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::HeadsetDisconnected(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetDisconnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameController_Impl::RemoveHeadsetDisconnected(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UserChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::UserChanged(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGameController_Impl::RemoveUserChanged(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Headset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::Headset(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWireless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::IsWireless(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameController_Impl::User(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameController, OFFSET>(),
            HeadsetConnected: HeadsetConnected::<Identity, OFFSET>,
            RemoveHeadsetConnected: RemoveHeadsetConnected::<Identity, OFFSET>,
            HeadsetDisconnected: HeadsetDisconnected::<Identity, OFFSET>,
            RemoveHeadsetDisconnected: RemoveHeadsetDisconnected::<Identity, OFFSET>,
            UserChanged: UserChanged::<Identity, OFFSET>,
            RemoveUserChanged: RemoveUserChanged::<Identity, OFFSET>,
            Headset: Headset::<Identity, OFFSET>,
            IsWireless: IsWireless::<Identity, OFFSET>,
            User: User::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameController as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Power")]
pub trait IGameControllerBatteryInfo_Impl: Sized {
    fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "Devices_Power")]
impl windows_core::RuntimeName for IGameControllerBatteryInfo {
    const NAME: &'static str = "Windows.Gaming.Input.IGameControllerBatteryInfo";
}
#[cfg(feature = "Devices_Power")]
impl IGameControllerBatteryInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGameControllerBatteryInfo_Vtbl
    where
        Identity: IGameControllerBatteryInfo_Impl,
    {
        unsafe extern "system" fn TryGetBatteryReport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGameControllerBatteryInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGameControllerBatteryInfo_Impl::TryGetBatteryReport(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameControllerBatteryInfo, OFFSET>(),
            TryGetBatteryReport: TryGetBatteryReport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameControllerBatteryInfo as windows_core::Interface>::IID
    }
}
