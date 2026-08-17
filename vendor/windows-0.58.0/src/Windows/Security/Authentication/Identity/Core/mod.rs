windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl, 0x0fd340a5_f574_4320_a08e_0a19a82322aa);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetOneTimePassCodeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateWnsChannelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAndUnregisteredAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAndUnregisteredAccountsAsync: usize,
    pub ApproveSessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MicrosoftAccountMultiFactorSessionAuthenticationStatus, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveSessionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, MicrosoftAccountMultiFactorSessionAuthenticationStatus, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, MicrosoftAccountMultiFactorAuthenticationType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DenySessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DenySessionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, MicrosoftAccountMultiFactorAuthenticationType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorAuthenticatorStatics, IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl, 0xd964c2e6_f446_4c71_8b79_6ea4024aa9b8);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorGetSessionsResult, IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl, 0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    pub ServiceResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicrosoftAccountMultiFactorServiceResponse) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorOneTimeCodedInfo, IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl, 0x82ba264b_d87c_4668_a976_40cfae547d08);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TimeInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub TimeToLive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ServiceResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicrosoftAccountMultiFactorServiceResponse) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorSessionInfo, IMicrosoftAccountMultiFactorSessionInfo_Vtbl, 0x5f7eabb4_a278_4635_b765_b494eb260af4);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorSessionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplaySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ApprovalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicrosoftAccountMultiFactorAuthenticationType) -> windows_core::HRESULT,
    pub RequestTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl, 0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
impl windows_core::RuntimeType for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnregisteredAccounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnregisteredAccounts: usize,
    pub ServiceResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicrosoftAccountMultiFactorServiceResponse) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorAuthenticationManager, windows_core::IUnknown, windows_core::IInspectable);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    pub fn GetOneTimePassCodeAsync(&self, useraccountid: &windows_core::HSTRING, codelength: u32) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOneTimePassCodeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(useraccountid), codelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddDeviceAsync(&self, useraccountid: &windows_core::HSTRING, authenticationtoken: &windows_core::HSTRING, wnschannelid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddDeviceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(useraccountid), core::mem::transmute_copy(authenticationtoken), core::mem::transmute_copy(wnschannelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveDeviceAsync(&self, useraccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveDeviceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(useraccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateWnsChannelAsync(&self, useraccountid: &windows_core::HSTRING, channeluri: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateWnsChannelAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(useraccountid), core::mem::transmute_copy(channeluri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAsync<P0>(&self, useraccountidlist: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSessionsAsync)(windows_core::Interface::as_raw(this), useraccountidlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionsAndUnregisteredAccountsAsync<P0>(&self, useraccountidlist: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSessionsAndUnregisteredAccountsAsync)(windows_core::Interface::as_raw(this), useraccountidlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApproveSessionUsingAuthSessionInfoAsync<P0>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>
    where
        P0: windows_core::Param<MicrosoftAccountMultiFactorSessionInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApproveSessionUsingAuthSessionInfoAsync)(windows_core::Interface::as_raw(this), sessionauthentictionstatus, authenticationsessioninfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApproveSessionAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApproveSessionAsync)(windows_core::Interface::as_raw(this), sessionauthentictionstatus, core::mem::transmute_copy(useraccountid), core::mem::transmute_copy(sessionid), sessionauthenticationtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DenySessionUsingAuthSessionInfoAsync<P0>(&self, authenticationsessioninfo: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>
    where
        P0: windows_core::Param<MicrosoftAccountMultiFactorSessionInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DenySessionUsingAuthSessionInfoAsync)(windows_core::Interface::as_raw(this), authenticationsessioninfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DenySessionAsync(&self, useraccountid: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DenySessionAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(useraccountid), core::mem::transmute_copy(sessionid), sessionauthenticationtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicrosoftAccountMultiFactorAuthenticationManager>();
}
unsafe impl windows_core::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_Vtbl;
    const IID: windows_core::GUID = <IMicrosoftAccountMultiFactorAuthenticationManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
unsafe impl Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorGetSessionsResult, windows_core::IUnknown, windows_core::IInspectable);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sessions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceResponse(&self) -> windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceResponse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicrosoftAccountMultiFactorGetSessionsResult>();
}
unsafe impl windows_core::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_Vtbl;
    const IID: windows_core::GUID = <IMicrosoftAccountMultiFactorGetSessionsResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
unsafe impl Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorOneTimeCodedInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    pub fn Code(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimeInterval(&self) -> windows_core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TimeToLive(&self) -> windows_core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeToLive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceResponse(&self) -> windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceResponse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicrosoftAccountMultiFactorOneTimeCodedInfo>();
}
unsafe impl windows_core::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_Vtbl;
    const IID: windows_core::GUID = <IMicrosoftAccountMultiFactorOneTimeCodedInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
unsafe impl Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MicrosoftAccountMultiFactorSessionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorSessionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MicrosoftAccountMultiFactorSessionInfo {
    pub fn UserAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplaySessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplaySessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApprovalStatus(&self) -> windows_core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApprovalStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AuthenticationType(&self) -> windows_core::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicrosoftAccountMultiFactorSessionInfo>();
}
unsafe impl windows_core::Interface for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_Vtbl;
    const IID: windows_core::GUID = <IMicrosoftAccountMultiFactorSessionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
unsafe impl Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sessions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnregisteredAccounts(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisteredAccounts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceResponse(&self) -> windows_core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceResponse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>();
}
unsafe impl windows_core::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_Vtbl;
    const IID: windows_core::GUID = <IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
unsafe impl Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: Self = Self(0i32);
    pub const Device: Self = Self(1i32);
}
impl windows_core::TypeKind for MicrosoftAccountMultiFactorAuthenticationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const NoNetworkConnection: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const TotpSetupDenied: Self = Self(4i32);
    pub const NgcNotSetup: Self = Self(5i32);
    pub const SessionAlreadyDenied: Self = Self(6i32);
    pub const SessionAlreadyApproved: Self = Self(7i32);
    pub const SessionExpired: Self = Self(8i32);
    pub const NgcNonceExpired: Self = Self(9i32);
    pub const InvalidSessionId: Self = Self(10i32);
    pub const InvalidSessionType: Self = Self(11i32);
    pub const InvalidOperation: Self = Self(12i32);
    pub const InvalidStateTransition: Self = Self(13i32);
    pub const DeviceNotFound: Self = Self(14i32);
    pub const FlowDisabled: Self = Self(15i32);
    pub const SessionNotApproved: Self = Self(16i32);
    pub const OperationCanceledByUser: Self = Self(17i32);
    pub const NgcDisabledByServer: Self = Self(18i32);
    pub const NgcKeyNotFoundOnServer: Self = Self(19i32);
    pub const UIRequired: Self = Self(20i32);
    pub const DeviceIdChanged: Self = Self(21i32);
}
impl windows_core::TypeKind for MicrosoftAccountMultiFactorServiceResponse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MicrosoftAccountMultiFactorServiceResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorServiceResponse").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl windows_core::TypeKind for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionApprovalStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
}
impl windows_core::TypeKind for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionAuthenticationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
}
