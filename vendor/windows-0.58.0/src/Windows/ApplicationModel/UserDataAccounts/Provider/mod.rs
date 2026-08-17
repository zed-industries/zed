windows_core::imp::define_interface!(IUserDataAccountPartnerAccountInfo, IUserDataAccountPartnerAccountInfo_Vtbl, 0x5f200037_f6ef_4ec3_8630_012c59c1149f);
impl windows_core::RuntimeType for IUserDataAccountPartnerAccountInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountPartnerAccountInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AccountKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataAccountProviderPartnerAccountKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountProviderAddAccountOperation, IUserDataAccountProviderAddAccountOperation_Vtbl, 0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
impl windows_core::RuntimeType for IUserDataAccountProviderAddAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountProviderAddAccountOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentKinds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UserDataAccountContentKinds) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PartnerAccountInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PartnerAccountInfos: usize,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountProviderOperation, IUserDataAccountProviderOperation_Vtbl, 0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
impl core::ops::Deref for IUserDataAccountProviderOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserDataAccountProviderOperation, windows_core::IUnknown, windows_core::IInspectable);
impl IUserDataAccountProviderOperation {
    pub fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountProviderOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataAccountProviderOperationKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountProviderResolveErrorsOperation, IUserDataAccountProviderResolveErrorsOperation_Vtbl, 0x6235dc15_bfcb_41e1_9957_9759a28846cc);
impl windows_core::RuntimeType for IUserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountProviderResolveErrorsOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserDataAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountProviderSettingsOperation, IUserDataAccountProviderSettingsOperation_Vtbl, 0x92034db7_8648_4f30_acfa_3002658ca80d);
impl windows_core::RuntimeType for IUserDataAccountProviderSettingsOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountProviderSettingsOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserDataAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountPartnerAccountInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountPartnerAccountInfo, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAccountPartnerAccountInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Priority(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AccountKind(&self) -> windows_core::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountPartnerAccountInfo>();
}
unsafe impl windows_core::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountPartnerAccountInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
unsafe impl Send for UserDataAccountPartnerAccountInfo {}
unsafe impl Sync for UserDataAccountPartnerAccountInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountProviderAddAccountOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountProviderAddAccountOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserDataAccountProviderAddAccountOperation, IUserDataAccountProviderOperation);
impl UserDataAccountProviderAddAccountOperation {
    pub fn ContentKinds(&self) -> windows_core::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentKinds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartnerAccountInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompleted(&self, userdataaccountid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdataaccountid)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountProviderAddAccountOperation>();
}
unsafe impl windows_core::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountProviderAddAccountOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
unsafe impl Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl Sync for UserDataAccountProviderAddAccountOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountProviderResolveErrorsOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountProviderResolveErrorsOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserDataAccountProviderResolveErrorsOperation, IUserDataAccountProviderOperation);
impl UserDataAccountProviderResolveErrorsOperation {
    pub fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserDataAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDataAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountProviderResolveErrorsOperation>();
}
unsafe impl windows_core::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountProviderResolveErrorsOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
unsafe impl Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl Sync for UserDataAccountProviderResolveErrorsOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountProviderSettingsOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountProviderSettingsOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserDataAccountProviderSettingsOperation, IUserDataAccountProviderOperation);
impl UserDataAccountProviderSettingsOperation {
    pub fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserDataAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDataAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompleted(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountProviderSettingsOperation>();
}
unsafe impl windows_core::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountProviderSettingsOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
unsafe impl Send for UserDataAccountProviderSettingsOperation {}
unsafe impl Sync for UserDataAccountProviderSettingsOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: Self = Self(0i32);
    pub const Settings: Self = Self(1i32);
    pub const ResolveErrors: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataAccountProviderOperationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAccountProviderOperationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderOperationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: Self = Self(0i32);
    pub const PopOrImap: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataAccountProviderPartnerAccountKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAccountProviderPartnerAccountKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderPartnerAccountKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
