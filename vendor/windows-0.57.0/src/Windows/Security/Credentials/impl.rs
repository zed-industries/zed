pub trait IWebAccount_Impl: Sized {
    fn WebAccountProvider(&self) -> windows_core::Result<WebAccountProvider>;
    fn UserName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn State(&self) -> windows_core::Result<WebAccountState>;
}
impl windows_core::RuntimeName for IWebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.IWebAccount";
}
impl IWebAccount_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>() -> IWebAccount_Vtbl {
        unsafe extern "system" fn WebAccountProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccount_Impl::WebAccountProvider(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccount_Impl::UserName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAccount_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut WebAccountState) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IWebAccount_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAccount, OFFSET>(),
            WebAccountProvider: WebAccountProvider::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAccount as windows_core::Interface>::IID
    }
}
