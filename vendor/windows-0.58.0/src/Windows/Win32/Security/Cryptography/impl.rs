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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICertSrvSetup_Vtbl
    where
        Identity: ICertSrvSetup_Impl,
    {
        unsafe extern "system" fn CAErrorId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::CAErrorId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAErrorString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::CAErrorString(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::InitializeDefaults(this, core::mem::transmute_copy(&bserver), core::mem::transmute_copy(&bclient)).into()
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetCASetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    ppropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::SetCASetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::IsPropertyEditable(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    pbeditable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcatypes: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetSupportedCATypes(this) {
                Ok(ok__) => {
                    pcatypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetProviderNameList(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetKeyLengthList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetHashAlgorithmList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetPrivateKeyContainerList(this, core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::GetExistingCACertificates(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAImportPFX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, bstrpasswd: core::mem::MaybeUninit<windows_core::BSTR>, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetup_Impl::CAImportPFX(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrpasswd), core::mem::transmute_copy(&boverwriteexistingkey)) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcadn: core::mem::MaybeUninit<windows_core::BSTR>, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::SetCADistinguishedName(this, core::mem::transmute(&bstrcadn), core::mem::transmute_copy(&bignoreunicode), core::mem::transmute_copy(&boverwriteexistingkey), core::mem::transmute_copy(&boverwriteexistingcainds)).into()
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdbdirectory: core::mem::MaybeUninit<windows_core::BSTR>, bstrlogdirectory: core::mem::MaybeUninit<windows_core::BSTR>, bstrsharedfolder: core::mem::MaybeUninit<windows_core::BSTR>, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::SetDatabaseInformation(this, core::mem::transmute(&bstrdbdirectory), core::mem::transmute(&bstrlogdirectory), core::mem::transmute(&bstrsharedfolder), core::mem::transmute_copy(&bforceoverwrite)).into()
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::SetParentCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaconfiguration: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::SetWebCAInformation(this, core::mem::transmute(&bstrcaconfiguration)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bclientonly: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::PreUnInstall(this, core::mem::transmute_copy(&bclientonly)).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetup_Impl::PostUnInstall(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CAErrorId: CAErrorId::<Identity, OFFSET>,
            CAErrorString: CAErrorString::<Identity, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICertSrvSetupKeyInformation_Vtbl
    where
        Identity: ICertSrvSetupKeyInformation_Impl,
    {
        unsafe extern "system" fn ProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetProviderName(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::Length(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetLength(this, core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn Existing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::Existing(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetExisting(this, core::mem::transmute_copy(&bval)).into()
        }
        unsafe extern "system" fn ContainerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::ContainerName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetContainerName(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetHashAlgorithm(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformation_Impl::ExistingCACertificate(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformation_Impl::SetExistingCACertificate(this, core::mem::transmute(&varval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProviderName: ProviderName::<Identity, OFFSET>,
            SetProviderName: SetProviderName::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            Existing: Existing::<Identity, OFFSET>,
            SetExisting: SetExisting::<Identity, OFFSET>,
            ContainerName: ContainerName::<Identity, OFFSET>,
            SetContainerName: SetContainerName::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICertSrvSetupKeyInformationCollection_Vtbl
    where
        Identity: ICertSrvSetupKeyInformationCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformationCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformationCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformationCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformationCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformationCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertSrvSetupKeyInformationCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pikeyinformation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertSrvSetupKeyInformationCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertSrvSetupKeyInformationCollection_Impl::Add(this, windows_core::from_raw_borrowed(&pikeyinformation)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICertificateEnrollmentPolicyServerSetup_Vtbl
    where
        Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
    {
        unsafe extern "system" fn ErrorString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateEnrollmentPolicyServerSetup_Impl::ErrorString(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentPolicyServerSetup_Impl::InitializeInstallDefaults(this).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateEnrollmentPolicyServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    ppropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentPolicyServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentPolicyServerSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn UnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthkeybasedrenewal: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentPolicyServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentPolicyServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pauthkeybasedrenewal)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ErrorString: ErrorString::<Identity, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            UnInstall: UnInstall::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICertificateEnrollmentServerSetup_Vtbl
    where
        Identity: ICertificateEnrollmentServerSetup_Impl,
    {
        unsafe extern "system" fn ErrorString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateEnrollmentServerSetup_Impl::ErrorString(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentServerSetup_Impl::InitializeInstallDefaults(this).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateEnrollmentServerSetup_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    ppropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentServerSetup_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentServerSetup_Impl::SetApplicationPoolCredentials(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentServerSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn UnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaconfig: *const core::mem::MaybeUninit<windows_core::VARIANT>, pauthentication: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICertificateEnrollmentServerSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateEnrollmentServerSetup_Impl::UnInstall(this, core::mem::transmute_copy(&pcaconfig), core::mem::transmute_copy(&pauthentication)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ErrorString: ErrorString::<Identity, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            UnInstall: UnInstall::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSCEPSetup_Vtbl
    where
        Identity: IMSCEPSetup_Impl,
    {
        unsafe extern "system" fn MSCEPErrorId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::MSCEPErrorId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::MSCEPErrorString(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::InitializeDefaults(this).into()
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::GetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::SetMSCEPSetupProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ppropertyvalue)).into()
        }
        unsafe extern "system" fn SetAccountInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::SetAccountInformation(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbempty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::IsMSCEPStoreEmpty(this) {
                Ok(ok__) => {
                    pbempty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderNameList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::GetProviderNameList(this, core::mem::transmute_copy(&bexchange)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSCEPSetup_Impl::GetKeyLengthList(this, core::mem::transmute_copy(&bexchange), core::mem::transmute(&bstrprovidername)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::Install(this).into()
        }
        unsafe extern "system" fn PreUnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::PreUnInstall(this).into()
        }
        unsafe extern "system" fn PostUnInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSCEPSetup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSCEPSetup_Impl::PostUnInstall(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MSCEPErrorId: MSCEPErrorId::<Identity, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSCEPSetup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
