windows_core::imp::define_interface!(IOnlineIdAuthenticator, IOnlineIdAuthenticator_Vtbl, 0xa003f58a_29ab_4817_b884_d7516dad18b9);
impl windows_core::RuntimeType for IOnlineIdAuthenticator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdAuthenticator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AuthenticateUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateUserAsyncAdvanced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CredentialPromptType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateUserAsyncAdvanced: usize,
    pub SignOutUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CanSignOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AuthenticatedSafeCustomerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlineIdServiceTicket, IOnlineIdServiceTicket_Vtbl, 0xc95c547f_d781_4a94_acb8_c59874238c26);
impl windows_core::RuntimeType for IOnlineIdServiceTicket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdServiceTicket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequest_Vtbl, 0x297445d3_fb63_4135_8909_4e354c061466);
impl windows_core::RuntimeType for IOnlineIdServiceTicketRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdServiceTicketRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlineIdServiceTicketRequestFactory, IOnlineIdServiceTicketRequestFactory_Vtbl, 0xbebb0a08_9e73_4077_9614_08614c0bc245);
impl windows_core::RuntimeType for IOnlineIdServiceTicketRequestFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdServiceTicketRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateOnlineIdServiceTicketRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOnlineIdServiceTicketRequestAdvanced: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlineIdSystemAuthenticatorForUser, IOnlineIdSystemAuthenticatorForUser_Vtbl, 0x5798befb_1de4_4186_a2e6_b563f86aaf44);
impl windows_core::RuntimeType for IOnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdSystemAuthenticatorForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTicketAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IOnlineIdSystemAuthenticatorStatics, IOnlineIdSystemAuthenticatorStatics_Vtbl, 0x85047792_f634_41e3_96a4_5164e902c740);
impl windows_core::RuntimeType for IOnlineIdSystemAuthenticatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdSystemAuthenticatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IOnlineIdSystemIdentity, IOnlineIdSystemIdentity_Vtbl, 0x743cd20d_b6ca_434d_8124_53ea12685307);
impl windows_core::RuntimeType for IOnlineIdSystemIdentity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdSystemIdentity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Ticket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOnlineIdSystemTicketResult, IOnlineIdSystemTicketResult_Vtbl, 0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
impl windows_core::RuntimeType for IOnlineIdSystemTicketResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IOnlineIdSystemTicketResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Identity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OnlineIdSystemTicketStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserIdentity, IUserIdentity_Vtbl, 0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
impl windows_core::RuntimeType for IUserIdentity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserIdentity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Tickets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tickets: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SafeCustomerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignInName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsBetaAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsConfirmedPC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdAuthenticator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdAuthenticator, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdAuthenticator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OnlineIdAuthenticator, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AuthenticateUserAsync<P0>(&self, request: P0) -> windows_core::Result<UserAuthenticationOperation>
    where
        P0: windows_core::Param<OnlineIdServiceTicketRequest>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticateUserAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AuthenticateUserAsyncAdvanced<P0>(&self, requests: P0, credentialprompttype: CredentialPromptType) -> windows_core::Result<UserAuthenticationOperation>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticateUserAsyncAdvanced)(windows_core::Interface::as_raw(this), requests.param().abi(), credentialprompttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignOutUserAsync(&self) -> windows_core::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignOutUserAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetApplicationId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetApplicationId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanSignOut(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSignOut)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AuthenticatedSafeCustomerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticatedSafeCustomerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdAuthenticator>();
}
unsafe impl windows_core::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdAuthenticator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
unsafe impl Send for OnlineIdAuthenticator {}
unsafe impl Sync for OnlineIdAuthenticator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdServiceTicket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdServiceTicket, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdServiceTicket {
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Request(&self) -> windows_core::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdServiceTicket>();
}
unsafe impl windows_core::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdServiceTicket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
unsafe impl Send for OnlineIdServiceTicket {}
unsafe impl Sync for OnlineIdServiceTicket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdServiceTicketRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdServiceTicketRequest, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdServiceTicketRequest {
    pub fn Service(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Service)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Policy(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Policy)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateOnlineIdServiceTicketRequest(service: &windows_core::HSTRING, policy: &windows_core::HSTRING) -> windows_core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOnlineIdServiceTicketRequest)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(service), core::mem::transmute_copy(policy), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateOnlineIdServiceTicketRequestAdvanced(service: &windows_core::HSTRING) -> windows_core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOnlineIdServiceTicketRequestAdvanced)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(service), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdServiceTicketRequest>();
}
unsafe impl windows_core::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdServiceTicketRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
unsafe impl Send for OnlineIdServiceTicketRequest {}
unsafe impl Sync for OnlineIdServiceTicketRequest {}
pub struct OnlineIdSystemAuthenticator;
impl OnlineIdSystemAuthenticator {
    pub fn Default() -> windows_core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Default)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<OnlineIdSystemAuthenticatorForUser>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdSystemAuthenticatorForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdSystemAuthenticatorForUser, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdSystemAuthenticatorForUser {
    pub fn GetTicketAsync<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>
    where
        P0: windows_core::Param<OnlineIdServiceTicketRequest>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTicketAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetApplicationId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetApplicationId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdSystemAuthenticatorForUser>();
}
unsafe impl windows_core::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdSystemAuthenticatorForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
unsafe impl Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl Sync for OnlineIdSystemAuthenticatorForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdSystemIdentity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdSystemIdentity, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdSystemIdentity {
    pub fn Ticket(&self) -> windows_core::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ticket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdSystemIdentity>();
}
unsafe impl windows_core::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdSystemIdentity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
unsafe impl Send for OnlineIdSystemIdentity {}
unsafe impl Sync for OnlineIdSystemIdentity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct OnlineIdSystemTicketResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OnlineIdSystemTicketResult, windows_core::IUnknown, windows_core::IInspectable);
impl OnlineIdSystemTicketResult {
    pub fn Identity(&self) -> windows_core::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Identity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOnlineIdSystemTicketResult>();
}
unsafe impl windows_core::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_Vtbl;
    const IID: windows_core::GUID = <IOnlineIdSystemTicketResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
