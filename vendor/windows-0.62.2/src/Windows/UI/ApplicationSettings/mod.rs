#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsSettingsPane(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccountsSettingsPane, windows_core::IUnknown, windows_core::IInspectable);
impl AccountsSettingsPane {
    pub fn AccountCommandsRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountCommandsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAccountCommandsRequested(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAccountCommandsRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<AccountsSettingsPane> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Show() -> windows_core::Result<()> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ShowManageAccountsAsync() -> windows_core::Result<windows_future::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowManageAccountsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAddAccountAsync() -> windows_core::Result<windows_future::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAccountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn ShowManageAccountsForUserAsync<P0>(user: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowManageAccountsForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn ShowAddAccountForUserAsync<P0>(user: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAccountForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAccountsSettingsPaneStatics<R, F: FnOnce(&IAccountsSettingsPaneStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAccountsSettingsPaneStatics2<R, F: FnOnce(&IAccountsSettingsPaneStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAccountsSettingsPaneStatics3<R, F: FnOnce(&IAccountsSettingsPaneStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AccountsSettingsPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccountsSettingsPane>();
}
unsafe impl windows_core::Interface for AccountsSettingsPane {
    type Vtable = <IAccountsSettingsPane as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccountsSettingsPane as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPane";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccountsSettingsPaneCommandsRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccountsSettingsPaneCommandsRequestedEventArgs {
    pub fn WebAccountProviderCommands(&self) -> windows_core::Result<windows_collections::IVector<WebAccountProviderCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccountProviderCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WebAccountCommands(&self) -> windows_core::Result<windows_collections::IVector<WebAccountCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccountCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CredentialCommands(&self) -> windows_core::Result<windows_collections::IVector<CredentialCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CredentialCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn Commands(&self) -> windows_core::Result<windows_collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Commands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HeaderText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetHeaderText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeaderText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<AccountsSettingsPaneEventDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IAccountsSettingsPaneCommandsRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AccountsSettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccountsSettingsPaneCommandsRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = <IAccountsSettingsPaneCommandsRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccountsSettingsPaneCommandsRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsSettingsPaneEventDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccountsSettingsPaneEventDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl AccountsSettingsPaneEventDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AccountsSettingsPaneEventDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccountsSettingsPaneEventDeferral>();
}
unsafe impl windows_core::Interface for AccountsSettingsPaneEventDeferral {
    type Vtable = <IAccountsSettingsPaneEventDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccountsSettingsPaneEventDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialCommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CredentialCommand, windows_core::IUnknown, windows_core::IInspectable);
impl CredentialCommand {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasswordCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CredentialDeleted(&self) -> windows_core::Result<CredentialCommandCredentialDeletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CredentialDeleted)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommand<P0>(passwordcredential: P0) -> windows_core::Result<CredentialCommand>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCredentialCommand)(windows_core::Interface::as_raw(this), passwordcredential.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommandWithHandler<P0, P1>(passwordcredential: P0, deleted: P1) -> windows_core::Result<CredentialCommand>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
        P1: windows_core::Param<CredentialCommandCredentialDeletedHandler>,
    {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCredentialCommandWithHandler)(windows_core::Interface::as_raw(this), passwordcredential.param().abi(), deleted.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICredentialCommandFactory<R, F: FnOnce(&ICredentialCommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CredentialCommand, ICredentialCommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CredentialCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICredentialCommand>();
}
unsafe impl windows_core::Interface for CredentialCommand {
    type Vtable = <ICredentialCommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICredentialCommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.CredentialCommand";
}
windows_core::imp::define_interface!(CredentialCommandCredentialDeletedHandler, CredentialCommandCredentialDeletedHandler_Vtbl, 0x61c0e185_0977_4678_b4e2_98727afbeed9);
impl windows_core::RuntimeType for CredentialCommandCredentialDeletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl CredentialCommandCredentialDeletedHandler {
    pub fn new<F: Fn(windows_core::Ref<CredentialCommand>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = CredentialCommandCredentialDeletedHandlerBox { vtable: &CredentialCommandCredentialDeletedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, command: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CredentialCommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), command.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CredentialCommandCredentialDeletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct CredentialCommandCredentialDeletedHandlerBox<F: Fn(windows_core::Ref<CredentialCommand>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const CredentialCommandCredentialDeletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<CredentialCommand>) -> windows_core::Result<()> + Send + 'static> CredentialCommandCredentialDeletedHandlerBox<F> {
    const VTABLE: CredentialCommandCredentialDeletedHandler_Vtbl = CredentialCommandCredentialDeletedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <CredentialCommandCredentialDeletedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&command)).into()
        }
    }
}
windows_core::imp::define_interface!(IAccountsSettingsPane, IAccountsSettingsPane_Vtbl, 0x81ea942c_4f09_4406_a538_838d9b14b7e6);
impl windows_core::RuntimeType for IAccountsSettingsPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPane_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountCommandsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAccountCommandsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneCommandsRequestedEventArgs, IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl, 0x3b68c099_db19_45d0_9abf_95d3773c9330);
impl windows_core::RuntimeType for IAccountsSettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WebAccountProviderCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WebAccountCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CredentialCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub Commands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    Commands: usize,
    pub HeaderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHeaderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneCommandsRequestedEventArgs2, IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl, 0x362f7bad_4e37_4967_8c40_e78ee7a1e5bb);
impl windows_core::RuntimeType for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneEventDeferral, IAccountsSettingsPaneEventDeferral_Vtbl, 0xcbf25d3f_e5ba_40ef_93da_65e096e5fb04);
impl windows_core::RuntimeType for IAccountsSettingsPaneEventDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneEventDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneStatics, IAccountsSettingsPaneStatics_Vtbl, 0x561f8b60_b0ec_4150_a8dc_208ee44b068a);
impl windows_core::RuntimeType for IAccountsSettingsPaneStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneStatics2, IAccountsSettingsPaneStatics2_Vtbl, 0xd21df7c2_ce0d_484f_b8e8_e823c215765e);
impl windows_core::RuntimeType for IAccountsSettingsPaneStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowManageAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAddAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccountsSettingsPaneStatics3, IAccountsSettingsPaneStatics3_Vtbl, 0x08410458_a2ba_4c6f_b4ac_48f514331216);
impl windows_core::RuntimeType for IAccountsSettingsPaneStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub ShowManageAccountsForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShowManageAccountsForUserAsync: usize,
    #[cfg(feature = "System")]
    pub ShowAddAccountForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShowAddAccountForUserAsync: usize,
}
windows_core::imp::define_interface!(ICredentialCommand, ICredentialCommand_Vtbl, 0xa5f665e6_6143_4a7a_a971_b017ba978ce2);
impl windows_core::RuntimeType for ICredentialCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasswordCredential: usize,
    pub CredentialDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICredentialCommandFactory, ICredentialCommandFactory_Vtbl, 0x27e88c17_bc3e_4b80_9495_4ed720e48a91);
