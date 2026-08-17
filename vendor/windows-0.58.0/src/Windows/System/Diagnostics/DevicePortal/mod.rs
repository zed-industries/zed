windows_core::imp::define_interface!(IDevicePortalConnection, IDevicePortalConnection_Vtbl, 0x0f447f51_1198_4da1_8d54_bdef393e09b6);
impl windows_core::RuntimeType for IDevicePortalConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRequestReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePortalConnectionClosedEventArgs, IDevicePortalConnectionClosedEventArgs_Vtbl, 0xfcf70e38_7032_428c_9f50_945c15a9f0cb);
impl windows_core::RuntimeType for IDevicePortalConnectionClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalConnectionClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DevicePortalConnectionClosedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDevicePortalConnectionRequestReceivedEventArgs, IDevicePortalConnectionRequestReceivedEventArgs_Vtbl, 0x64dae045_6fda_4459_9ebd_ecce22e38559);
impl windows_core::RuntimeType for IDevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web_Http")]
    pub RequestMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    RequestMessage: usize,
    #[cfg(feature = "Web_Http")]
    pub ResponseMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    ResponseMessage: usize,
}
windows_core::imp::define_interface!(IDevicePortalConnectionStatics, IDevicePortalConnectionStatics_Vtbl, 0x4bbe31e7_e9b9_4645_8fed_a53eea0edbd6);
impl windows_core::RuntimeType for IDevicePortalConnectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalConnectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub GetForAppServiceConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    GetForAppServiceConnection: usize,
}
windows_core::imp::define_interface!(IDevicePortalWebSocketConnection, IDevicePortalWebSocketConnection_Vtbl, 0x67657920_d65a_42f0_aef4_787808098b7b);
impl windows_core::RuntimeType for IDevicePortalWebSocketConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalWebSocketConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Networking::Sockets::SocketMessageType, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest2: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerMessageWebSocketForRequest3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Networking::Sockets::SocketMessageType, core::mem::MaybeUninit<windows_core::HSTRING>, u32, u32, super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerMessageWebSocketForRequest3: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub GetServerStreamWebSocketForRequest2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "Web_Http")))]
    GetServerStreamWebSocketForRequest2: usize,
}
windows_core::imp::define_interface!(IDevicePortalWebSocketConnectionRequestReceivedEventArgs, IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl, 0x79fdcaba_175c_4739_9f74_dda797c35b3f);
impl windows_core::RuntimeType for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsWebSocketUpgradeRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub WebSocketProtocolsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebSocketProtocolsRequested: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePortalConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePortalConnection, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePortalConnection {
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRequestReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn GetForAppServiceConnection<P0>(appserviceconnection: P0) -> windows_core::Result<DevicePortalConnection>
    where
        P0: windows_core::Param<super::super::super::ApplicationModel::AppService::AppServiceConnection>,
    {
        Self::IDevicePortalConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForAppServiceConnection)(windows_core::Interface::as_raw(this), appserviceconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest2<P0>(&self, request: P0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest2)(windows_core::Interface::as_raw(this), request.param().abi(), messagetype, core::mem::transmute_copy(protocol), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerMessageWebSocketForRequest3<P0>(&self, request: P0, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &windows_core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> windows_core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetServerMessageWebSocketForRequest3)(windows_core::Interface::as_raw(this), request.param().abi(), messagetype, core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, maxmessagesize, receivemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Networking_Sockets", feature = "Web_Http"))]
    pub fn GetServerStreamWebSocketForRequest2<P0>(&self, request: P0, protocol: &windows_core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool) -> windows_core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>
    where
        P0: windows_core::Param<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnection>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetServerStreamWebSocketForRequest2)(windows_core::Interface::as_raw(this), request.param().abi(), core::mem::transmute_copy(protocol), outboundbuffersizeinbytes, nodelay, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IDevicePortalConnectionStatics<R, F: FnOnce(&IDevicePortalConnectionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DevicePortalConnection, IDevicePortalConnectionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DevicePortalConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePortalConnection>();
}
unsafe impl windows_core::Interface for DevicePortalConnection {
    type Vtable = IDevicePortalConnection_Vtbl;
    const IID: windows_core::GUID = <IDevicePortalConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnection";
}
unsafe impl Send for DevicePortalConnection {}
unsafe impl Sync for DevicePortalConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePortalConnectionClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePortalConnectionClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePortalConnectionClosedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<DevicePortalConnectionClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DevicePortalConnectionClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePortalConnectionClosedEventArgs>();
}
unsafe impl windows_core::Interface for DevicePortalConnectionClosedEventArgs {
    type Vtable = IDevicePortalConnectionClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDevicePortalConnectionClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedEventArgs";
}
unsafe impl Send for DevicePortalConnectionClosedEventArgs {}
unsafe impl Sync for DevicePortalConnectionClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DevicePortalConnectionRequestReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DevicePortalConnectionRequestReceivedEventArgs {
    #[cfg(feature = "Web_Http")]
    pub fn RequestMessage(&self) -> windows_core::Result<super::super::super::Web::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Web_Http")]
    pub fn ResponseMessage(&self) -> windows_core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWebSocketUpgradeRequest(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWebSocketUpgradeRequest)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebSocketProtocolsRequested(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebSocketProtocolsRequested)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = &windows_core::Interface::cast::<IDevicePortalWebSocketConnectionRequestReceivedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DevicePortalConnectionRequestReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDevicePortalConnectionRequestReceivedEventArgs>();
}
unsafe impl windows_core::Interface for DevicePortalConnectionRequestReceivedEventArgs {
    type Vtable = IDevicePortalConnectionRequestReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDevicePortalConnectionRequestReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionRequestReceivedEventArgs";
}
unsafe impl Send for DevicePortalConnectionRequestReceivedEventArgs {}
unsafe impl Sync for DevicePortalConnectionRequestReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: Self = Self(0i32);
    pub const ResourceLimitsExceeded: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const NotAuthorized: Self = Self(3i32);
    pub const UserNotPresent: Self = Self(4i32);
    pub const ServiceTerminated: Self = Self(5i32);
}
impl windows_core::TypeKind for DevicePortalConnectionClosedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DevicePortalConnectionClosedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DevicePortalConnectionClosedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DevicePortalConnectionClosedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.DevicePortal.DevicePortalConnectionClosedReason;i4)");
}
