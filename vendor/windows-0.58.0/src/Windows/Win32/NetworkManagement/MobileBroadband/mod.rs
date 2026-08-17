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
impl IDummyMBNUCMExt {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDummyMBNUCMExt_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IMbnConnection, IMbnConnection_Vtbl, 0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnection, windows_core::IUnknown);
impl IMbnConnection {
    pub unsafe fn ConnectionID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectionID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Connect<P0>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), connectionmode, strprofile.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectionState)(windows_core::Interface::as_raw(self), connectionstate, core::mem::transmute(profilename)).ok()
    }
    pub unsafe fn GetVoiceCallState(&self) -> windows_core::Result<MBN_VOICE_CALL_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVoiceCallState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetActivationNetworkError(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivationNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_CONNECTION_MODE, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnectionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_ACTIVATION_STATE, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetVoiceCallState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_VOICE_CALL_STATE) -> windows_core::HRESULT,
    pub GetActivationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionContext, IMbnConnectionContext_Vtbl, 0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionContext, windows_core::IUnknown);
impl IMbnConnectionContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProvisionedContexts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProvisionedContext<P0>(&self, provisionedcontexts: MBN_CONTEXT, providerid: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetProvisionedContext)(windows_core::Interface::as_raw(self), core::mem::transmute(provisionedcontexts), providerid.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnConnectionContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProvisionedContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProvisionedContexts: usize,
    pub SetProvisionedContext: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_CONTEXT, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionContextEvents, IMbnConnectionContextEvents_Vtbl, 0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionContextEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionContextEvents, windows_core::IUnknown);