impl windows_core::RuntimeType for ICredentialCommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommand: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommandWithHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommandWithHandler: usize,
}
windows_core::imp::define_interface!(ISettingsCommandFactory, ISettingsCommandFactory_Vtbl, 0x68e15b33_1c83_433a_aa5a_ceeea5bd4764);
impl windows_core::RuntimeType for ISettingsCommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Popups")]
    pub CreateSettingsCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    CreateSettingsCommand: usize,
}
windows_core::imp::define_interface!(ISettingsCommandStatics, ISettingsCommandStatics_Vtbl, 0x749ae954_2f69_4b17_8aba_d05ce5778e46);
impl windows_core::RuntimeType for ISettingsCommandStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Popups")]
    pub AccountsCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    AccountsCommand: usize,
}
windows_core::imp::define_interface!(ISettingsPane, ISettingsPane_Vtbl, 0xb1cd0932_4570_4c69_8d38_89446561ace0);
impl windows_core::RuntimeType for ISettingsPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPane_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CommandsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCommandsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsPaneCommandsRequest, ISettingsPaneCommandsRequest_Vtbl, 0x44df23ae_5d6e_4068_a168_f47643182114);
impl windows_core::RuntimeType for ISettingsPaneCommandsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Popups")]
    pub ApplicationCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ApplicationCommands: usize,
}
windows_core::imp::define_interface!(ISettingsPaneCommandsRequestedEventArgs, ISettingsPaneCommandsRequestedEventArgs_Vtbl, 0x205f5d24_1b48_4629_a6ca_2fdfedafb75d);
impl windows_core::RuntimeType for ISettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISettingsPaneStatics, ISettingsPaneStatics_Vtbl, 0x1c6a52c5_ff19_471b_ba6b_f8f35694ad9a);
impl windows_core::RuntimeType for ISettingsPaneStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Edge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SettingsEdgeLocation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountCommand, IWebAccountCommand_Vtbl, 0xcaa39398_9cfa_4246_b0c4_a913a3896541);
impl windows_core::RuntimeType for IWebAccountCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub Invoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SupportedWebAccountActions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountCommandFactory, IWebAccountCommandFactory_Vtbl, 0xbfa6cdff_2f2d_42f5_81de_1d56bafc496d);
impl windows_core::RuntimeType for IWebAccountCommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, SupportedWebAccountActions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountCommand: usize,
}
windows_core::imp::define_interface!(IWebAccountInvokedArgs, IWebAccountInvokedArgs_Vtbl, 0xe7abcc40_a1d8_4c5d_9a7f_1d34b2f90ad2);
impl windows_core::RuntimeType for IWebAccountInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountInvokedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebAccountAction) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderCommand, IWebAccountProviderCommand_Vtbl, 0xd69bdd9a_a0a6_4e9b_88dc_c71e757a3501);
impl windows_core::RuntimeType for IWebAccountProviderCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Invoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderCommandFactory, IWebAccountProviderCommandFactory_Vtbl, 0xd5658a1b_b176_4776_8469_a9d3ff0b3f59);
impl windows_core::RuntimeType for IWebAccountProviderCommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountProviderCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountProviderCommand: usize,
}
#[cfg(feature = "UI_Popups")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsCommand(windows_core::IUnknown);
#[cfg(feature = "UI_Popups")]
windows_core::imp::interface_hierarchy!(SettingsCommand, windows_core::IUnknown, windows_core::IInspectable, super::Popups::IUICommand);
#[cfg(feature = "UI_Popups")]
impl SettingsCommand {
    pub fn CreateSettingsCommand<P0, P2>(settingscommandid: P0, label: &windows_core::HSTRING, handler: P2) -> windows_core::Result<SettingsCommand>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<super::Popups::UICommandInvokedHandler>,
    {
        Self::ISettingsCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSettingsCommand)(windows_core::Interface::as_raw(this), settingscommandid.param().abi(), core::mem::transmute_copy(label), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AccountsCommand() -> windows_core::Result<SettingsCommand> {
        Self::ISettingsCommandStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountsCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Invoked(&self) -> windows_core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoked)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInvoked<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Popups::UICommandInvokedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInvoked)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn ISettingsCommandFactory<R, F: FnOnce(&ISettingsCommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SettingsCommand, ISettingsCommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISettingsCommandStatics<R, F: FnOnce(&ISettingsCommandStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SettingsCommand, ISettingsCommandStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Popups")]
impl windows_core::RuntimeType for SettingsCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::Popups::IUICommand>();
}
#[cfg(feature = "UI_Popups")]
unsafe impl windows_core::Interface for SettingsCommand {
    type Vtable = <super::Popups::IUICommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <super::Popups::IUICommand as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Popups")]
impl windows_core::RuntimeName for SettingsCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsCommand";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SettingsEdgeLocation(pub i32);
impl SettingsEdgeLocation {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl windows_core::TypeKind for SettingsEdgeLocation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SettingsEdgeLocation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SettingsEdgeLocation;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsPane(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SettingsPane, windows_core::IUnknown, windows_core::IInspectable);
impl SettingsPane {
    pub fn CommandsRequested<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCommandsRequested(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCommandsRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<SettingsPane> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Show() -> windows_core::Result<()> {
        Self::ISettingsPaneStatics(|this| unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn Edge() -> windows_core::Result<SettingsEdgeLocation> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Edge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn ISettingsPaneStatics<R, F: FnOnce(&ISettingsPaneStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SettingsPane, ISettingsPaneStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SettingsPane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISettingsPane>();
}
unsafe impl windows_core::Interface for SettingsPane {
    type Vtable = <ISettingsPane as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISettingsPane as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPane";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsPaneCommandsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SettingsPaneCommandsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl SettingsPaneCommandsRequest {
    #[cfg(feature = "UI_Popups")]
    pub fn ApplicationCommands(&self) -> windows_core::Result<windows_collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SettingsPaneCommandsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISettingsPaneCommandsRequest>();
}
unsafe impl windows_core::Interface for SettingsPaneCommandsRequest {
    type Vtable = <ISettingsPaneCommandsRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISettingsPaneCommandsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsPaneCommandsRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SettingsPaneCommandsRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SettingsPaneCommandsRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<SettingsPaneCommandsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISettingsPaneCommandsRequestedEventArgs>();
}
unsafe impl windows_core::Interface for SettingsPaneCommandsRequestedEventArgs {
    type Vtable = <ISettingsPaneCommandsRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISettingsPaneCommandsRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SupportedWebAccountActions(pub u32);
impl SupportedWebAccountActions {
    pub const None: Self = Self(0u32);
    pub const Reconnect: Self = Self(1u32);
    pub const Remove: Self = Self(2u32);
    pub const ViewDetails: Self = Self(4u32);
    pub const Manage: Self = Self(8u32);
    pub const More: Self = Self(16u32);
}
impl windows_core::TypeKind for SupportedWebAccountActions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SupportedWebAccountActions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SupportedWebAccountActions;u4)");
}
impl SupportedWebAccountActions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SupportedWebAccountActions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SupportedWebAccountActions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SupportedWebAccountActions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SupportedWebAccountActions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SupportedWebAccountActions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WebAccountAction(pub i32);
impl WebAccountAction {
    pub const Reconnect: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const ViewDetails: Self = Self(2i32);
    pub const Manage: Self = Self(3i32);
    pub const More: Self = Self(4i32);
}
impl windows_core::TypeKind for WebAccountAction {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebAccountAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.WebAccountAction;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebAccountCommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountCommand, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountCommand {
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> windows_core::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Invoked(&self) -> windows_core::Result<WebAccountCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoked)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Actions(&self) -> windows_core::Result<SupportedWebAccountActions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Actions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountCommand<P0, P1>(webaccount: P0, invoked: P1, actions: SupportedWebAccountActions) -> windows_core::Result<WebAccountCommand>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccount>,
        P1: windows_core::Param<WebAccountCommandInvokedHandler>,
    {
        Self::IWebAccountCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWebAccountCommand)(windows_core::Interface::as_raw(this), webaccount.param().abi(), invoked.param().abi(), actions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWebAccountCommandFactory<R, F: FnOnce(&IWebAccountCommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountCommand, IWebAccountCommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebAccountCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountCommand>();
}
unsafe impl windows_core::Interface for WebAccountCommand {
    type Vtable = <IWebAccountCommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAccountCommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountCommand";
}
windows_core::imp::define_interface!(WebAccountCommandInvokedHandler, WebAccountCommandInvokedHandler_Vtbl, 0x1ee6e459_1705_4a9a_b599_a0c3d6921973);
impl windows_core::RuntimeType for WebAccountCommandInvokedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl WebAccountCommandInvokedHandler {
    pub fn new<F: Fn(windows_core::Ref<WebAccountCommand>, windows_core::Ref<WebAccountInvokedArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = WebAccountCommandInvokedHandlerBox { vtable: &WebAccountCommandInvokedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, command: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WebAccountCommand>,
        P1: windows_core::Param<WebAccountInvokedArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), command.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountCommandInvokedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct WebAccountCommandInvokedHandlerBox<F: Fn(windows_core::Ref<WebAccountCommand>, windows_core::Ref<WebAccountInvokedArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const WebAccountCommandInvokedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<WebAccountCommand>, windows_core::Ref<WebAccountInvokedArgs>) -> windows_core::Result<()> + Send + 'static> WebAccountCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountCommandInvokedHandler_Vtbl = WebAccountCommandInvokedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <WebAccountCommandInvokedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&command), core::mem::transmute_copy(&args)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebAccountInvokedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountInvokedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountInvokedArgs {
    pub fn Action(&self) -> windows_core::Result<WebAccountAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAccountInvokedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountInvokedArgs>();
}
unsafe impl windows_core::Interface for WebAccountInvokedArgs {
    type Vtable = <IWebAccountInvokedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAccountInvokedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebAccountProviderCommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderCommand, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountProviderCommand {
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> windows_core::Result<super::super::Security::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccountProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Invoked(&self) -> windows_core::Result<WebAccountProviderCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoked)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountProviderCommand<P0, P1>(webaccountprovider: P0, invoked: P1) -> windows_core::Result<WebAccountProviderCommand>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccountProvider>,
        P1: windows_core::Param<WebAccountProviderCommandInvokedHandler>,
    {
        Self::IWebAccountProviderCommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWebAccountProviderCommand)(windows_core::Interface::as_raw(this), webaccountprovider.param().abi(), invoked.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWebAccountProviderCommandFactory<R, F: FnOnce(&IWebAccountProviderCommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountProviderCommand, IWebAccountProviderCommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebAccountProviderCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderCommand>();
}
unsafe impl windows_core::Interface for WebAccountProviderCommand {
    type Vtable = <IWebAccountProviderCommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAccountProviderCommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountProviderCommand";
}
windows_core::imp::define_interface!(WebAccountProviderCommandInvokedHandler, WebAccountProviderCommandInvokedHandler_Vtbl, 0xb7de5527_4c8f_42dd_84da_5ec493abdb9a);
impl windows_core::RuntimeType for WebAccountProviderCommandInvokedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl WebAccountProviderCommandInvokedHandler {
    pub fn new<F: Fn(windows_core::Ref<WebAccountProviderCommand>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = WebAccountProviderCommandInvokedHandlerBox { vtable: &WebAccountProviderCommandInvokedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, command: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WebAccountProviderCommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), command.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountProviderCommandInvokedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct WebAccountProviderCommandInvokedHandlerBox<F: Fn(windows_core::Ref<WebAccountProviderCommand>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const WebAccountProviderCommandInvokedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<WebAccountProviderCommand>) -> windows_core::Result<()> + Send + 'static> WebAccountProviderCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountProviderCommandInvokedHandler_Vtbl = WebAccountProviderCommandInvokedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <WebAccountProviderCommandInvokedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, command: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&command)).into()
        }
    }
}
