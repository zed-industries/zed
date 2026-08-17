windows_core::imp::define_interface!(IContactPickerUI, IContactPickerUI_Vtbl, 0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
impl windows_core::RuntimeType for IContactPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactPickerUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AddContact: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut AddContactResult) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddContact: usize,
    pub RemoveContact: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub DesiredFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    DesiredFields: usize,
    pub SelectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ContactSelectionMode) -> windows_core::HRESULT,
    pub ContactRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveContactRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactPickerUI2, IContactPickerUI2_Vtbl, 0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
impl windows_core::RuntimeType for IContactPickerUI2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactPickerUI2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut AddContactResult) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
}
windows_core::imp::define_interface!(IContactRemovedEventArgs, IContactRemovedEventArgs_Vtbl, 0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
impl windows_core::RuntimeType for IContactRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContactPickerUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactPickerUI, windows_core::IUnknown, windows_core::IInspectable);
impl ContactPickerUI {
    #[cfg(feature = "deprecated")]
    pub fn AddContact<P0>(&self, id: &windows_core::HSTRING, contact: P0) -> windows_core::Result<AddContactResult>
    where
        P0: windows_core::Param<super::Contact>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddContact)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), contact.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContact(&self, id: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContact)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsContact(&self, id: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsContact)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn DesiredFields(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredFields)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionMode(&self) -> windows_core::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContactRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContactRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContactRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddContact2<P0>(&self, contact: P0) -> windows_core::Result<AddContactResult>
    where
        P0: windows_core::Param<super::Contact>,
    {
        let this = &windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddContact)(windows_core::Interface::as_raw(this), contact.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactPickerUI>();
}
unsafe impl windows_core::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
    const IID: windows_core::GUID = <IContactPickerUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContactRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactRemovedEventArgs>();
}
unsafe impl windows_core::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IContactRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl windows_core::TypeKind for AddContactResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AddContactResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AddContactResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AddContactResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
}