impl IMbnConnectionContextEvents {
    pub unsafe fn OnProvisionedContextListChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionContext>,
    {
        (windows_core::Interface::vtable(self).OnProvisionedContextListChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnSetProvisionedContextComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionContext>,
    {
        (windows_core::Interface::vtable(self).OnSetProvisionedContextComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionContextEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProvisionedContextListChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetProvisionedContextComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionEvents, IMbnConnectionEvents_Vtbl, 0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionEvents, windows_core::IUnknown);
impl IMbnConnectionEvents {
    pub unsafe fn OnConnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnConnectComplete)(windows_core::Interface::as_raw(self), newconnection.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnDisconnectComplete<P0>(&self, newconnection: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnDisconnectComplete)(windows_core::Interface::as_raw(self), newconnection.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnConnectStateChange<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnConnectStateChange)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok()
    }
    pub unsafe fn OnVoiceCallStateChange<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnVoiceCallStateChange)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnDisconnectComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnConnectStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnVoiceCallStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionManager, IMbnConnectionManager_Vtbl, 0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionManager, windows_core::IUnknown);
impl IMbnConnectionManager {
    pub unsafe fn GetConnection<P0>(&self, connectionid: P0) -> windows_core::Result<IMbnConnection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), connectionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnConnectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnections: usize,
}
windows_core::imp::define_interface!(IMbnConnectionManagerEvents, IMbnConnectionManagerEvents_Vtbl, 0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionManagerEvents, windows_core::IUnknown);
impl IMbnConnectionManagerEvents {
    pub unsafe fn OnConnectionArrival<P0>(&self, newconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnConnectionArrival)(windows_core::Interface::as_raw(self), newconnection.param().abi()).ok()
    }
    pub unsafe fn OnConnectionRemoval<P0>(&self, oldconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnection>,
    {
        (windows_core::Interface::vtable(self).OnConnectionRemoval)(windows_core::Interface::as_raw(self), oldconnection.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnConnectionRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionProfile, IMbnConnectionProfile_Vtbl, 0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionProfile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionProfile, windows_core::IUnknown);
impl IMbnConnectionProfile {
    pub unsafe fn GetProfileXmlData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProfileXmlData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UpdateProfile<P0>(&self, strprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UpdateProfile)(windows_core::Interface::as_raw(self), strprofile.param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProfileXmlData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UpdateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionProfileEvents, IMbnConnectionProfileEvents_Vtbl, 0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionProfileEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileEvents, windows_core::IUnknown);
impl IMbnConnectionProfileEvents {
    pub unsafe fn OnProfileUpdate<P0>(&self, newprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        (windows_core::Interface::vtable(self).OnProfileUpdate)(windows_core::Interface::as_raw(self), newprofile.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionProfileEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProfileUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionProfileManager, IMbnConnectionProfileManager_Vtbl, 0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionProfileManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileManager, windows_core::IUnknown);
impl IMbnConnectionProfileManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<P0>(&self, mbninterface: P0) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionProfiles)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectionProfile<P0, P1>(&self, mbninterface: P0, profilename: P1) -> windows_core::Result<IMbnConnectionProfile>
    where
        P0: windows_core::Param<IMbnInterface>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionProfile)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), profilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateConnectionProfile<P0>(&self, xmlprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateConnectionProfile)(windows_core::Interface::as_raw(self), xmlprofile.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionProfileManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConnectionProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConnectionProfiles: usize,
    pub GetConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnConnectionProfileManagerEvents, IMbnConnectionProfileManagerEvents_Vtbl, 0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnConnectionProfileManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnConnectionProfileManagerEvents, windows_core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    pub unsafe fn OnConnectionProfileArrival<P0>(&self, newconnectionprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        (windows_core::Interface::vtable(self).OnConnectionProfileArrival)(windows_core::Interface::as_raw(self), newconnectionprofile.param().abi()).ok()
    }
    pub unsafe fn OnConnectionProfileRemoval<P0>(&self, oldconnectionprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnConnectionProfile>,
    {
        (windows_core::Interface::vtable(self).OnConnectionProfileRemoval)(windows_core::Interface::as_raw(self), oldconnectionprofile.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnConnectionProfileManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionProfileArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnConnectionProfileRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnDeviceService, IMbnDeviceService_Vtbl, 0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
impl core::ops::Deref for IMbnDeviceService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnDeviceService, windows_core::IUnknown);
impl IMbnDeviceService {
    pub unsafe fn QuerySupportedCommands(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySupportedCommands)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenCommandSession(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenCommandSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CloseCommandSession(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CloseCommandSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetCommand)(windows_core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryCommand)(windows_core::Interface::as_raw(self), commandid, deviceservicedata, &mut result__).map(|| result__)
    }
    pub unsafe fn OpenDataSession(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CloseDataSession(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CloseDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WriteData)(windows_core::Interface::as_raw(self), deviceservicedata, &mut result__).map(|| result__)
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeviceServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceServiceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsCommandSessionOpen(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCommandSessionOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDataSessionOpen(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDataSessionOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeviceServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsCommandSessionOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsDataSessionOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnDeviceServiceStateEvents, IMbnDeviceServiceStateEvents_Vtbl, 0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
impl core::ops::Deref for IMbnDeviceServiceStateEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnDeviceServiceStateEvents, windows_core::IUnknown);
impl IMbnDeviceServiceStateEvents {
    pub unsafe fn OnSessionsStateChange<P0>(&self, interfaceid: P0, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnSessionsStateChange)(windows_core::Interface::as_raw(self), interfaceid.param().abi(), statechange).ok()
    }
}
#[repr(C)]
pub struct IMbnDeviceServiceStateEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSessionsStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, MBN_DEVICE_SERVICE_SESSIONS_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnDeviceServicesContext, IMbnDeviceServicesContext_Vtbl, 0xfc5ac347_1592_4068_80bb_6a57580150d8);
impl core::ops::Deref for IMbnDeviceServicesContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesContext, windows_core::IUnknown);
impl IMbnDeviceServicesContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateDeviceServices)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceService<P0>(&self, deviceserviceid: P0) -> windows_core::Result<IMbnDeviceService>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceService)(windows_core::Interface::as_raw(self), deviceserviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MaxCommandSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxCommandSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaxDataSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxDataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnDeviceServicesContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateDeviceServices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateDeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxCommandSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxDataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnDeviceServicesEvents, IMbnDeviceServicesEvents_Vtbl, 0x0a900c19_6824_4e97_b76e_cf239d0ca642);
impl core::ops::Deref for IMbnDeviceServicesEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesEvents, windows_core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<P0>(&self, deviceservice: P0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnQuerySupportedCommandsComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), commandidlist, status, requestid).ok()
    }
    pub unsafe fn OnOpenCommandSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnOpenCommandSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnCloseCommandSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnCloseCommandSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnSetCommandComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), responseid, deviceservicedata, status, requestid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<P0>(&self, deviceservice: P0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnQueryCommandComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), responseid, deviceservicedata, status, requestid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, deviceservice: P0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnEventNotification)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), eventid, deviceservicedata).ok()
    }
    pub unsafe fn OnOpenDataSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnOpenDataSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnCloseDataSessionComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnCloseDataSessionComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok()
    }
    pub unsafe fn OnWriteDataComplete<P0>(&self, deviceservice: P0, status: windows_core::HRESULT, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnWriteDataComplete)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), status, requestid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<P0>(&self, deviceservice: P0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnDeviceService>,
    {
        (windows_core::Interface::vtable(self).OnReadData)(windows_core::Interface::as_raw(self), deviceservice.param().abi(), deviceservicedata).ok()
    }
    pub unsafe fn OnInterfaceStateChange<P0>(&self, interfaceid: P0, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnInterfaceStateChange)(windows_core::Interface::as_raw(self), interfaceid.param().abi(), statechange).ok()
    }
}
#[repr(C)]
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
    pub OnInterfaceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, MBN_DEVICE_SERVICES_INTERFACE_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnDeviceServicesManager, IMbnDeviceServicesManager_Vtbl, 0x20a26258_6811_4478_ac1d_13324e45e41c);
