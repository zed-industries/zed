pub trait IContactField_Impl: Sized {
    fn Type(&self) -> windows_core::Result<ContactFieldType>;
    fn Category(&self) -> windows_core::Result<ContactFieldCategory>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Value(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactField";
}
impl IContactField_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>() -> IContactField_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ContactFieldType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactField_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ContactFieldCategory) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactField_Impl::Category(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactField_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactField_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactField, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactField as windows_core::Interface>::IID
    }
}
pub trait IContactFieldFactory_Impl: Sized {
    fn CreateField_Default(&self, value: &windows_core::HSTRING, r#type: ContactFieldType) -> windows_core::Result<ContactField>;
    fn CreateField_Category(&self, value: &windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> windows_core::Result<ContactField>;
    fn CreateField_Custom(&self, name: &windows_core::HSTRING, value: &windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> windows_core::Result<ContactField>;
}
impl windows_core::RuntimeName for IContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactFieldFactory";
}
impl IContactFieldFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>() -> IContactFieldFactory_Vtbl {
        unsafe extern "system" fn CreateField_Default<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, r#type: ContactFieldType, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactFieldFactory_Impl::CreateField_Default(this, core::mem::transmute(&value), r#type) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Category<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactFieldFactory_Impl::CreateField_Category(this, core::mem::transmute(&value), r#type, category) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Custom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>, value: core::mem::MaybeUninit<windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactFieldFactory_Impl::CreateField_Custom(this, core::mem::transmute(&name), core::mem::transmute(&value), r#type, category) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactFieldFactory, OFFSET>(),
            CreateField_Default: CreateField_Default::<Identity, Impl, OFFSET>,
            CreateField_Category: CreateField_Category::<Identity, Impl, OFFSET>,
            CreateField_Custom: CreateField_Custom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactFieldFactory as windows_core::Interface>::IID
    }
}
pub trait IContactInstantMessageFieldFactory_Impl: Sized {
    fn CreateInstantMessage_Default(&self, username: &windows_core::HSTRING) -> windows_core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_Category(&self, username: &windows_core::HSTRING, category: ContactFieldCategory) -> windows_core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_All(&self, username: &windows_core::HSTRING, category: ContactFieldCategory, service: &windows_core::HSTRING, displaytext: &windows_core::HSTRING, verb: Option<&super::super::Foundation::Uri>) -> windows_core::Result<ContactInstantMessageField>;
}
impl windows_core::RuntimeName for IContactInstantMessageFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactInstantMessageFieldFactory";
}
impl IContactInstantMessageFieldFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>() -> IContactInstantMessageFieldFactory_Vtbl {
        unsafe extern "system" fn CreateInstantMessage_Default<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactInstantMessageFieldFactory_Impl::CreateInstantMessage_Default(this, core::mem::transmute(&username)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_Category<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactInstantMessageFieldFactory_Impl::CreateInstantMessage_Category(this, core::mem::transmute(&username), category) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_All<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::HSTRING>, category: ContactFieldCategory, service: core::mem::MaybeUninit<windows_core::HSTRING>, displaytext: core::mem::MaybeUninit<windows_core::HSTRING>, verb: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactInstantMessageFieldFactory_Impl::CreateInstantMessage_All(this, core::mem::transmute(&username), category, core::mem::transmute(&service), core::mem::transmute(&displaytext), windows_core::from_raw_borrowed(&verb)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactInstantMessageFieldFactory, OFFSET>(),
            CreateInstantMessage_Default: CreateInstantMessage_Default::<Identity, Impl, OFFSET>,
            CreateInstantMessage_Category: CreateInstantMessage_Category::<Identity, Impl, OFFSET>,
            CreateInstantMessage_All: CreateInstantMessage_All::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactInstantMessageFieldFactory as windows_core::Interface>::IID
    }
}
pub trait IContactLocationFieldFactory_Impl: Sized {
    fn CreateLocation_Default(&self, unstructuredaddress: &windows_core::HSTRING) -> windows_core::Result<ContactLocationField>;
    fn CreateLocation_Category(&self, unstructuredaddress: &windows_core::HSTRING, category: ContactFieldCategory) -> windows_core::Result<ContactLocationField>;
    fn CreateLocation_All(&self, unstructuredaddress: &windows_core::HSTRING, category: ContactFieldCategory, street: &windows_core::HSTRING, city: &windows_core::HSTRING, region: &windows_core::HSTRING, country: &windows_core::HSTRING, postalcode: &windows_core::HSTRING) -> windows_core::Result<ContactLocationField>;
}
impl windows_core::RuntimeName for IContactLocationFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactLocationFieldFactory";
}
impl IContactLocationFieldFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>() -> IContactLocationFieldFactory_Vtbl {
        unsafe extern "system" fn CreateLocation_Default<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unstructuredaddress: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactLocationFieldFactory_Impl::CreateLocation_Default(this, core::mem::transmute(&unstructuredaddress)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_Category<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unstructuredaddress: core::mem::MaybeUninit<windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactLocationFieldFactory_Impl::CreateLocation_Category(this, core::mem::transmute(&unstructuredaddress), category) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_All<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unstructuredaddress: core::mem::MaybeUninit<windows_core::HSTRING>, category: ContactFieldCategory, street: core::mem::MaybeUninit<windows_core::HSTRING>, city: core::mem::MaybeUninit<windows_core::HSTRING>, region: core::mem::MaybeUninit<windows_core::HSTRING>, country: core::mem::MaybeUninit<windows_core::HSTRING>, postalcode: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContactLocationFieldFactory_Impl::CreateLocation_All(this, core::mem::transmute(&unstructuredaddress), category, core::mem::transmute(&street), core::mem::transmute(&city), core::mem::transmute(&region), core::mem::transmute(&country), core::mem::transmute(&postalcode)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactLocationFieldFactory, OFFSET>(),
            CreateLocation_Default: CreateLocation_Default::<Identity, Impl, OFFSET>,
            CreateLocation_Category: CreateLocation_Category::<Identity, Impl, OFFSET>,
            CreateLocation_All: CreateLocation_All::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactLocationFieldFactory as windows_core::Interface>::IID
    }
}
