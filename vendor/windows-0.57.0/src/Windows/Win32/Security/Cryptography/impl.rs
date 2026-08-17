#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CAErrorId(&self) -> windows_core::Result<i32>;
    fn CAErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeDefaults(&self, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetCASetupProperty(&self, propertyid: CASetupProperty) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCASetupProperty(&self, propertyid: CASetupProperty, ppropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn IsPropertyEditable(&self, propertyid: CASetupProperty) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSupportedCATypes(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn GetProviderNameList(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn GetKeyLengthList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetHashAlgorithmList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetPrivateKeyContainerList(&self, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetExistingCACertificates(&self) -> windows_core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(&self, bstrfilename: &windows_core::BSTR, bstrpasswd: &windows_core::BSTR, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(&self, bstrcadn: &windows_core::BSTR, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDatabaseInformation(&self, bstrdbdirectory: &windows_core::BSTR, bstrlogdirectory: &windows_core::BSTR, bstrsharedfolder: &windows_core::BSTR, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetParentCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWebCAInformation(&self, bstrcaconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn PreUnInstall(&self, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PostUnInstall(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICertSrvSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>() -> ICertSrvSetup_Vtbl {
        unsafe extern "system" fn CAErrorId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::CAErrorId(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAErrorString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::CAErrorString(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::InitializeDefaults(this, core::mem::transmute_copy(&bserver), core::mem::transmute_copy(&bclient)).into()
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetCASetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(ppropertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::SetCASetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::IsPropertyEditable(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(pbeditable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcatypes: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetSupportedCATypes(this) {
                Ok(ok__) => {
                    core::ptr::write(pcatypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetProviderNameList(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetKeyLengthList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetHashAlgorithmList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetPrivateKeyContainerList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::GetExistingCACertificates(this) {
                Ok(ok__) => {
                    core::ptr::write(ppval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAImportPFX<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, bstrpasswd: core::mem::MaybeUninit<windows_core::BSTR>, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetup_Impl::CAImportPFX(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrpasswd), core::mem::transmute_copy(&boverwriteexistingkey)) {
                Ok(ok__) => {
                    core::ptr::write(ppval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcadn: core::mem::MaybeUninit<windows_core::BSTR>, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::SetCADistinguishedName(this, core::mem::transmute(&bstrcadn), core::mem::transmute_copy(&bignoreunicode), core::mem::transmute_copy(&boverwriteexistingkey), core::mem::transmute_copy(&boverwriteexistingcainds)).into()
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdbdirectory: core::mem::MaybeUninit<windows_core::BSTR>, bstrlogdirectory: core::mem::MaybeUninit<windows_core::BSTR>, bstrsharedfolder: core::mem::MaybeUninit<windows_core::BSTR>, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::SetDatabaseInformation(this, core::mem::transmute(&bstrdbdirectory), core::mem::transmute(&bstrlogdirectory), core::mem::transmute(&bstrsharedfolder), core::mem::transmute_copy(&bforceoverwrite)).into()
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::SetParentCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::SetWebCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::PreUnInstall(this, core::mem::transmute_copy(&bclientonly)).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetup_Impl::PostUnInstall(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CAErrorId: CAErrorId::<Identity, Impl, OFFSET>,
            CAErrorString: CAErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, Impl, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, Impl, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, Impl, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, Impl, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, Impl, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, Impl, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, Impl, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, Impl, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, Impl, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, Impl, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetupKeyInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn SetLength(&self, lval: i32) -> windows_core::Result<()>;
    fn Existing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExisting(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContainerName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHashAlgorithm(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExistingCACertificate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExistingCACertificate(&self, varval: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICertSrvSetupKeyInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetupKeyInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformation_Vtbl {
        unsafe extern "system" fn ProviderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::ProviderName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetProviderName(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::Length(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetLength(this, core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn Existing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::Existing(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetExisting(this, core::mem::transmute_copy(&bval)).into()
        }
        unsafe extern "system" fn ContainerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::ContainerName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetContainerName(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetHashAlgorithm(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformation_Impl::ExistingCACertificate(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformation_Impl::SetExistingCACertificate(this, core::mem::transmute(&varval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            SetProviderName: SetProviderName::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            Existing: Existing::<Identity, Impl, OFFSET>,
            SetExisting: SetExisting::<Identity, Impl, OFFSET>,
            ContainerName: ContainerName::<Identity, Impl, OFFSET>,
            SetContainerName: SetContainerName::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, Impl, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetupKeyInformationCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, pikeyinformation: Option<&ICertSrvSetupKeyInformation>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICertSrvSetupKeyInformationCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ICertSrvSetupKeyInformationCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>() -> ICertSrvSetupKeyInformationCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformationCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformationCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertSrvSetupKeyInformationCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pikeyinformation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertSrvSetupKeyInformationCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pikeyinformation)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertSrvSetupKeyInformationCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateEnrollmentPolicyServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeInstallDefaults(&self) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CEPSetupProperty) -> windows_core::Result<windows_core::VARIANT>;
    fn SetProperty(&self, propertyid: CEPSetupProperty, ppropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn UnInstall(&self, pauthkeybasedrenewal: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICertificateEnrollmentPolicyServerSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ICertificateEnrollmentPolicyServerSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertificateEnrollmentPolicyServerSetup_Impl::ErrorString(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentPolicyServerSetup_Impl::InitializeInstallDefaults(this).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertificateEnrollmentPolicyServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(ppropertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentPolicyServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentPolicyServerSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn UnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthkeybasedrenewal: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentPolicyServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentPolicyServerSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateEnrollmentServerSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeInstallDefaults(&self) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: CESSetupProperty) -> windows_core::Result<windows_core::VARIANT>;
    fn SetProperty(&self, propertyid: CESSetupProperty, ppropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetApplicationPoolCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Install(&self) -> windows_core::Result<()>;
    fn UnInstall(&self, pcaconfig: *const windows_core::VARIANT, pauthentication: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICertificateEnrollmentServerSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ICertificateEnrollmentServerSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>() -> ICertificateEnrollmentServerSetup_Vtbl {
        unsafe extern "system" fn ErrorString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertificateEnrollmentServerSetup_Impl::ErrorString(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentServerSetup_Impl::InitializeInstallDefaults(this).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICertificateEnrollmentServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(ppropertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentServerSetup_Impl::SetApplicationPoolCredentials(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentServerSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn UnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaconfig: *const core::mem::MaybeUninit<windows_core::VARIANT>, pauthentication: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICertificateEnrollmentServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pcaconfig), core::mem::transmute_copy(&pauthentication)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateEnrollmentServerSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSCEPSetup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MSCEPErrorId(&self) -> windows_core::Result<i32>;
    fn MSCEPErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializeDefaults(&self) -> windows_core::Result<()>;
    fn GetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty) -> windows_core::Result<windows_core::VARIANT>;
    fn SetMSCEPSetupProperty(&self, propertyid: MSCEPSetupProperty, ppropertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetAccountInformation(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsMSCEPStoreEmpty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProviderNameList(&self, bexchange: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::VARIANT>;
    fn GetKeyLengthList(&self, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn Install(&self) -> windows_core::Result<()>;
    fn PreUnInstall(&self) -> windows_core::Result<()>;
    fn PostUnInstall(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSCEPSetup {}
#[cfg(feature = "Win32_System_Com")]
impl IMSCEPSetup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>() -> IMSCEPSetup_Vtbl {
        unsafe extern "system" fn MSCEPErrorId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::MSCEPErrorId(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::MSCEPErrorString(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::InitializeDefaults(this).into()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::GetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::SetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetAccountInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::SetAccountInformation(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbempty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::IsMSCEPStoreEmpty(this) {
                Ok(ok__) => {
                    core::ptr::write(pbempty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::GetProviderNameList(this, core::mem::transmute_copy(&bexchange)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMSCEPSetup_Impl::GetKeyLengthList(this, core::mem::transmute_copy(&bexchange), core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::PreUnInstall(this).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMSCEPSetup_Impl::PostUnInstall(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Identity, Impl, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSCEPSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