impl core::ops::Deref for IMbnDeviceServicesManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnDeviceServicesManager, windows_core::IUnknown);
impl IMbnDeviceServicesManager {
    pub unsafe fn GetDeviceServicesContext<P0>(&self, networkinterfaceid: P0) -> windows_core::Result<IMbnDeviceServicesContext>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceServicesContext)(windows_core::Interface::as_raw(self), networkinterfaceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMbnDeviceServicesManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceServicesContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnInterface, IMbnInterface_Vtbl, 0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnInterface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnInterface, windows_core::IUnknown);
impl IMbnInterface {
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInterfaceCapability(&self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInterfaceCapability)(windows_core::Interface::as_raw(self), interfacecaps).ok()
    }
    pub unsafe fn GetSubscriberInformation(&self) -> windows_core::Result<IMbnSubscriberInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriberInformation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetReadyState(&self) -> windows_core::Result<MBN_READY_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InEmergencyMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InEmergencyMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHomeProvider(&self) -> windows_core::Result<MBN_PROVIDER> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHomeProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredProviders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetPreferredProviders)(windows_core::Interface::as_raw(self), preferredproviders, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisibleProviders)(windows_core::Interface::as_raw(self), age, &mut result__).map(|| result__)
    }
    pub unsafe fn ScanNetwork(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScanNetwork)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnection(&self) -> windows_core::Result<IMbnConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMbnInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
windows_core::imp::define_interface!(IMbnInterfaceEvents, IMbnInterfaceEvents_Vtbl, 0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnInterfaceEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnInterfaceEvents, windows_core::IUnknown);
impl IMbnInterfaceEvents {
    pub unsafe fn OnInterfaceCapabilityAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnInterfaceCapabilityAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnSubscriberInformationChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnSubscriberInformationChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnReadyStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnReadyStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnEmergencyModeChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnEmergencyModeChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnHomeProviderAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnHomeProviderAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnPreferredProvidersChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnSetPreferredProvidersComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnSetPreferredProvidersComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnScanNetworkComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnInterfaceManager, IMbnInterfaceManager_Vtbl, 0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnInterfaceManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnInterfaceManager, windows_core::IUnknown);
