windows_core::imp::define_interface!(IFindAllAccountsResult, IFindAllAccountsResult_Vtbl, 0xa5812b5d_b72e_420c_86ab_aac0d7b7261f);
impl windows_core::RuntimeType for IFindAllAccountsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFindAllAccountsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub Accounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    Accounts: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FindAllWebAccountsStatus) -> windows_core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountEventArgs, IWebAccountEventArgs_Vtbl, 0x6fb7037d_424e_44ec_977c_ef2415462a5a);
impl windows_core::RuntimeType for IWebAccountEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
windows_core::imp::define_interface!(IWebAccountMonitor, IWebAccountMonitor_Vtbl, 0x7445f5fd_aa9d_4619_8d5d_c138a4ede3e5);
impl windows_core::RuntimeType for IWebAccountMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountMonitor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DefaultSignInAccountChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDefaultSignInAccountChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountMonitor2, IWebAccountMonitor2_Vtbl, 0xa7adc1f8_24b8_4f01_9ae5_24545e71233a);
impl windows_core::RuntimeType for IWebAccountMonitor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountMonitor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountPictureUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAccountPictureUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerStatics, IWebAuthenticationCoreManagerStatics_Vtbl, 0x6aca7c92_a581_4479_9c10_752eff44fd34);
impl windows_core::RuntimeType for IWebAuthenticationCoreManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTokenSilentlyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub GetTokenSilentlyWithWebAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetTokenSilentlyWithWebAccountAsync: usize,
    pub RequestTokenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub RequestTokenWithWebAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    RequestTokenWithWebAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindAccountProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindAccountProviderAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindAccountProviderWithAuthorityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindAccountProviderWithAuthorityAsync: usize,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerStatics2, IWebAuthenticationCoreManagerStatics2_Vtbl, 0xf584184a_8b57_4820_b6a4_70a5b6fcf44a);
impl windows_core::RuntimeType for IWebAuthenticationCoreManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Security_Credentials", feature = "System"))]
    pub FindAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "System")))]
    FindAccountProviderWithAuthorityForUserAsync: usize,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerStatics3, IWebAuthenticationCoreManagerStatics3_Vtbl, 0x2404eeb2_8924_4d93_ab3a_99688b419d56);
impl windows_core::RuntimeType for IWebAuthenticationCoreManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub CreateWebAccountMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    CreateWebAccountMonitor: usize,
}
windows_core::imp::define_interface!(IWebAuthenticationCoreManagerStatics4, IWebAuthenticationCoreManagerStatics4_Vtbl, 0x54e633fe_96e0_41e8_9832_1298897c2aaf);
impl windows_core::RuntimeType for IWebAuthenticationCoreManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub FindAllAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindAllAccountsAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindAllAccountsWithClientIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindAllAccountsWithClientIdAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindSystemAccountProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindSystemAccountProviderAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub FindSystemAccountProviderWithAuthorityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    FindSystemAccountProviderWithAuthorityAsync: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "System"))]
    pub FindSystemAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "System")))]
    FindSystemAccountProviderWithAuthorityForUserAsync: usize,
}
windows_core::imp::define_interface!(IWebProviderError, IWebProviderError_Vtbl, 0xdb191bb1_50c5_4809_8dca_09c99410245c);
impl windows_core::RuntimeType for IWebProviderError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderError_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IWebProviderErrorFactory, IWebProviderErrorFactory_Vtbl, 0xe3c40a2d_89ef_4e37_847f_a8b9d5a32910);
impl windows_core::RuntimeType for IWebProviderErrorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderErrorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebTokenRequest, IWebTokenRequest_Vtbl, 0xb77b4d68_adcb_4673_b364_0cf7b35caf97);
impl windows_core::RuntimeType for IWebTokenRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PromptType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebTokenRequestPromptType) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IWebTokenRequest2, IWebTokenRequest2_Vtbl, 0xd700c079_30c8_4397_9654_961c3be8b855);
impl windows_core::RuntimeType for IWebTokenRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppProperties: usize,
}
windows_core::imp::define_interface!(IWebTokenRequest3, IWebTokenRequest3_Vtbl, 0x5a755b51_3bb1_41a5_a63d_90bc32c7db9a);
impl windows_core::RuntimeType for IWebTokenRequest3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenRequest3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebTokenRequestFactory, IWebTokenRequestFactory_Vtbl, 0x6cf2141c_0ff0_4c67_b84f_99ddbe4a72c9);
impl windows_core::RuntimeType for IWebTokenRequestFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithPromptType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, WebTokenRequestPromptType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithPromptType: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithProvider: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithScope: usize,
}
windows_core::imp::define_interface!(IWebTokenRequestResult, IWebTokenRequestResult_Vtbl, 0xc12a8305_d1f8_4483_8d54_38fe292784ff);
impl windows_core::RuntimeType for IWebTokenRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenRequestResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ResponseData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResponseData: usize,
    pub ResponseStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebTokenRequestStatus) -> windows_core::HRESULT,
    pub ResponseError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvalidateCacheAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebTokenResponse, IWebTokenResponse_Vtbl, 0x67a7c5ca_83f6_44c6_a3b1_0eb69e41fa8a);
