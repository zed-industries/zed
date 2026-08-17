windows_core::imp::define_interface!(IWebAccountClientView, IWebAccountClientView_Vtbl, 0xe7bd66ba_0bc7_4c66_bfd4_65d3082cbca8);
impl windows_core::RuntimeType for IWebAccountClientView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountClientView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ApplicationCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebAccountClientViewType) -> windows_core::HRESULT,
    pub AccountPairwiseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountClientViewFactory, IWebAccountClientViewFactory_Vtbl, 0x616d16a4_de22_4855_a326_06cebf2a3f23);
impl windows_core::RuntimeType for IWebAccountClientViewFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountClientViewFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, WebAccountClientViewType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithPairwiseId: unsafe extern "system" fn(*mut core::ffi::c_void, WebAccountClientViewType, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountManagerStatics, IWebAccountManagerStatics_Vtbl, 0xb2e8e1a6_d49a_4032_84bf_1a2847747bf1);
impl windows_core::RuntimeType for IWebAccountManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub UpdateWebAccountPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    UpdateWebAccountPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub DeleteWebAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeleteWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub FindAllProviderWebAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    FindAllProviderWebAccountsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub PushCookiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    PushCookiesAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetViewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetViewAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ClearViewAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ClearViewAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub GetViewsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    GetViewsAsync: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub SetWebAccountPictureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "Storage_Streams")))]
    SetWebAccountPictureAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ClearWebAccountPictureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ClearWebAccountPictureAsync: usize,
}
windows_core::imp::define_interface!(IWebAccountManagerStatics2, IWebAccountManagerStatics2_Vtbl, 0x68a7a829_2d5f_4653_8bb0_bd2fa6bd2d87);
impl windows_core::RuntimeType for IWebAccountManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PullCookiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountManagerStatics3, IWebAccountManagerStatics3_Vtbl, 0xdd4523a6_8a4f_4aa2_b15e_03f550af1359);
impl windows_core::RuntimeType for IWebAccountManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub FindAllProviderWebAccountsForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    FindAllProviderWebAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, WebAccountScope, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeAndMapForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, WebAccountScope, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeAndMapForUserAsync: usize,
}
windows_core::imp::define_interface!(IWebAccountManagerStatics4, IWebAccountManagerStatics4_Vtbl, 0x59ebc2d2_f7db_412f_bc3f_f2fea04430b4);
impl windows_core::RuntimeType for IWebAccountManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvalidateAppCacheForAllAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub InvalidateAppCacheForAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    InvalidateAppCacheForAccountAsync: usize,
}
windows_core::imp::define_interface!(IWebAccountMapManagerStatics, IWebAccountMapManagerStatics_Vtbl, 0xe8fa446f_3a1b_48a4_8e90_1e59ca6f54db);
impl windows_core::RuntimeType for IWebAccountMapManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountMapManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAndMapAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, WebAccountScope, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAndMapAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPerAppToPerUserAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPerAppToPerUserAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub GetPerUserFromPerAppAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetPerUserFromPerAppAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ClearPerUserFromPerAppAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ClearPerUserFromPerAppAccountAsync: usize,
}
windows_core::imp::define_interface!(IWebAccountProviderAddAccountOperation, IWebAccountProviderAddAccountOperation_Vtbl, 0x73ebdccf_4378_4c79_9335_a5d7ab81594e);
impl windows_core::RuntimeType for IWebAccountProviderAddAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderAddAccountOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderBaseReportOperation, IWebAccountProviderBaseReportOperation_Vtbl, 0xbba4acbb_993b_4d57_bbe4_1421e3668b4c);
impl core::ops::Deref for IWebAccountProviderBaseReportOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderBaseReportOperation, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAccountProviderBaseReportOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderBaseReportOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderBaseReportOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportError: usize,
}
windows_core::imp::define_interface!(IWebAccountProviderDeleteAccountOperation, IWebAccountProviderDeleteAccountOperation_Vtbl, 0x0abb48b8_9e01_49c9_a355_7d48caf7d6ca);
impl windows_core::RuntimeType for IWebAccountProviderDeleteAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderDeleteAccountOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
}
windows_core::imp::define_interface!(IWebAccountProviderManageAccountOperation, IWebAccountProviderManageAccountOperation_Vtbl, 0xed20dc5c_d21b_463e_a9b7_c1fd0edae978);
impl windows_core::RuntimeType for IWebAccountProviderManageAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderManageAccountOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderOperation, IWebAccountProviderOperation_Vtbl, 0x6d5d2426_10b1_419a_a44e_f9c5161574e6);
impl core::ops::Deref for IWebAccountProviderOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderOperation, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAccountProviderOperation {
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebAccountProviderOperationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderRetrieveCookiesOperation, IWebAccountProviderRetrieveCookiesOperation_Vtbl, 0x5a040441_0fa3_4ab1_a01c_20b110358594);
impl windows_core::RuntimeType for IWebAccountProviderRetrieveCookiesOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderRetrieveCookiesOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub Cookies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    Cookies: usize,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderSignOutAccountOperation, IWebAccountProviderSignOutAccountOperation_Vtbl, 0xb890e21d_0c55_47bc_8c72_04a6fc7cac07);
impl windows_core::RuntimeType for IWebAccountProviderSignOutAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderSignOutAccountOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub ApplicationCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderSilentReportOperation, IWebAccountProviderSilentReportOperation_Vtbl, 0xe0b545f8_3b0f_44da_924c_7b18baaa62a9);
impl core::ops::Deref for IWebAccountProviderSilentReportOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderSilentReportOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAccountProviderSilentReportOperation, IWebAccountProviderBaseReportOperation);
impl IWebAccountProviderSilentReportOperation {
    pub fn ReportUserInteractionRequired(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportUserInteractionRequired)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderSilentReportOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderSilentReportOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportUserInteractionRequired: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportUserInteractionRequiredWithError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportUserInteractionRequiredWithError: usize,
}
windows_core::imp::define_interface!(IWebAccountProviderTokenObjects, IWebAccountProviderTokenObjects_Vtbl, 0x408f284b_1328_42db_89a4_0bce7a717d8e);
impl core::ops::Deref for IWebAccountProviderTokenObjects {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenObjects, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAccountProviderTokenObjects {
    pub fn Operation(&self) -> windows_core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderTokenObjects {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderTokenObjects2, IWebAccountProviderTokenObjects2_Vtbl, 0x1020b893_5ca5_4fff_95fb_b820273fc395);
impl core::ops::Deref for IWebAccountProviderTokenObjects2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenObjects2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAccountProviderTokenObjects2, IWebAccountProviderTokenObjects);
impl IWebAccountProviderTokenObjects2 {
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<IWebAccountProviderOperation> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderTokenObjects>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderTokenObjects2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IWebAccountProviderTokenOperation, IWebAccountProviderTokenOperation_Vtbl, 0x95c613be_2034_4c38_9434_d26c14b2b4b2);
impl core::ops::Deref for IWebAccountProviderTokenOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAccountProviderTokenOperation, IWebAccountProviderOperation);
impl IWebAccountProviderTokenOperation {
    pub fn ProviderRequest(&self) -> windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderResponses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCacheExpirationTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheExpirationTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderTokenOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderTokenOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderResponses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderResponses: usize,
    pub SetCacheExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub CacheExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountProviderUIReportOperation, IWebAccountProviderUIReportOperation_Vtbl, 0x28ff92d3_8f80_42fb_944f_b2107bbd42e6);
