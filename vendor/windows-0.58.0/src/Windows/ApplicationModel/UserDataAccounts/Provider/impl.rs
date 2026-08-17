pub trait IUserDataAccountProviderOperation_Impl: Sized {
    fn Kind(&self) -> windows_core::Result<UserDataAccountProviderOperationKind>;
}
impl windows_core::RuntimeName for IUserDataAccountProviderOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
}
impl IUserDataAccountProviderOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserDataAccountProviderOperation_Vtbl
    where
        Identity: IUserDataAccountProviderOperation_Impl,
    {
        unsafe extern "system" fn Kind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> windows_core::HRESULT
        where
            Identity: IUserDataAccountProviderOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUserDataAccountProviderOperation_Impl::Kind(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserDataAccountProviderOperation, OFFSET>(), Kind: Kind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDataAccountProviderOperation as windows_core::Interface>::IID
    }
}
