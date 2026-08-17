windows_core::imp::define_interface!(IChannelCredentials, IChannelCredentials_Vtbl, 0x181b448c_c17c_4b17_ac6d_06699b93198f);
impl core::ops::Deref for IChannelCredentials {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChannelCredentials, windows_core::IUnknown, super::IDispatch);
impl IChannelCredentials {
    pub unsafe fn SetWindowsCredential(&self, domain: &windows_core::BSTR, username: &windows_core::BSTR, password: &windows_core::BSTR, impersonationlevel: i32, allowntlm: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWindowsCredential)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(domain), core::mem::transmute_copy(username), core::mem::transmute_copy(password), impersonationlevel, allowntlm.into()).ok() }
    }
    pub unsafe fn SetUserNameCredential(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserNameCredential)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(username), core::mem::transmute_copy(password)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetClientCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findyype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateFromStore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(storelocation), core::mem::transmute_copy(storename), core::mem::transmute_copy(findyype), core::mem::transmute_copy(findvalue)).ok() }
    }
    pub unsafe fn SetClientCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateFromStoreByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subjectname), core::mem::transmute_copy(storelocation), core::mem::transmute_copy(storename)).ok() }
    }
    pub unsafe fn SetClientCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateFromFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename), core::mem::transmute_copy(password), core::mem::transmute_copy(keystorageflags)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findtype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(storelocation), core::mem::transmute_copy(storename), core::mem::transmute_copy(findtype), core::mem::transmute_copy(findvalue)).ok() }
    }
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStoreByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subjectname), core::mem::transmute_copy(storelocation), core::mem::transmute_copy(storename)).ok() }
    }
    pub unsafe fn SetDefaultServiceCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename), core::mem::transmute_copy(password), core::mem::transmute_copy(keystorageflags)).ok() }
    }
    pub unsafe fn SetServiceCertificateAuthentication(&self, storelocation: &windows_core::BSTR, revocationmode: &windows_core::BSTR, certificatevalidationmode: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServiceCertificateAuthentication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(storelocation), core::mem::transmute_copy(revocationmode), core::mem::transmute_copy(certificatevalidationmode)).ok() }
    }
    pub unsafe fn SetIssuedToken(&self, localissueraddres: &windows_core::BSTR, localissuerbindingtype: &windows_core::BSTR, localissuerbinding: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIssuedToken)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localissueraddres), core::mem::transmute_copy(localissuerbindingtype), core::mem::transmute_copy(localissuerbinding)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetWindowsCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetUserNameCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetClientCertificateFromStore: usize,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDefaultServiceCertificateFromStore: usize,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IChannelCredentials_Impl: super::IDispatch_Impl {
    fn SetWindowsCredential(&self, domain: &windows_core::BSTR, username: &windows_core::BSTR, password: &windows_core::BSTR, impersonationlevel: i32, allowntlm: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetUserNameCredential(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findyype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetClientCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStore(&self, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR, findtype: &windows_core::BSTR, findvalue: &super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &windows_core::BSTR, storelocation: &windows_core::BSTR, storename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDefaultServiceCertificateFromFile(&self, filename: &windows_core::BSTR, password: &windows_core::BSTR, keystorageflags: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServiceCertificateAuthentication(&self, storelocation: &windows_core::BSTR, revocationmode: &windows_core::BSTR, certificatevalidationmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIssuedToken(&self, localissueraddres: &windows_core::BSTR, localissuerbindingtype: &windows_core::BSTR, localissuerbinding: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IChannelCredentials_Vtbl {
    pub const fn new<Identity: IChannelCredentials_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWindowsCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: *mut core::ffi::c_void, username: *mut core::ffi::c_void, password: *mut core::ffi::c_void, impersonationlevel: i32, allowntlm: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetWindowsCredential(this, core::mem::transmute(&domain), core::mem::transmute(&username), core::mem::transmute(&password), core::mem::transmute_copy(&impersonationlevel), core::mem::transmute_copy(&allowntlm)).into()
            }
        }
        unsafe extern "system" fn SetUserNameCredential<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetUserNameCredential(this, core::mem::transmute(&username), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: *mut core::ffi::c_void, storename: *mut core::ffi::c_void, findyype: *mut core::ffi::c_void, findvalue: super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetClientCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findyype), core::mem::transmute(&findvalue)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: *mut core::ffi::c_void, storelocation: *mut core::ffi::c_void, storename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetClientCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void, password: *mut core::ffi::c_void, keystorageflags: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetClientCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
            }
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStore<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: *mut core::ffi::c_void, storename: *mut core::ffi::c_void, findtype: *mut core::ffi::c_void, findvalue: super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetDefaultServiceCertificateFromStore(this, core::mem::transmute(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&findtype), core::mem::transmute(&findvalue)).into()
            }
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromStoreByName<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subjectname: *mut core::ffi::c_void, storelocation: *mut core::ffi::c_void, storename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetDefaultServiceCertificateFromStoreByName(this, core::mem::transmute(&subjectname), core::mem::transmute(&storelocation), core::mem::transmute(&storename)).into()
            }
        }
        unsafe extern "system" fn SetDefaultServiceCertificateFromFile<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void, password: *mut core::ffi::c_void, keystorageflags: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetDefaultServiceCertificateFromFile(this, core::mem::transmute(&filename), core::mem::transmute(&password), core::mem::transmute(&keystorageflags)).into()
            }
        }
        unsafe extern "system" fn SetServiceCertificateAuthentication<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: *mut core::ffi::c_void, revocationmode: *mut core::ffi::c_void, certificatevalidationmode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetServiceCertificateAuthentication(this, core::mem::transmute(&storelocation), core::mem::transmute(&revocationmode), core::mem::transmute(&certificatevalidationmode)).into()
            }
        }
        unsafe extern "system" fn SetIssuedToken<Identity: IChannelCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localissueraddres: *mut core::ffi::c_void, localissuerbindingtype: *mut core::ffi::c_void, localissuerbinding: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelCredentials_Impl::SetIssuedToken(this, core::mem::transmute(&localissueraddres), core::mem::transmute(&localissuerbindingtype), core::mem::transmute(&localissuerbinding)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetWindowsCredential: SetWindowsCredential::<Identity, OFFSET>,
            SetUserNameCredential: SetUserNameCredential::<Identity, OFFSET>,
            SetClientCertificateFromStore: SetClientCertificateFromStore::<Identity, OFFSET>,
            SetClientCertificateFromStoreByName: SetClientCertificateFromStoreByName::<Identity, OFFSET>,
            SetClientCertificateFromFile: SetClientCertificateFromFile::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStore: SetDefaultServiceCertificateFromStore::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromStoreByName: SetDefaultServiceCertificateFromStoreByName::<Identity, OFFSET>,
            SetDefaultServiceCertificateFromFile: SetDefaultServiceCertificateFromFile::<Identity, OFFSET>,
            SetServiceCertificateAuthentication: SetServiceCertificateAuthentication::<Identity, OFFSET>,
            SetIssuedToken: SetIssuedToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelCredentials as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IChannelCredentials {}
