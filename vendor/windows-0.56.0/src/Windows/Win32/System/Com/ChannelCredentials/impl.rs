pub trait IChannelCredentials_Impl: Sized + super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &windows_core::BSTR, username: &windows_core::BSTR, password: &windows_core::BSTR, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetUserNameCredential(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findyype: &windows_core::BSTR, findvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findtype: &windows_core::BSTR, findvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &windows_core::BSTR, revocationmode: &windows_core::BSTR, certificatevalidationmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &windows_core::BSTR, localissuerbindingtype: &windows_core::BSTR, localissuerbinding: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChannelCredentials {}
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>() -> IChannelCredentials_Vtbl {
        unsafe extern "system" fn SetWindowsCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: std::mem::MaybeUninit<windows_core::BSTR>, username: std::mem::MaybeUninit<windows_core::BSTR>, password: std::mem::MaybeUninit<windows_core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetWindowsCredential(this, core::mem::transmute(&domain), core::mem::transmute(&username), core::mem::transmute(&password), core::mem::transmute_copy(&impersonationlevel), core::mem::transmute_copy(&allowntlm)).into()
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: std::mem::MaybeUninit<windows_core::BSTR>, password: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetUserNameCredential(this, core::mem::transmute(&username), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: std::mem::MaybeUninit<windows_core::BSTR>, storename: std::mem::MaybeUninit<windows_core::BSTR>, findyype: std::mem::MaybeUninit<windows_core::BSTR>, findvalue: std::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetClientCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findyype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: std::mem::MaybeUninit<windows_core::BSTR>, storelocation: std::mem::MaybeUninit<windows_core::BSTR>, storename: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetClientCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: std::mem::MaybeUninit<windows_core::BSTR>, password: std::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetClientCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: std::mem::MaybeUninit<windows_core::BSTR>, storename: std::mem::MaybeUninit<windows_core::BSTR>, findtype: std::mem::MaybeUninit<windows_core::BSTR>, findvalue: std::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findtype), core::mem::transmute(&findvalue)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: std::mem::MaybeUninit<windows_core::BSTR>, storelocation: std::mem::MaybeUninit<windows_core::BSTR>, storename: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: std::mem::MaybeUninit<windows_core::BSTR>, password: std::mem::MaybeUninit<windows_core::BSTR>, keystorageflags: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetDefaultServiceCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: std::mem::MaybeUninit<windows_core::BSTR>, revocationmode: std::mem::MaybeUninit<windows_core::BSTR>, certificatevalidationmode: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetServiceCertificateAuthentication(this, core::mem::transmute(&storelocation), core::mem::transmute(&revocationmode), core::mem::transmute(&certificatevalidationmode)).into()
        }
        unsafe extern "system" fn SetIssuedToken<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localissueraddres: std::mem::MaybeUninit<windows_core::BSTR>, localissuerbindingtype: std::mem::MaybeUninit<windows_core::BSTR>, localissuerbinding: std::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChannelCredentials_Impl::SetIssuedToken(this, core::mem::transmute(&localissueraddres), core::mem::transmute(&localissuerbindingtype), core::mem::transmute(&localissuerbinding)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, Impl, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, Impl, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, Impl, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, Impl, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, Impl, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelCredentials as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