unsafe impl Send for OnlineIdSystemTicketResult {}
unsafe impl Sync for OnlineIdSystemTicketResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SignOutUserOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SignOutUserOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SignOutUserOperation, super::super::super::Foundation::IAsyncAction, super::super::super::Foundation::IAsyncInfo);
impl SignOutUserOperation {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::AsyncActionCompletedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<super::super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SignOutUserOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::IAsyncAction>();
}
unsafe impl windows_core::Interface for SignOutUserOperation {
    type Vtable = super::super::super::Foundation::IAsyncAction_Vtbl;
    const IID: windows_core::GUID = <super::super::super::Foundation::IAsyncAction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
impl SignOutUserOperation {
    pub fn get(&self) -> windows_core::Result<()> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = windows_core::imp::Waiter::new()?;
            self.SetCompleted(&super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl Send for SignOutUserOperation {}
unsafe impl Sync for SignOutUserOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserAuthenticationOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserAuthenticationOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserAuthenticationOperation, super::super::super::Foundation::IAsyncInfo, super::super::super::Foundation::IAsyncOperation::<UserIdentity>);
impl UserAuthenticationOperation {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::IAsyncOperation<UserIdentity>>();
}
unsafe impl windows_core::Interface for UserAuthenticationOperation {
    type Vtable = super::super::super::Foundation::IAsyncOperation_Vtbl<UserIdentity>;
    const IID: windows_core::GUID = <super::super::super::Foundation::IAsyncOperation<UserIdentity> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
impl UserAuthenticationOperation {
    pub fn get(&self) -> windows_core::Result<UserIdentity> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = windows_core::imp::Waiter::new()?;
            self.SetCompleted(&super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl Send for UserAuthenticationOperation {}
unsafe impl Sync for UserAuthenticationOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserIdentity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserIdentity, windows_core::IUnknown, windows_core::IInspectable);
impl UserIdentity {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tickets(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tickets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SafeCustomerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SafeCustomerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignInName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignInName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsBetaAccount(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBetaAccount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsConfirmedPC(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConfirmedPC)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserIdentity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserIdentity>();
}
unsafe impl windows_core::Interface for UserIdentity {
    type Vtable = IUserIdentity_Vtbl;
    const IID: windows_core::GUID = <IUserIdentity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
unsafe impl Send for UserIdentity {}
unsafe impl Sync for UserIdentity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl windows_core::TypeKind for CredentialPromptType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CredentialPromptType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CredentialPromptType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CredentialPromptType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl windows_core::TypeKind for OnlineIdSystemTicketStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OnlineIdSystemTicketStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
}
