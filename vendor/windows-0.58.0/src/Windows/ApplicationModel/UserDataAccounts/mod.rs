#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
windows_core::imp::define_interface!(IUserDataAccount, IUserDataAccount_Vtbl, 0xb9c4367e_b348_4910_be94_4ad4bba6dea7);
impl windows_core::RuntimeType for IUserDataAccount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccount_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetUserDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataAccountOtherAppReadAccess) -> windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataAccountOtherAppReadAccess) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    pub DeviceAccountTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections")))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub FindEmailMailboxesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections")))]
    FindEmailMailboxesAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactListsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactAnnotationListsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactAnnotationListsAsync: usize,
}
windows_core::imp::define_interface!(IUserDataAccount2, IUserDataAccount2_Vtbl, 0x078cd89f_de82_404b_8195_c8a3ac198f60);
impl windows_core::RuntimeType for IUserDataAccount2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccount2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsProtectedUnderLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccount3, IUserDataAccount3_Vtbl, 0x01533845_6c43_4286_9d69_3e1709a1f266);
impl windows_core::RuntimeType for IUserDataAccount3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccount3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExplictReadAccessPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExplictReadAccessPackageFamilyNames: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccount4, IUserDataAccount4_Vtbl, 0xc4315210_eae5_4f0a_a8b2_1cca115e008f);
impl windows_core::RuntimeType for IUserDataAccount4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccount4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanShowCreateContactGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanShowCreateContactGroup: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub FindUserDataTaskListsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections")))]
    FindUserDataTaskListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactGroupsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactGroupsAsync: usize,
    pub TryShowCreateContactGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsProtectedUnderLock: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetIcon: usize,
}
windows_core::imp::define_interface!(IUserDataAccountManagerForUser, IUserDataAccountManagerForUser_Vtbl, 0x56a6e8db_db8f_41ab_a65f_8c5971aac982);
impl windows_core::RuntimeType for IUserDataAccountManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataAccountStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IUserDataAccountManagerStatics, IUserDataAccountManagerStatics_Vtbl, 0x0d9b89ea_1928_4a20_86d5_3c737f7dc3b0);
impl windows_core::RuntimeType for IUserDataAccountManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataAccountStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAddAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataAccountContentKinds, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAccountSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAccountErrorResolverAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountManagerStatics2, IUserDataAccountManagerStatics2_Vtbl, 0x6a3ded88_316b_435e_b534_f7d4b4b7dba6);
impl windows_core::RuntimeType for IUserDataAccountManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IUserDataAccountStore, IUserDataAccountStore_Vtbl, 0x2073b0ad_7d0a_4e76_bf45_2368f978a59a);
impl windows_core::RuntimeType for IUserDataAccountStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAccountsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAccountsAsync: usize,
    pub GetAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountStore2, IUserDataAccountStore2_Vtbl, 0xb1e0aef7_9560_4631_8af0_061d30161469);