impl windows_core::RuntimeType for IWebTokenResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Token: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IWebTokenResponseFactory, IWebTokenResponseFactory_Vtbl, 0xab6bf7f8_5450_4ef6_97f7_052b0431c0f0);
impl windows_core::RuntimeType for IWebTokenResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebTokenResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithToken: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAndAccount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAndAccount: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAccountAndError: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAccountAndError: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FindAllAccountsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FindAllAccountsResult, windows_core::IUnknown, windows_core::IInspectable);
impl FindAllAccountsResult {
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn Accounts(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Accounts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<FindAllWebAccountsStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProviderError(&self) -> windows_core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FindAllAccountsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFindAllAccountsResult>();
}
unsafe impl windows_core::Interface for FindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_Vtbl;
    const IID: windows_core::GUID = <IFindAllAccountsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
}
unsafe impl Send for FindAllAccountsResult {}
unsafe impl Sync for FindAllAccountsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebAccountEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountEventArgs {
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(&self) -> windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Account)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAccountEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountEventArgs>();
}
unsafe impl windows_core::Interface for WebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWebAccountEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
}
unsafe impl Send for WebAccountEventArgs {}
unsafe impl Sync for WebAccountEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebAccountMonitor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountMonitor, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountMonitor {
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DefaultSignInAccountChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultSignInAccountChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDefaultSignInAccountChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDefaultSignInAccountChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AccountPictureUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPictureUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccountPictureUpdated(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccountPictureUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for WebAccountMonitor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountMonitor>();
}
unsafe impl windows_core::Interface for WebAccountMonitor {
    type Vtable = IWebAccountMonitor_Vtbl;
    const IID: windows_core::GUID = <IWebAccountMonitor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountMonitor";
}
unsafe impl Send for WebAccountMonitor {}
unsafe impl Sync for WebAccountMonitor {}
pub struct WebAuthenticationCoreManager;
impl WebAuthenticationCoreManager {
    pub fn GetTokenSilentlyAsync<P0>(request: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>
    where
        P0: windows_core::Param<WebTokenRequest>,
    {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokenSilentlyAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn GetTokenSilentlyWithWebAccountAsync<P0, P1>(request: P0, webaccount: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>
    where
        P0: windows_core::Param<WebTokenRequest>,
        P1: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTokenSilentlyWithWebAccountAsync)(windows_core::Interface::as_raw(this), request.param().abi(), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestTokenAsync<P0>(request: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>
    where
        P0: windows_core::Param<WebTokenRequest>,
    {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestTokenAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn RequestTokenWithWebAccountAsync<P0, P1>(request: P0, webaccount: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>
    where
        P0: windows_core::Param<WebTokenRequest>,
        P1: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestTokenWithWebAccountAsync)(windows_core::Interface::as_raw(this), request.param().abi(), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindAccountAsync<P0>(provider: P0, webaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAccountAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), core::mem::transmute_copy(webaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindAccountProviderAsync(webaccountproviderid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAccountProviderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindAccountProviderWithAuthorityAsync(webaccountproviderid: &windows_core::HSTRING, authority: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAccountProviderWithAuthorityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), core::mem::transmute_copy(authority), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Security_Credentials", feature = "System"))]
    pub fn FindAccountProviderWithAuthorityForUserAsync<P0>(webaccountproviderid: &windows_core::HSTRING, authority: &windows_core::HSTRING, user: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        Self::IWebAuthenticationCoreManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAccountProviderWithAuthorityForUserAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), core::mem::transmute_copy(authority), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn CreateWebAccountMonitor<P0>(webaccounts: P0) -> windows_core::Result<WebAccountMonitor>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>,
    {
        Self::IWebAuthenticationCoreManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWebAccountMonitor)(windows_core::Interface::as_raw(this), webaccounts.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindAllAccountsAsync<P0>(provider: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAccountsAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindAllAccountsWithClientIdAsync<P0>(provider: P0, clientid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAccountsWithClientIdAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), core::mem::transmute_copy(clientid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindSystemAccountProviderAsync(webaccountproviderid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSystemAccountProviderAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn FindSystemAccountProviderWithAuthorityAsync(webaccountproviderid: &windows_core::HSTRING, authority: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), core::mem::transmute_copy(authority), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Security_Credentials", feature = "System"))]
    pub fn FindSystemAccountProviderWithAuthorityForUserAsync<P0>(webaccountproviderid: &windows_core::HSTRING, authority: &windows_core::HSTRING, user: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSystemAccountProviderWithAuthorityForUserAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountproviderid), core::mem::transmute_copy(authority), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics2<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics3<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerStatics4<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebAuthenticationCoreManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebProviderError(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebProviderError, windows_core::IUnknown, windows_core::IInspectable);
impl WebProviderError {
    pub fn ErrorCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(errorcode: u32, errormessage: &windows_core::HSTRING) -> windows_core::Result<WebProviderError> {
        Self::IWebProviderErrorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), errorcode, core::mem::transmute_copy(errormessage), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderErrorFactory<R, F: FnOnce(&IWebProviderErrorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebProviderError, IWebProviderErrorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebProviderError {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebProviderError>();
}
unsafe impl windows_core::Interface for WebProviderError {
    type Vtable = IWebProviderError_Vtbl;
    const IID: windows_core::GUID = <IWebProviderError as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebProviderError";
}
unsafe impl Send for WebProviderError {}
unsafe impl Sync for WebProviderError {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebTokenRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebTokenRequest, windows_core::IUnknown, windows_core::IInspectable);
impl WebTokenRequest {
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> windows_core::Result<super::super::super::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccountProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Scope(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scope)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClientId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PromptType(&self) -> windows_core::Result<WebTokenRequestPromptType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PromptType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppProperties(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IWebTokenRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CorrelationId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCorrelationId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCorrelationId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn Create<P0>(provider: P0, scope: &windows_core::HSTRING, clientid: &windows_core::HSTRING) -> windows_core::Result<WebTokenRequest>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), provider.param().abi(), core::mem::transmute_copy(scope), core::mem::transmute_copy(clientid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithPromptType<P0>(provider: P0, scope: &windows_core::HSTRING, clientid: &windows_core::HSTRING, prompttype: WebTokenRequestPromptType) -> windows_core::Result<WebTokenRequest>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPromptType)(windows_core::Interface::as_raw(this), provider.param().abi(), core::mem::transmute_copy(scope), core::mem::transmute_copy(clientid), prompttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithProvider<P0>(provider: P0) -> windows_core::Result<WebTokenRequest>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProvider)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithScope<P0>(provider: P0, scope: &windows_core::HSTRING) -> windows_core::Result<WebTokenRequest>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccountProvider>,
    {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithScope)(windows_core::Interface::as_raw(this), provider.param().abi(), core::mem::transmute_copy(scope), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenRequestFactory<R, F: FnOnce(&IWebTokenRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebTokenRequest, IWebTokenRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebTokenRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebTokenRequest>();
}
unsafe impl windows_core::Interface for WebTokenRequest {
    type Vtable = IWebTokenRequest_Vtbl;
    const IID: windows_core::GUID = <IWebTokenRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequest";
}
unsafe impl Send for WebTokenRequest {}
unsafe impl Sync for WebTokenRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebTokenRequestResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebTokenRequestResult, windows_core::IUnknown, windows_core::IInspectable);
impl WebTokenRequestResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResponseData(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResponseStatus(&self) -> windows_core::Result<WebTokenRequestStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResponseError(&self) -> windows_core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InvalidateCacheAsync(&self) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidateCacheAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebTokenRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebTokenRequestResult>();
}
unsafe impl windows_core::Interface for WebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_Vtbl;
    const IID: windows_core::GUID = <IWebTokenRequestResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
}
unsafe impl Send for WebTokenRequestResult {}
unsafe impl Sync for WebTokenRequestResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebTokenResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebTokenResponse, windows_core::IUnknown, windows_core::IInspectable);
impl WebTokenResponse {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebTokenResponse, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Token(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Token)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProviderError(&self) -> windows_core::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderError)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWithToken(token: &windows_core::HSTRING) -> windows_core::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAndAccount<P0>(token: &windows_core::HSTRING, webaccount: P0) -> windows_core::Result<WebTokenResponse>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTokenAndAccount)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWithTokenAccountAndError<P0, P1>(token: &windows_core::HSTRING, webaccount: P0, error: P1) -> windows_core::Result<WebTokenResponse>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
        P1: windows_core::Param<WebProviderError>,
    {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithTokenAccountAndError)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), webaccount.param().abi(), error.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebTokenResponseFactory<R, F: FnOnce(&IWebTokenResponseFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebTokenResponse, IWebTokenResponseFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebTokenResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebTokenResponse>();
}
unsafe impl windows_core::Interface for WebTokenResponse {
    type Vtable = IWebTokenResponse_Vtbl;
    const IID: windows_core::GUID = <IWebTokenResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenResponse";
}
unsafe impl Send for WebTokenResponse {}
unsafe impl Sync for WebTokenResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAllowedByProvider: Self = Self(1i32);
    pub const NotSupportedByProvider: Self = Self(2i32);
    pub const ProviderError: Self = Self(3i32);
}
impl windows_core::TypeKind for FindAllWebAccountsStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FindAllWebAccountsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FindAllWebAccountsStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FindAllWebAccountsStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: Self = Self(0i32);
    pub const ForceAuthentication: Self = Self(1i32);
}
impl windows_core::TypeKind for WebTokenRequestPromptType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebTokenRequestPromptType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebTokenRequestPromptType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebTokenRequestPromptType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const AccountSwitch: Self = Self(2i32);
    pub const UserInteractionRequired: Self = Self(3i32);
    pub const AccountProviderNotAvailable: Self = Self(4i32);
    pub const ProviderError: Self = Self(5i32);
}
impl windows_core::TypeKind for WebTokenRequestStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebTokenRequestStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebTokenRequestStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebTokenRequestStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestStatus;i4)");
}
