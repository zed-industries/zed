windows_core::imp::define_interface!(IHttpBaseProtocolFilter, IHttpBaseProtocolFilter_Vtbl, 0x71c89b09_e131_4b54_a53c_eb43ff37e9bb);
impl windows_core::RuntimeType for IHttpBaseProtocolFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowAutoRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowAutoRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowUI: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutomaticDecompression: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutomaticDecompression: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CookieManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
    pub MaxConnectionsPerServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxConnectionsPerServer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    pub UseProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseProxy: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpBaseProtocolFilter2, IHttpBaseProtocolFilter2_Vtbl, 0x2ec30013_9427_4900_a017_fa7da3b5c9ae);
impl windows_core::RuntimeType for IHttpBaseProtocolFilter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HttpVersion) -> windows_core::HRESULT,
    pub SetMaxVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::HttpVersion) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpBaseProtocolFilter3, IHttpBaseProtocolFilter3_Vtbl, 0xd43f4d4c_bd42_43ae_8717_ad2c8f4b2937);
impl windows_core::RuntimeType for IHttpBaseProtocolFilter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CookieUsageBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HttpCookieUsageBehavior) -> windows_core::HRESULT,
    pub SetCookieUsageBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, HttpCookieUsageBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpBaseProtocolFilter4, IHttpBaseProtocolFilter4_Vtbl, 0x9fe36ccf_2983_4893_941f_eb518ca8cef9);
impl windows_core::RuntimeType for IHttpBaseProtocolFilter4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ClearAuthenticationCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpBaseProtocolFilter5, IHttpBaseProtocolFilter5_Vtbl, 0x416e4993_31e3_4816_bf09_e018ee8dc1f5);
impl windows_core::RuntimeType for IHttpBaseProtocolFilter5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilter5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IHttpBaseProtocolFilterStatics, IHttpBaseProtocolFilterStatics_Vtbl, 0x6d4dee0c_e908_494e_b5a3_1263c9b8242a);
impl windows_core::RuntimeType for IHttpBaseProtocolFilterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpBaseProtocolFilterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
windows_core::imp::define_interface!(IHttpCacheControl, IHttpCacheControl_Vtbl, 0xc77e1cb4_3cea_4eb5_ac85_04e186e63ab7);
impl windows_core::RuntimeType for IHttpCacheControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpCacheControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HttpCacheReadBehavior) -> windows_core::HRESULT,
    pub SetReadBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, HttpCacheReadBehavior) -> windows_core::HRESULT,
    pub WriteBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HttpCacheWriteBehavior) -> windows_core::HRESULT,
    pub SetWriteBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, HttpCacheWriteBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpFilter, IHttpFilter_Vtbl, 0xa4cb6dd5_0902_439e_bfd7_e12552b165ce);
impl core::ops::Deref for IHttpFilter {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IHttpFilter, super::super::super::Foundation::IClosable);
impl IHttpFilter {
    pub fn SendRequestAsync<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: windows_core::Param<super::HttpRequestMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRequestAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IHttpFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpServerCustomValidationRequestedEventArgs, IHttpServerCustomValidationRequestedEventArgs_Vtbl, 0x3165fe32_e7dd_48b7_a361_939c750e63cc);