impl IMbnInterfaceManager {
    pub unsafe fn GetInterface<P0>(&self, interfaceid: P0) -> windows_core::Result<IMbnInterface>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInterface)(windows_core::Interface::as_raw(self), interfaceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnInterfaceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInterfaces: usize,
}
windows_core::imp::define_interface!(IMbnInterfaceManagerEvents, IMbnInterfaceManagerEvents_Vtbl, 0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnInterfaceManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnInterfaceManagerEvents, windows_core::IUnknown);
impl IMbnInterfaceManagerEvents {
    pub unsafe fn OnInterfaceArrival<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnInterfaceArrival)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnInterfaceRemoval<P0>(&self, oldinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnInterface>,
    {
        (windows_core::Interface::vtable(self).OnInterfaceRemoval)(windows_core::Interface::as_raw(self), oldinterface.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnInterfaceManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInterfaceArrival: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInterfaceRemoval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnMultiCarrier, IMbnMultiCarrier_Vtbl, 0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnMultiCarrier {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnMultiCarrier, windows_core::IUnknown);
impl IMbnMultiCarrier {
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetHomeProvider)(windows_core::Interface::as_raw(self), homeprovider, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreferredProviders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVisibleProviders)(windows_core::Interface::as_raw(self), age, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedCellularClasses)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentCellularClass(&self) -> windows_core::Result<MBN_CELLULAR_CLASS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentCellularClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ScanNetwork(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScanNetwork)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnMultiCarrierEvents, IMbnMultiCarrierEvents_Vtbl, 0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnMultiCarrierEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnMultiCarrierEvents, windows_core::IUnknown);
impl IMbnMultiCarrierEvents {
    pub unsafe fn OnSetHomeProviderComplete<P0>(&self, mbninterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        (windows_core::Interface::vtable(self).OnSetHomeProviderComplete)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnCurrentCellularClassChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        (windows_core::Interface::vtable(self).OnCurrentCellularClassChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        (windows_core::Interface::vtable(self).OnPreferredProvidersChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok()
    }
    pub unsafe fn OnScanNetworkComplete<P0>(&self, mbninterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        (windows_core::Interface::vtable(self).OnScanNetworkComplete)(windows_core::Interface::as_raw(self), mbninterface.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnInterfaceCapabilityChange<P0>(&self, mbninterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnMultiCarrier>,
    {
        (windows_core::Interface::vtable(self).OnInterfaceCapabilityChange)(windows_core::Interface::as_raw(self), mbninterface.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnMultiCarrierEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSetHomeProviderComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnCurrentCellularClassChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPreferredProvidersChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnScanNetworkComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnInterfaceCapabilityChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnPin, IMbnPin_Vtbl, 0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnPin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnPin, windows_core::IUnknown);
impl IMbnPin {
    pub unsafe fn PinType(&self) -> windows_core::Result<MBN_PIN_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PinType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PinFormat(&self) -> windows_core::Result<MBN_PIN_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PinFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PinLengthMin(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PinLengthMin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PinLengthMax(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PinLengthMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PinMode(&self) -> windows_core::Result<MBN_PIN_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PinMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Enable<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Disable<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Enter<P0>(&self, pin: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Change<P0, P1>(&self, pin: P0, newpin: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Change)(windows_core::Interface::as_raw(self), pin.param().abi(), newpin.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unblock<P0, P1>(&self, puk: P0, newpin: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Unblock)(windows_core::Interface::as_raw(self), puk.param().abi(), newpin.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPinManager(&self) -> windows_core::Result<IMbnPinManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPinManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnPinEvents, IMbnPinEvents_Vtbl, 0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnPinEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnPinEvents, windows_core::IUnknown);
impl IMbnPinEvents {
    pub unsafe fn OnEnableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        (windows_core::Interface::vtable(self).OnEnableComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnDisableComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        (windows_core::Interface::vtable(self).OnDisableComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnEnterComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        (windows_core::Interface::vtable(self).OnEnterComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnChangeComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        (windows_core::Interface::vtable(self).OnChangeComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok()
    }
    pub unsafe fn OnUnblockComplete<P0>(&self, pin: P0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPin>,
    {
        (windows_core::Interface::vtable(self).OnUnblockComplete)(windows_core::Interface::as_raw(self), pin.param().abi(), pininfo, requestid, status).ok()
    }
}
#[repr(C)]
pub struct IMbnPinEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnEnableComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnDisableComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnEnterComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnChangeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnUnblockComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnPinManager, IMbnPinManager_Vtbl, 0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnPinManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnPinManager, windows_core::IUnknown);
impl IMbnPinManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPinList)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> windows_core::Result<IMbnPin> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPin)(windows_core::Interface::as_raw(self), pintype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPinState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPinState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnPinManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPinList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPinList: usize,
    pub GetPin: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_PIN_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPinState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnPinManagerEvents, IMbnPinManagerEvents_Vtbl, 0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnPinManagerEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnPinManagerEvents, windows_core::IUnknown);
impl IMbnPinManagerEvents {
    pub unsafe fn OnPinListAvailable<P0>(&self, pinmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPinManager>,
    {
        (windows_core::Interface::vtable(self).OnPinListAvailable)(windows_core::Interface::as_raw(self), pinmanager.param().abi()).ok()
    }
    pub unsafe fn OnGetPinStateComplete<P0>(&self, pinmanager: P0, pininfo: MBN_PIN_INFO, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnPinManager>,
    {
        (windows_core::Interface::vtable(self).OnGetPinStateComplete)(windows_core::Interface::as_raw(self), pinmanager.param().abi(), core::mem::transmute(pininfo), requestid, status).ok()
    }
}
#[repr(C)]
pub struct IMbnPinManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPinListAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnGetPinStateComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MBN_PIN_INFO, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnRadio, IMbnRadio_Vtbl, 0xdccccab6_201f_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnRadio {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnRadio, windows_core::IUnknown);
impl IMbnRadio {
    pub unsafe fn SoftwareRadioState(&self) -> windows_core::Result<MBN_RADIO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SoftwareRadioState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HardwareRadioState(&self) -> windows_core::Result<MBN_RADIO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HardwareRadioState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetSoftwareRadioState)(windows_core::Interface::as_raw(self), radiostate, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnRadio_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SoftwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_RADIO) -> windows_core::HRESULT,
    pub HardwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_RADIO) -> windows_core::HRESULT,
    pub SetSoftwareRadioState: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_RADIO, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnRadioEvents, IMbnRadioEvents_Vtbl, 0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnRadioEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnRadioEvents, windows_core::IUnknown);
impl IMbnRadioEvents {
    pub unsafe fn OnRadioStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRadio>,
    {
        (windows_core::Interface::vtable(self).OnRadioStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnSetSoftwareRadioStateComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRadio>,
    {
        (windows_core::Interface::vtable(self).OnSetSoftwareRadioStateComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok()
    }
}
#[repr(C)]
pub struct IMbnRadioEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRadioStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetSoftwareRadioStateComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnRegistration, IMbnRegistration_Vtbl, 0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnRegistration, windows_core::IUnknown);
impl IMbnRegistration {
    pub unsafe fn GetRegisterState(&self) -> windows_core::Result<MBN_REGISTER_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisterState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRegisterMode(&self) -> windows_core::Result<MBN_REGISTER_MODE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisterMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProviderID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProviderName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRoamingText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRoamingText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAvailableDataClasses(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableDataClasses)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentDataClass(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentDataClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRegistrationNetworkError(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegistrationNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPacketAttachNetworkError(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPacketAttachNetworkError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRegisterMode<P0>(&self, registermode: MBN_REGISTER_MODE, providerid: P0, dataclass: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetRegisterMode)(windows_core::Interface::as_raw(self), registermode, providerid.param().abi(), dataclass, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRegisterState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_REGISTER_STATE) -> windows_core::HRESULT,
    pub GetRegisterMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_REGISTER_MODE) -> windows_core::HRESULT,
    pub GetProviderID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRoamingText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAvailableDataClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRegistrationNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPacketAttachNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRegisterMode: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_REGISTER_MODE, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnRegistrationEvents, IMbnRegistrationEvents_Vtbl, 0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnRegistrationEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnRegistrationEvents, windows_core::IUnknown);
impl IMbnRegistrationEvents {
    pub unsafe fn OnRegisterModeAvailable<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        (windows_core::Interface::vtable(self).OnRegisterModeAvailable)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnRegisterStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        (windows_core::Interface::vtable(self).OnRegisterStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnPacketServiceStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        (windows_core::Interface::vtable(self).OnPacketServiceStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
    pub unsafe fn OnSetRegisterModeComplete<P0>(&self, newinterface: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnRegistration>,
    {
        (windows_core::Interface::vtable(self).OnSetRegisterModeComplete)(windows_core::Interface::as_raw(self), newinterface.param().abi(), requestid, status).ok()
    }
}
#[repr(C)]
pub struct IMbnRegistrationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRegisterModeAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnRegisterStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPacketServiceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetRegisterModeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnServiceActivation, IMbnServiceActivation_Vtbl, 0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnServiceActivation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnServiceActivation, windows_core::IUnknown);
impl IMbnServiceActivation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), vendorspecificdata, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnServiceActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
}
windows_core::imp::define_interface!(IMbnServiceActivationEvents, IMbnServiceActivationEvents_Vtbl, 0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnServiceActivationEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnServiceActivationEvents, windows_core::IUnknown);
impl IMbnServiceActivationEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<P0>(&self, serviceactivation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: windows_core::HRESULT, networkerror: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnServiceActivation>,
    {
        (windows_core::Interface::vtable(self).OnActivationComplete)(windows_core::Interface::as_raw(self), serviceactivation.param().abi(), vendorspecificdata, requestid, status, networkerror).ok()
    }
}
#[repr(C)]
pub struct IMbnServiceActivationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, u32, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivationComplete: usize,
}
windows_core::imp::define_interface!(IMbnSignal, IMbnSignal_Vtbl, 0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSignal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSignal, windows_core::IUnknown);
impl IMbnSignal {
    pub unsafe fn GetSignalStrength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignalStrength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignalError(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignalError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnSignal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSignalStrength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSignalError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnSignalEvents, IMbnSignalEvents_Vtbl, 0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSignalEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSignalEvents, windows_core::IUnknown);
impl IMbnSignalEvents {
    pub unsafe fn OnSignalStateChange<P0>(&self, newinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSignal>,
    {
        (windows_core::Interface::vtable(self).OnSignalStateChange)(windows_core::Interface::as_raw(self), newinterface.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMbnSignalEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSignalStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnSms, IMbnSms_Vtbl, 0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSms {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSms, windows_core::IUnknown);
impl IMbnSms {
    pub unsafe fn GetSmsConfiguration(&self) -> windows_core::Result<IMbnSmsConfiguration> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSmsConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSmsConfiguration<P0>(&self, smsconfiguration: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IMbnSmsConfiguration>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetSmsConfiguration)(windows_core::Interface::as_raw(self), smsconfiguration.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SmsSendPdu<P0>(&self, pdudata: P0, size: u8) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsSendPdu)(windows_core::Interface::as_raw(self), pdudata.param().abi(), size, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdma<P0>(&self, address: P0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsSendCdma)(windows_core::Interface::as_raw(self), address.param().abi(), encoding, language, sizeincharacters, message, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsSendCdmaPdu)(windows_core::Interface::as_raw(self), message, &mut result__).map(|| result__)
    }
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsRead)(windows_core::Interface::as_raw(self), smsfilter, smsformat, &mut result__).map(|| result__)
    }
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsDelete)(windows_core::Interface::as_raw(self), smsfilter, &mut result__).map(|| result__)
    }
    pub unsafe fn GetSmsStatus(&self) -> windows_core::Result<MBN_SMS_STATUS_INFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSmsStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnSmsConfiguration, IMbnSmsConfiguration_Vtbl, 0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSmsConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSmsConfiguration, windows_core::IUnknown);
impl IMbnSmsConfiguration {
    pub unsafe fn ServiceCenterAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceCenterAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceCenterAddress<P0>(&self, scaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceCenterAddress)(windows_core::Interface::as_raw(self), scaddress.param().abi()).ok()
    }
    pub unsafe fn MaxMessageIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxMessageIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CdmaShortMsgSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CdmaShortMsgSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SmsFormat(&self) -> windows_core::Result<MBN_SMS_FORMAT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmsFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSmsFormat)(windows_core::Interface::as_raw(self), smsformat).ok()
    }
}
#[repr(C)]
pub struct IMbnSmsConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ServiceCenterAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceCenterAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MaxMessageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CdmaShortMsgSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SmsFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_FORMAT) -> windows_core::HRESULT,
    pub SetSmsFormat: unsafe extern "system" fn(*mut core::ffi::c_void, MBN_SMS_FORMAT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMbnSmsEvents, IMbnSmsEvents_Vtbl, 0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSmsEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSmsEvents, windows_core::IUnknown);
impl IMbnSmsEvents {
    pub unsafe fn OnSmsConfigurationChange<P0>(&self, sms: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSmsConfigurationChange)(windows_core::Interface::as_raw(self), sms.param().abi()).ok()
    }
    pub unsafe fn OnSetSmsConfigurationComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSetSmsConfigurationComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnSmsSendComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSmsSendComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsReadComplete<P0, P1>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: P1, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).OnSmsReadComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), smsformat, readmsgs, moremsgs.param().abi(), requestid, status).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<P0>(&self, sms: P0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSmsNewClass0Message)(windows_core::Interface::as_raw(self), sms.param().abi(), smsformat, readmsgs).ok()
    }
    pub unsafe fn OnSmsDeleteComplete<P0>(&self, sms: P0, requestid: u32, status: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSmsDeleteComplete)(windows_core::Interface::as_raw(self), sms.param().abi(), requestid, status).ok()
    }
    pub unsafe fn OnSmsStatusChange<P0>(&self, sms: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnSms>,
    {
        (windows_core::Interface::vtable(self).OnSmsStatusChange)(windows_core::Interface::as_raw(self), sms.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnSmsReadMsgPdu, IMbnSmsReadMsgPdu_Vtbl, 0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSmsReadMsgPdu {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSmsReadMsgPdu, windows_core::IUnknown);
impl IMbnSmsReadMsgPdu {
    pub unsafe fn Index(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PduData(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PduData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnSmsReadMsgPdu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_MSG_STATUS) -> windows_core::HRESULT,
    pub PduData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
windows_core::imp::define_interface!(IMbnSmsReadMsgTextCdma, IMbnSmsReadMsgTextCdma_Vtbl, 0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnSmsReadMsgTextCdma {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSmsReadMsgTextCdma, windows_core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    pub unsafe fn Index(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Status(&self) -> windows_core::Result<MBN_MSG_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EncodingID(&self) -> windows_core::Result<MBN_SMS_CDMA_ENCODING> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EncodingID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LanguageID(&self) -> windows_core::Result<MBN_SMS_CDMA_LANG> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SizeInCharacters(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SizeInCharacters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnSmsReadMsgTextCdma_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_MSG_STATUS) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EncodingID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_CDMA_ENCODING) -> windows_core::HRESULT,
    pub LanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MBN_SMS_CDMA_LANG) -> windows_core::HRESULT,
    pub SizeInCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
}
windows_core::imp::define_interface!(IMbnSubscriberInformation, IMbnSubscriberInformation_Vtbl, 0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
impl core::ops::Deref for IMbnSubscriberInformation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnSubscriberInformation, windows_core::IUnknown);
impl IMbnSubscriberInformation {
    pub unsafe fn SubscriberID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriberID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SimIccID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SimIccID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneNumbers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnSubscriberInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubscriberID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SimIccID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TelephoneNumbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TelephoneNumbers: usize,
}
windows_core::imp::define_interface!(IMbnVendorSpecificEvents, IMbnVendorSpecificEvents_Vtbl, 0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnVendorSpecificEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnVendorSpecificEvents, windows_core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnVendorSpecificOperation>,
    {
        (windows_core::Interface::vtable(self).OnEventNotification)(windows_core::Interface::as_raw(self), vendoroperation.param().abi(), vendorspecificdata).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<P0>(&self, vendoroperation: P0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMbnVendorSpecificOperation>,
    {
        (windows_core::Interface::vtable(self).OnSetVendorSpecificComplete)(windows_core::Interface::as_raw(self), vendoroperation.param().abi(), vendorspecificdata, requestid).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IMbnVendorSpecificOperation, IMbnVendorSpecificOperation_Vtbl, 0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
impl core::ops::Deref for IMbnVendorSpecificOperation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMbnVendorSpecificOperation, windows_core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetVendorSpecific)(windows_core::Interface::as_raw(self), vendorspecificdata, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMbnVendorSpecificOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetVendorSpecific: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetVendorSpecific: usize,
}
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(100i32);
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(1i32);
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(2i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(3i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(4i32);
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(0i32);
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(2i32);
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(3i32);
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(0i32);
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(1i32);
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
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(2i32);
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(1i32);
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(0i32);
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = MBN_COMPRESSION(1i32);
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = MBN_COMPRESSION(0i32);
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(0i32);
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(1i32);
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(-1i32);
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(5i32);
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(1i32);
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(0i32);
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(6i32);
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(4i32);
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(3i32);
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(2i32);
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(4i32);
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(8i32);
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = MBN_CTRL_CAPS(2i32);
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = MBN_CTRL_CAPS(32i32);
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(128i32);
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(0i32);
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = MBN_CTRL_CAPS(16i32);
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = MBN_CTRL_CAPS(1i32);
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = MBN_CTRL_CAPS(64i32);
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
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(0i32);
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(1i32);
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = MBN_DEVICE_SERVICE_SESSIONS_STATE(0i32);
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = MBN_MSG_STATUS(2i32);
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = MBN_MSG_STATUS(0i32);
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = MBN_MSG_STATUS(1i32);
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = MBN_MSG_STATUS(3i32);
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(2i32);
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = MBN_PIN_FORMAT(0i32);
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(1i32);
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = MBN_PIN_MODE(2i32);
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = MBN_PIN_MODE(1i32);
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = MBN_PIN_STATE(1i32);
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = MBN_PIN_STATE(0i32);
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = MBN_PIN_STATE(2i32);
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
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(6i32);
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(20i32);
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(2i32);
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(1i32);
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(0i32);
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(4i32);
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(32i32);
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(16i32);
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(8i32);
pub const MBN_RADIO_OFF: MBN_RADIO = MBN_RADIO(0i32);
pub const MBN_RADIO_ON: MBN_RADIO = MBN_RADIO(1i32);
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = MBN_READY_STATE(3i32);
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = MBN_READY_STATE(7i32);
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = MBN_READY_STATE(6i32);
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = MBN_READY_STATE(4i32);
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = MBN_READY_STATE(1i32);
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = MBN_READY_STATE(5i32);
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = MBN_READY_STATE(8i32);
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = MBN_READY_STATE(0i32);
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = MBN_READY_STATE(2i32);
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = MBN_REGISTER_MODE(1i32);
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = MBN_REGISTER_MODE(2i32);
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = MBN_REGISTER_MODE(0i32);
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(6i32);
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(1i32);
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = MBN_REGISTER_STATE(3i32);
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = MBN_REGISTER_STATE(0i32);
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = MBN_REGISTER_STATE(5i32);
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(4i32);
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(2i32);
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(64i32);
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(-1i32);
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(0i32);
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = MBN_SMS_CAPS(0i32);
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(1i32);
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(2i32);
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(4i32);
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(8i32);
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
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(6i32);
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(1i32);
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(2i32);
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(7i32);
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(4i32);
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(5i32);
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(0i32);
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(3i32);
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = MBN_SMS_FLAG(0i32);
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = MBN_SMS_FLAG(5i32);
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = MBN_SMS_FLAG(1i32);
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(1i32);
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = MBN_SMS_FLAG(2i32);
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(2i32);
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(0i32);
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = MBN_SMS_FLAG(3i32);
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = MBN_SMS_FLAG(4i32);
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = MBN_SMS_FORMAT(0i32);
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = MBN_SMS_FORMAT(1i32);
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = MBN_SMS_FORMAT(2i32);
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(2i32);
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(1i32);
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(0i32);
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(0i32);
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(1i32);
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(2i32);
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(3i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_ACTIVATION_STATE(pub i32);
impl windows_core::TypeKind for MBN_ACTIVATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_ACTIVATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_ACTIVATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_AUTH_PROTOCOL(pub i32);
impl windows_core::TypeKind for MBN_AUTH_PROTOCOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_BAND_CLASS(pub i32);
impl windows_core::TypeKind for MBN_BAND_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_BAND_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_BAND_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_CELLULAR_CLASS(pub i32);
impl windows_core::TypeKind for MBN_CELLULAR_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_CELLULAR_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_CELLULAR_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_COMPRESSION(pub i32);
impl windows_core::TypeKind for MBN_COMPRESSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_COMPRESSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_COMPRESSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_CONNECTION_MODE(pub i32);
impl windows_core::TypeKind for MBN_CONNECTION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_CONNECTION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_CONNECTION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_CONTEXT_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_CONTEXT_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_CONTEXT_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_CONTEXT_TYPE(pub i32);
impl windows_core::TypeKind for MBN_CONTEXT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_CONTEXT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_CTRL_CAPS(pub i32);
impl windows_core::TypeKind for MBN_CTRL_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_CTRL_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_CTRL_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_DATA_CLASS(pub i32);
impl windows_core::TypeKind for MBN_DATA_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_DATA_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_DATA_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_DEVICE_SERVICES_INTERFACE_STATE(pub i32);
impl windows_core::TypeKind for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICES_INTERFACE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_DEVICE_SERVICE_SESSIONS_STATE(pub i32);
impl windows_core::TypeKind for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_DEVICE_SERVICE_SESSIONS_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_INTERFACE_CAPS_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_INTERFACE_CAPS_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_INTERFACE_CAPS_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_INTERFACE_CAPS_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_MSG_STATUS(pub i32);
impl windows_core::TypeKind for MBN_MSG_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_MSG_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_MSG_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PIN_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_PIN_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PIN_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PIN_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PIN_FORMAT(pub i32);
impl windows_core::TypeKind for MBN_PIN_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PIN_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PIN_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PIN_MODE(pub i32);
impl windows_core::TypeKind for MBN_PIN_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PIN_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PIN_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PIN_STATE(pub i32);
impl windows_core::TypeKind for MBN_PIN_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PIN_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PIN_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PIN_TYPE(pub i32);
impl windows_core::TypeKind for MBN_PIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PROVIDER_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_PROVIDER_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PROVIDER_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_PROVIDER_STATE(pub i32);
impl windows_core::TypeKind for MBN_PROVIDER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_PROVIDER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_PROVIDER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_RADIO(pub i32);
impl windows_core::TypeKind for MBN_RADIO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_RADIO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_RADIO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_READY_STATE(pub i32);
impl windows_core::TypeKind for MBN_READY_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_READY_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_READY_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_REGISTER_MODE(pub i32);
impl windows_core::TypeKind for MBN_REGISTER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_REGISTER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_REGISTER_STATE(pub i32);
impl windows_core::TypeKind for MBN_REGISTER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_REGISTER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_REGISTER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_REGISTRATION_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_REGISTRATION_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_REGISTRATION_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_REGISTRATION_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SIGNAL_CONSTANTS(pub i32);
impl windows_core::TypeKind for MBN_SIGNAL_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SIGNAL_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SIGNAL_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_CAPS(pub i32);
impl windows_core::TypeKind for MBN_SMS_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_CAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_CAPS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_CDMA_ENCODING(pub i32);
impl windows_core::TypeKind for MBN_SMS_CDMA_ENCODING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_CDMA_ENCODING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_ENCODING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_CDMA_LANG(pub i32);
impl windows_core::TypeKind for MBN_SMS_CDMA_LANG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_CDMA_LANG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_CDMA_LANG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_FLAG(pub i32);
impl windows_core::TypeKind for MBN_SMS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_FORMAT(pub i32);
impl windows_core::TypeKind for MBN_SMS_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_SMS_STATUS_FLAG(pub i32);
impl windows_core::TypeKind for MBN_SMS_STATUS_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_SMS_STATUS_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_SMS_STATUS_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_VOICE_CALL_STATE(pub i32);
impl windows_core::TypeKind for MBN_VOICE_CALL_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_VOICE_CALL_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CALL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MBN_VOICE_CLASS(pub i32);
impl windows_core::TypeKind for MBN_VOICE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MBN_VOICE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MBN_VOICE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WWAEXT_SMS_CONSTANTS(pub i32);
impl windows_core::TypeKind for WWAEXT_SMS_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WWAEXT_SMS_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WWAEXT_SMS_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub userName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub password: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
impl Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MBN_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dataWriteSupported: super::super::Foundation::VARIANT_BOOL,
    pub dataReadSupported: super::super::Foundation::VARIANT_BOOL,
}
impl Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MBN_DEVICE_SERVICE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_DEVICE_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
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
impl Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MBN_INTERFACE_CAPS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_INTERFACE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl windows_core::TypeKind for MBN_PIN_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MBN_PROVIDER {
    pub providerID: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub providerState: u32,
    pub providerName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dataClass: u32,
}
impl Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MBN_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
impl Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MBN_PROVIDER2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_PROVIDER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl windows_core::TypeKind for MBN_SMS_FILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_SMS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl windows_core::TypeKind for MBN_SMS_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MBN_SMS_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MbnConnectionManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
pub const MbnConnectionProfileManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
pub const MbnDeviceServicesManager: windows_core::GUID = windows_core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
pub const MbnInterfaceManager: windows_core::GUID = windows_core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl windows_core::TypeKind for __DummyPinType__ {
    type TypeKind = windows_core::CopyType;
}
impl Default for __DummyPinType__ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for __mbnapi_ReferenceRemainingTypes__ {
    type TypeKind = windows_core::CopyType;
}
impl Default for __mbnapi_ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
