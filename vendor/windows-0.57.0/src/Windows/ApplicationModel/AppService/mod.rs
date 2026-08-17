windows_core::imp::define_interface!(IAppServiceCatalogStatics, IAppServiceCatalogStatics_Vtbl, 0xef0d2507_d132_4c85_8395_3c31d5a1e941);
impl windows_core::RuntimeType for IAppServiceCatalogStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceCatalogStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppServiceProvidersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppServiceProvidersAsync: usize,
}
windows_core::imp::define_interface!(IAppServiceClosedEventArgs, IAppServiceClosedEventArgs_Vtbl, 0xde6016f6_cb03_4d35_ac8d_cc6303239731);
impl windows_core::RuntimeType for IAppServiceClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppServiceClosedStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceConnection, IAppServiceConnection_Vtbl, 0x9dd474a2_871f_4d52_89a9_9e090531bd27);
impl windows_core::RuntimeType for IAppServiceConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAppServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SendMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendMessageAsync: usize,
    pub RequestReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRequestReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ServiceClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServiceClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceConnection2, IAppServiceConnection2_Vtbl, 0x8bdfcd5f_2302_4fbd_8061_52511c2f8bf9);
impl windows_core::RuntimeType for IAppServiceConnection2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceConnection2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System_RemoteSystems")]
    pub OpenRemoteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    OpenRemoteAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "System")]
    pub SetUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetUser: usize,
}
windows_core::imp::define_interface!(IAppServiceConnectionStatics, IAppServiceConnectionStatics_Vtbl, 0xadc56ce9_d408_5673_8637_827a4b274168);
impl windows_core::RuntimeType for IAppServiceConnectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceConnectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub SendStatelessMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    SendStatelessMessageAsync: usize,
}
windows_core::imp::define_interface!(IAppServiceDeferral, IAppServiceDeferral_Vtbl, 0x7e1b5322_eab0_4248_ae04_fdf93838e472);
impl windows_core::RuntimeType for IAppServiceDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceRequest, IAppServiceRequest_Vtbl, 0x20e58d9d_18de_4b01_80ba_90a76204e3c8);
impl windows_core::RuntimeType for IAppServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendResponseAsync: usize,
}
windows_core::imp::define_interface!(IAppServiceRequestReceivedEventArgs, IAppServiceRequestReceivedEventArgs_Vtbl, 0x6e122360_ff65_44ae_9e45_857fe4180681);
impl windows_core::RuntimeType for IAppServiceRequestReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceRequestReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceResponse, IAppServiceResponse_Vtbl, 0x8d503cec_9aa3_4e68_9559_9de63e372ce4);
impl windows_core::RuntimeType for IAppServiceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppServiceResponseStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceTriggerDetails, IAppServiceTriggerDetails_Vtbl, 0x88a2dcac_ad28_41b8_80bb_bdf1b2169e19);
impl windows_core::RuntimeType for IAppServiceTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CallerPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppServiceConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceTriggerDetails2, IAppServiceTriggerDetails2_Vtbl, 0xe83d54b2_28cc_43f2_b465_c0482e59e2dc);
impl windows_core::RuntimeType for IAppServiceTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemoteSystemConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceTriggerDetails3, IAppServiceTriggerDetails3_Vtbl, 0xfbd71e21_7939_4e68_9e3c_7780147aabb6);
impl windows_core::RuntimeType for IAppServiceTriggerDetails3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceTriggerDetails3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CheckCallerForCapabilityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppServiceTriggerDetails4, IAppServiceTriggerDetails4_Vtbl, 0x1185b180_8861_5e30_ab55_1cf4d08bbf6d);
impl windows_core::RuntimeType for IAppServiceTriggerDetails4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppServiceTriggerDetails4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CallerRemoteConnectionToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStatelessAppServiceResponse, IStatelessAppServiceResponse_Vtbl, 0x43754af7_a9ec_52fe_82e7_939b68dc9388);
impl windows_core::RuntimeType for IStatelessAppServiceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStatelessAppServiceResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StatelessAppServiceResponseStatus) -> windows_core::HRESULT,
}
pub struct AppServiceCatalog;
impl AppServiceCatalog {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppServiceProvidersAsync(appservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>> {
        Self::IAppServiceCatalogStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppServiceProvidersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppServiceCatalogStatics<R, F: FnOnce(&IAppServiceCatalogStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppServiceCatalog, IAppServiceCatalogStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AppServiceCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceCatalog";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceClosedEventArgs {
    pub fn Status(&self) -> windows_core::Result<AppServiceClosedStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppServiceClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceClosedEventArgs>();
}
unsafe impl windows_core::Interface for AppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppServiceClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
}
unsafe impl Send for AppServiceClosedEventArgs {}
unsafe impl Sync for AppServiceClosedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppServiceConnection, super::super::Foundation::IClosable);
impl AppServiceConnection {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppServiceConnection, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AppServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppServiceName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppServiceName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPackageFamilyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OpenAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendMessageAsync<P0>(&self, message: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRequestReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRequestReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ServiceClosed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceClosed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServiceClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveServiceClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System_RemoteSystems")]
    pub fn OpenRemoteAsync<P0>(&self, remotesystemconnectionrequest: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>
    where
        P0: windows_core::Param<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>,
    {
        let this = &windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenRemoteAsync)(windows_core::Interface::as_raw(this), remotesystemconnectionrequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn SetUser<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppServiceConnection2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUser)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn SendStatelessMessageAsync<P0, P1, P2>(connection: P0, connectionrequest: P1, message: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>
    where
        P0: windows_core::Param<AppServiceConnection>,
        P1: windows_core::Param<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>,
        P2: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        Self::IAppServiceConnectionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendStatelessMessageAsync)(windows_core::Interface::as_raw(this), connection.param().abi(), connectionrequest.param().abi(), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAppServiceConnectionStatics<R, F: FnOnce(&IAppServiceConnectionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppServiceConnection, IAppServiceConnectionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppServiceConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceConnection>();
}
unsafe impl windows_core::Interface for AppServiceConnection {
    type Vtable = IAppServiceConnection_Vtbl;
    const IID: windows_core::GUID = <IAppServiceConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceConnection";
}
unsafe impl Send for AppServiceConnection {}
unsafe impl Sync for AppServiceConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppServiceDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceDeferral>();
}
unsafe impl windows_core::Interface for AppServiceDeferral {
    type Vtable = IAppServiceDeferral_Vtbl;
    const IID: windows_core::GUID = <IAppServiceDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceDeferral";
}
unsafe impl Send for AppServiceDeferral {}
unsafe impl Sync for AppServiceDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceRequest {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendResponseAsync<P0>(&self, message: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendResponseAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceRequest>();
}
unsafe impl windows_core::Interface for AppServiceRequest {
    type Vtable = IAppServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IAppServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequest";
}
unsafe impl Send for AppServiceRequest {}
unsafe impl Sync for AppServiceRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceRequestReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceRequestReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceRequestReceivedEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<AppServiceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppServiceRequestReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceRequestReceivedEventArgs>();
}
unsafe impl windows_core::Interface for AppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppServiceRequestReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
}
unsafe impl Send for AppServiceRequestReceivedEventArgs {}
unsafe impl Sync for AppServiceRequestReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceResponse, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<AppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppServiceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceResponse>();
}
unsafe impl windows_core::Interface for AppServiceResponse {
    type Vtable = IAppServiceResponse_Vtbl;
    const IID: windows_core::GUID = <IAppServiceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceResponse";
}
unsafe impl Send for AppServiceResponse {}
unsafe impl Sync for AppServiceResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppServiceTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppServiceTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppServiceTriggerDetails {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppServiceConnection(&self) -> windows_core::Result<AppServiceConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppServiceConnection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRemoteSystemConnection(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRemoteSystemConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CheckCallerForCapabilityAsync(&self, capabilityname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IAppServiceTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckCallerForCapabilityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(capabilityname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CallerRemoteConnectionToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppServiceTriggerDetails4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallerRemoteConnectionToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppServiceTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppServiceTriggerDetails>();
}
unsafe impl windows_core::Interface for AppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IAppServiceTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
}
unsafe impl Send for AppServiceTriggerDetails {}
unsafe impl Sync for AppServiceTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StatelessAppServiceResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StatelessAppServiceResponse, windows_core::IUnknown, windows_core::IInspectable);
impl StatelessAppServiceResponse {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<StatelessAppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for StatelessAppServiceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStatelessAppServiceResponse>();
}
unsafe impl windows_core::Interface for StatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_Vtbl;
    const IID: windows_core::GUID = <IStatelessAppServiceResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
}
unsafe impl Send for StatelessAppServiceResponse {}
unsafe impl Sync for StatelessAppServiceResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl windows_core::TypeKind for AppServiceClosedStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppServiceClosedStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppServiceClosedStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppServiceClosedStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceClosedStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl windows_core::TypeKind for AppServiceConnectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppServiceConnectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppServiceConnectionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppServiceConnectionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl windows_core::TypeKind for AppServiceResponseStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppServiceResponseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppServiceResponseStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppServiceResponseStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceResponseStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl windows_core::TypeKind for StatelessAppServiceResponseStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StatelessAppServiceResponseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponseStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StatelessAppServiceResponseStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus;i4)");
}
