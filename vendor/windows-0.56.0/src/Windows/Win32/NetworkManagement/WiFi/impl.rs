pub trait IDot11AdHocInterface_Impl: Sized {
    fn GetDeviceSignature(&self, psignature: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsDot11d(&self, pf11d: *mut u8) -> windows_core::Result<()>;
    fn IsAdHocCapable(&self, pfadhoccapable: *mut u8) -> windows_core::Result<()>;
    fn IsRadioOn(&self, pfisradioon: *mut u8) -> windows_core::Result<()>;
    fn GetActiveNetwork(&self) -> windows_core::Result<IDot11AdHocNetwork>;
    fn GetIEnumSecuritySettings(&self) -> windows_core::Result<IEnumDot11AdHocSecuritySettings>;
    fn GetIEnumDot11AdHocNetworks(&self, pfilterguid: *const windows_core::GUID) -> windows_core::Result<IEnumDot11AdHocNetworks>;
    fn GetStatus(&self, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocInterface {}
impl IDot11AdHocInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>() -> IDot11AdHocInterface_Vtbl {
        unsafe extern "system" fn GetDeviceSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignature: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterface_Impl::GetDeviceSignature(this, core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocInterface_Impl::GetFriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDot11d<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pf11d: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterface_Impl::IsDot11d(this, core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn IsAdHocCapable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfadhoccapable: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterface_Impl::IsAdHocCapable(this, core::mem::transmute_copy(&pfadhoccapable)).into()
        }
        unsafe extern "system" fn IsRadioOn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisradioon: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterface_Impl::IsRadioOn(this, core::mem::transmute_copy(&pfisradioon)).into()
        }
        unsafe extern "system" fn GetActiveNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocInterface_Impl::GetActiveNetwork(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumSecuritySettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocInterface_Impl::GetIEnumSecuritySettings(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilterguid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocInterface_Impl::GetIEnumDot11AdHocNetworks(this, core::mem::transmute_copy(&pfilterguid)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterface_Impl::GetStatus(this, core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceSignature: GetDeviceSignature::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            IsDot11d: IsDot11d::<Identity, Impl, OFFSET>,
            IsAdHocCapable: IsAdHocCapable::<Identity, Impl, OFFSET>,
            IsRadioOn: IsRadioOn::<Identity, Impl, OFFSET>,
            GetActiveNetwork: GetActiveNetwork::<Identity, Impl, OFFSET>,
            GetIEnumSecuritySettings: GetIEnumSecuritySettings::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocInterface as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocInterfaceNotificationSink_Impl: Sized {
    fn OnConnectionStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocInterfaceNotificationSink {}
impl IDot11AdHocInterfaceNotificationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocInterfaceNotificationSink_Vtbl {
        unsafe extern "system" fn OnConnectionStatusChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocInterfaceNotificationSink_Impl::OnConnectionStatusChange(this, core::mem::transmute_copy(&estatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnectionStatusChange: OnConnectionStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocInterfaceNotificationSink as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocManager_Impl: Sized {
    fn CreateNetwork(&self, name: &windows_core::PCWSTR, password: &windows_core::PCWSTR, geographicalid: i32, pinterface: Option<&IDot11AdHocInterface>, psecurity: Option<&IDot11AdHocSecuritySettings>, pcontextguid: *const windows_core::GUID) -> windows_core::Result<IDot11AdHocNetwork>;
    fn CommitCreatedNetwork(&self, piadhoc: Option<&IDot11AdHocNetwork>, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn GetIEnumDot11AdHocNetworks(&self, pcontextguid: *const windows_core::GUID) -> windows_core::Result<IEnumDot11AdHocNetworks>;
    fn GetIEnumDot11AdHocInterfaces(&self) -> windows_core::Result<IEnumDot11AdHocInterfaces>;
    fn GetNetwork(&self, networksignature: *const windows_core::GUID) -> windows_core::Result<IDot11AdHocNetwork>;
}
impl windows_core::RuntimeName for IDot11AdHocManager {}
impl IDot11AdHocManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>() -> IDot11AdHocManager_Vtbl {
        unsafe extern "system" fn CreateNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, password: windows_core::PCWSTR, geographicalid: i32, pinterface: *mut core::ffi::c_void, psecurity: *mut core::ffi::c_void, pcontextguid: *const windows_core::GUID, piadhoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocManager_Impl::CreateNetwork(this, core::mem::transmute(&name), core::mem::transmute(&password), core::mem::transmute_copy(&geographicalid), windows_core::from_raw_borrowed(&pinterface), windows_core::from_raw_borrowed(&psecurity), core::mem::transmute_copy(&pcontextguid)) {
                Ok(ok__) => {
                    core::ptr::write(piadhoc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitCreatedNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piadhoc: *mut core::ffi::c_void, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocManager_Impl::CommitCreatedNetwork(this, windows_core::from_raw_borrowed(&piadhoc), core::mem::transmute_copy(&fsaveprofile), core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontextguid: *const windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocManager_Impl::GetIEnumDot11AdHocNetworks(this, core::mem::transmute_copy(&pcontextguid)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocManager_Impl::GetIEnumDot11AdHocInterfaces(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networksignature: *const windows_core::GUID, pnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocManager_Impl::GetNetwork(this, core::mem::transmute_copy(&networksignature)) {
                Ok(ok__) => {
                    core::ptr::write(pnetwork, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateNetwork: CreateNetwork::<Identity, Impl, OFFSET>,
            CommitCreatedNetwork: CommitCreatedNetwork::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocInterfaces: GetIEnumDot11AdHocInterfaces::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocManager as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocManagerNotificationSink_Impl: Sized {
    fn OnNetworkAdd(&self, piadhocnetwork: Option<&IDot11AdHocNetwork>) -> windows_core::Result<()>;
    fn OnNetworkRemove(&self, signature: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnInterfaceAdd(&self, piadhocinterface: Option<&IDot11AdHocInterface>) -> windows_core::Result<()>;
    fn OnInterfaceRemove(&self, signature: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocManagerNotificationSink {}
impl IDot11AdHocManagerNotificationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocManagerNotificationSink_Vtbl {
        unsafe extern "system" fn OnNetworkAdd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piadhocnetwork: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocManagerNotificationSink_Impl::OnNetworkAdd(this, windows_core::from_raw_borrowed(&piadhocnetwork)).into()
        }
        unsafe extern "system" fn OnNetworkRemove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocManagerNotificationSink_Impl::OnNetworkRemove(this, core::mem::transmute_copy(&signature)).into()
        }
        unsafe extern "system" fn OnInterfaceAdd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piadhocinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocManagerNotificationSink_Impl::OnInterfaceAdd(this, windows_core::from_raw_borrowed(&piadhocinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocManagerNotificationSink_Impl::OnInterfaceRemove(this, core::mem::transmute_copy(&signature)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNetworkAdd: OnNetworkAdd::<Identity, Impl, OFFSET>,
            OnNetworkRemove: OnNetworkRemove::<Identity, Impl, OFFSET>,
            OnInterfaceAdd: OnInterfaceAdd::<Identity, Impl, OFFSET>,
            OnInterfaceRemove: OnInterfaceRemove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocManagerNotificationSink as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocNetwork_Impl: Sized {
    fn GetStatus(&self, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::Result<()>;
    fn GetSSID(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn HasProfile(&self, pf11d: *mut u8) -> windows_core::Result<()>;
    fn GetProfileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn DeleteProfile(&self) -> windows_core::Result<()>;
    fn GetSignalQuality(&self, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> windows_core::Result<()>;
    fn GetSecuritySetting(&self) -> windows_core::Result<IDot11AdHocSecuritySettings>;
    fn GetContextGuid(&self, pcontextguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetSignature(&self, psignature: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetInterface(&self) -> windows_core::Result<IDot11AdHocInterface>;
    fn Connect(&self, passphrase: &windows_core::PCWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocNetwork {}
impl IDot11AdHocNetwork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>() -> IDot11AdHocNetwork_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::GetStatus(this, core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn GetSSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwssid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocNetwork_Impl::GetSSID(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszwssid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pf11d: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::HasProfile(this, core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn GetProfileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszwprofilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocNetwork_Impl::GetProfileName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppszwprofilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::DeleteProfile(this).into()
        }
        unsafe extern "system" fn GetSignalQuality<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::GetSignalQuality(this, core::mem::transmute_copy(&pustrengthvalue), core::mem::transmute_copy(&pustrengthmax)).into()
        }
        unsafe extern "system" fn GetSecuritySetting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padhocsecuritysetting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocNetwork_Impl::GetSecuritySetting(this) {
                Ok(ok__) => {
                    core::ptr::write(padhocsecuritysetting, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontextguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::GetContextGuid(this, core::mem::transmute_copy(&pcontextguid)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignature: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::GetSignature(this, core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padhocinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDot11AdHocNetwork_Impl::GetInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(padhocinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, passphrase: windows_core::PCWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::Connect(this, core::mem::transmute(&passphrase), core::mem::transmute_copy(&geographicalid), core::mem::transmute_copy(&fsaveprofile), core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetwork_Impl::Disconnect(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetSSID: GetSSID::<Identity, Impl, OFFSET>,
            HasProfile: HasProfile::<Identity, Impl, OFFSET>,
            GetProfileName: GetProfileName::<Identity, Impl, OFFSET>,
            DeleteProfile: DeleteProfile::<Identity, Impl, OFFSET>,
            GetSignalQuality: GetSignalQuality::<Identity, Impl, OFFSET>,
            GetSecuritySetting: GetSecuritySetting::<Identity, Impl, OFFSET>,
            GetContextGuid: GetContextGuid::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocNetwork as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocNetworkNotificationSink_Impl: Sized {
    fn OnStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::Result<()>;
    fn OnConnectFail(&self, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocNetworkNotificationSink {}
impl IDot11AdHocNetworkNotificationSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocNetworkNotificationSink_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetworkNotificationSink_Impl::OnStatusChange(this, core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn OnConnectFail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocNetworkNotificationSink_Impl::OnConnectFail(this, core::mem::transmute_copy(&efailreason)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnConnectFail: OnConnectFail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocNetworkNotificationSink as windows_core::Interface>::IID
    }
}
pub trait IDot11AdHocSecuritySettings_Impl: Sized {
    fn GetDot11AuthAlgorithm(&self, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> windows_core::Result<()>;
    fn GetDot11CipherAlgorithm(&self, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDot11AdHocSecuritySettings {}
impl IDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn GetDot11AuthAlgorithm<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocSecuritySettings_Impl::GetDot11AuthAlgorithm(this, core::mem::transmute_copy(&pauth)).into()
        }
        unsafe extern "system" fn GetDot11CipherAlgorithm<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDot11AdHocSecuritySettings_Impl::GetDot11CipherAlgorithm(this, core::mem::transmute_copy(&pcipher)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDot11AuthAlgorithm: GetDot11AuthAlgorithm::<Identity, Impl, OFFSET>,
            GetDot11CipherAlgorithm: GetDot11CipherAlgorithm::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDot11AdHocSecuritySettings as windows_core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocInterfaces_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IDot11AdHocInterface>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDot11AdHocInterfaces>;
}
impl windows_core::RuntimeName for IEnumDot11AdHocInterfaces {}
impl IEnumDot11AdHocInterfaces_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>() -> IEnumDot11AdHocInterfaces_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocInterfaces_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocInterfaces_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocInterfaces_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDot11AdHocInterfaces_Impl::Clone(this) {
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
        iid == &<IEnumDot11AdHocInterfaces as windows_core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocNetworks_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IDot11AdHocNetwork>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDot11AdHocNetworks>;
}
impl windows_core::RuntimeName for IEnumDot11AdHocNetworks {}
impl IEnumDot11AdHocNetworks_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>() -> IEnumDot11AdHocNetworks_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocNetworks_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocNetworks_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocNetworks_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDot11AdHocNetworks_Impl::Clone(this) {
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
        iid == &<IEnumDot11AdHocNetworks as windows_core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocSecuritySettings_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<IDot11AdHocSecuritySettings>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDot11AdHocSecuritySettings>;
}
impl windows_core::RuntimeName for IEnumDot11AdHocSecuritySettings {}
impl IEnumDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IEnumDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocSecuritySettings_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocSecuritySettings_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDot11AdHocSecuritySettings_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDot11AdHocSecuritySettings_Impl::Clone(this) {
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
        iid == &<IEnumDot11AdHocSecuritySettings as windows_core::Interface>::IID
    }
}
