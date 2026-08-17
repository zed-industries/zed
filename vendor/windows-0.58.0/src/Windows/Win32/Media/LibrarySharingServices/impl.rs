#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeviceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Authorization(&self) -> windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus>;
    fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDevice {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevice_Vtbl
    where
        Identity: IWindowsMediaLibrarySharingDevice_Impl,
    {
        unsafe extern "system" fn DeviceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevice_Impl::DeviceID(this) {
                Ok(ok__) => {
                    deviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authorization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevice_Impl::Authorization(this) {
                Ok(ok__) => {
                    authorization.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingDevice_Impl::SetAuthorization(this, core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevice_Impl::Properties(this) {
                Ok(ok__) => {
                    deviceproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DeviceID: DeviceID::<Identity, OFFSET>,
            Authorization: Authorization::<Identity, OFFSET>,
            SetAuthorization: SetAuthorization::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevice as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDeviceProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn GetProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperties {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperties_Vtbl
    where
        Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDeviceProperties_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    property.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDeviceProperties_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDeviceProperties_Impl::GetProperty(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    property.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDeviceProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperty {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperty_Vtbl
    where
        Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDeviceProperty_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDeviceProperty_Impl::Value(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Name: Name::<Identity, OFFSET>, Value: Value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDevice>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn GetDevice(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDevice>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDevices {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevices_Vtbl
    where
        Identity: IWindowsMediaLibrarySharingDevices_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevices_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    device.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevices_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: core::mem::MaybeUninit<windows_core::BSTR>, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingDevices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingDevices_Impl::GetDevice(this, core::mem::transmute(&deviceid)) {
                Ok(ok__) => {
                    device.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn showShareMediaCPL(&self, device: &windows_core::BSTR) -> windows_core::Result<()>;
    fn userHomeMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserHomeMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn userHomeMediaSharingLibraryName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetuserHomeMediaSharingLibraryName(&self, libraryname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn computerHomeMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn userInternetMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserInternetMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn computerInternetMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn internetMediaSharingSecurityGroup(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetinternetMediaSharingSecurityGroup(&self, securitygroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn allowSharingToAllDevices(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetallowSharingToAllDevices(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setDefaultAuthorization(&self, macaddresses: &windows_core::BSTR, friendlyname: &windows_core::BSTR, authorization: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setAuthorizationState(&self, macaddress: &windows_core::BSTR, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAllDevices(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDevices>;
    fn customSettingsApplied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingServices {}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingServices_Vtbl
    where
        Identity: IWindowsMediaLibrarySharingServices_Impl,
    {
        unsafe extern "system" fn showShareMediaCPL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::showShareMediaCPL(this, core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::userHomeMediaSharingState(this) {
                Ok(ok__) => {
                    sharingenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetuserHomeMediaSharingState(this, core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, libraryname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::userHomeMediaSharingLibraryName(this) {
                Ok(ok__) => {
                    libraryname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, libraryname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetuserHomeMediaSharingLibraryName(this, core::mem::transmute(&libraryname)).into()
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::computerHomeMediaSharingAllowedState(this) {
                Ok(ok__) => {
                    sharingallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetcomputerHomeMediaSharingAllowedState(this, core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn userInternetMediaSharingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::userInternetMediaSharingState(this) {
                Ok(ok__) => {
                    sharingenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetuserInternetMediaSharingState(this, core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::computerInternetMediaSharingAllowedState(this) {
                Ok(ok__) => {
                    sharingallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetcomputerInternetMediaSharingAllowedState(this, core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, securitygroup: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::internetMediaSharingSecurityGroup(this) {
                Ok(ok__) => {
                    securitygroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, securitygroup: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetinternetMediaSharingSecurityGroup(this, core::mem::transmute(&securitygroup)).into()
        }
        unsafe extern "system" fn allowSharingToAllDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::allowSharingToAllDevices(this) {
                Ok(ok__) => {
                    sharingenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::SetallowSharingToAllDevices(this, core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn setDefaultAuthorization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, macaddresses: core::mem::MaybeUninit<windows_core::BSTR>, friendlyname: core::mem::MaybeUninit<windows_core::BSTR>, authorization: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::setDefaultAuthorization(this, core::mem::transmute(&macaddresses), core::mem::transmute(&friendlyname), core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn setAuthorizationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, macaddress: core::mem::MaybeUninit<windows_core::BSTR>, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWindowsMediaLibrarySharingServices_Impl::setAuthorizationState(this, core::mem::transmute(&macaddress), core::mem::transmute_copy(&authorizationstate)).into()
        }
        unsafe extern "system" fn getAllDevices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::getAllDevices(this) {
                Ok(ok__) => {
                    devices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customSettingsApplied<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customsettingsapplied: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWindowsMediaLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWindowsMediaLibrarySharingServices_Impl::customSettingsApplied(this) {
                Ok(ok__) => {
                    customsettingsapplied.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            showShareMediaCPL: showShareMediaCPL::<Identity, OFFSET>,
            userHomeMediaSharingState: userHomeMediaSharingState::<Identity, OFFSET>,
            SetuserHomeMediaSharingState: SetuserHomeMediaSharingState::<Identity, OFFSET>,
            userHomeMediaSharingLibraryName: userHomeMediaSharingLibraryName::<Identity, OFFSET>,
            SetuserHomeMediaSharingLibraryName: SetuserHomeMediaSharingLibraryName::<Identity, OFFSET>,
            computerHomeMediaSharingAllowedState: computerHomeMediaSharingAllowedState::<Identity, OFFSET>,
            SetcomputerHomeMediaSharingAllowedState: SetcomputerHomeMediaSharingAllowedState::<Identity, OFFSET>,
            userInternetMediaSharingState: userInternetMediaSharingState::<Identity, OFFSET>,
            SetuserInternetMediaSharingState: SetuserInternetMediaSharingState::<Identity, OFFSET>,
            computerInternetMediaSharingAllowedState: computerInternetMediaSharingAllowedState::<Identity, OFFSET>,
            SetcomputerInternetMediaSharingAllowedState: SetcomputerInternetMediaSharingAllowedState::<Identity, OFFSET>,
            internetMediaSharingSecurityGroup: internetMediaSharingSecurityGroup::<Identity, OFFSET>,
            SetinternetMediaSharingSecurityGroup: SetinternetMediaSharingSecurityGroup::<Identity, OFFSET>,
            allowSharingToAllDevices: allowSharingToAllDevices::<Identity, OFFSET>,
            SetallowSharingToAllDevices: SetallowSharingToAllDevices::<Identity, OFFSET>,
            setDefaultAuthorization: setDefaultAuthorization::<Identity, OFFSET>,
            setAuthorizationState: setAuthorizationState::<Identity, OFFSET>,
            getAllDevices: getAllDevices::<Identity, OFFSET>,
            customSettingsApplied: customSettingsApplied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingServices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
