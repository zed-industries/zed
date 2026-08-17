#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDummyMBNUCMExt, IDummyMBNUCMExt_Vtbl, 0xdcbbbab6_ffff_4bbb_aaee_338e368af6fa);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDummyMBNUCMExt {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDummyMBNUCMExt, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDummyMBNUCMExt_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDummyMBNUCMExt_Impl: super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDummyMBNUCMExt_Vtbl {
    pub const fn new<Identity: IDummyMBNUCMExt_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDummyMBNUCMExt as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDummyMBNUCMExt {}
windows_core::imp::define_interface!(IMbnConnection, IMbnConnection_Vtbl, 0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnection, windows_core::IUnknown);
impl IMbnConnection {
    pub unsafe fn ConnectionID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Connect<P1>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), connectionmode, strprofile.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConnectionState)(windows_core::Interface::as_raw(self), connectionstate as _, core::mem::transmute(profilename)).ok() }
    }
    pub unsafe fn GetVoiceCallState(&self) -> windows_core::Result<MBN_VOICE_CALL_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVoiceCallState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetActivationNetworkError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivationNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_CONNECTION_MODE, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnectionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_ACTIVATION_STATE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVoiceCallState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_VOICE_CALL_STATE) -> windows_core::HRESULT,
    pub GetActivationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMbnConnection_Impl: windows_core::IUnknownImpl {
    fn ConnectionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Connect(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn Disconnect(&self) -> windows_core::Result<u32>;
    fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetVoiceCallState(&self) -> windows_core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(&self) -> windows_core::Result<u32>;
}
impl IMbnConnection_Vtbl {
    pub const fn new<Identity: IMbnConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectionID<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::ConnectionID(this) {
                    Ok(ok__) => {
                        connectionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::InterfaceID(this) {
                    Ok(ok__) => {
                        interfaceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::Connect(this, core::mem::transmute_copy(&connectionmode), core::mem::transmute(&strprofile)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::Disconnect(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectionState<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnection_Impl::GetConnectionState(this, core::mem::transmute_copy(&connectionstate), core::mem::transmute_copy(&profilename)).into()
            }
        }
        unsafe extern "system" fn GetVoiceCallState<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::GetVoiceCallState(this) {
                    Ok(ok__) => {
                        voicecallstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Identity: IMbnConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnection_Impl::GetActivationNetworkError(this) {
                    Ok(ok__) => {
                        networkerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionID: ConnectionID::<Identity, OFFSET>,
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            GetConnectionState: GetConnectionState::<Identity, OFFSET>,
            GetVoiceCallState: GetVoiceCallState::<Identity, OFFSET>,
            GetActivationNetworkError: GetActivationNetworkError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnection {}
windows_core::imp::define_interface!(IMbnConnectionContext, IMbnConnectionContext_Vtbl, 0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionContext, windows_core::IUnknown);
impl IMbnConnectionContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProvisionedContexts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProvisionedContext<P1>(&self, provisionedcontexts: &MBN_CONTEXT, providerid: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetProvisionedContext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(provisionedcontexts), providerid.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProvisionedContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProvisionedContexts: usize,
    pub SetProvisionedContext: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_CONTEXT, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionContext_Impl: windows_core::IUnknownImpl {
    fn GetProvisionedContexts(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(&self, provisionedcontexts: &MBN_CONTEXT, providerid: &windows_core::PCWSTR) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionContext_Vtbl {
    pub const fn new<Identity: IMbnConnectionContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProvisionedContexts<Identity: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionContext_Impl::GetProvisionedContexts(this) {
                    Ok(ok__) => {
                        provisionedcontexts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Identity: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provisionedcontexts: MBN_CONTEXT, providerid: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionContext_Impl::SetProvisionedContext(this, core::mem::transmute(&provisionedcontexts), core::mem::transmute(&providerid)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProvisionedContexts: GetProvisionedContexts::<Identity, OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionContext {}
windows_core::imp::define_interface!(IMbnConnectionContextEvents, IMbnConnectionContextEvents_Vtbl, 0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionContextEvents, windows_core::IUnknown);
impl IMbnConnectionContextEvents {
    pub unsafe fn OnProvisionedContextListChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProvisionedContextListChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnSetProvisionedContextComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetProvisionedContextComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProvisionedContextListChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetProvisionedContextComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnConnectionContextEvents_Impl: windows_core::IUnknownImpl {
    fn OnProvisionedContextListChange(&self, newinterface: windows_core::Ref<IMbnConnectionContext>) -> windows_core::Result<()>;
    fn OnSetProvisionedContextComplete(&self, newinterface: windows_core::Ref<IMbnConnectionContext>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnConnectionContextEvents_Vtbl {
    pub const fn new<Identity: IMbnConnectionContextEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnProvisionedContextListChange<Identity: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionContextEvents_Impl::OnProvisionedContextListChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Identity: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionContextEvents_Impl::OnSetProvisionedContextComplete(this, core::mem::transmute_copy(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Identity, OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionContextEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionContextEvents {}
windows_core::imp::define_interface!(IMbnConnectionEvents, IMbnConnectionEvents_Vtbl, 0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionEvents, windows_core::IUnknown);
impl IMbnConnectionEvents {
    pub unsafe fn OnConnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectComplete)(windows_core::Interface::as_raw(self), newconnection.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnDisconnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisconnectComplete)(windows_core::Interface::as_raw(self), newconnection.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnConnectStateChange<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectStateChange)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok() }
    }
    pub unsafe fn OnVoiceCallStateChange<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnVoiceCallStateChange)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnDisconnectComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnConnectStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnVoiceCallStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnConnectionEvents_Impl: windows_core::IUnknownImpl {
    fn OnConnectComplete(&self, newconnection: windows_core::Ref<IMbnConnection>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnDisconnectComplete(&self, newconnection: windows_core::Ref<IMbnConnection>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnConnectStateChange(&self, newconnection: windows_core::Ref<IMbnConnection>) -> windows_core::Result<()>;
    fn OnVoiceCallStateChange(&self, newconnection: windows_core::Ref<IMbnConnection>) -> windows_core::Result<()>;
}
impl IMbnConnectionEvents_Vtbl {
    pub const fn new<Identity: IMbnConnectionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnectComplete<Identity: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionEvents_Impl::OnConnectComplete(this, core::mem::transmute_copy(&newconnection), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnDisconnectComplete<Identity: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionEvents_Impl::OnDisconnectComplete(this, core::mem::transmute_copy(&newconnection), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnConnectStateChange<Identity: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionEvents_Impl::OnConnectStateChange(this, core::mem::transmute_copy(&newconnection)).into()
            }
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Identity: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionEvents_Impl::OnVoiceCallStateChange(this, core::mem::transmute_copy(&newconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectComplete: OnConnectComplete::<Identity, OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Identity, OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Identity, OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionEvents {}
windows_core::imp::define_interface!(IMbnConnectionManager, IMbnConnectionManager_Vtbl, 0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionManager, windows_core::IUnknown);
impl IMbnConnectionManager {
    pub unsafe fn GetConnection<P0>(&self, connectionid: P0) -> windows_core::Result<IMbnConnection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), connectionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnections: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionManager_Impl: windows_core::IUnknownImpl {
    fn GetConnection(&self, connectionid: &windows_core::PCWSTR) -> windows_core::Result<IMbnConnection>;
    fn GetConnections(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionManager_Vtbl {
    pub const fn new<Identity: IMbnConnectionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnection<Identity: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionid: windows_core::PCWSTR, mbnconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionManager_Impl::GetConnection(this, core::mem::transmute(&connectionid)) {
                    Ok(ok__) => {
                        mbnconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnections<Identity: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionManager_Impl::GetConnections(this) {
                    Ok(ok__) => {
                        mbnconnections.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnection: GetConnection::<Identity, OFFSET>,
            GetConnections: GetConnections::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionManager {}
windows_core::imp::define_interface!(IMbnConnectionManagerEvents, IMbnConnectionManagerEvents_Vtbl, 0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionManagerEvents, windows_core::IUnknown);
impl IMbnConnectionManagerEvents {
    pub unsafe fn OnConnectionArrival<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectionArrival)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok() }
    }
    pub unsafe fn OnConnectionRemoval<P0>(&self, oldconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectionRemoval)(windows_core::Interface::as_raw(self), oldconnection.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnConnectionRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnConnectionManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnConnectionArrival(&self, newconnection: windows_core::Ref<IMbnConnection>) -> windows_core::Result<()>;
    fn OnConnectionRemoval(&self, oldconnection: windows_core::Ref<IMbnConnection>) -> windows_core::Result<()>;
}
impl IMbnConnectionManagerEvents_Vtbl {
    pub const fn new<Identity: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnectionArrival<Identity: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionManagerEvents_Impl::OnConnectionArrival(this, core::mem::transmute_copy(&newconnection)).into()
            }
        }
        unsafe extern "system" fn OnConnectionRemoval<Identity: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionManagerEvents_Impl::OnConnectionRemoval(this, core::mem::transmute_copy(&oldconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionArrival: OnConnectionArrival::<Identity, OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionManagerEvents {}
windows_core::imp::define_interface!(IMbnConnectionProfile, IMbnConnectionProfile_Vtbl, 0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionProfile, windows_core::IUnknown);
impl IMbnConnectionProfile {
    pub unsafe fn GetProfileXmlData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProfileXmlData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UpdateProfile<P0>(&self, strprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateProfile)(windows_core::Interface::as_raw(self), strprofile.param().abi()).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProfileXmlData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnConnectionProfile_Impl: windows_core::IUnknownImpl {
    fn GetProfileXmlData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UpdateProfile(&self, strprofile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
impl IMbnConnectionProfile_Vtbl {
    pub const fn new<Identity: IMbnConnectionProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProfileXmlData<Identity: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiledata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionProfile_Impl::GetProfileXmlData(this) {
                    Ok(ok__) => {
                        profiledata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateProfile<Identity: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprofile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfile_Impl::UpdateProfile(this, core::mem::transmute(&strprofile)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfile_Impl::Delete(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProfileXmlData: GetProfileXmlData::<Identity, OFFSET>,
            UpdateProfile: UpdateProfile::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionProfile {}
windows_core::imp::define_interface!(IMbnConnectionProfileEvents, IMbnConnectionProfileEvents_Vtbl, 0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileEvents, windows_core::IUnknown);
impl IMbnConnectionProfileEvents {
    pub unsafe fn OnProfileUpdate<P0>(&self, newprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProfileUpdate)(windows_core::Interface::as_raw(self), newprofile.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProfileUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnConnectionProfileEvents_Impl: windows_core::IUnknownImpl {
    fn OnProfileUpdate(&self, newprofile: windows_core::Ref<IMbnConnectionProfile>) -> windows_core::Result<()>;
}
impl IMbnConnectionProfileEvents_Vtbl {
    pub const fn new<Identity: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnProfileUpdate<Identity: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfileEvents_Impl::OnProfileUpdate(this, core::mem::transmute_copy(&newprofile)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionProfileEvents {}
windows_core::imp::define_interface!(IMbnConnectionProfileManager, IMbnConnectionProfileManager_Vtbl, 0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileManager, windows_core::IUnknown);
impl IMbnConnectionProfileManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<P0>(&self, mbninterface: P0) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionProfiles)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectionProfile<P0, P1>(&self, mbninterface: P0, profilename: P1) -> windows_core::Result<IMbnConnectionProfile>
    where
        P0: windows_core::Param<IMbnInterface>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionProfile)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), profilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateConnectionProfile<P0>(&self, xmlprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateConnectionProfile)(windows_core::Interface::as_raw(self), xmlprofile.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnectionProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnectionProfiles: usize,
    pub GetConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionProfileManager_Impl: windows_core::IUnknownImpl {
    fn GetConnectionProfiles(&self, mbninterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(&self, mbninterface: windows_core::Ref<IMbnInterface>, profilename: &windows_core::PCWSTR) -> windows_core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(&self, xmlprofile: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionProfileManager_Vtbl {
    pub const fn new<Identity: IMbnConnectionProfileManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectionProfiles<Identity: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionProfileManager_Impl::GetConnectionProfiles(this, core::mem::transmute_copy(&mbninterface)) {
                    Ok(ok__) => {
                        connectionprofiles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Identity: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, profilename: windows_core::PCWSTR, connectionprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnConnectionProfileManager_Impl::GetConnectionProfile(this, core::mem::transmute_copy(&mbninterface), core::mem::transmute(&profilename)) {
                    Ok(ok__) => {
                        connectionprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Identity: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlprofile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfileManager_Impl::CreateConnectionProfile(this, core::mem::transmute(&xmlprofile)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Identity, OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Identity, OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnConnectionProfileManager {}
windows_core::imp::define_interface!(IMbnConnectionProfileManagerEvents, IMbnConnectionProfileManagerEvents_Vtbl, 0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileManagerEvents, windows_core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    pub unsafe fn OnConnectionProfileArrival<P0>(&self, newconnectionprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectionProfileArrival)(windows_core::Interface::as_raw(self), newconnectionprofile.param().abi()).ok() }
    }
    pub unsafe fn OnConnectionProfileRemoval<P0>(&self, oldconnectionprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectionProfileRemoval)(windows_core::Interface::as_raw(self), oldconnectionprofile.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionProfileArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnConnectionProfileRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnConnectionProfileManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnConnectionProfileArrival(&self, newconnectionprofile: windows_core::Ref<IMbnConnectionProfile>) -> windows_core::Result<()>;
    fn OnConnectionProfileRemoval(&self, oldconnectionprofile: windows_core::Ref<IMbnConnectionProfile>) -> windows_core::Result<()>;
}
impl IMbnConnectionProfileManagerEvents_Vtbl {
    pub const fn new<Identity: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnectionProfileArrival<Identity: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newconnectionprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfileManagerEvents_Impl::OnConnectionProfileArrival(this, core::mem::transmute_copy(&newconnectionprofile)).into()
            }
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Identity: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldconnectionprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnConnectionProfileManagerEvents_Impl::OnConnectionProfileRemoval(this, core::mem::transmute_copy(&oldconnectionprofile)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Identity, OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnConnectionProfileManagerEvents {}
windows_core::imp::define_interface!(IMbnDeviceService, IMbnDeviceService_Vtbl, 0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
windows_core::imp::interface_hierarchy!(IMbnDeviceService, windows_core::IUnknown);
impl IMbnDeviceService {
    pub unsafe fn QuerySupportedCommands(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenCommandSession(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenCommandSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseCommandSession(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloseCommandSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetCommand)(windows_core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryCommand)(windows_core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenDataSession(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseDataSession(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloseDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteData)(windows_core::Interface::as_raw(self), deviceservicedata, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeviceServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsCommandSessionOpen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCommandSessionOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDataSessionOpen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDataSessionOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QuerySupportedCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CloseCommandSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCommand: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryCommand: usize,
    pub OpenDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CloseDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteData: usize,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCommandSessionOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDataSessionOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceService_Impl: windows_core::IUnknownImpl {
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
    fn IsCommandSessionOpen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsDataSessionOpen(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceService_Vtbl {
    pub const fn new<Identity: IMbnDeviceService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QuerySupportedCommands<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::QuerySupportedCommands(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenCommandSession<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::OpenCommandSession(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseCommandSession<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::CloseCommandSession(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCommand<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::SetCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&deviceservicedata)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryCommand<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::QueryCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&deviceservicedata)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenDataSession<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::OpenDataSession(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseDataSession<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::CloseDataSession(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteData<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::WriteData(this, core::mem::transmute_copy(&deviceservicedata)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::InterfaceID(this) {
                    Ok(ok__) => {
                        interfaceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceServiceID<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceserviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::DeviceServiceID(this) {
                    Ok(ok__) => {
                        deviceserviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::IsCommandSessionOpen(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Identity: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceService_Impl::IsDataSessionOpen(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuerySupportedCommands: QuerySupportedCommands::<Identity, OFFSET>,
            OpenCommandSession: OpenCommandSession::<Identity, OFFSET>,
            CloseCommandSession: CloseCommandSession::<Identity, OFFSET>,
            SetCommand: SetCommand::<Identity, OFFSET>,
            QueryCommand: QueryCommand::<Identity, OFFSET>,
            OpenDataSession: OpenDataSession::<Identity, OFFSET>,
            CloseDataSession: CloseDataSession::<Identity, OFFSET>,
            WriteData: WriteData::<Identity, OFFSET>,
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            DeviceServiceID: DeviceServiceID::<Identity, OFFSET>,
            IsCommandSessionOpen: IsCommandSessionOpen::<Identity, OFFSET>,
            IsDataSessionOpen: IsDataSessionOpen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceService {}
windows_core::imp::define_interface!(IMbnDeviceServiceStateEvents, IMbnDeviceServiceStateEvents_Vtbl, 0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
windows_core::imp::interface_hierarchy!(IMbnDeviceServiceStateEvents, windows_core::IUnknown);
impl IMbnDeviceServiceStateEvents {
    pub unsafe fn OnSessionsStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSessionsStateChange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfaceid), statechange).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceStateEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSessionsStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::HRESULT,
}
pub trait IMbnDeviceServiceStateEvents_Impl: windows_core::IUnknownImpl {
    fn OnSessionsStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::Result<()>;
}
impl IMbnDeviceServiceStateEvents_Vtbl {
    pub const fn new<Identity: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSessionsStateChange<Identity: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut core::ffi::c_void, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServiceStateEvents_Impl::OnSessionsStateChange(this, core::mem::transmute(&interfaceid), core::mem::transmute_copy(&statechange)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnDeviceServiceStateEvents {}
windows_core::imp::define_interface!(IMbnDeviceServicesContext, IMbnDeviceServicesContext_Vtbl, 0xfc5ac347_1592_4068_80bb_6a57580150d8);
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesContext, windows_core::IUnknown);
impl IMbnDeviceServicesContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateDeviceServices)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceService(&self, deviceserviceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceserviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MaxCommandSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxCommandSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MaxDataSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxDataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateDeviceServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateDeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxCommandSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesContext_Impl: windows_core::IUnknownImpl {
    fn EnumerateDeviceServices(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(&self, deviceserviceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceService>;
    fn MaxCommandSize(&self) -> windows_core::Result<u32>;
    fn MaxDataSize(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesContext_Vtbl {
    pub const fn new<Identity: IMbnDeviceServicesContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumerateDeviceServices<Identity: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceServicesContext_Impl::EnumerateDeviceServices(this) {
                    Ok(ok__) => {
                        deviceservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceService<Identity: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceserviceid: *mut core::ffi::c_void, mbndeviceservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceServicesContext_Impl::GetDeviceService(this, core::mem::transmute(&deviceserviceid)) {
                    Ok(ok__) => {
                        mbndeviceservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxCommandSize<Identity: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxcommandsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceServicesContext_Impl::MaxCommandSize(this) {
                    Ok(ok__) => {
                        maxcommandsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxDataSize<Identity: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxdatasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceServicesContext_Impl::MaxDataSize(this) {
                    Ok(ok__) => {
                        maxdatasize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumerateDeviceServices: EnumerateDeviceServices::<Identity, OFFSET>,
            GetDeviceService: GetDeviceService::<Identity, OFFSET>,
            MaxCommandSize: MaxCommandSize::<Identity, OFFSET>,
            MaxDataSize: MaxDataSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceServicesContext {}
windows_core::imp::define_interface!(IMbnDeviceServicesEvents, IMbnDeviceServicesEvents_Vtbl, 0x0a900c19_6824_4e97_b76e_cf239d0ca642);
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesEvents, windows_core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<P0>(&self, deviceservice: P0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnQuerySupportedCommandsComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), commandidlist, status, requestid).ok() }
    }
    pub unsafe fn OnOpenCommandSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOpenCommandSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok() }
    }
    pub unsafe fn OnCloseCommandSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCloseCommandSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetCommandComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), responseid, deviceservicedata, status, requestid).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnQueryCommandComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), responseid, deviceservicedata, status, requestid).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, deviceservice: P0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEventNotification)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), eventid, deviceservicedata).ok() }
    }
    pub unsafe fn OnOpenDataSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOpenDataSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok() }
    }
    pub unsafe fn OnCloseDataSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCloseDataSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok() }
    }
    pub unsafe fn OnWriteDataComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnWriteDataComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<P0>(&self, deviceservice: P0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadData)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), deviceservicedata).ok() }
    }
    pub unsafe fn OnInterfaceStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnInterfaceStateChange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfaceid), statechange).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuerySupportedCommandsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuerySupportedCommandsComplete: usize,
    pub OnOpenCommandSessionComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub OnCloseCommandSessionComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetCommandComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueryCommandComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueryCommandComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    pub OnOpenDataSessionComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub OnCloseDataSessionComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub OnWriteDataComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReadData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReadData: usize,
    pub OnInterfaceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesEvents_Impl: windows_core::IUnknownImpl {
    fn OnQuerySupportedCommandsComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnOpenCommandSessionComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnCloseCommandSessionComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnSetCommandComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnQueryCommandComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnEventNotification(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnOpenDataSessionComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnCloseDataSessionComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnWriteDataComplete(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>;
    fn OnReadData(&self, deviceservice: windows_core::Ref<IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnInterfaceStateChange(&self, interfaceid: &windows_core::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesEvents_Vtbl {
    pub const fn new<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnQuerySupportedCommandsComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&commandidlist), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnOpenCommandSessionComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnCloseCommandSessionComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnSetCommandComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnSetCommandComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&responseid), core::mem::transmute_copy(&deviceservicedata), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnQueryCommandComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnQueryCommandComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&responseid), core::mem::transmute_copy(&deviceservicedata), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnEventNotification<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnEventNotification(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&deviceservicedata)).into()
            }
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnOpenDataSessionComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnCloseDataSessionComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnWriteDataComplete<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, status: windows_core::HRESULT, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnWriteDataComplete(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&status), core::mem::transmute_copy(&requestid)).into()
            }
        }
        unsafe extern "system" fn OnReadData<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceservice: *mut core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnReadData(this, core::mem::transmute_copy(&deviceservice), core::mem::transmute_copy(&deviceservicedata)).into()
            }
        }
        unsafe extern "system" fn OnInterfaceStateChange<Identity: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut core::ffi::c_void, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnDeviceServicesEvents_Impl::OnInterfaceStateChange(this, core::mem::transmute(&interfaceid), core::mem::transmute_copy(&statechange)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQuerySupportedCommandsComplete: OnQuerySupportedCommandsComplete::<Identity, OFFSET>,
            OnOpenCommandSessionComplete: OnOpenCommandSessionComplete::<Identity, OFFSET>,
            OnCloseCommandSessionComplete: OnCloseCommandSessionComplete::<Identity, OFFSET>,
            OnSetCommandComplete: OnSetCommandComplete::<Identity, OFFSET>,
            OnQueryCommandComplete: OnQueryCommandComplete::<Identity, OFFSET>,
            OnEventNotification: OnEventNotification::<Identity, OFFSET>,
            OnOpenDataSessionComplete: OnOpenDataSessionComplete::<Identity, OFFSET>,
            OnCloseDataSessionComplete: OnCloseDataSessionComplete::<Identity, OFFSET>,
            OnWriteDataComplete: OnWriteDataComplete::<Identity, OFFSET>,
            OnReadData: OnReadData::<Identity, OFFSET>,
            OnInterfaceStateChange: OnInterfaceStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnDeviceServicesEvents {}
windows_core::imp::define_interface!(IMbnDeviceServicesManager, IMbnDeviceServicesManager_Vtbl, 0x20a26258_6811_4478_ac1d_13324e45e41c);
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesManager, windows_core::IUnknown);
impl IMbnDeviceServicesManager {
    pub unsafe fn GetDeviceServicesContext(&self, networkinterfaceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceServicesContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceServicesContext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(networkinterfaceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceServicesContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnDeviceServicesManager_Impl: windows_core::IUnknownImpl {
    fn GetDeviceServicesContext(&self, networkinterfaceid: &windows_core::BSTR) -> windows_core::Result<IMbnDeviceServicesContext>;
}
impl IMbnDeviceServicesManager_Vtbl {
    pub const fn new<Identity: IMbnDeviceServicesManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceServicesContext<Identity: IMbnDeviceServicesManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networkinterfaceid: *mut core::ffi::c_void, mbndevicescontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnDeviceServicesManager_Impl::GetDeviceServicesContext(this, core::mem::transmute(&networkinterfaceid)) {
                    Ok(ok__) => {
                        mbndevicescontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnDeviceServicesManager {}
windows_core::imp::define_interface!(IMbnInterface, IMbnInterface_Vtbl, 0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnInterface, windows_core::IUnknown);
impl IMbnInterface {
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetInterfaceCapability(&self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceCapability)(windows_core::Interface::as_raw(self), core::mem::transmute(interfacecaps)).ok() }
    }
    pub unsafe fn GetSubscriberInformation(&self) -> windows_core::Result<IMbnSubscriberInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubscriberInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReadyState(&self) -> windows_core::Result<MBN_READY_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InEmergencyMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InEmergencyMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHomeProvider(&self) -> windows_core::Result<MBN_PROVIDER> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHomeProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreferredProviders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetPreferredProviders)(windows_core::Interface::as_raw(self), preferredproviders, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleProviders)(windows_core::Interface::as_raw(self), age as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScanNetwork(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanNetwork)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnection(&self) -> windows_core::Result<IMbnConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInterfaceCapability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_INTERFACE_CAPS) -> windows_core::HRESULT,
    pub GetSubscriberInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_READY_STATE) -> windows_core::HRESULT,
    pub InEmergencyMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetHomeProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_PROVIDER) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPreferredProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    pub ScanNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterface_Impl: windows_core::IUnknownImpl {
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
impl IMbnInterface_Vtbl {
    pub const fn new<Identity: IMbnInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InterfaceID<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::InterfaceID(this) {
                    Ok(ok__) => {
                        interfaceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterface_Impl::GetInterfaceCapability(this, core::mem::transmute_copy(&interfacecaps)).into()
            }
        }
        unsafe extern "system" fn GetSubscriberInformation<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscriberinformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetSubscriberInformation(this) {
                    Ok(ok__) => {
                        subscriberinformation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetReadyState(this) {
                    Ok(ok__) => {
                        readystate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InEmergencyMode<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emergencymode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::InEmergencyMode(this) {
                    Ok(ok__) => {
                        emergencymode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHomeProvider<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetHomeProvider(this) {
                    Ok(ok__) => {
                        homeprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetPreferredProviders(this) {
                    Ok(ok__) => {
                        preferredproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::SetPreferredProviders(this, core::mem::transmute_copy(&preferredproviders)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetVisibleProviders(this, core::mem::transmute_copy(&age)) {
                    Ok(ok__) => {
                        visibleproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::ScanNetwork(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnection<Identity: IMbnInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbnconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterface_Impl::GetConnection(this) {
                    Ok(ok__) => {
                        mbnconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            GetInterfaceCapability: GetInterfaceCapability::<Identity, OFFSET>,
            GetSubscriberInformation: GetSubscriberInformation::<Identity, OFFSET>,
            GetReadyState: GetReadyState::<Identity, OFFSET>,
            InEmergencyMode: InEmergencyMode::<Identity, OFFSET>,
            GetHomeProvider: GetHomeProvider::<Identity, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, OFFSET>,
            SetPreferredProviders: SetPreferredProviders::<Identity, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, OFFSET>,
            GetConnection: GetConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterface as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnInterface {}
windows_core::imp::define_interface!(IMbnInterfaceEvents, IMbnInterfaceEvents_Vtbl, 0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnInterfaceEvents, windows_core::IUnknown);
impl IMbnInterfaceEvents {
    pub unsafe fn OnInterfaceCapabilityAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInterfaceCapabilityAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnSubscriberInformationChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSubscriberInformationChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnReadyStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadyStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnEmergencyModeChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEmergencyModeChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnHomeProviderAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnHomeProviderAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPreferredProvidersChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnSetPreferredProvidersComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetPreferredProvidersComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnScanNetworkComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInterfaceCapabilityAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSubscriberInformationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnReadyStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEmergencyModeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnHomeProviderAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetPreferredProvidersComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnInterfaceEvents_Impl: windows_core::IUnknownImpl {
    fn OnInterfaceCapabilityAvailable(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnSubscriberInformationChange(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnReadyStateChange(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnEmergencyModeChange(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnHomeProviderAvailable(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnPreferredProvidersChange(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnSetPreferredProvidersComplete(&self, newinterface: windows_core::Ref<IMbnInterface>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnScanNetworkComplete(&self, newinterface: windows_core::Ref<IMbnInterface>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnInterfaceEvents_Vtbl {
    pub const fn new<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnInterfaceCapabilityAvailable(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnSubscriberInformationChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnReadyStateChange<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnReadyStateChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnEmergencyModeChange<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnEmergencyModeChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnHomeProviderAvailable(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnPreferredProvidersChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnSetPreferredProvidersComplete(this, core::mem::transmute_copy(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceEvents_Impl::OnScanNetworkComplete(this, core::mem::transmute_copy(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInterfaceCapabilityAvailable: OnInterfaceCapabilityAvailable::<Identity, OFFSET>,
            OnSubscriberInformationChange: OnSubscriberInformationChange::<Identity, OFFSET>,
            OnReadyStateChange: OnReadyStateChange::<Identity, OFFSET>,
            OnEmergencyModeChange: OnEmergencyModeChange::<Identity, OFFSET>,
            OnHomeProviderAvailable: OnHomeProviderAvailable::<Identity, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, OFFSET>,
            OnSetPreferredProvidersComplete: OnSetPreferredProvidersComplete::<Identity, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnInterfaceEvents {}
windows_core::imp::define_interface!(IMbnInterfaceManager, IMbnInterfaceManager_Vtbl, 0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnInterfaceManager, windows_core::IUnknown);
impl IMbnInterfaceManager {
    pub unsafe fn GetInterface<P0>(&self, interfaceid: P0) -> windows_core::Result<IMbnInterface>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInterface)(windows_core::Interface::as_raw(self), interfaceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInterfaces: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterfaceManager_Impl: windows_core::IUnknownImpl {
    fn GetInterface(&self, interfaceid: &windows_core::PCWSTR) -> windows_core::Result<IMbnInterface>;
    fn GetInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnInterfaceManager_Vtbl {
    pub const fn new<Identity: IMbnInterfaceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInterface<Identity: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaceid: windows_core::PCWSTR, mbninterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterfaceManager_Impl::GetInterface(this, core::mem::transmute(&interfaceid)) {
                    Ok(ok__) => {
                        mbninterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInterfaces<Identity: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnInterfaceManager_Impl::GetInterfaces(this) {
                    Ok(ok__) => {
                        mbninterfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterface: GetInterface::<Identity, OFFSET>,
            GetInterfaces: GetInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnInterfaceManager {}
windows_core::imp::define_interface!(IMbnInterfaceManagerEvents, IMbnInterfaceManagerEvents_Vtbl, 0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnInterfaceManagerEvents, windows_core::IUnknown);
impl IMbnInterfaceManagerEvents {
    pub unsafe fn OnInterfaceArrival<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInterfaceArrival)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnInterfaceRemoval<P0>(&self, oldinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInterfaceRemoval)(windows_core::Interface::as_raw(self), oldinterface.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInterfaceArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInterfaceRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnInterfaceManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnInterfaceArrival(&self, newinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
    fn OnInterfaceRemoval(&self, oldinterface: windows_core::Ref<IMbnInterface>) -> windows_core::Result<()>;
}
impl IMbnInterfaceManagerEvents_Vtbl {
    pub const fn new<Identity: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInterfaceArrival<Identity: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceManagerEvents_Impl::OnInterfaceArrival(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnInterfaceRemoval<Identity: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnInterfaceManagerEvents_Impl::OnInterfaceRemoval(this, core::mem::transmute_copy(&oldinterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInterfaceArrival: OnInterfaceArrival::<Identity, OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnInterfaceManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnInterfaceManagerEvents {}
windows_core::imp::define_interface!(IMbnMultiCarrier, IMbnMultiCarrier_Vtbl, 0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnMultiCarrier, windows_core::IUnknown);
impl IMbnMultiCarrier {
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetHomeProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(homeprovider), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreferredProviders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleProviders)(windows_core::Interface::as_raw(self), age as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedCellularClasses)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentCellularClass(&self) -> windows_core::Result<MBN_CELLULAR_CLASS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentCellularClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScanNetwork(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanNetwork)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHomeProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const MBN_PROVIDER2, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPreferredProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPreferredProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVisibleProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVisibleProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSupportedCellularClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSupportedCellularClasses: usize,
    pub GetCurrentCellularClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_CELLULAR_CLASS) -> windows_core::HRESULT,
    pub ScanNetwork: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnMultiCarrier_Impl: windows_core::IUnknownImpl {
    fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> windows_core::Result<u32>;
    fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetSupportedCellularClasses(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(&self) -> windows_core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnMultiCarrier_Vtbl {
    pub const fn new<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHomeProvider<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::SetHomeProvider(this, core::mem::transmute_copy(&homeprovider)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::GetPreferredProviders(this) {
                    Ok(ok__) => {
                        preferredmulticarrierproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::GetVisibleProviders(this, core::mem::transmute_copy(&age)) {
                    Ok(ok__) => {
                        visibleproviders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::GetSupportedCellularClasses(this) {
                    Ok(ok__) => {
                        cellularclasses.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::GetCurrentCellularClass(this) {
                    Ok(ok__) => {
                        currentcellularclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnMultiCarrier_Impl::ScanNetwork(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomeProvider: SetHomeProvider::<Identity, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Identity, OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Identity, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnMultiCarrier as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnMultiCarrier {}
windows_core::imp::define_interface!(IMbnMultiCarrierEvents, IMbnMultiCarrierEvents_Vtbl, 0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnMultiCarrierEvents, windows_core::IUnknown);
impl IMbnMultiCarrierEvents {
    pub unsafe fn OnSetHomeProviderComplete<P0>(&self, mbninterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetHomeProviderComplete)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnCurrentCellularClassChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCurrentCellularClassChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok() }
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPreferredProvidersChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok() }
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, mbninterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnScanNetworkComplete)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnInterfaceCapabilityChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInterfaceCapabilityChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetHomeProviderComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnCurrentCellularClassChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnInterfaceCapabilityChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnMultiCarrierEvents_Impl: windows_core::IUnknownImpl {
    fn OnSetHomeProviderComplete(&self, mbninterface: windows_core::Ref<IMbnMultiCarrier>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnCurrentCellularClassChange(&self, mbninterface: windows_core::Ref<IMbnMultiCarrier>) -> windows_core::Result<()>;
    fn OnPreferredProvidersChange(&self, mbninterface: windows_core::Ref<IMbnMultiCarrier>) -> windows_core::Result<()>;
    fn OnScanNetworkComplete(&self, mbninterface: windows_core::Ref<IMbnMultiCarrier>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnInterfaceCapabilityChange(&self, mbninterface: windows_core::Ref<IMbnMultiCarrier>) -> windows_core::Result<()>;
}
impl IMbnMultiCarrierEvents_Vtbl {
    pub const fn new<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetHomeProviderComplete<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnMultiCarrierEvents_Impl::OnSetHomeProviderComplete(this, core::mem::transmute_copy(&mbninterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnMultiCarrierEvents_Impl::OnCurrentCellularClassChange(this, core::mem::transmute_copy(&mbninterface)).into()
            }
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnMultiCarrierEvents_Impl::OnPreferredProvidersChange(this, core::mem::transmute_copy(&mbninterface)).into()
            }
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnMultiCarrierEvents_Impl::OnScanNetworkComplete(this, core::mem::transmute_copy(&mbninterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Identity: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mbninterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnMultiCarrierEvents_Impl::OnInterfaceCapabilityChange(this, core::mem::transmute_copy(&mbninterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Identity, OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Identity, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnMultiCarrierEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnMultiCarrierEvents {}
windows_core::imp::define_interface!(IMbnPin, IMbnPin_Vtbl, 0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnPin, windows_core::IUnknown);
impl IMbnPin {
    pub unsafe fn PinType(&self) -> windows_core::Result<MBN_PIN_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PinFormat(&self) -> windows_core::Result<MBN_PIN_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PinLengthMin(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinLengthMin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PinLengthMax(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinLengthMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PinMode(&self) -> windows_core::Result<MBN_PIN_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PinMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enable<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Disable<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enter<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Change<P0, P1>(&self, pin: P0, newpin: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Change)(windows_core::Interface::as_raw(self), pin.param().abi(), newpin.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unblock<P0, P1>(&self, puk: P0, newpin: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unblock)(windows_core::Interface::as_raw(self), puk.param().abi(), newpin.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPinManager(&self) -> windows_core::Result<IMbnPinManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPinManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PinType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_PIN_TYPE) -> windows_core::HRESULT,
    pub PinFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_PIN_FORMAT) -> windows_core::HRESULT,
    pub PinLengthMin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PinLengthMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PinMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_PIN_MODE) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Change: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Unblock: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetPinManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnPin_Impl: windows_core::IUnknownImpl {
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
impl IMbnPin_Vtbl {
    pub const fn new<Identity: IMbnPin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PinType<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::PinType(this) {
                    Ok(ok__) => {
                        pintype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PinFormat<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::PinFormat(this) {
                    Ok(ok__) => {
                        pinformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PinLengthMin<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlengthmin: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::PinLengthMin(this) {
                    Ok(ok__) => {
                        pinlengthmin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PinLengthMax<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlengthmax: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::PinLengthMax(this) {
                    Ok(ok__) => {
                        pinlengthmax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PinMode<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::PinMode(this) {
                    Ok(ok__) => {
                        pinmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enable<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::Enable(this, core::mem::transmute(&pin)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disable<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::Disable(this, core::mem::transmute(&pin)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enter<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::Enter(this, core::mem::transmute(&pin)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Change<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: windows_core::PCWSTR, newpin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::Change(this, core::mem::transmute(&pin), core::mem::transmute(&newpin)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unblock<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puk: windows_core::PCWSTR, newpin: windows_core::PCWSTR, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::Unblock(this, core::mem::transmute(&puk), core::mem::transmute(&newpin)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPinManager<Identity: IMbnPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPin_Impl::GetPinManager(this) {
                    Ok(ok__) => {
                        pinmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PinType: PinType::<Identity, OFFSET>,
            PinFormat: PinFormat::<Identity, OFFSET>,
            PinLengthMin: PinLengthMin::<Identity, OFFSET>,
            PinLengthMax: PinLengthMax::<Identity, OFFSET>,
            PinMode: PinMode::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Disable: Disable::<Identity, OFFSET>,
            Enter: Enter::<Identity, OFFSET>,
            Change: Change::<Identity, OFFSET>,
            Unblock: Unblock::<Identity, OFFSET>,
            GetPinManager: GetPinManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnPin {}
windows_core::imp::define_interface!(IMbnPinEvents, IMbnPinEvents_Vtbl, 0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnPinEvents, windows_core::IUnknown);
impl IMbnPinEvents {
    pub unsafe fn OnEnableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEnableComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok() }
    }
    pub unsafe fn OnDisableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisableComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok() }
    }
    pub unsafe fn OnEnterComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEnterComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok() }
    }
    pub unsafe fn OnChangeComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChangeComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok() }
    }
    pub unsafe fn OnUnblockComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnUnblockComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEnableComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnDisableComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnEnterComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnChangeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnUnblockComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnPinEvents_Impl: windows_core::IUnknownImpl {
    fn OnEnableComplete(&self, pin: windows_core::Ref<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnDisableComplete(&self, pin: windows_core::Ref<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnEnterComplete(&self, pin: windows_core::Ref<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnChangeComplete(&self, pin: windows_core::Ref<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnUnblockComplete(&self, pin: windows_core::Ref<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnPinEvents_Vtbl {
    pub const fn new<Identity: IMbnPinEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEnableComplete<Identity: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinEvents_Impl::OnEnableComplete(this, core::mem::transmute_copy(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnDisableComplete<Identity: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinEvents_Impl::OnDisableComplete(this, core::mem::transmute_copy(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnEnterComplete<Identity: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinEvents_Impl::OnEnterComplete(this, core::mem::transmute_copy(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnChangeComplete<Identity: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinEvents_Impl::OnChangeComplete(this, core::mem::transmute_copy(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnUnblockComplete<Identity: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinEvents_Impl::OnUnblockComplete(this, core::mem::transmute_copy(&pin), core::mem::transmute_copy(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEnableComplete: OnEnableComplete::<Identity, OFFSET>,
            OnDisableComplete: OnDisableComplete::<Identity, OFFSET>,
            OnEnterComplete: OnEnterComplete::<Identity, OFFSET>,
            OnChangeComplete: OnChangeComplete::<Identity, OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnPinEvents {}
windows_core::imp::define_interface!(IMbnPinManager, IMbnPinManager_Vtbl, 0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnPinManager, windows_core::IUnknown);
impl IMbnPinManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPinList)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> windows_core::Result<IMbnPin> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPin)(windows_core::Interface::as_raw(self), pintype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPinState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPinState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPinList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPinList: usize,
    pub GetPin: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_PIN_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPinState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManager_Impl: windows_core::IUnknownImpl {
    fn GetPinList(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(&self, pintype: MBN_PIN_TYPE) -> windows_core::Result<IMbnPin>;
    fn GetPinState(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManager_Vtbl {
    pub const fn new<Identity: IMbnPinManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPinList<Identity: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPinManager_Impl::GetPinList(this) {
                    Ok(ok__) => {
                        pinlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPin<Identity: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPinManager_Impl::GetPin(this, core::mem::transmute_copy(&pintype)) {
                    Ok(ok__) => {
                        pin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPinState<Identity: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnPinManager_Impl::GetPinState(this) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPinList: GetPinList::<Identity, OFFSET>,
            GetPin: GetPin::<Identity, OFFSET>,
            GetPinState: GetPinState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnPinManager {}
windows_core::imp::define_interface!(IMbnPinManagerEvents, IMbnPinManagerEvents_Vtbl, 0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnPinManagerEvents, windows_core::IUnknown);
impl IMbnPinManagerEvents {
    pub unsafe fn OnPinListAvailable<P0>(&self, pinmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPinManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPinListAvailable)(windows_core::Interface::as_raw(self), pinmanager.param().abi()).ok() }
    }
    pub unsafe fn OnGetPinStateComplete<P0>(&self, pinmanager: P0, pininfo: MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPinManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnGetPinStateComplete)(windows_core::Interface::as_raw(self), pinmanager.param().abi(), core::mem::transmute(pininfo), requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPinListAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnGetPinStateComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnPinManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnPinListAvailable(&self, pinmanager: windows_core::Ref<IMbnPinManager>) -> windows_core::Result<()>;
    fn OnGetPinStateComplete(&self, pinmanager: windows_core::Ref<IMbnPinManager>, pininfo: &MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnPinManagerEvents_Vtbl {
    pub const fn new<Identity: IMbnPinManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPinListAvailable<Identity: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinManagerEvents_Impl::OnPinListAvailable(this, core::mem::transmute_copy(&pinmanager)).into()
            }
        }
        unsafe extern "system" fn OnGetPinStateComplete<Identity: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinmanager: *mut core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnPinManagerEvents_Impl::OnGetPinStateComplete(this, core::mem::transmute_copy(&pinmanager), core::mem::transmute(&pininfo), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPinListAvailable: OnPinListAvailable::<Identity, OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnPinManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnPinManagerEvents {}
windows_core::imp::define_interface!(IMbnRadio, IMbnRadio_Vtbl, 0xdccccab6_201f_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnRadio, windows_core::IUnknown);
impl IMbnRadio {
    pub unsafe fn SoftwareRadioState(&self) -> windows_core::Result<MBN_RADIO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SoftwareRadioState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HardwareRadioState(&self) -> windows_core::Result<MBN_RADIO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HardwareRadioState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetSoftwareRadioState)(windows_core::Interface::as_raw(self), radiostate, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadio_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SoftwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_RADIO) -> windows_core::HRESULT,
    pub HardwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_RADIO) -> windows_core::HRESULT,
    pub SetSoftwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_RADIO, *mut u32) -> windows_core::HRESULT,
}
pub trait IMbnRadio_Impl: windows_core::IUnknownImpl {
    fn SoftwareRadioState(&self) -> windows_core::Result<MBN_RADIO>;
    fn HardwareRadioState(&self) -> windows_core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> windows_core::Result<u32>;
}
impl IMbnRadio_Vtbl {
    pub const fn new<Identity: IMbnRadio_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SoftwareRadioState<Identity: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRadio_Impl::SoftwareRadioState(this) {
                    Ok(ok__) => {
                        softwareradiostate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HardwareRadioState<Identity: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRadio_Impl::HardwareRadioState(this) {
                    Ok(ok__) => {
                        hardwareradiostate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Identity: IMbnRadio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRadio_Impl::SetSoftwareRadioState(this, core::mem::transmute_copy(&radiostate)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SoftwareRadioState: SoftwareRadioState::<Identity, OFFSET>,
            HardwareRadioState: HardwareRadioState::<Identity, OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRadio as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnRadio {}
windows_core::imp::define_interface!(IMbnRadioEvents, IMbnRadioEvents_Vtbl, 0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnRadioEvents, windows_core::IUnknown);
impl IMbnRadioEvents {
    pub unsafe fn OnRadioStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRadio>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRadioStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnSetSoftwareRadioStateComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRadio>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetSoftwareRadioStateComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRadioStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetSoftwareRadioStateComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnRadioEvents_Impl: windows_core::IUnknownImpl {
    fn OnRadioStateChange(&self, newinterface: windows_core::Ref<IMbnRadio>) -> windows_core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(&self, newinterface: windows_core::Ref<IMbnRadio>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnRadioEvents_Vtbl {
    pub const fn new<Identity: IMbnRadioEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRadioStateChange<Identity: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRadioEvents_Impl::OnRadioStateChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Identity: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRadioEvents_Impl::OnSetSoftwareRadioStateComplete(this, core::mem::transmute_copy(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRadioStateChange: OnRadioStateChange::<Identity, OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRadioEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnRadioEvents {}
windows_core::imp::define_interface!(IMbnRegistration, IMbnRegistration_Vtbl, 0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnRegistration, windows_core::IUnknown);
impl IMbnRegistration {
    pub unsafe fn GetRegisterState(&self) -> windows_core::Result<MBN_REGISTER_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisterState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRegisterMode(&self) -> windows_core::Result<MBN_REGISTER_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisterMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRoamingText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRoamingText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAvailableDataClasses(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableDataClasses)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentDataClass(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentDataClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRegistrationNetworkError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistrationNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPacketAttachNetworkError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPacketAttachNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRegisterMode<P1>(&self, registermode: MBN_REGISTER_MODE, providerid: P1, dataclass: u32) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetRegisterMode)(windows_core::Interface::as_raw(self), registermode, providerid.param().abi(), dataclass, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRegisterState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_REGISTER_STATE) -> windows_core::HRESULT,
    pub GetRegisterMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_REGISTER_MODE) -> windows_core::HRESULT,
    pub GetProviderID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRoamingText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAvailableDataClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRegistrationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPacketAttachNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRegisterMode: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_REGISTER_MODE, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMbnRegistration_Impl: windows_core::IUnknownImpl {
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
impl IMbnRegistration_Vtbl {
    pub const fn new<Identity: IMbnRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRegisterState<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetRegisterState(this) {
                    Ok(ok__) => {
                        registerstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisterMode<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetRegisterMode(this) {
                    Ok(ok__) => {
                        registermode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderID<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetProviderID(this) {
                    Ok(ok__) => {
                        providerid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderName<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetProviderName(this) {
                    Ok(ok__) => {
                        providername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRoamingText<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, roamingtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetRoamingText(this) {
                    Ok(ok__) => {
                        roamingtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availabledataclasses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetAvailableDataClasses(this) {
                    Ok(ok__) => {
                        availabledataclasses.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentdataclass: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetCurrentDataClass(this) {
                    Ok(ok__) => {
                        currentdataclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registrationnetworkerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetRegistrationNetworkError(this) {
                    Ok(ok__) => {
                        registrationnetworkerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packetattachnetworkerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::GetPacketAttachNetworkError(this) {
                    Ok(ok__) => {
                        packetattachnetworkerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegisterMode<Identity: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: windows_core::PCWSTR, dataclass: u32, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnRegistration_Impl::SetRegisterMode(this, core::mem::transmute_copy(&registermode), core::mem::transmute(&providerid), core::mem::transmute_copy(&dataclass)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRegisterState: GetRegisterState::<Identity, OFFSET>,
            GetRegisterMode: GetRegisterMode::<Identity, OFFSET>,
            GetProviderID: GetProviderID::<Identity, OFFSET>,
            GetProviderName: GetProviderName::<Identity, OFFSET>,
            GetRoamingText: GetRoamingText::<Identity, OFFSET>,
            GetAvailableDataClasses: GetAvailableDataClasses::<Identity, OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Identity, OFFSET>,
            GetRegistrationNetworkError: GetRegistrationNetworkError::<Identity, OFFSET>,
            GetPacketAttachNetworkError: GetPacketAttachNetworkError::<Identity, OFFSET>,
            SetRegisterMode: SetRegisterMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnRegistration {}
windows_core::imp::define_interface!(IMbnRegistrationEvents, IMbnRegistrationEvents_Vtbl, 0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnRegistrationEvents, windows_core::IUnknown);
impl IMbnRegistrationEvents {
    pub unsafe fn OnRegisterModeAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRegisterModeAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnRegisterStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRegisterStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnPacketServiceStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPacketServiceStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
    pub unsafe fn OnSetRegisterModeComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetRegisterModeComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRegisterModeAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnRegisterStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPacketServiceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetRegisterModeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMbnRegistrationEvents_Impl: windows_core::IUnknownImpl {
    fn OnRegisterModeAvailable(&self, newinterface: windows_core::Ref<IMbnRegistration>) -> windows_core::Result<()>;
    fn OnRegisterStateChange(&self, newinterface: windows_core::Ref<IMbnRegistration>) -> windows_core::Result<()>;
    fn OnPacketServiceStateChange(&self, newinterface: windows_core::Ref<IMbnRegistration>) -> windows_core::Result<()>;
    fn OnSetRegisterModeComplete(&self, newinterface: windows_core::Ref<IMbnRegistration>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMbnRegistrationEvents_Vtbl {
    pub const fn new<Identity: IMbnRegistrationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRegisterModeAvailable<Identity: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRegistrationEvents_Impl::OnRegisterModeAvailable(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnRegisterStateChange<Identity: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRegistrationEvents_Impl::OnRegisterStateChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Identity: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRegistrationEvents_Impl::OnPacketServiceStateChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Identity: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnRegistrationEvents_Impl::OnSetRegisterModeComplete(this, core::mem::transmute_copy(&newinterface), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Identity, OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Identity, OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Identity, OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnRegistrationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnRegistrationEvents {}
windows_core::imp::define_interface!(IMbnServiceActivation, IMbnServiceActivation_Vtbl, 0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnServiceActivation, windows_core::IUnknown);
impl IMbnServiceActivation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), vendorspecificdata, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivation_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivation_Vtbl {
    pub const fn new<Identity: IMbnServiceActivation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMbnServiceActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnServiceActivation_Impl::Activate(this, core::mem::transmute_copy(&vendorspecificdata)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnServiceActivation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnServiceActivation {}
windows_core::imp::define_interface!(IMbnServiceActivationEvents, IMbnServiceActivationEvents_Vtbl, 0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnServiceActivationEvents, windows_core::IUnknown);
impl IMbnServiceActivationEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<P0>(&self, serviceactivation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnServiceActivation>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnActivationComplete)(windows_core::Interface::as_raw(self), serviceactivation.param().abi(), vendorspecificdata, requestid, status, networkerror).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, u32, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivationComplete: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEvents_Impl: windows_core::IUnknownImpl {
    fn OnActivationComplete(&self, serviceactivation: windows_core::Ref<IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEvents_Vtbl {
    pub const fn new<Identity: IMbnServiceActivationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivationComplete<Identity: IMbnServiceActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceactivation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnServiceActivationEvents_Impl::OnActivationComplete(this, core::mem::transmute_copy(&serviceactivation), core::mem::transmute_copy(&vendorspecificdata), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status), core::mem::transmute_copy(&networkerror)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivationComplete: OnActivationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnServiceActivationEvents {}
windows_core::imp::define_interface!(IMbnSignal, IMbnSignal_Vtbl, 0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSignal, windows_core::IUnknown);
impl IMbnSignal {
    pub unsafe fn GetSignalStrength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignalStrength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignalError(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignalError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSignalStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSignalError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMbnSignal_Impl: windows_core::IUnknownImpl {
    fn GetSignalStrength(&self) -> windows_core::Result<u32>;
    fn GetSignalError(&self) -> windows_core::Result<u32>;
}
impl IMbnSignal_Vtbl {
    pub const fn new<Identity: IMbnSignal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSignalStrength<Identity: IMbnSignal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signalstrength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSignal_Impl::GetSignalStrength(this) {
                    Ok(ok__) => {
                        signalstrength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignalError<Identity: IMbnSignal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signalerror: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSignal_Impl::GetSignalError(this) {
                    Ok(ok__) => {
                        signalerror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignalStrength: GetSignalStrength::<Identity, OFFSET>,
            GetSignalError: GetSignalError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSignal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnSignal {}
windows_core::imp::define_interface!(IMbnSignalEvents, IMbnSignalEvents_Vtbl, 0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSignalEvents, windows_core::IUnknown);
impl IMbnSignalEvents {
    pub unsafe fn OnSignalStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSignal>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSignalStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSignalStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMbnSignalEvents_Impl: windows_core::IUnknownImpl {
    fn OnSignalStateChange(&self, newinterface: windows_core::Ref<IMbnSignal>) -> windows_core::Result<()>;
}
impl IMbnSignalEvents_Vtbl {
    pub const fn new<Identity: IMbnSignalEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSignalStateChange<Identity: IMbnSignalEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSignalEvents_Impl::OnSignalStateChange(this, core::mem::transmute_copy(&newinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSignalEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnSignalEvents {}
windows_core::imp::define_interface!(IMbnSms, IMbnSms_Vtbl, 0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSms, windows_core::IUnknown);
impl IMbnSms {
    pub unsafe fn GetSmsConfiguration(&self) -> windows_core::Result<IMbnSmsConfiguration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSmsConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSmsConfiguration<P0>(&self, smsconfiguration: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IMbnSmsConfiguration>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetSmsConfiguration)(windows_core::Interface::as_raw(self), smsconfiguration.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SmsSendPdu<P0>(&self, pdudata: P0, size: u8) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsSendPdu)(windows_core::Interface::as_raw(self), pdudata.param().abi(), size, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdma<P0>(&self, address: P0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsSendCdma)(windows_core::Interface::as_raw(self), address.param().abi(), encoding, language, sizeincharacters, message, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsSendCdmaPdu)(windows_core::Interface::as_raw(self), message, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsRead)(windows_core::Interface::as_raw(self), smsfilter, smsformat, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsDelete)(windows_core::Interface::as_raw(self), smsfilter, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSmsStatus(&self) -> windows_core::Result<MBN_SMS_STATUS_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSmsStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSms_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSmsConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSmsConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SmsSendPdu: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdma: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, MBN_SMS_CDMA_ENCODING, MBN_SMS_CDMA_LANG, u32, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdma: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SmsSendCdmaPdu: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmsSendCdmaPdu: usize,
    pub SmsRead: unsafe extern "system" fn(*mut core::ffi::c_void, *const MBN_SMS_FILTER, MBN_SMS_FORMAT, *mut u32) -> windows_core::HRESULT,
    pub SmsDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *const MBN_SMS_FILTER, *mut u32) -> windows_core::HRESULT,
    pub GetSmsStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_STATUS_INFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSms_Impl: windows_core::IUnknownImpl {
    fn GetSmsConfiguration(&self) -> windows_core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(&self, smsconfiguration: windows_core::Ref<IMbnSmsConfiguration>) -> windows_core::Result<u32>;
    fn SmsSendPdu(&self, pdudata: &windows_core::PCWSTR, size: u8) -> windows_core::Result<u32>;
    fn SmsSendCdma(&self, address: &windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
    fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<u32>;
    fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> windows_core::Result<u32>;
    fn GetSmsStatus(&self) -> windows_core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSms_Vtbl {
    pub const fn new<Identity: IMbnSms_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSmsConfiguration<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::GetSmsConfiguration(this) {
                    Ok(ok__) => {
                        smsconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsconfiguration: *mut core::ffi::c_void, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SetSmsConfiguration(this, core::mem::transmute_copy(&smsconfiguration)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsSendPdu<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdudata: windows_core::PCWSTR, size: u8, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SmsSendPdu(this, core::mem::transmute(&pdudata), core::mem::transmute_copy(&size)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsSendCdma<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SmsSendCdma(this, core::mem::transmute(&address), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&language), core::mem::transmute_copy(&sizeincharacters), core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SmsSendCdmaPdu(this, core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsRead<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SmsRead(this, core::mem::transmute_copy(&smsfilter), core::mem::transmute_copy(&smsformat)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsDelete<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::SmsDelete(this, core::mem::transmute_copy(&smsfilter)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSmsStatus<Identity: IMbnSms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSms_Impl::GetSmsStatus(this) {
                    Ok(ok__) => {
                        smsstatusinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSmsConfiguration: GetSmsConfiguration::<Identity, OFFSET>,
            SetSmsConfiguration: SetSmsConfiguration::<Identity, OFFSET>,
            SmsSendPdu: SmsSendPdu::<Identity, OFFSET>,
            SmsSendCdma: SmsSendCdma::<Identity, OFFSET>,
            SmsSendCdmaPdu: SmsSendCdmaPdu::<Identity, OFFSET>,
            SmsRead: SmsRead::<Identity, OFFSET>,
            SmsDelete: SmsDelete::<Identity, OFFSET>,
            GetSmsStatus: GetSmsStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSms as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSms {}
windows_core::imp::define_interface!(IMbnSmsConfiguration, IMbnSmsConfiguration_Vtbl, 0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSmsConfiguration, windows_core::IUnknown);
impl IMbnSmsConfiguration {
    pub unsafe fn ServiceCenterAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceCenterAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceCenterAddress<P0>(&self, scaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServiceCenterAddress)(windows_core::Interface::as_raw(self), scaddress.param().abi()).ok() }
    }
    pub unsafe fn MaxMessageIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxMessageIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CdmaShortMsgSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CdmaShortMsgSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SmsFormat(&self) -> windows_core::Result<MBN_SMS_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmsFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSmsFormat)(windows_core::Interface::as_raw(self), smsformat).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ServiceCenterAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceCenterAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MaxMessageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CdmaShortMsgSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SmsFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_FORMAT) -> windows_core::HRESULT,
    pub SetSmsFormat: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_SMS_FORMAT) -> windows_core::HRESULT,
}
pub trait IMbnSmsConfiguration_Impl: windows_core::IUnknownImpl {
    fn ServiceCenterAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceCenterAddress(&self, scaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn MaxMessageIndex(&self) -> windows_core::Result<u32>;
    fn CdmaShortMsgSize(&self) -> windows_core::Result<u32>;
    fn SmsFormat(&self) -> windows_core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<()>;
}
impl IMbnSmsConfiguration_Vtbl {
    pub const fn new<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServiceCenterAddress<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsConfiguration_Impl::ServiceCenterAddress(this) {
                    Ok(ok__) => {
                        scaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scaddress: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsConfiguration_Impl::SetServiceCenterAddress(this, core::mem::transmute(&scaddress)).into()
            }
        }
        unsafe extern "system" fn MaxMessageIndex<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsConfiguration_Impl::MaxMessageIndex(this) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shortmsgsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsConfiguration_Impl::CdmaShortMsgSize(this) {
                    Ok(ok__) => {
                        shortmsgsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmsFormat<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsConfiguration_Impl::SmsFormat(this) {
                    Ok(ok__) => {
                        smsformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmsFormat<Identity: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsConfiguration_Impl::SetSmsFormat(this, core::mem::transmute_copy(&smsformat)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ServiceCenterAddress: ServiceCenterAddress::<Identity, OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Identity, OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Identity, OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Identity, OFFSET>,
            SmsFormat: SmsFormat::<Identity, OFFSET>,
            SetSmsFormat: SetSmsFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsConfiguration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMbnSmsConfiguration {}
windows_core::imp::define_interface!(IMbnSmsEvents, IMbnSmsEvents_Vtbl, 0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSmsEvents, windows_core::IUnknown);
impl IMbnSmsEvents {
    pub unsafe fn OnSmsConfigurationChange<P0>(&self, sms: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsConfigurationChange)(windows_core::Interface::as_raw(self), sms.param().abi()).ok() }
    }
    pub unsafe fn OnSetSmsConfigurationComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetSmsConfigurationComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnSmsSendComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsSendComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsReadComplete<P0>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsReadComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), smsformat, readmsgs, moremsgs, requestid, status).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<P0>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsNewClass0Message)(windows_core::Interface::as_raw(self), sms.param().abi(), smsformat, readmsgs).ok() }
    }
    pub unsafe fn OnSmsDeleteComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsDeleteComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok() }
    }
    pub unsafe fn OnSmsStatusChange<P0>(&self, sms: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSmsStatusChange)(windows_core::Interface::as_raw(self), sms.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSmsConfigurationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetSmsConfigurationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnSmsSendComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsReadComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_SMS_FORMAT, *const super::super::System::Com::SAFEARRAY, super::super::Foundation::VARIANT_BOOL, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsReadComplete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSmsNewClass0Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_SMS_FORMAT, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSmsNewClass0Message: usize,
    pub OnSmsDeleteComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnSmsStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsEvents_Impl: windows_core::IUnknownImpl {
    fn OnSmsConfigurationChange(&self, sms: windows_core::Ref<IMbnSms>) -> windows_core::Result<()>;
    fn OnSetSmsConfigurationComplete(&self, sms: windows_core::Ref<IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsSendComplete(&self, sms: windows_core::Ref<IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsReadComplete(&self, sms: windows_core::Ref<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsNewClass0Message(&self, sms: windows_core::Ref<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnSmsDeleteComplete(&self, sms: windows_core::Ref<IMbnSms>, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnSmsStatusChange(&self, sms: windows_core::Ref<IMbnSms>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsEvents_Vtbl {
    pub const fn new<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSmsConfigurationChange<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsConfigurationChange(this, core::mem::transmute_copy(&sms)).into()
            }
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSetSmsConfigurationComplete(this, core::mem::transmute_copy(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnSmsSendComplete<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsSendComplete(this, core::mem::transmute_copy(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnSmsReadComplete<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsReadComplete(this, core::mem::transmute_copy(&sms), core::mem::transmute_copy(&smsformat), core::mem::transmute_copy(&readmsgs), core::mem::transmute_copy(&moremsgs), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsNewClass0Message(this, core::mem::transmute_copy(&sms), core::mem::transmute_copy(&smsformat), core::mem::transmute_copy(&readmsgs)).into()
            }
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void, requestid: u32, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsDeleteComplete(this, core::mem::transmute_copy(&sms), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn OnSmsStatusChange<Identity: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sms: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnSmsEvents_Impl::OnSmsStatusChange(this, core::mem::transmute_copy(&sms)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSmsConfigurationChange: OnSmsConfigurationChange::<Identity, OFFSET>,
            OnSetSmsConfigurationComplete: OnSetSmsConfigurationComplete::<Identity, OFFSET>,
            OnSmsSendComplete: OnSmsSendComplete::<Identity, OFFSET>,
            OnSmsReadComplete: OnSmsReadComplete::<Identity, OFFSET>,
            OnSmsNewClass0Message: OnSmsNewClass0Message::<Identity, OFFSET>,
            OnSmsDeleteComplete: OnSmsDeleteComplete::<Identity, OFFSET>,
            OnSmsStatusChange: OnSmsStatusChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsEvents {}
windows_core::imp::define_interface!(IMbnSmsReadMsgPdu, IMbnSmsReadMsgPdu_Vtbl, 0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSmsReadMsgPdu, windows_core::IUnknown);
impl IMbnSmsReadMsgPdu {
    pub unsafe fn Index(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PduData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PduData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgPdu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_MSG_STATUS) -> windows_core::HRESULT,
    pub PduData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgPdu_Impl: windows_core::IUnknownImpl {
    fn Index(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS>;
    fn PduData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsReadMsgPdu_Vtbl {
    pub const fn new<Identity: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Index<Identity: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgPdu_Impl::Index(this) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgPdu_Impl::Status(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PduData<Identity: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdudata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgPdu_Impl::PduData(this) {
                    Ok(ok__) => {
                        pdudata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Message<Identity: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgPdu_Impl::Message(this) {
                    Ok(ok__) => {
                        message.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            PduData: PduData::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgPdu as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsReadMsgPdu {}
windows_core::imp::define_interface!(IMbnSmsReadMsgTextCdma, IMbnSmsReadMsgTextCdma_Vtbl, 0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnSmsReadMsgTextCdma, windows_core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    pub unsafe fn Index(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EncodingID(&self) -> windows_core::Result<MBN_SMS_CDMA_ENCODING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EncodingID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LanguageID(&self) -> windows_core::Result<MBN_SMS_CDMA_LANG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SizeInCharacters(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeInCharacters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgTextCdma_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_MSG_STATUS) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EncodingID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_CDMA_ENCODING) -> windows_core::HRESULT,
    pub LanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_CDMA_LANG) -> windows_core::HRESULT,
    pub SizeInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgTextCdma_Impl: windows_core::IUnknownImpl {
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
impl IMbnSmsReadMsgTextCdma_Vtbl {
    pub const fn new<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Index<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::Index(this) {
                    Ok(ok__) => {
                        index.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::Status(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Address<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::Address(this) {
                    Ok(ok__) => {
                        address.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        timestamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EncodingID<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::EncodingID(this) {
                    Ok(ok__) => {
                        encodingid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LanguageID<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::LanguageID(this) {
                    Ok(ok__) => {
                        languageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeInCharacters<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizeincharacters: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::SizeInCharacters(this) {
                    Ok(ok__) => {
                        sizeincharacters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Message<Identity: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSmsReadMsgTextCdma_Impl::Message(this) {
                    Ok(ok__) => {
                        message.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            Address: Address::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            EncodingID: EncodingID::<Identity, OFFSET>,
            LanguageID: LanguageID::<Identity, OFFSET>,
            SizeInCharacters: SizeInCharacters::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgTextCdma as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSmsReadMsgTextCdma {}
windows_core::imp::define_interface!(IMbnSubscriberInformation, IMbnSubscriberInformation_Vtbl, 0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
windows_core::imp::interface_hierarchy!(IMbnSubscriberInformation, windows_core::IUnknown);
impl IMbnSubscriberInformation {
    pub unsafe fn SubscriberID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubscriberID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SimIccID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SimIccID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TelephoneNumbers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSubscriberInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubscriberID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SimIccID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TelephoneNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TelephoneNumbers: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSubscriberInformation_Impl: windows_core::IUnknownImpl {
    fn SubscriberID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SimIccID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TelephoneNumbers(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSubscriberInformation_Vtbl {
    pub const fn new<Identity: IMbnSubscriberInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscriberID<Identity: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscriberid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSubscriberInformation_Impl::SubscriberID(this) {
                    Ok(ok__) => {
                        subscriberid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SimIccID<Identity: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, simiccid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSubscriberInformation_Impl::SimIccID(this) {
                    Ok(ok__) => {
                        simiccid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Identity: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnSubscriberInformation_Impl::TelephoneNumbers(this) {
                    Ok(ok__) => {
                        telephonenumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriberID: SubscriberID::<Identity, OFFSET>,
            SimIccID: SimIccID::<Identity, OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnSubscriberInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnSubscriberInformation {}
windows_core::imp::define_interface!(IMbnVendorSpecificEvents, IMbnVendorSpecificEvents_Vtbl, 0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnVendorSpecificEvents, windows_core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnVendorSpecificOperation>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEventNotification)(windows_core::Interface::as_raw(self), vendoroperation.param().abi(), vendorspecificdata).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnVendorSpecificOperation>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetVendorSpecificComplete)(windows_core::Interface::as_raw(self), vendoroperation.param().abi(), vendorspecificdata, requestid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventNotification: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetVendorSpecificComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetVendorSpecificComplete: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEvents_Impl: windows_core::IUnknownImpl {
    fn OnEventNotification(&self, vendoroperation: windows_core::Ref<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnSetVendorSpecificComplete(&self, vendoroperation: windows_core::Ref<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEvents_Vtbl {
    pub const fn new<Identity: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnEventNotification<Identity: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendoroperation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnVendorSpecificEvents_Impl::OnEventNotification(this, core::mem::transmute_copy(&vendoroperation), core::mem::transmute_copy(&vendorspecificdata)).into()
            }
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Identity: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendoroperation: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMbnVendorSpecificEvents_Impl::OnSetVendorSpecificComplete(this, core::mem::transmute_copy(&vendoroperation), core::mem::transmute_copy(&vendorspecificdata), core::mem::transmute_copy(&requestid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotification: OnEventNotification::<Identity, OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnVendorSpecificEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnVendorSpecificEvents {}
windows_core::imp::define_interface!(IMbnVendorSpecificOperation, IMbnVendorSpecificOperation_Vtbl, 0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
windows_core::imp::interface_hierarchy!(IMbnVendorSpecificOperation, windows_core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetVendorSpecific)(windows_core::Interface::as_raw(self), vendorspecificdata, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetVendorSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetVendorSpecific: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperation_Impl: windows_core::IUnknownImpl {
    fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperation_Vtbl {
    pub const fn new<Identity: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVendorSpecific<Identity: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMbnVendorSpecificOperation_Impl::SetVendorSpecific(this, core::mem::transmute_copy(&vendorspecificdata)) {
                    Ok(ok__) => {
                        requestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMbnVendorSpecificOperation {}
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(100i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_ACTIVATION_STATE(pub i32);
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(1i32);
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(2i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(3i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(4i32);
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(0i32);
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_AUTH_PROTOCOL(pub i32);
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(2i32);
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(3i32);
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(0i32);
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_BAND_CLASS(pub i32);
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = MBN_BAND_CLASS(1i32);
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = MBN_BAND_CLASS(-2147483648i32);
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = MBN_BAND_CLASS(2i32);
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = MBN_BAND_CLASS(4i32);
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = MBN_BAND_CLASS(8i32);
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = MBN_BAND_CLASS(16i32);
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = MBN_BAND_CLASS(512i32);
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = MBN_BAND_CLASS(0i32);
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = MBN_BAND_CLASS(32i32);
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = MBN_BAND_CLASS(64i32);
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = MBN_BAND_CLASS(128i32);
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = MBN_BAND_CLASS(256i32);
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = MBN_BAND_CLASS(1024i32);
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = MBN_BAND_CLASS(2048i32);
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = MBN_BAND_CLASS(4096i32);
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = MBN_BAND_CLASS(8192i32);
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = MBN_BAND_CLASS(16384i32);
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = MBN_BAND_CLASS(32768i32);
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = MBN_BAND_CLASS(65536i32);
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = MBN_BAND_CLASS(131072i32);
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(0i32);
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(160i32);
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_CELLULAR_CLASS(pub i32);
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(2i32);
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(1i32);
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_COMPRESSION(pub i32);
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = MBN_COMPRESSION(1i32);
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = MBN_COMPRESSION(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_CONNECTION_MODE(pub i32);
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(0i32);
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(1i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub userName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub password: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_CONTEXT_CONSTANTS(pub i32);
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_CONTEXT_TYPE(pub i32);
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(5i32);
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(1i32);
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(0i32);
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(6i32);
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(4i32);
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(3i32);
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_CTRL_CAPS(pub i32);
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(4i32);
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(8i32);
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = MBN_CTRL_CAPS(2i32);
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = MBN_CTRL_CAPS(32i32);
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(128i32);
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(0i32);
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = MBN_CTRL_CAPS(16i32);
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = MBN_CTRL_CAPS(1i32);
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = MBN_CTRL_CAPS(64i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_DATA_CLASS(pub i32);
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = MBN_DATA_CLASS(131072i32);
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = MBN_DATA_CLASS(262144i32);
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = MBN_DATA_CLASS(2097152i32);
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = MBN_DATA_CLASS(524288i32);
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(65536i32);
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(1048576i32);
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = MBN_DATA_CLASS(64i32);
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = MBN_DATA_CLASS(128i32);
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = MBN_DATA_CLASS(-2147483648i32);
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = MBN_DATA_CLASS(2i32);
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = MBN_DATA_CLASS(1i32);
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = MBN_DATA_CLASS(8i32);
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = MBN_DATA_CLASS(16i32);
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = MBN_DATA_CLASS(32i32);
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = MBN_DATA_CLASS(0i32);
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = MBN_DATA_CLASS(4194304i32);
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = MBN_DATA_CLASS(4i32);
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(18i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dataWriteSupported: super::super::Foundation::VARIANT_BOOL,
    pub dataReadSupported: super::super::Foundation::VARIANT_BOOL,
}
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(0i32);
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_DEVICE_SERVICES_INTERFACE_STATE(pub i32);
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = MBN_DEVICE_SERVICE_SESSIONS_STATE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_DEVICE_SERVICE_SESSIONS_STATE(pub i32);
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub manufacturer: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub model: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub firmwareInfo: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_INTERFACE_CAPS_CONSTANTS(pub i32);
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_MSG_STATUS(pub i32);
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = MBN_MSG_STATUS(2i32);
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = MBN_MSG_STATUS(0i32);
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = MBN_MSG_STATUS(1i32);
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = MBN_MSG_STATUS(3i32);
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PIN_CONSTANTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PIN_FORMAT(pub i32);
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(2i32);
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = MBN_PIN_FORMAT(0i32);
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PIN_MODE(pub i32);
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = MBN_PIN_MODE(2i32);
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = MBN_PIN_MODE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PIN_STATE(pub i32);
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = MBN_PIN_STATE(1i32);
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = MBN_PIN_STATE(0i32);
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = MBN_PIN_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PIN_TYPE(pub i32);
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(9i32);
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = MBN_PIN_TYPE(1i32);
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(5i32);
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(4i32);
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(6i32);
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(7i32);
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = MBN_PIN_TYPE(0i32);
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = MBN_PIN_TYPE(2i32);
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = MBN_PIN_TYPE(3i32);
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = MBN_PIN_TYPE(10i32);
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(8i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MBN_PROVIDER {
    pub providerID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub providerState: u32,
    pub providerName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dataClass: u32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(6i32);
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(20i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PROVIDER_CONSTANTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_PROVIDER_STATE(pub i32);
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(2i32);
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(1i32);
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(0i32);
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(4i32);
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(32i32);
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(16i32);
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_RADIO(pub i32);
pub const MBN_RADIO_OFF: MBN_RADIO = MBN_RADIO(0i32);
pub const MBN_RADIO_ON: MBN_RADIO = MBN_RADIO(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_READY_STATE(pub i32);
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = MBN_READY_STATE(3i32);
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = MBN_READY_STATE(7i32);
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = MBN_READY_STATE(6i32);
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = MBN_READY_STATE(4i32);
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = MBN_READY_STATE(1i32);
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = MBN_READY_STATE(5i32);
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = MBN_READY_STATE(8i32);
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = MBN_READY_STATE(0i32);
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = MBN_READY_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_REGISTER_MODE(pub i32);
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = MBN_REGISTER_MODE(1i32);
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = MBN_REGISTER_MODE(2i32);
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = MBN_REGISTER_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_REGISTER_STATE(pub i32);
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(6i32);
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(1i32);
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = MBN_REGISTER_STATE(3i32);
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = MBN_REGISTER_STATE(0i32);
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = MBN_REGISTER_STATE(5i32);
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(4i32);
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_REGISTRATION_CONSTANTS(pub i32);
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(64i32);
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(-1i32);
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(0i32);
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SIGNAL_CONSTANTS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_CAPS(pub i32);
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = MBN_SMS_CAPS(0i32);
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(1i32);
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(2i32);
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(4i32);
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_CDMA_ENCODING(pub i32);
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(2i32);
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(1i32);
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(9i32);
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(3i32);
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(6i32);
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(8i32);
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(7i32);
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(0i32);
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(5i32);
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_CDMA_LANG(pub i32);
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(6i32);
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(1i32);
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(2i32);
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(7i32);
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(4i32);
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(5i32);
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(0i32);
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_FLAG(pub i32);
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = MBN_SMS_FLAG(0i32);
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = MBN_SMS_FLAG(5i32);
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = MBN_SMS_FLAG(1i32);
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(1i32);
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = MBN_SMS_FLAG(2i32);
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(2i32);
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(0i32);
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = MBN_SMS_FLAG(3i32);
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = MBN_SMS_FLAG(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_FORMAT(pub i32);
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = MBN_SMS_FORMAT(0i32);
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = MBN_SMS_FORMAT(1i32);
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = MBN_SMS_FORMAT(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_SMS_STATUS_FLAG(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_VOICE_CALL_STATE(pub i32);
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(2i32);
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(1i32);
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MBN_VOICE_CLASS(pub i32);
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(0i32);
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(1i32);
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(2i32);
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(3i32);
pub const MbnConnectionManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
pub const MbnConnectionProfileManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
pub const MbnDeviceServicesManager: windows_core::GUID = windows_core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
pub const MbnInterfaceManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WWAEXT_SMS_CONSTANTS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
