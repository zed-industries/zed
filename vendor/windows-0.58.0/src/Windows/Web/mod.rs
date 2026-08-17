#[cfg(feature = "Web_AtomPub")]
pub mod AtomPub;
#[cfg(feature = "Web_Http")]
pub mod Http;
#[cfg(feature = "Web_Syndication")]
pub mod Syndication;
#[cfg(feature = "Web_UI")]
pub mod UI;
windows_core::imp::define_interface!(IUriToStreamResolver, IUriToStreamResolver_Vtbl, 0xb0aba86a_9aeb_4d3a_9590_003e3ca7e290);
impl core::ops::Deref for IUriToStreamResolver {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUriToStreamResolver, windows_core::IUnknown, windows_core::IInspectable);
impl IUriToStreamResolver {
    #[cfg(feature = "Storage_Streams")]
    pub fn UriToStreamAsync<P0>(&self, uri: P0) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>
    where
        P0: windows_core::Param<super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UriToStreamAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IUriToStreamResolver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriToStreamResolver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub UriToStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UriToStreamAsync: usize,
}
windows_core::imp::define_interface!(IWebErrorStatics, IWebErrorStatics_Vtbl, 0xfe616766_bf27_4064_87b7_6563bb11ce2e);
impl windows_core::RuntimeType for IWebErrorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebErrorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut WebErrorStatus) -> windows_core::HRESULT,
}
pub struct WebError;
impl WebError {
    pub fn GetStatus(hresult: i32) -> windows_core::Result<WebErrorStatus> {
        Self::IWebErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebErrorStatics<R, F: FnOnce(&IWebErrorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebError, IWebErrorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebError {
    const NAME: &'static str = "Windows.Web.WebError";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebErrorStatus(pub i32);
impl WebErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(1i32);
    pub const CertificateExpired: Self = Self(2i32);
    pub const CertificateContainsErrors: Self = Self(3i32);
    pub const CertificateRevoked: Self = Self(4i32);
    pub const CertificateIsInvalid: Self = Self(5i32);
    pub const ServerUnreachable: Self = Self(6i32);
    pub const Timeout: Self = Self(7i32);
    pub const ErrorHttpInvalidServerResponse: Self = Self(8i32);
    pub const ConnectionAborted: Self = Self(9i32);
    pub const ConnectionReset: Self = Self(10i32);
    pub const Disconnected: Self = Self(11i32);
    pub const HttpToHttpsOnRedirection: Self = Self(12i32);
    pub const HttpsToHttpOnRedirection: Self = Self(13i32);
    pub const CannotConnect: Self = Self(14i32);
    pub const HostNameNotResolved: Self = Self(15i32);
    pub const OperationCanceled: Self = Self(16i32);
    pub const RedirectFailed: Self = Self(17i32);
    pub const UnexpectedStatusCode: Self = Self(18i32);
    pub const UnexpectedRedirection: Self = Self(19i32);
    pub const UnexpectedClientError: Self = Self(20i32);
    pub const UnexpectedServerError: Self = Self(21i32);
    pub const InsufficientRangeSupport: Self = Self(22i32);
    pub const MissingContentLengthSupport: Self = Self(23i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
}
impl windows_core::TypeKind for WebErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebErrorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.WebErrorStatus;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