impl windows_core::RuntimeType for IUserDataAccountStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAccountWithPackageRelativeAppIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountStore3, IUserDataAccountStore3_Vtbl, 0x8142c094_f3c9_478b_b117_6585bebb6789);
impl windows_core::RuntimeType for IUserDataAccountStore3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountStore3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataAccountStoreChangedEventArgs, IUserDataAccountStoreChangedEventArgs_Vtbl, 0x84e3e2e5_8820_4512_b1f6_2e035be1072c);
impl windows_core::RuntimeType for IUserDataAccountStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAccountStoreChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccount(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccount, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAccount {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUserDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OtherAppReadAccess(&self) -> windows_core::Result<UserDataAccountOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppReadAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: UserDataAccountOtherAppReadAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppReadAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeviceAccountTypeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccountTypeId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub fn FindAppointmentCalendarsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentCalendarsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub fn FindEmailMailboxesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindEmailMailboxesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactListsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindContactListsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactAnnotationListsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindContactAnnotationListsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsProtectedUnderLock(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsProtectedUnderLock)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExplictReadAccessPackageFamilyNames(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExplictReadAccessPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CanShowCreateContactGroup(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanShowCreateContactGroup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanShowCreateContactGroup(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanShowCreateContactGroup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub fn FindUserDataTaskListsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindUserDataTaskListsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactGroupsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindContactGroupsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryShowCreateContactGroupAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowCreateContactGroupAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsProtectedUnderLock(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsProtectedUnderLock)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &windows_core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UserDataAccount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccount>();
}
unsafe impl windows_core::Interface for UserDataAccount {
    type Vtable = IUserDataAccount_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccount as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccount {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
}
unsafe impl Send for UserDataAccount {}
unsafe impl Sync for UserDataAccount {}
pub struct UserDataAccountManager;
impl UserDataAccountManager {
    pub fn RequestStoreAsync(storeaccesstype: UserDataAccountStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), storeaccesstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAddAccountAsync(contentkinds: UserDataAccountContentKinds) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAccountAsync)(windows_core::Interface::as_raw(this), contentkinds, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAccountSettingsAsync(id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAccountSettingsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAccountErrorResolverAsync(id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAccountErrorResolverAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<UserDataAccountManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IUserDataAccountManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserDataAccountManagerStatics<R, F: FnOnce(&IUserDataAccountManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserDataAccountManagerStatics2<R, F: FnOnce(&IUserDataAccountManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for UserDataAccountManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAccountManagerForUser {
    pub fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), storeaccesstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountManagerForUser>();
}
unsafe impl windows_core::Interface for UserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
}
unsafe impl Send for UserDataAccountManagerForUser {}
unsafe impl Sync for UserDataAccountManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountStore, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAccountStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAccountsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAccountsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAccountAsync(&self, id: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAccountAsync(&self, userdisplayname: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdisplayname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAccountWithPackageRelativeAppIdAsync(&self, userdisplayname: &windows_core::HSTRING, packagerelativeappid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdisplayname), core::mem::transmute_copy(packagerelativeappid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StoreChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UserDataAccountStore, UserDataAccountStoreChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveStoreChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync(&self, userdisplayname: &windows_core::HSTRING, packagerelativeappid: &windows_core::HSTRING, enterpriseid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &windows_core::Interface::cast::<IUserDataAccountStore3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdisplayname), core::mem::transmute_copy(packagerelativeappid), core::mem::transmute_copy(enterpriseid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountStore>();
}
unsafe impl windows_core::Interface for UserDataAccountStore {
    type Vtable = IUserDataAccountStore_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
}
unsafe impl Send for UserDataAccountStore {}
unsafe impl Sync for UserDataAccountStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataAccountStoreChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAccountStoreChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAccountStoreChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataAccountStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAccountStoreChangedEventArgs>();
}
unsafe impl windows_core::Interface for UserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUserDataAccountStoreChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAccountStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
}
unsafe impl Send for UserDataAccountStoreChangedEventArgs {}
unsafe impl Sync for UserDataAccountStoreChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAccountContentKinds(pub u32);
impl UserDataAccountContentKinds {
    pub const Email: Self = Self(1u32);
    pub const Contact: Self = Self(2u32);
    pub const Appointment: Self = Self(4u32);
}
impl windows_core::TypeKind for UserDataAccountContentKinds {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAccountContentKinds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAccountContentKinds").field(&self.0).finish()
    }
}
impl UserDataAccountContentKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UserDataAccountContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UserDataAccountContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UserDataAccountContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UserDataAccountContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UserDataAccountContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for UserDataAccountContentKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAccountOtherAppReadAccess(pub i32);
impl UserDataAccountOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataAccountOtherAppReadAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAccountOtherAppReadAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAccountOtherAppReadAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataAccountOtherAppReadAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAccountStoreAccessType(pub i32);
impl UserDataAccountStoreAccessType {
    pub const AllAccountsReadOnly: Self = Self(0i32);
    pub const AppAccountsReadWrite: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataAccountStoreAccessType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAccountStoreAccessType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreAccessType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataAccountStoreAccessType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType;i4)");
}