impl windows_core::RuntimeType for IHttpServerCustomValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHttpServerCustomValidationRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Networking::Sockets::SocketSslErrorSeverity) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ServerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpBaseProtocolFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpBaseProtocolFilter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HttpBaseProtocolFilter, super::super::super::Foundation::IClosable, IHttpFilter);
impl HttpBaseProtocolFilter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpBaseProtocolFilter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AllowAutoRedirect(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowAutoRedirect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowAutoRedirect(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowAutoRedirect)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUI(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowUI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowUI(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowUI)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticDecompression(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticDecompression)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutomaticDecompression(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutomaticDecompression)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheControl(&self) -> windows_core::Result<HttpCacheControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheControl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CookieManager(&self) -> windows_core::Result<super::HttpCookieManager> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CookieManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> windows_core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClientCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxConnectionsPerServer(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxConnectionsPerServer)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxConnectionsPerServer(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxConnectionsPerServer)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UseProxy(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseProxy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseProxy(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUseProxy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVersion(&self) -> windows_core::Result<super::HttpVersion> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxVersion(&self, value: super::HttpVersion) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CookieUsageBehavior(&self) -> windows_core::Result<HttpCookieUsageBehavior> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CookieUsageBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCookieUsageBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ServerCustomValidationRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCustomValidationRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServerCustomValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveServerCustomValidationRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ClearAuthenticationCache(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ClearAuthenticationCache)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<IHttpBaseProtocolFilter5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> windows_core::Result<HttpBaseProtocolFilter>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IHttpBaseProtocolFilterStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SendRequestAsync<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>
    where
        P0: windows_core::Param<super::HttpRequestMessage>,
    {
        let this = &windows_core::Interface::cast::<IHttpFilter>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRequestAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IHttpBaseProtocolFilterStatics<R, F: FnOnce(&IHttpBaseProtocolFilterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HttpBaseProtocolFilter, IHttpBaseProtocolFilterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HttpBaseProtocolFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpBaseProtocolFilter>();
}
unsafe impl windows_core::Interface for HttpBaseProtocolFilter {
    type Vtable = IHttpBaseProtocolFilter_Vtbl;
    const IID: windows_core::GUID = <IHttpBaseProtocolFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpBaseProtocolFilter {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpBaseProtocolFilter";
}
unsafe impl Send for HttpBaseProtocolFilter {}
unsafe impl Sync for HttpBaseProtocolFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpCacheControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpCacheControl, windows_core::IUnknown, windows_core::IInspectable);
impl HttpCacheControl {
    pub fn ReadBehavior(&self) -> windows_core::Result<HttpCacheReadBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReadBehavior(&self, value: HttpCacheReadBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBehavior(&self) -> windows_core::Result<HttpCacheWriteBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWriteBehavior(&self, value: HttpCacheWriteBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWriteBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for HttpCacheControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpCacheControl>();
}
unsafe impl windows_core::Interface for HttpCacheControl {
    type Vtable = IHttpCacheControl_Vtbl;
    const IID: windows_core::GUID = <IHttpCacheControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpCacheControl {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpCacheControl";
}
unsafe impl Send for HttpCacheControl {}
unsafe impl Sync for HttpCacheControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HttpServerCustomValidationRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HttpServerCustomValidationRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HttpServerCustomValidationRequestedEventArgs {
    pub fn RequestMessage(&self) -> windows_core::Result<super::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<super::super::super::Networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reject(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reject)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HttpServerCustomValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHttpServerCustomValidationRequestedEventArgs>();
}
unsafe impl windows_core::Interface for HttpServerCustomValidationRequestedEventArgs {
    type Vtable = IHttpServerCustomValidationRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IHttpServerCustomValidationRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HttpServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.Http.Filters.HttpServerCustomValidationRequestedEventArgs";
}
unsafe impl Send for HttpServerCustomValidationRequestedEventArgs {}
unsafe impl Sync for HttpServerCustomValidationRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: Self = Self(0i32);
    pub const MostRecent: Self = Self(1i32);
    pub const OnlyFromCache: Self = Self(2i32);
    pub const NoCache: Self = Self(3i32);
}
impl windows_core::TypeKind for HttpCacheReadBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HttpCacheReadBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HttpCacheReadBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HttpCacheReadBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheReadBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCache: Self = Self(1i32);
}
impl windows_core::TypeKind for HttpCacheWriteBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HttpCacheWriteBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HttpCacheWriteBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HttpCacheWriteBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCacheWriteBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCookies: Self = Self(1i32);
}
impl windows_core::TypeKind for HttpCookieUsageBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HttpCookieUsageBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HttpCookieUsageBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HttpCookieUsageBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.Http.Filters.HttpCookieUsageBehavior;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
