#[cfg(feature = "Win32_System_Com")]
pub trait IDummyMBNUCMExt_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDummyMBNUCMExt {}
#[cfg(feature = "Win32_System_Com")]
impl IDummyMBNUCMExt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDummyMBNUCMExt_Impl, const OFFSET: isize>() -> IDummyMBNUCMExt_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDummyMBNUCMExt as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IMbnConnection_Impl: Sized {
    fn ConnectionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Connect(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Disconnect(&self) -> windows_core::Result<u32>;
    fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetVoiceCallState(&self) -> windows_core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMbnConnection {}
impl IMbnConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>() -> IMbnConnection_Vtbl {
        unsafe extern "system" fn ConnectionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::ConnectionID(this) {
                Ok(ok__) => {
                    core::ptr::write(connectionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(interfaceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::Connect(this, core::mem::transmute_copy(&connectionmode), core::mem::transmute(&strprofile)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::Disconnect(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnection_Impl::GetConnectionState(this, core::mem::transmute_copy(&connectionstate), core::mem::transmute_copy(&profilename)).into()
        }
        unsafe extern "system" fn GetVoiceCallState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::GetVoiceCallState(this) {
                Ok(ok__) => {
                    core::ptr::write(voicecallstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnection_Impl::GetActivationNetworkError(this) {
                Ok(ok__) => {
                    core::ptr::write(networkerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionID: ConnectionID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetConnectionState: GetConnectionState::<Identity, Impl, OFFSET>,
            GetVoiceCallState: GetVoiceCallState::<Identity, Impl, OFFSET>,
            GetActivationNetworkError: GetActivationNetworkError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionContext_Impl: Sized {
    fn GetProvisionedContexts(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(&self, provisionedcontexts: &MBN_CONTEXT, providerid: &windows_core::PCWSTR) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionContext {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>() -> IMbnConnectionContext_Vtbl {
        unsafe extern "system" fn GetProvisionedContexts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionContext_Impl::GetProvisionedContexts(this) {
                Ok(ok__) => {
                    core::ptr::write(provisionedcontexts, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provisionedcontexts: MBN_CONTEXT, providerid: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionContext_Impl::SetProvisionedContext(this, core::mem::transmute(&provisionedcontexts), core::mem::transmute(&providerid)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProvisionedContexts: GetProvisionedContexts::<Identity, Impl, OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionContext as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionContextEvents_Impl: Sized {
    fn OnProvisionedContextListChange(&self, newinterface: Option<&IMbnConnectionContext>) -> windows_core::Result<()>;
    fn OnSetProvisionedContextComplete(&self, newinterface: Option<&IMbnConnectionContext>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionContextEvents {}
impl IMbnConnectionContextEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>() -> IMbnConnectionContextEvents_Vtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionContextEvents_Impl::OnProvisionedContextListChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionContextEvents_Impl::OnSetProvisionedContextComplete(this, windows_core::from_raw_borrowed(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Identity, Impl, OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionContextEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionEvents_Impl: Sized {
    fn OnConnectComplete(&self, newconnection: Option<&IMbnConnection>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnDisconnectComplete(&self, newconnection: Option<&IMbnConnection>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnConnectStateChange(&self, newconnection: Option<&IMbnConnection>) -> windows_core::Result<()>;
    fn OnVoiceCallStateChange(&self, newconnection: Option<&IMbnConnection>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionEvents {}
impl IMbnConnectionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>() -> IMbnConnectionEvents_Vtbl {
        unsafe extern "system" fn OnConnectComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionEvents_Impl::OnConnectComplete(this, windows_core::from_raw_borrowed(&newconnection), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisconnectComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionEvents_Impl::OnDisconnectComplete(this, windows_core::from_raw_borrowed(&newconnection), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnConnectStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionEvents_Impl::OnConnectStateChange(this, windows_core::from_raw_borrowed(&newconnection)).into()
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionEvents_Impl::OnVoiceCallStateChange(this, windows_core::from_raw_borrowed(&newconnection)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectComplete: OnConnectComplete::<Identity, Impl, OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Identity, Impl, OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Identity, Impl, OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionManager_Impl: Sized {
    fn GetConnection(&self, connectionid: &windows_core::PCWSTR) -> windows_core::Result<IMbnConnection>;
    fn GetConnections(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>() -> IMbnConnectionManager_Vtbl {
        unsafe extern "system" fn GetConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::PCWSTR, mbnconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionManager_Impl::GetConnection(this, core::mem::transmute(&connectionid)) {
                Ok(ok__) => {
                    core::ptr::write(mbnconnection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionManager_Impl::GetConnections(this) {
                Ok(ok__) => {
                    core::ptr::write(mbnconnections, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
            GetConnections: GetConnections::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionManager as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionManagerEvents_Impl: Sized {
    fn OnConnectionArrival(&self, newconnection: Option<&IMbnConnection>) -> windows_core::Result<()>;
    fn OnConnectionRemoval(&self, oldconnection: Option<&IMbnConnection>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionManagerEvents {}
impl IMbnConnectionManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionArrival<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionManagerEvents_Impl::OnConnectionArrival(this, windows_core::from_raw_borrowed(&newconnection)).into()
        }
        unsafe extern "system" fn OnConnectionRemoval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionManagerEvents_Impl::OnConnectionRemoval(this, windows_core::from_raw_borrowed(&oldconnection)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionArrival: OnConnectionArrival::<Identity, Impl, OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionManagerEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionProfile_Impl: Sized {
    fn GetProfileXmlData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UpdateProfile(&self, strprofile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionProfile {}
impl IMbnConnectionProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>() -> IMbnConnectionProfile_Vtbl {
        unsafe extern "system" fn GetProfileXmlData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiledata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionProfile_Impl::GetProfileXmlData(this) {
                Ok(ok__) => {
                    core::ptr::write(profiledata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprofile: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfile_Impl::UpdateProfile(this, core::mem::transmute(&strprofile)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfile_Impl::Delete(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProfileXmlData: GetProfileXmlData::<Identity, Impl, OFFSET>,
            UpdateProfile: UpdateProfile::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfile as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileEvents_Impl: Sized {
    fn OnProfileUpdate(&self, newprofile: Option<&IMbnConnectionProfile>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionProfileEvents {}
impl IMbnConnectionProfileEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileEvents_Vtbl {
        unsafe extern "system" fn OnProfileUpdate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfileEvents_Impl::OnProfileUpdate(this, windows_core::from_raw_borrowed(&newprofile)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionProfileManager_Impl: Sized {
    fn GetConnectionProfiles(&self, mbninterface: Option<&IMbnInterface>) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(&self, mbninterface: Option<&IMbnInterface>, profilename: &windows_core::PCWSTR) -> windows_core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(&self, xmlprofile: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionProfileManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionProfileManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManager_Vtbl {
        unsafe extern "system" fn GetConnectionProfiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionProfileManager_Impl::GetConnectionProfiles(this, windows_core::from_raw_borrowed(&mbninterface)) {
                Ok(ok__) => {
                    core::ptr::write(connectionprofiles, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, profilename: windows_core::PCWSTR, connectionprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnConnectionProfileManager_Impl::GetConnectionProfile(this, windows_core::from_raw_borrowed(&mbninterface), core::mem::transmute(&profilename)) {
                Ok(ok__) => {
                    core::ptr::write(connectionprofile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlprofile: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfileManager_Impl::CreateConnectionProfile(this, core::mem::transmute(&xmlprofile)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Identity, Impl, OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Identity, Impl, OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManager as windows_core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileManagerEvents_Impl: Sized {
    fn OnConnectionProfileArrival(&self, newconnectionprofile: Option<&IMbnConnectionProfile>) -> windows_core::Result<()>;
    fn OnConnectionProfileRemoval(&self, oldconnectionprofile: Option<&IMbnConnectionProfile>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnConnectionProfileManagerEvents {}
impl IMbnConnectionProfileManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnectionprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfileManagerEvents_Impl::OnConnectionProfileArrival(this, windows_core::from_raw_borrowed(&newconnectionprofile)).into()
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldconnectionprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnConnectionProfileManagerEvents_Impl::OnConnectionProfileRemoval(this, windows_core::from_raw_borrowed(&oldconnectionprofile)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Identity, Impl, OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManagerEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceService_Impl: Sized {
    fn QuerySupportedCommands(&self) -> windows_core::Result<u32>;
    fn OpenCommandSession(&self) -> windows_core::Result<u32>;
    fn CloseCommandSession(&self) -> windows_core::Result<u32>;
    fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn OpenDataSession(&self) -> windows_core::Result<u32>;
    fn CloseDataSession(&self) -> windows_core::Result<u32>;
    fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DeviceServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsCommandSessionOpen(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDataSessionOpen(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceService {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>() -> IMbnDeviceService_Vtbl {
        unsafe extern "system" fn QuerySupportedCommands<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::QuerySupportedCommands(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::OpenCommandSession(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCommandSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::CloseCommandSession(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::SetCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&deviceservicedata)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::QueryCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&deviceservicedata)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::OpenDataSession(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDataSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::CloseDataSession(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::WriteData(this, core::mem::transmute_copy(&deviceservicedata)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(interfaceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceserviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::DeviceServiceID(this) {
                Ok(ok__) => {
                    core::ptr::write(deviceserviceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::IsCommandSessionOpen(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceService_Impl::IsDataSessionOpen(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuerySupportedCommands: QuerySupportedCommands::<Identity, Impl, OFFSET>,
            OpenCommandSession: OpenCommandSession::<Identity, Impl, OFFSET>,
            CloseCommandSession: CloseCommandSession::<Identity, Impl, OFFSET>,
            SetCommand: SetCommand::<Identity, Impl, OFFSET>,
            QueryCommand: QueryCommand::<Identity, Impl, OFFSET>,
            OpenDataSession: OpenDataSession::<Identity, Impl, OFFSET>,
            CloseDataSession: CloseDataSession::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            DeviceServiceID: DeviceServiceID::<Identity, Impl, OFFSET>,
            IsCommandSessionOpen: IsCommandSessionOpen::<Identity, Impl, OFFSET>,
            IsDataSessionOpen: IsDataSessionOpen::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceService as windows_core::Interface>::IID
    }
}
pub trait IMbnDeviceServiceStateEvents_Impl: Sized {
    fn OnSessionsStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnDeviceServiceStateEvents {}
impl IMbnDeviceServiceStateEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServiceStateEvents_Vtbl {
        unsafe extern "system" fn OnSessionsStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: core::mem::MaybeUninit<windows_core::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServiceStateEvents_Impl::OnSessionsStateChange(this, core::mem::transmute(&interfaceid), core::mem::transmute_copy(&statechange)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesContext_Impl: Sized {
    fn EnumerateDeviceServices(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(&self, deviceserviceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceService>;
    fn MaxCommandSize(&self) -> windows_core::Result<u32>;
    fn MaxDataSize(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceServicesContext {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>() -> IMbnDeviceServicesContext_Vtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceServicesContext_Impl::EnumerateDeviceServices(this) {
                Ok(ok__) => {
                    core::ptr::write(deviceservices, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceserviceid: core::mem::MaybeUninit<windows_core::BSTR>, mbndeviceservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceServicesContext_Impl::GetDeviceService(this, core::mem::transmute(&deviceserviceid)) {
                Ok(ok__) => {
                    core::ptr::write(mbndeviceservice, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCommandSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxcommandsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceServicesContext_Impl::MaxCommandSize(this) {
                Ok(ok__) => {
                    core::ptr::write(maxcommandsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDataSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxdatasize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceServicesContext_Impl::MaxDataSize(this) {
                Ok(ok__) => {
                    core::ptr::write(maxdatasize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumerateDeviceServices: EnumerateDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceService: GetDeviceService::<Identity, Impl, OFFSET>,
            MaxCommandSize: MaxCommandSize::<Identity, Impl, OFFSET>,
            MaxDataSize: MaxDataSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesEvents_Impl: Sized {
    fn OnQuerySupportedCommandsComplete(&self, deviceservice: Option<&IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnOpenCommandSessionComplete(&self, deviceservice: Option<&IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnCloseCommandSessionComplete(&self, deviceservice: Option<&IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnSetCommandComplete(&self, deviceservice: Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnQueryCommandComplete(&self, deviceservice: Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnEventNotification(&self, deviceservice: Option<&IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnOpenDataSessionComplete(&self, deviceservice: Option<&IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnCloseDataSessionComplete(&self, deviceservice: Option<&IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnWriteDataComplete(&self, deviceservice: Option<&IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnReadData(&self, deviceservice: Option<&IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnInterfaceStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceServicesEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServicesEvents_Vtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnQuerySupportedCommandsComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&commandidlist), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnOpenCommandSessionComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnCloseCommandSessionComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnSetCommandComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnSetCommandComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&responseid), core::mem::transmute_copy(&deviceservicedata), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnQueryCommandComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnQueryCommandComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&responseid), core::mem::transmute_copy(&deviceservicedata), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnEventNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnEventNotification(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnOpenDataSessionComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnCloseDataSessionComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnWriteDataComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnWriteDataComplete(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnReadData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnReadData(this, windows_core::from_raw_borrowed(&deviceservice), core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnInterfaceStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: core::mem::MaybeUninit<windows_core::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnDeviceServicesEvents_Impl::OnInterfaceStateChange(this, core::mem::transmute(&interfaceid), core::mem::transmute_copy(&statechange)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQuerySupportedCommandsComplete: OnQuerySupportedCommandsComplete::<Identity, Impl, OFFSET>,
            OnOpenCommandSessionComplete: OnOpenCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseCommandSessionComplete: OnCloseCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnSetCommandComplete: OnSetCommandComplete::<Identity, Impl, OFFSET>,
            OnQueryCommandComplete: OnQueryCommandComplete::<Identity, Impl, OFFSET>,
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnOpenDataSessionComplete: OnOpenDataSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseDataSessionComplete: OnCloseDataSessionComplete::<Identity, Impl, OFFSET>,
            OnWriteDataComplete: OnWriteDataComplete::<Identity, Impl, OFFSET>,
            OnReadData: OnReadData::<Identity, Impl, OFFSET>,
            OnInterfaceStateChange: OnInterfaceStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnDeviceServicesManager_Impl: Sized {
    fn GetDeviceServicesContext(&self, networkinterfaceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceServicesContext>;
}
impl windows_core::RuntimeName for IMbnDeviceServicesManager {}
impl IMbnDeviceServicesManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>() -> IMbnDeviceServicesManager_Vtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>, mbndevicescontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnDeviceServicesManager_Impl::GetDeviceServicesContext(this, core::mem::transmute(&networkinterfaceid)) {
                Ok(ok__) => {
                    core::ptr::write(mbndevicescontext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterface_Impl: Sized {
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInterfaceCapability(&self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> windows_core::Result<()>;
    fn GetSubscriberInformation(&self) -> windows_core::Result<IMbnSubscriberInformation>;
    fn GetReadyState(&self) -> windows_core::Result<MBN_READY_STATE>;
    fn InEmergencyMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetHomeProvider(&self) -> windows_core::Result<MBN_PROVIDER>;
    fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ScanNetwork(&self) -> windows_core::Result<u32>;
    fn GetConnection(&self) -> windows_core::Result<IMbnConnection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnInterface {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>() -> IMbnInterface_Vtbl {
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(interfaceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterface_Impl::GetInterfaceCapability(this, core::mem::transmute_copy(&interfacecaps)).into()
        }
        unsafe extern "system" fn GetSubscriberInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscriberinformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetSubscriberInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(subscriberinformation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetReadyState(this) {
                Ok(ok__) => {
                    core::ptr::write(readystate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InEmergencyMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emergencymode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::InEmergencyMode(this) {
                Ok(ok__) => {
                    core::ptr::write(emergencymode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomeProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetHomeProvider(this) {
                Ok(ok__) => {
                    core::ptr::write(homeprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetPreferredProviders(this) {
                Ok(ok__) => {
                    core::ptr::write(preferredproviders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::SetPreferredProviders(this, core::mem::transmute_copy(&preferredproviders)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetVisibleProviders(this, core::mem::transmute_copy(&age)) {
                Ok(ok__) => {
                    core::ptr::write(visibleproviders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::ScanNetwork(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbnconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterface_Impl::GetConnection(this) {
                Ok(ok__) => {
                    core::ptr::write(mbnconnection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            GetInterfaceCapability: GetInterfaceCapability::<Identity, Impl, OFFSET>,
            GetSubscriberInformation: GetSubscriberInformation::<Identity, Impl, OFFSET>,
            GetReadyState: GetReadyState::<Identity, Impl, OFFSET>,
            InEmergencyMode: InEmergencyMode::<Identity, Impl, OFFSET>,
            GetHomeProvider: GetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            SetPreferredProviders: SetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterface as windows_core::Interface>::IID
    }
}
pub trait IMbnInterfaceEvents_Impl: Sized {
    fn OnInterfaceCapabilityAvailable(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnSubscriberInformationChange(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnReadyStateChange(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnEmergencyModeChange(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnHomeProviderAvailable(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnPreferredProvidersChange(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnSetPreferredProvidersComplete(&self, newinterface: Option<&IMbnInterface>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnScanNetworkComplete(&self, newinterface: Option<&IMbnInterface>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnInterfaceEvents {}
impl IMbnInterfaceEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnInterfaceCapabilityAvailable(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnSubscriberInformationChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnReadyStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnReadyStateChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnEmergencyModeChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnEmergencyModeChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnHomeProviderAvailable(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnPreferredProvidersChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnSetPreferredProvidersComplete(this, windows_core::from_raw_borrowed(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceEvents_Impl::OnScanNetworkComplete(this, windows_core::from_raw_borrowed(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInterfaceCapabilityAvailable: OnInterfaceCapabilityAvailable::<Identity, Impl, OFFSET>,
            OnSubscriberInformationChange: OnSubscriberInformationChange::<Identity, Impl, OFFSET>,
            OnReadyStateChange: OnReadyStateChange::<Identity, Impl, OFFSET>,
            OnEmergencyModeChange: OnEmergencyModeChange::<Identity, Impl, OFFSET>,
            OnHomeProviderAvailable: OnHomeProviderAvailable::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnSetPreferredProvidersComplete: OnSetPreferredProvidersComplete::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterfaceManager_Impl: Sized {
    fn GetInterface(&self, interfaceid: &windows_core::PCWSTR) -> windows_core::Result<IMbnInterface>;
    fn GetInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnInterfaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnInterfaceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>() -> IMbnInterfaceManager_Vtbl {
        unsafe extern "system" fn GetInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: windows_core::PCWSTR, mbninterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterfaceManager_Impl::GetInterface(this, core::mem::transmute(&interfaceid)) {
                Ok(ok__) => {
                    core::ptr::write(mbninterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnInterfaceManager_Impl::GetInterfaces(this) {
                Ok(ok__) => {
                    core::ptr::write(mbninterfaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            GetInterfaces: GetInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceManager as windows_core::Interface>::IID
    }
}
pub trait IMbnInterfaceManagerEvents_Impl: Sized {
    fn OnInterfaceArrival(&self, newinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
    fn OnInterfaceRemoval(&self, oldinterface: Option<&IMbnInterface>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnInterfaceManagerEvents {}
impl IMbnInterfaceManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceManagerEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceArrival<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceManagerEvents_Impl::OnInterfaceArrival(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemoval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnInterfaceManagerEvents_Impl::OnInterfaceRemoval(this, windows_core::from_raw_borrowed(&oldinterface)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInterfaceArrival: OnInterfaceArrival::<Identity, Impl, OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceManagerEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnMultiCarrier_Impl: Sized {
    fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> windows_core::Result<u32>;
    fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetSupportedCellularClasses(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(&self) -> windows_core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnMultiCarrier {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnMultiCarrier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>() -> IMbnMultiCarrier_Vtbl {
        unsafe extern "system" fn SetHomeProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::SetHomeProvider(this, core::mem::transmute_copy(&homeprovider)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::GetPreferredProviders(this) {
                Ok(ok__) => {
                    core::ptr::write(preferredmulticarrierproviders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::GetVisibleProviders(this, core::mem::transmute_copy(&age)) {
                Ok(ok__) => {
                    core::ptr::write(visibleproviders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::GetSupportedCellularClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(cellularclasses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::GetCurrentCellularClass(this) {
                Ok(ok__) => {
                    core::ptr::write(currentcellularclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnMultiCarrier_Impl::ScanNetwork(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomeProvider: SetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Identity, Impl, OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnMultiCarrier as windows_core::Interface>::IID
    }
}
pub trait IMbnMultiCarrierEvents_Impl: Sized {
    fn OnSetHomeProviderComplete(&self, mbninterface: Option<&IMbnMultiCarrier>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnCurrentCellularClassChange(&self, mbninterface: Option<&IMbnMultiCarrier>) -> windows_core::Result<()>;
    fn OnPreferredProvidersChange(&self, mbninterface: Option<&IMbnMultiCarrier>) -> windows_core::Result<()>;
    fn OnScanNetworkComplete(&self, mbninterface: Option<&IMbnMultiCarrier>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnInterfaceCapabilityChange(&self, mbninterface: Option<&IMbnMultiCarrier>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnMultiCarrierEvents {}
impl IMbnMultiCarrierEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>() -> IMbnMultiCarrierEvents_Vtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnMultiCarrierEvents_Impl::OnSetHomeProviderComplete(this, windows_core::from_raw_borrowed(&mbninterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnMultiCarrierEvents_Impl::OnCurrentCellularClassChange(this, windows_core::from_raw_borrowed(&mbninterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnMultiCarrierEvents_Impl::OnPreferredProvidersChange(this, windows_core::from_raw_borrowed(&mbninterface)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnMultiCarrierEvents_Impl::OnScanNetworkComplete(this, windows_core::from_raw_borrowed(&mbninterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnMultiCarrierEvents_Impl::OnInterfaceCapabilityChange(this, windows_core::from_raw_borrowed(&mbninterface)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Identity, Impl, OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnMultiCarrierEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnPin_Impl: Sized {
    fn PinType(&self) -> windows_core::Result<MBN_PIN_TYPE>;
    fn PinFormat(&self) -> windows_core::Result<MBN_PIN_FORMAT>;
    fn PinLengthMin(&self) -> windows_core::Result<u32>;
    fn PinLengthMax(&self) -> windows_core::Result<u32>;
    fn PinMode(&self) -> windows_core::Result<MBN_PIN_MODE>;
    fn Enable(&self, pin: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Disable(&self, pin: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Enter(&self, pin: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Change(&self, pin: &windows_core::PCWSTR, newpin: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Unblock(&self, puk: &windows_core::PCWSTR, newpin: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetPinManager(&self) -> windows_core::Result<IMbnPinManager>;
}
impl windows_core::RuntimeName for IMbnPin {}
impl IMbnPin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>() -> IMbnPin_Vtbl {
        unsafe extern "system" fn PinType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::PinType(this) {
                Ok(ok__) => {
                    core::ptr::write(pintype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::PinFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(pinformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlengthmin: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::PinLengthMin(this) {
                Ok(ok__) => {
                    core::ptr::write(pinlengthmin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMax<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlengthmax: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::PinLengthMax(this) {
                Ok(ok__) => {
                    core::ptr::write(pinlengthmax, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::PinMode(this) {
                Ok(ok__) => {
                    core::ptr::write(pinmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::Enable(this, core::mem::transmute(&pin)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::Disable(this, core::mem::transmute(&pin)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::Enter(this, core::mem::transmute(&pin)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Change<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, newpin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::Change(this, core::mem::transmute(&pin), core::mem::transmute(&newpin)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unblock<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puk: windows_core::PCWSTR, newpin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::Unblock(this, core::mem::transmute(&puk), core::mem::transmute(&newpin)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPin_Impl::GetPinManager(this) {
                Ok(ok__) => {
                    core::ptr::write(pinmanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PinType: PinType::<Identity, Impl, OFFSET>,
            PinFormat: PinFormat::<Identity, Impl, OFFSET>,
            PinLengthMin: PinLengthMin::<Identity, Impl, OFFSET>,
            PinLengthMax: PinLengthMax::<Identity, Impl, OFFSET>,
            PinMode: PinMode::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Change: Change::<Identity, Impl, OFFSET>,
            Unblock: Unblock::<Identity, Impl, OFFSET>,
            GetPinManager: GetPinManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPin as windows_core::Interface>::IID
    }
}
pub trait IMbnPinEvents_Impl: Sized {
    fn OnEnableComplete(&self, pin: Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnDisableComplete(&self, pin: Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnEnterComplete(&self, pin: Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnChangeComplete(&self, pin: Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnUnblockComplete(&self, pin: Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnPinEvents {}
impl IMbnPinEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>() -> IMbnPinEvents_Vtbl {
        unsafe extern "system" fn OnEnableComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinEvents_Impl::OnEnableComplete(this, windows_core::from_raw_borrowed(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisableComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinEvents_Impl::OnDisableComplete(this, windows_core::from_raw_borrowed(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnEnterComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinEvents_Impl::OnEnterComplete(this, windows_core::from_raw_borrowed(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnChangeComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinEvents_Impl::OnChangeComplete(this, windows_core::from_raw_borrowed(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnUnblockComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinEvents_Impl::OnUnblockComplete(this, windows_core::from_raw_borrowed(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEnableComplete: OnEnableComplete::<Identity, Impl, OFFSET>,
            OnDisableComplete: OnDisableComplete::<Identity, Impl, OFFSET>,
            OnEnterComplete: OnEnterComplete::<Identity, Impl, OFFSET>,
            OnChangeComplete: OnChangeComplete::<Identity, Impl, OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManager_Impl: Sized {
    fn GetPinList(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(&self, pintype: MBN_PIN_TYPE) -> windows_core::Result<IMbnPin>;
    fn GetPinState(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnPinManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>() -> IMbnPinManager_Vtbl {
        unsafe extern "system" fn GetPinList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPinManager_Impl::GetPinList(this) {
                Ok(ok__) => {
                    core::ptr::write(pinlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPinManager_Impl::GetPin(this, core::mem::transmute_copy(&pintype)) {
                Ok(ok__) => {
                    core::ptr::write(pin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnPinManager_Impl::GetPinState(this) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPinList: GetPinList::<Identity, Impl, OFFSET>,
            GetPin: GetPin::<Identity, Impl, OFFSET>,
            GetPinState: GetPinState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinManager as windows_core::Interface>::IID
    }
}
pub trait IMbnPinManagerEvents_Impl: Sized {
    fn OnPinListAvailable(&self, pinmanager: Option<&IMbnPinManager>) -> windows_core::Result<()>;
    fn OnGetPinStateComplete(&self, pinmanager: Option<&IMbnPinManager>, pininfo: &MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnPinManagerEvents {}
impl IMbnPinManagerEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>() -> IMbnPinManagerEvents_Vtbl {
        unsafe extern "system" fn OnPinListAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinManagerEvents_Impl::OnPinListAvailable(this, windows_core::from_raw_borrowed(&pinmanager)).into()
        }
        unsafe extern "system" fn OnGetPinStateComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnPinManagerEvents_Impl::OnGetPinStateComplete(this, windows_core::from_raw_borrowed(&pinmanager), core::mem::transmute(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPinListAvailable: OnPinListAvailable::<Identity, Impl, OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinManagerEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnRadio_Impl: Sized {
    fn SoftwareRadioState(&self) -> windows_core::Result<MBN_RADIO>;
    fn HardwareRadioState(&self) -> windows_core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMbnRadio {}
impl IMbnRadio_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>() -> IMbnRadio_Vtbl {
        unsafe extern "system" fn SoftwareRadioState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRadio_Impl::SoftwareRadioState(this) {
                Ok(ok__) => {
                    core::ptr::write(softwareradiostate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRadioState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRadio_Impl::HardwareRadioState(this) {
                Ok(ok__) => {
                    core::ptr::write(hardwareradiostate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRadio_Impl::SetSoftwareRadioState(this, core::mem::transmute_copy(&radiostate)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SoftwareRadioState: SoftwareRadioState::<Identity, Impl, OFFSET>,
            HardwareRadioState: HardwareRadioState::<Identity, Impl, OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRadio as windows_core::Interface>::IID
    }
}
pub trait IMbnRadioEvents_Impl: Sized {
    fn OnRadioStateChange(&self, newinterface: Option<&IMbnRadio>) -> windows_core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(&self, newinterface: Option<&IMbnRadio>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnRadioEvents {}
impl IMbnRadioEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>() -> IMbnRadioEvents_Vtbl {
        unsafe extern "system" fn OnRadioStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRadioEvents_Impl::OnRadioStateChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRadioEvents_Impl::OnSetSoftwareRadioStateComplete(this, windows_core::from_raw_borrowed(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRadioStateChange: OnRadioStateChange::<Identity, Impl, OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRadioEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnRegistration_Impl: Sized {
    fn GetRegisterState(&self) -> windows_core::Result<MBN_REGISTER_STATE>;
    fn GetRegisterMode(&self) -> windows_core::Result<MBN_REGISTER_MODE>;
    fn GetProviderID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRoamingText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAvailableDataClasses(&self) -> windows_core::Result<u32>;
    fn GetCurrentDataClass(&self) -> windows_core::Result<u32>;
    fn GetRegistrationNetworkError(&self) -> windows_core::Result<u32>;
    fn GetPacketAttachNetworkError(&self) -> windows_core::Result<u32>;
    fn SetRegisterMode(&self, registermode: MBN_REGISTER_MODE, providerid: &windows_core::PCWSTR, dataclass: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMbnRegistration {}
impl IMbnRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>() -> IMbnRegistration_Vtbl {
        unsafe extern "system" fn GetRegisterState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetRegisterState(this) {
                Ok(ok__) => {
                    core::ptr::write(registerstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetRegisterMode(this) {
                Ok(ok__) => {
                    core::ptr::write(registermode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetProviderID(this) {
                Ok(ok__) => {
                    core::ptr::write(providerid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetProviderName(this) {
                Ok(ok__) => {
                    core::ptr::write(providername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoamingText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roamingtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetRoamingText(this) {
                Ok(ok__) => {
                    core::ptr::write(roamingtext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availabledataclasses: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetAvailableDataClasses(this) {
                Ok(ok__) => {
                    core::ptr::write(availabledataclasses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentdataclass: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetCurrentDataClass(this) {
                Ok(ok__) => {
                    core::ptr::write(currentdataclass, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registrationnetworkerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetRegistrationNetworkError(this) {
                Ok(ok__) => {
                    core::ptr::write(registrationnetworkerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetattachnetworkerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::GetPacketAttachNetworkError(this) {
                Ok(ok__) => {
                    core::ptr::write(packetattachnetworkerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegisterMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: windows_core::PCWSTR, dataclass: u32, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnRegistration_Impl::SetRegisterMode(this, core::mem::transmute_copy(&registermode), core::mem::transmute(&providerid), core::mem::transmute_copy(&dataclass)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRegisterState: GetRegisterState::<Identity, Impl, OFFSET>,
            GetRegisterMode: GetRegisterMode::<Identity, Impl, OFFSET>,
            GetProviderID: GetProviderID::<Identity, Impl, OFFSET>,
            GetProviderName: GetProviderName::<Identity, Impl, OFFSET>,
            GetRoamingText: GetRoamingText::<Identity, Impl, OFFSET>,
            GetAvailableDataClasses: GetAvailableDataClasses::<Identity, Impl, OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Identity, Impl, OFFSET>,
            GetRegistrationNetworkError: GetRegistrationNetworkError::<Identity, Impl, OFFSET>,
            GetPacketAttachNetworkError: GetPacketAttachNetworkError::<Identity, Impl, OFFSET>,
            SetRegisterMode: SetRegisterMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRegistration as windows_core::Interface>::IID
    }
}
pub trait IMbnRegistrationEvents_Impl: Sized {
    fn OnRegisterModeAvailable(&self, newinterface: Option<&IMbnRegistration>) -> windows_core::Result<()>;
    fn OnRegisterStateChange(&self, newinterface: Option<&IMbnRegistration>) -> windows_core::Result<()>;
    fn OnPacketServiceStateChange(&self, newinterface: Option<&IMbnRegistration>) -> windows_core::Result<()>;
    fn OnSetRegisterModeComplete(&self, newinterface: Option<&IMbnRegistration>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnRegistrationEvents {}
impl IMbnRegistrationEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>() -> IMbnRegistrationEvents_Vtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRegistrationEvents_Impl::OnRegisterModeAvailable(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnRegisterStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRegistrationEvents_Impl::OnRegisterStateChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRegistrationEvents_Impl::OnPacketServiceStateChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnRegistrationEvents_Impl::OnSetRegisterModeComplete(this, windows_core::from_raw_borrowed(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Identity, Impl, OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Identity, Impl, OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Identity, Impl, OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRegistrationEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivation_Impl: Sized {
    fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnServiceActivation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>() -> IMbnServiceActivation_Vtbl {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnServiceActivation_Impl::Activate(this, core::mem::transmute_copy(&vendorspecificdata)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnServiceActivation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEvents_Impl: Sized {
    fn OnActivationComplete(&self, serviceactivation: Option<&IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnServiceActivationEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>() -> IMbnServiceActivationEvents_Vtbl {
        unsafe extern "system" fn OnActivationComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceactivation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnServiceActivationEvents_Impl::OnActivationComplete(this, windows_core::from_raw_borrowed(&serviceactivation), core::mem::transmute_copy(&vendorspecificdata), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status), core::mem::transmute_copy(&networkerror)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivationComplete: OnActivationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as windows_core::Interface>::IID
    }
}
pub trait IMbnSignal_Impl: Sized {
    fn GetSignalStrength(&self) -> windows_core::Result<u32>;
    fn GetSignalError(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMbnSignal {}
impl IMbnSignal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>() -> IMbnSignal_Vtbl {
        unsafe extern "system" fn GetSignalStrength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signalstrength: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSignal_Impl::GetSignalStrength(this) {
                Ok(ok__) => {
                    core::ptr::write(signalstrength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signalerror: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSignal_Impl::GetSignalError(this) {
                Ok(ok__) => {
                    core::ptr::write(signalerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignalStrength: GetSignalStrength::<Identity, Impl, OFFSET>,
            GetSignalError: GetSignalError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSignal as windows_core::Interface>::IID
    }
}
pub trait IMbnSignalEvents_Impl: Sized {
    fn OnSignalStateChange(&self, newinterface: Option<&IMbnSignal>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnSignalEvents {}
impl IMbnSignalEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>() -> IMbnSignalEvents_Vtbl {
        unsafe extern "system" fn OnSignalStateChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSignalEvents_Impl::OnSignalStateChange(this, windows_core::from_raw_borrowed(&newinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSignalEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSms_Impl: Sized {
    fn GetSmsConfiguration(&self) -> windows_core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(&self, smsconfiguration: Option<&IMbnSmsConfiguration>) -> windows_core::Result<u32>;
    fn SmsSendPdu(&self, pdudata: &windows_core::PCWSTR, size: u8) -> windows_core::Result<u32>;
    fn SmsSendCdma(&self, address: &windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<u32>;
    fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> windows_core::Result<u32>;
    fn GetSmsStatus(&self) -> windows_core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSms {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSms_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>() -> IMbnSms_Vtbl {
        unsafe extern "system" fn GetSmsConfiguration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::GetSmsConfiguration(this) {
                Ok(ok__) => {
                    core::ptr::write(smsconfiguration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsconfiguration: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SetSmsConfiguration(this, windows_core::from_raw_borrowed(&smsconfiguration)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendPdu<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdudata: windows_core::PCWSTR, size: u8, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SmsSendPdu(this, core::mem::transmute(&pdudata), core::mem::transmute_copy(&size)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdma<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SmsSendCdma(this, core::mem::transmute(&address), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&language), core::mem::transmute_copy(&sizeincharacters), core::mem::transmute_copy(&message)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SmsSendCdmaPdu(this, core::mem::transmute_copy(&message)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsRead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SmsRead(this, core::mem::transmute_copy(&smsfilter), core::mem::transmute_copy(&smsformat)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::SmsDelete(this, core::mem::transmute_copy(&smsfilter)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmsStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSms_Impl::GetSmsStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(smsstatusinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSmsConfiguration: GetSmsConfiguration::<Identity, Impl, OFFSET>,
            SetSmsConfiguration: SetSmsConfiguration::<Identity, Impl, OFFSET>,
            SmsSendPdu: SmsSendPdu::<Identity, Impl, OFFSET>,
            SmsSendCdma: SmsSendCdma::<Identity, Impl, OFFSET>,
            SmsSendCdmaPdu: SmsSendCdmaPdu::<Identity, Impl, OFFSET>,
            SmsRead: SmsRead::<Identity, Impl, OFFSET>,
            SmsDelete: SmsDelete::<Identity, Impl, OFFSET>,
            GetSmsStatus: GetSmsStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSms as windows_core::Interface>::IID
    }
}
pub trait IMbnSmsConfiguration_Impl: Sized {
    fn ServiceCenterAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceCenterAddress(&self, scaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn MaxMessageIndex(&self) -> windows_core::Result<u32>;
    fn CdmaShortMsgSize(&self) -> windows_core::Result<u32>;
    fn SmsFormat(&self) -> windows_core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMbnSmsConfiguration {}
impl IMbnSmsConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>() -> IMbnSmsConfiguration_Vtbl {
        unsafe extern "system" fn ServiceCenterAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsConfiguration_Impl::ServiceCenterAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(scaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaddress: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsConfiguration_Impl::SetServiceCenterAddress(this, core::mem::transmute(&scaddress)).into()
        }
        unsafe extern "system" fn MaxMessageIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsConfiguration_Impl::MaxMessageIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(index, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortmsgsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsConfiguration_Impl::CdmaShortMsgSize(this) {
                Ok(ok__) => {
                    core::ptr::write(shortmsgsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsConfiguration_Impl::SmsFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(smsformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsConfiguration_Impl::SetSmsFormat(this, core::mem::transmute_copy(&smsformat)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ServiceCenterAddress: ServiceCenterAddress::<Identity, Impl, OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Identity, Impl, OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Identity, Impl, OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Identity, Impl, OFFSET>,
            SmsFormat: SmsFormat::<Identity, Impl, OFFSET>,
            SetSmsFormat: SetSmsFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsConfiguration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsEvents_Impl: Sized {
    fn OnSmsConfigurationChange(&self, sms: Option<&IMbnSms>) -> windows_core::Result<()>;
    fn OnSetSmsConfigurationComplete(&self, sms: Option<&IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsSendComplete(&self, sms: Option<&IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsReadComplete(&self, sms: Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsNewClass0Message(&self, sms: Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnSmsDeleteComplete(&self, sms: Option<&IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsStatusChange(&self, sms: Option<&IMbnSms>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>() -> IMbnSmsEvents_Vtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsConfigurationChange(this, windows_core::from_raw_borrowed(&sms)).into()
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSetSmsConfigurationComplete(this, windows_core::from_raw_borrowed(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsSendComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsSendComplete(this, windows_core::from_raw_borrowed(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsReadComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsReadComplete(this, windows_core::from_raw_borrowed(&sms), core::mem::transmute_copy(&smsformat), core::mem::transmute_copy(&readmsgs), core::mem::transmute_copy(&moremsgs), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsNewClass0Message(this, windows_core::from_raw_borrowed(&sms), core::mem::transmute_copy(&smsformat), core::mem::transmute_copy(&readmsgs)).into()
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsDeleteComplete(this, windows_core::from_raw_borrowed(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsStatusChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnSmsEvents_Impl::OnSmsStatusChange(this, windows_core::from_raw_borrowed(&sms)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSmsConfigurationChange: OnSmsConfigurationChange::<Identity, Impl, OFFSET>,
            OnSetSmsConfigurationComplete: OnSetSmsConfigurationComplete::<Identity, Impl, OFFSET>,
            OnSmsSendComplete: OnSmsSendComplete::<Identity, Impl, OFFSET>,
            OnSmsReadComplete: OnSmsReadComplete::<Identity, Impl, OFFSET>,
            OnSmsNewClass0Message: OnSmsNewClass0Message::<Identity, Impl, OFFSET>,
            OnSmsDeleteComplete: OnSmsDeleteComplete::<Identity, Impl, OFFSET>,
            OnSmsStatusChange: OnSmsStatusChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgPdu_Impl: Sized {
    fn Index(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS>;
    fn PduData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsReadMsgPdu {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsReadMsgPdu_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgPdu_Vtbl {
        unsafe extern "system" fn Index<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgPdu_Impl::Index(this) {
                Ok(ok__) => {
                    core::ptr::write(index, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgPdu_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PduData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdudata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgPdu_Impl::PduData(this) {
                Ok(ok__) => {
                    core::ptr::write(pdudata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgPdu_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(message, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            PduData: PduData::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgPdu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgTextCdma_Impl: Sized {
    fn Index(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS>;
    fn Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timestamp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EncodingID(&self) -> windows_core::Result<MBN_SMS_CDMA_ENCODING>;
    fn LanguageID(&self) -> windows_core::Result<MBN_SMS_CDMA_LANG>;
    fn SizeInCharacters(&self) -> windows_core::Result<u32>;
    fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsReadMsgTextCdma {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsReadMsgTextCdma_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgTextCdma_Vtbl {
        unsafe extern "system" fn Index<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::Index(this) {
                Ok(ok__) => {
                    core::ptr::write(index, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(address, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(timestamp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::EncodingID(this) {
                Ok(ok__) => {
                    core::ptr::write(encodingid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::LanguageID(this) {
                Ok(ok__) => {
                    core::ptr::write(languageid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInCharacters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizeincharacters: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::SizeInCharacters(this) {
                Ok(ok__) => {
                    core::ptr::write(sizeincharacters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSmsReadMsgTextCdma_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(message, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            EncodingID: EncodingID::<Identity, Impl, OFFSET>,
            LanguageID: LanguageID::<Identity, Impl, OFFSET>,
            SizeInCharacters: SizeInCharacters::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgTextCdma as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSubscriberInformation_Impl: Sized {
    fn SubscriberID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SimIccID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TelephoneNumbers(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSubscriberInformation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSubscriberInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>() -> IMbnSubscriberInformation_Vtbl {
        unsafe extern "system" fn SubscriberID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscriberid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSubscriberInformation_Impl::SubscriberID(this) {
                Ok(ok__) => {
                    core::ptr::write(subscriberid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, simiccid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSubscriberInformation_Impl::SimIccID(this) {
                Ok(ok__) => {
                    core::ptr::write(simiccid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnSubscriberInformation_Impl::TelephoneNumbers(this) {
                Ok(ok__) => {
                    core::ptr::write(telephonenumbers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriberID: SubscriberID::<Identity, Impl, OFFSET>,
            SimIccID: SimIccID::<Identity, Impl, OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSubscriberInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEvents_Impl: Sized {
    fn OnEventNotification(&self, vendoroperation: Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnSetVendorSpecificComplete(&self, vendoroperation: Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnVendorSpecificEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>() -> IMbnVendorSpecificEvents_Vtbl {
        unsafe extern "system" fn OnEventNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendoroperation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnVendorSpecificEvents_Impl::OnEventNotification(this, windows_core::from_raw_borrowed(&vendoroperation), core::mem::transmute_copy(&vendorspecificdata)).into()
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendoroperation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMbnVendorSpecificEvents_Impl::OnSetVendorSpecificComplete(this, windows_core::from_raw_borrowed(&vendoroperation), core::mem::transmute_copy(&vendorspecificdata), core::mem::transmute_copy(&requestid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnVendorSpecificEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperation_Impl: Sized {
    fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnVendorSpecificOperation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>() -> IMbnVendorSpecificOperation_Vtbl {
        unsafe extern "system" fn SetVendorSpecific<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMbnVendorSpecificOperation_Impl::SetVendorSpecific(this, core::mem::transmute_copy(&vendorspecificdata)) {
                Ok(ok__) => {
                    core::ptr::write(requestid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as windows_core::Interface>::IID
    }
}
