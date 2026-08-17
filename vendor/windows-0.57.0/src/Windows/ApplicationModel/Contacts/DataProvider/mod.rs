windows_core::imp::define_interface!(IContactDataProviderConnection, IContactDataProviderConnection_Vtbl, 0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
impl windows_core::RuntimeType for IContactDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactDataProviderConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ServerSearchReadBatchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServerSearchReadBatchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactDataProviderConnection2, IContactDataProviderConnection2_Vtbl, 0xa1d327b0_196c_4bfd_8f0f_c68d67f249d3);
impl windows_core::RuntimeType for IContactDataProviderConnection2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactDataProviderConnection2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateOrUpdateContactRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCreateOrUpdateContactRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DeleteContactRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDeleteContactRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactDataProviderTriggerDetails, IContactDataProviderTriggerDetails_Vtbl, 0x527104be_3c62_43c8_9ae7_db531685cd99);
impl windows_core::RuntimeType for IContactDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactDataProviderTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListCreateOrUpdateContactRequest, IContactListCreateOrUpdateContactRequest_Vtbl, 0xb4af411f_c849_47d0_b119_91cf605b2f2a);
impl windows_core::RuntimeType for IContactListCreateOrUpdateContactRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListCreateOrUpdateContactRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListCreateOrUpdateContactRequestEventArgs, IContactListCreateOrUpdateContactRequestEventArgs_Vtbl, 0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
impl windows_core::RuntimeType for IContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListDeleteContactRequest, IContactListDeleteContactRequest_Vtbl, 0x5e114687_ce03_4de5_8557_9ccf552d472a);
impl windows_core::RuntimeType for IContactListDeleteContactRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListDeleteContactRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContactId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListDeleteContactRequestEventArgs, IContactListDeleteContactRequestEventArgs_Vtbl, 0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
impl windows_core::RuntimeType for IContactListDeleteContactRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListDeleteContactRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListServerSearchReadBatchRequest, IContactListServerSearchReadBatchRequest_Vtbl, 0xba776a97_4030_4925_9fb4_143b295e653b);
impl windows_core::RuntimeType for IContactListServerSearchReadBatchRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListServerSearchReadBatchRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContactListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SaveContactAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::ContactBatchStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListServerSearchReadBatchRequestEventArgs, IContactListServerSearchReadBatchRequestEventArgs_Vtbl, 0x1a27e87b_69d7_4e4e_8042_861cba61471e);
impl windows_core::RuntimeType for IContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListServerSearchReadBatchRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListSyncManagerSyncRequest, IContactListSyncManagerSyncRequest_Vtbl, 0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
impl windows_core::RuntimeType for IContactListSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListSyncManagerSyncRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactListSyncManagerSyncRequestEventArgs, IContactListSyncManagerSyncRequestEventArgs_Vtbl, 0x158e4dac_446d_4f10_afc2_02683ec533a6);
impl windows_core::RuntimeType for IContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactListSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactDataProviderConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactDataProviderConnection, windows_core::IUnknown, windows_core::IInspectable);
impl ContactDataProviderConnection {
    pub fn SyncRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSyncRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ServerSearchReadBatchRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerSearchReadBatchRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServerSearchReadBatchRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveServerSearchReadBatchRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateOrUpdateContactRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOrUpdateContactRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCreateOrUpdateContactRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCreateOrUpdateContactRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeleteContactRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteContactRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeleteContactRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeleteContactRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ContactDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactDataProviderConnection>();
}
unsafe impl windows_core::Interface for ContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_Vtbl;
    const IID: windows_core::GUID = <IContactDataProviderConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
}
unsafe impl Send for ContactDataProviderConnection {}
unsafe impl Sync for ContactDataProviderConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactDataProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactDataProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ContactDataProviderTriggerDetails {
    pub fn Connection(&self) -> windows_core::Result<ContactDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactDataProviderTriggerDetails>();
}
unsafe impl windows_core::Interface for ContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IContactDataProviderTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
}
unsafe impl Send for ContactDataProviderTriggerDetails {}
unsafe impl Sync for ContactDataProviderTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListCreateOrUpdateContactRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListCreateOrUpdateContactRequest, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListCreateOrUpdateContactRequest {
    pub fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactListId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Contact(&self) -> windows_core::Result<super::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, createdorupdatedcontact: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), createdorupdatedcontact.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListCreateOrUpdateContactRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListCreateOrUpdateContactRequest>();
}
unsafe impl windows_core::Interface for ContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_Vtbl;
    const IID: windows_core::GUID = <IContactListCreateOrUpdateContactRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
}
unsafe impl Send for ContactListCreateOrUpdateContactRequest {}
unsafe impl Sync for ContactListCreateOrUpdateContactRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListCreateOrUpdateContactRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListCreateOrUpdateContactRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<ContactListCreateOrUpdateContactRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListCreateOrUpdateContactRequestEventArgs>();
}
unsafe impl windows_core::Interface for ContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IContactListCreateOrUpdateContactRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
}
unsafe impl Send for ContactListCreateOrUpdateContactRequestEventArgs {}
unsafe impl Sync for ContactListCreateOrUpdateContactRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListDeleteContactRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListDeleteContactRequest, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListDeleteContactRequest {
    pub fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactListId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContactId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListDeleteContactRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListDeleteContactRequest>();
}
unsafe impl windows_core::Interface for ContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_Vtbl;
    const IID: windows_core::GUID = <IContactListDeleteContactRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
}
unsafe impl Send for ContactListDeleteContactRequest {}
unsafe impl Sync for ContactListDeleteContactRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListDeleteContactRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListDeleteContactRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListDeleteContactRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<ContactListDeleteContactRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListDeleteContactRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListDeleteContactRequestEventArgs>();
}
unsafe impl windows_core::Interface for ContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IContactListDeleteContactRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
}
unsafe impl Send for ContactListDeleteContactRequestEventArgs {}
unsafe impl Sync for ContactListDeleteContactRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListServerSearchReadBatchRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListServerSearchReadBatchRequest, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListServerSearchReadBatchRequest {
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactListId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Options(&self) -> windows_core::Result<super::ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SuggestedBatchSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SaveContactAsync<P0>(&self, contact: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveContactAsync)(windows_core::Interface::as_raw(this), contact.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), batchstatus, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListServerSearchReadBatchRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListServerSearchReadBatchRequest>();
}
unsafe impl windows_core::Interface for ContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_Vtbl;
    const IID: windows_core::GUID = <IContactListServerSearchReadBatchRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
}
unsafe impl Send for ContactListServerSearchReadBatchRequest {}
unsafe impl Sync for ContactListServerSearchReadBatchRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListServerSearchReadBatchRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListServerSearchReadBatchRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<ContactListServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListServerSearchReadBatchRequestEventArgs>();
}
unsafe impl windows_core::Interface for ContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IContactListServerSearchReadBatchRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
}
unsafe impl Send for ContactListServerSearchReadBatchRequestEventArgs {}
unsafe impl Sync for ContactListServerSearchReadBatchRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListSyncManagerSyncRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListSyncManagerSyncRequest, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListSyncManagerSyncRequest {
    pub fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactListId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListSyncManagerSyncRequest>();
}
unsafe impl windows_core::Interface for ContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_Vtbl;
    const IID: windows_core::GUID = <IContactListSyncManagerSyncRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
}
unsafe impl Send for ContactListSyncManagerSyncRequest {}
unsafe impl Sync for ContactListSyncManagerSyncRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactListSyncManagerSyncRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactListSyncManagerSyncRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<ContactListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactListSyncManagerSyncRequestEventArgs>();
}
unsafe impl windows_core::Interface for ContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IContactListSyncManagerSyncRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
}
unsafe impl Send for ContactListSyncManagerSyncRequestEventArgs {}
unsafe impl Sync for ContactListSyncManagerSyncRequestEventArgs {}
