pub trait ICcgDomainAuthCredentials_Impl: Sized {
    fn GetPasswordCredentials(&self, plugininput: &windows_core::PCWSTR, domainname: *mut windows_core::PWSTR, username: *mut windows_core::PWSTR, password: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICcgDomainAuthCredentials {}
impl ICcgDomainAuthCredentials_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICcgDomainAuthCredentials_Vtbl
    where
        Identity: ICcgDomainAuthCredentials_Impl,
    {
        unsafe extern "system" fn GetPasswordCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugininput: windows_core::PCWSTR, domainname: *mut windows_core::PWSTR, username: *mut windows_core::PWSTR, password: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICcgDomainAuthCredentials_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICcgDomainAuthCredentials_Impl::GetPasswordCredentials(this, core::mem::transmute(&plugininput), core::mem::transmute_copy(&domainname), core::mem::transmute_copy(&username), core::mem::transmute_copy(&password)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPasswordCredentials: GetPasswordCredentials::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICcgDomainAuthCredentials as windows_core::Interface>::IID
    }
}