impl core::ops::Deref for IWebAccountProviderUIReportOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebAccountProviderUIReportOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebAccountProviderUIReportOperation, IWebAccountProviderBaseReportOperation);
impl IWebAccountProviderUIReportOperation {
    pub fn ReportUserCanceled(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportUserCanceled)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IWebAccountProviderUIReportOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountProviderUIReportOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReportUserCanceled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAccountScopeManagerStatics, IWebAccountScopeManagerStatics_Vtbl, 0x5c6ce37c_12b2_423a_bf3d_85b8d7e53656);
impl windows_core::RuntimeType for IWebAccountScopeManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAccountScopeManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, WebAccountScope, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetScopeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WebAccountScope, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetScopeAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub GetScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut WebAccountScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetScope: usize,
}
windows_core::imp::define_interface!(IWebProviderTokenRequest, IWebProviderTokenRequest_Vtbl, 0x1e18778b_8805_454b_9f11_468d2af1095a);
impl windows_core::RuntimeType for IWebProviderTokenRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderTokenRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientRequest: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub WebAccounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    WebAccounts: usize,
    pub WebAccountSelectionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebAccountSelectionOptions) -> windows_core::HRESULT,
    pub ApplicationCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Core")]
    pub GetApplicationTokenBindingKeyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::TokenBindingKeyType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Core"))]
    GetApplicationTokenBindingKeyAsync: usize,
}
windows_core::imp::define_interface!(IWebProviderTokenRequest2, IWebProviderTokenRequest2_Vtbl, 0xb5d72e4c_10b1_4aa6_88b1_0b6c9e0c1e46);
impl windows_core::RuntimeType for IWebProviderTokenRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderTokenRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetApplicationTokenBindingKeyIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::TokenBindingKeyType, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetApplicationTokenBindingKeyIdAsync: usize,
}
windows_core::imp::define_interface!(IWebProviderTokenRequest3, IWebProviderTokenRequest3_Vtbl, 0x1b2716aa_4289_446e_9256_dafb6f66a51e);
impl windows_core::RuntimeType for IWebProviderTokenRequest3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderTokenRequest3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ApplicationPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ApplicationProcessName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CheckApplicationForCapabilityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebProviderTokenResponse, IWebProviderTokenResponse_Vtbl, 0xef213793_ef55_4186_b7ce_8cb2e7f9849e);
impl windows_core::RuntimeType for IWebProviderTokenResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderTokenResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientResponse: usize,
}
windows_core::imp::define_interface!(IWebProviderTokenResponseFactory, IWebProviderTokenResponseFactory_Vtbl, 0xfa49d99a_25ba_4077_9cfa_9db4dea7b71a);
impl windows_core::RuntimeType for IWebProviderTokenResponseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebProviderTokenResponseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    Create: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountClientView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountClientView, windows_core::IUnknown, windows_core::IInspectable);
impl WebAccountClientView {
    pub fn ApplicationCallbackUri(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<WebAccountClientViewType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccountPairwiseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountPairwiseId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(viewtype: WebAccountClientViewType, applicationcallbackuri: P0) -> windows_core::Result<WebAccountClientView>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), viewtype, applicationcallbackuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithPairwiseId<P0>(viewtype: WebAccountClientViewType, applicationcallbackuri: P0, accountpairwiseid: &windows_core::HSTRING) -> windows_core::Result<WebAccountClientView>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithPairwiseId)(windows_core::Interface::as_raw(this), viewtype, applicationcallbackuri.param().abi(), core::mem::transmute_copy(accountpairwiseid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountClientViewFactory<R, F: FnOnce(&IWebAccountClientViewFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountClientView, IWebAccountClientViewFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebAccountClientView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountClientView>();
}
unsafe impl windows_core::Interface for WebAccountClientView {
    type Vtable = IWebAccountClientView_Vtbl;
    const IID: windows_core::GUID = <IWebAccountClientView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountClientView";
}
unsafe impl Send for WebAccountClientView {}
unsafe impl Sync for WebAccountClientView {}
pub struct WebAccountManager;
impl WebAccountManager {
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn UpdateWebAccountPropertiesAsync<P0, P1>(webaccount: P0, webaccountusername: &windows_core::HSTRING, additionalproperties: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
        P1: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateWebAccountPropertiesAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), core::mem::transmute_copy(webaccountusername), additionalproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountAsync<P0>(webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn DeleteWebAccountAsync<P0>(webaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteWebAccountAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn FindAllProviderWebAccountsAsync() -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllProviderWebAccountsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn PushCookiesAsync<P0, P1>(uri: P0, cookies: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PushCookiesAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), cookies.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetViewAsync<P0, P1>(webaccount: P0, view: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
        P1: windows_core::Param<WebAccountClientView>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetViewAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), view.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ClearViewAsync<P0, P1>(webaccount: P0, applicationcallbackuri: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
        P1: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearViewAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), applicationcallbackuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn GetViewsAsync<P0>(webaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetViewsAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub fn SetWebAccountPictureAsync<P0, P1>(webaccount: P0, webaccountpicture: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
        P1: windows_core::Param<super::super::super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetWebAccountPictureAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), webaccountpicture.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ClearWebAccountPictureAsync<P0>(webaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearWebAccountPictureAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PullCookiesAsync(uristring: &windows_core::HSTRING, callerpfn: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PullCookiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(uristring), core::mem::transmute_copy(callerpfn), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn FindAllProviderWebAccountsForUserAsync<P0>(user: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllProviderWebAccountsForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountForUserAsync<P0, P1>(user: P0, webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeForUserAsync<P0, P1>(user: P0, webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P1, scope: WebAccountScope) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountWithScopeForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), scope, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeAndMapForUserAsync<P0, P1>(user: P0, webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P1, scope: WebAccountScope, peruserwebaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountWithScopeAndMapForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), scope, core::mem::transmute_copy(peruserwebaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InvalidateAppCacheForAllAccountsAsync() -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidateAppCacheForAllAccountsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn InvalidateAppCacheForAccountAsync<P0>(webaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvalidateAppCacheForAccountAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAndMapAsync<P0>(webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P0, scope: WebAccountScope, peruserwebaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountWithScopeAndMapAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), scope, core::mem::transmute_copy(peruserwebaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPerAppToPerUserAccountAsync<P0>(perappaccount: P0, peruserwebaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPerAppToPerUserAccountAsync)(windows_core::Interface::as_raw(this), perappaccount.param().abi(), core::mem::transmute_copy(peruserwebaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn GetPerUserFromPerAppAccountAsync<P0>(perappaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPerUserFromPerAppAccountAsync)(windows_core::Interface::as_raw(this), perappaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ClearPerUserFromPerAppAccountAsync<P0>(perappaccount: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClearPerUserFromPerAppAccountAsync)(windows_core::Interface::as_raw(this), perappaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAsync<P0>(webaccountid: &windows_core::HSTRING, webaccountusername: &windows_core::HSTRING, props: P0, scope: WebAccountScope) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddWebAccountWithScopeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(webaccountid), core::mem::transmute_copy(webaccountusername), props.param().abi(), scope, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetScopeAsync<P0>(webaccount: P0, scope: WebAccountScope) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetScopeAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), scope, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn GetScope<P0>(webaccount: P0) -> windows_core::Result<WebAccountScope>
    where
        P0: windows_core::Param<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetScope)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics<R, F: FnOnce(&IWebAccountManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics2<R, F: FnOnce(&IWebAccountManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics3<R, F: FnOnce(&IWebAccountManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics4<R, F: FnOnce(&IWebAccountManagerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountMapManagerStatics<R, F: FnOnce(&IWebAccountMapManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountMapManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountScopeManagerStatics<R, F: FnOnce(&IWebAccountScopeManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebAccountManager, IWebAccountScopeManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebAccountManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderAddAccountOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderAddAccountOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderAddAccountOperation, IWebAccountProviderOperation);
impl WebAccountProviderAddAccountOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderAddAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderAddAccountOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderAddAccountOperation {
    type Vtable = IWebAccountProviderAddAccountOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderAddAccountOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation";
}
unsafe impl Send for WebAccountProviderAddAccountOperation {}
unsafe impl Sync for WebAccountProviderAddAccountOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderDeleteAccountOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderDeleteAccountOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderDeleteAccountOperation, IWebAccountProviderBaseReportOperation, IWebAccountProviderOperation);
impl WebAccountProviderDeleteAccountOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderDeleteAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderDeleteAccountOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderDeleteAccountOperation {
    type Vtable = IWebAccountProviderDeleteAccountOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderDeleteAccountOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation";
}
unsafe impl Send for WebAccountProviderDeleteAccountOperation {}
unsafe impl Sync for WebAccountProviderDeleteAccountOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderGetTokenSilentOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderGetTokenSilentOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderGetTokenSilentOperation, IWebAccountProviderBaseReportOperation, IWebAccountProviderOperation, IWebAccountProviderSilentReportOperation, IWebAccountProviderTokenOperation);
impl WebAccountProviderGetTokenSilentOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportUserInteractionRequired(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportUserInteractionRequired)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderRequest(&self) -> windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderResponses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCacheExpirationTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheExpirationTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderGetTokenSilentOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderTokenOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderGetTokenSilentOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderTokenOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderGetTokenSilentOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation";
}
unsafe impl Send for WebAccountProviderGetTokenSilentOperation {}
unsafe impl Sync for WebAccountProviderGetTokenSilentOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderManageAccountOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderManageAccountOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderManageAccountOperation, IWebAccountProviderOperation);
impl WebAccountProviderManageAccountOperation {
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderManageAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderManageAccountOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderManageAccountOperation {
    type Vtable = IWebAccountProviderManageAccountOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderManageAccountOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation";
}
unsafe impl Send for WebAccountProviderManageAccountOperation {}
unsafe impl Sync for WebAccountProviderManageAccountOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderRequestTokenOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderRequestTokenOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderRequestTokenOperation, IWebAccountProviderBaseReportOperation, IWebAccountProviderOperation, IWebAccountProviderTokenOperation, IWebAccountProviderUIReportOperation);
impl WebAccountProviderRequestTokenOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProviderRequest(&self) -> windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderResponses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCacheExpirationTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CacheExpirationTime(&self) -> windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CacheExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportUserCanceled(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderUIReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportUserCanceled)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WebAccountProviderRequestTokenOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderTokenOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderRequestTokenOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderTokenOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderRequestTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation";
}
unsafe impl Send for WebAccountProviderRequestTokenOperation {}
unsafe impl Sync for WebAccountProviderRequestTokenOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderRetrieveCookiesOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderRetrieveCookiesOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderRetrieveCookiesOperation, IWebAccountProviderBaseReportOperation, IWebAccountProviderOperation);
impl WebAccountProviderRetrieveCookiesOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Context(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn Cookies(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cookies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, uri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), uri.param().abi()).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplicationCallbackUri(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderRetrieveCookiesOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderRetrieveCookiesOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderRetrieveCookiesOperation {
    type Vtable = IWebAccountProviderRetrieveCookiesOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderRetrieveCookiesOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderRetrieveCookiesOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation";
}
unsafe impl Send for WebAccountProviderRetrieveCookiesOperation {}
unsafe impl Sync for WebAccountProviderRetrieveCookiesOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderSignOutAccountOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderSignOutAccountOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderSignOutAccountOperation, IWebAccountProviderBaseReportOperation, IWebAccountProviderOperation);
impl WebAccountProviderSignOutAccountOperation {
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Core::WebProviderError>,
    {
        let this = &windows_core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReportError)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<WebAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
    pub fn ApplicationCallbackUri(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClientId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderSignOutAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderSignOutAccountOperation>();
}
unsafe impl windows_core::Interface for WebAccountProviderSignOutAccountOperation {
    type Vtable = IWebAccountProviderSignOutAccountOperation_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderSignOutAccountOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation";
}
unsafe impl Send for WebAccountProviderSignOutAccountOperation {}
unsafe impl Sync for WebAccountProviderSignOutAccountOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebAccountProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAccountProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WebAccountProviderTriggerDetails, IWebAccountProviderTokenObjects, IWebAccountProviderTokenObjects2);
impl WebAccountProviderTriggerDetails {
    pub fn Operation(&self) -> windows_core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<IWebAccountProviderTokenObjects2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebAccountProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAccountProviderTokenObjects>();
}
unsafe impl windows_core::Interface for WebAccountProviderTriggerDetails {
    type Vtable = IWebAccountProviderTokenObjects_Vtbl;
    const IID: windows_core::GUID = <IWebAccountProviderTokenObjects as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAccountProviderTriggerDetails {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails";
}
unsafe impl Send for WebAccountProviderTriggerDetails {}
unsafe impl Sync for WebAccountProviderTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebProviderTokenRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebProviderTokenRequest, windows_core::IUnknown, windows_core::IInspectable);
impl WebProviderTokenRequest {
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientRequest(&self) -> windows_core::Result<super::Core::WebTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn WebAccounts(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccounts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WebAccountSelectionOptions(&self) -> windows_core::Result<WebAccountSelectionOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebAccountSelectionOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ApplicationCallbackUri(&self) -> windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationCallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Core")]
    pub fn GetApplicationTokenBindingKeyAsync<P0>(&self, keytype: super::TokenBindingKeyType, target: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicationTokenBindingKeyAsync)(windows_core::Interface::as_raw(this), keytype, target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetApplicationTokenBindingKeyIdAsync<P0>(&self, keytype: super::TokenBindingKeyType, target: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IWebProviderTokenRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicationTokenBindingKeyIdAsync)(windows_core::Interface::as_raw(this), keytype, target.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplicationPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApplicationProcessName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplicationProcessName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckApplicationForCapabilityAsync(&self, capabilityname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckApplicationForCapabilityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(capabilityname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebProviderTokenRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebProviderTokenRequest>();
}
unsafe impl windows_core::Interface for WebProviderTokenRequest {
    type Vtable = IWebProviderTokenRequest_Vtbl;
    const IID: windows_core::GUID = <IWebProviderTokenRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebProviderTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest";
}
unsafe impl Send for WebProviderTokenRequest {}
unsafe impl Sync for WebProviderTokenRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WebProviderTokenResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebProviderTokenResponse, windows_core::IUnknown, windows_core::IInspectable);
impl WebProviderTokenResponse {
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientResponse(&self) -> windows_core::Result<super::Core::WebTokenResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientResponse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn Create<P0>(webtokenresponse: P0) -> windows_core::Result<WebProviderTokenResponse>
    where
        P0: windows_core::Param<super::Core::WebTokenResponse>,
    {
        Self::IWebProviderTokenResponseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), webtokenresponse.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderTokenResponseFactory<R, F: FnOnce(&IWebProviderTokenResponseFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebProviderTokenResponse, IWebProviderTokenResponseFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WebProviderTokenResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebProviderTokenResponse>();
}
unsafe impl windows_core::Interface for WebProviderTokenResponse {
    type Vtable = IWebProviderTokenResponse_Vtbl;
    const IID: windows_core::GUID = <IWebProviderTokenResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse";
}
unsafe impl Send for WebProviderTokenResponse {}
unsafe impl Sync for WebProviderTokenResponse {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebAccountClientViewType(pub i32);
impl WebAccountClientViewType {
    pub const IdOnly: Self = Self(0i32);
    pub const IdAndProperties: Self = Self(1i32);
}
impl windows_core::TypeKind for WebAccountClientViewType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebAccountClientViewType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebAccountClientViewType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebAccountClientViewType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountClientViewType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebAccountProviderOperationKind(pub i32);
impl WebAccountProviderOperationKind {
    pub const RequestToken: Self = Self(0i32);
    pub const GetTokenSilently: Self = Self(1i32);
    pub const AddAccount: Self = Self(2i32);
    pub const ManageAccount: Self = Self(3i32);
    pub const DeleteAccount: Self = Self(4i32);
    pub const RetrieveCookies: Self = Self(5i32);
    pub const SignOutAccount: Self = Self(6i32);
}
impl windows_core::TypeKind for WebAccountProviderOperationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebAccountProviderOperationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebAccountProviderOperationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebAccountProviderOperationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountProviderOperationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebAccountScope(pub i32);
impl WebAccountScope {
    pub const PerUser: Self = Self(0i32);
    pub const PerApplication: Self = Self(1i32);
}
impl windows_core::TypeKind for WebAccountScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebAccountScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebAccountScope").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WebAccountScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountScope;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WebAccountSelectionOptions(pub u32);
impl WebAccountSelectionOptions {
    pub const Default: Self = Self(0u32);
    pub const New: Self = Self(1u32);
}
impl windows_core::TypeKind for WebAccountSelectionOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WebAccountSelectionOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WebAccountSelectionOptions").field(&self.0).finish()
    }
}
impl WebAccountSelectionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WebAccountSelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WebAccountSelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WebAccountSelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WebAccountSelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WebAccountSelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for WebAccountSelectionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountSelectionOptions;u4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
