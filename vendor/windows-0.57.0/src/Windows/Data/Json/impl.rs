pub trait IJsonValue_Impl: Sized {
    fn ValueType(&self) -> windows_core::Result<JsonValueType>;
    fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetNumber(&self) -> windows_core::Result<f64>;
    fn GetBoolean(&self) -> windows_core::Result<bool>;
    fn GetArray(&self) -> windows_core::Result<JsonArray>;
    fn GetObject(&self) -> windows_core::Result<JsonObject>;
}
impl windows_core::RuntimeName for IJsonValue {
    const NAME: &'static str = "Windows.Data.Json.IJsonValue";
}
impl IJsonValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>() -> IJsonValue_Vtbl {
        unsafe extern "system" fn ValueType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut JsonValueType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::ValueType(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stringify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::Stringify(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::GetString(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::GetNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::GetBoolean(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArray<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::GetArray(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IJsonValue_Impl::GetObject(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IJsonValue, OFFSET>(),
            ValueType: ValueType::<Identity, Impl, OFFSET>,
            Stringify: Stringify::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetNumber: GetNumber::<Identity, Impl, OFFSET>,
            GetBoolean: GetBoolean::<Identity, Impl, OFFSET>,
            GetArray: GetArray::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsonValue as windows_core::Interface>::IID
    }
}
