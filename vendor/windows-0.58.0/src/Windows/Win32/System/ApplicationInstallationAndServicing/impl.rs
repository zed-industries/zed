pub trait IAssemblyCache_Impl: Sized {
    fn UninstallAssembly(&self, dwflags: u32, pszassemblyname: &windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::Result<()>;
    fn QueryAssemblyInfo(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: &windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> windows_core::Result<()>;
    fn CreateAssemblyCacheItem(&self, dwflags: u32, pvreserved: *mut core::ffi::c_void, ppasmitem: *mut Option<IAssemblyCacheItem>, pszassemblyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Reserved(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn InstallAssembly(&self, dwflags: u32, pszmanifestfilepath: &windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAssemblyCache {}
impl IAssemblyCache_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAssemblyCache_Vtbl
    where
        Identity: IAssemblyCache_Impl,
    {
        unsafe extern "system" fn UninstallAssembly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszassemblyname: windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> windows_core::HRESULT
        where
            Identity: IAssemblyCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCache_Impl::UninstallAssembly(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszassemblyname), core::mem::transmute_copy(&prefdata), core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn QueryAssemblyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> windows_core::HRESULT
        where
            Identity: IAssemblyCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCache_Impl::QueryAssemblyInfo(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszassemblyname), core::mem::transmute_copy(&pasminfo)).into()
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pvreserved: *mut core::ffi::c_void, ppasmitem: *mut *mut core::ffi::c_void, pszassemblyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAssemblyCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCache_Impl::CreateAssemblyCacheItem(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&ppasmitem), core::mem::transmute(&pszassemblyname)).into()
        }
        unsafe extern "system" fn Reserved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssemblyCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAssemblyCache_Impl::Reserved(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallAssembly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszmanifestfilepath: windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> windows_core::HRESULT
        where
            Identity: IAssemblyCache_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCache_Impl::InstallAssembly(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszmanifestfilepath), core::mem::transmute_copy(&prefdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UninstallAssembly: UninstallAssembly::<Identity, OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Identity, OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Identity, OFFSET>,
            Reserved: Reserved::<Identity, OFFSET>,
            InstallAssembly: InstallAssembly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyCache as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyCacheItem_Impl: Sized {
    fn CreateStream(&self, dwflags: u32, pszstreamname: &windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut Option<super::Com::IStream>, pulimaxsize: *mut u64) -> windows_core::Result<()>;
    fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> windows_core::Result<()>;
    fn AbortItem(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAssemblyCacheItem {}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyCacheItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAssemblyCacheItem_Vtbl
    where
        Identity: IAssemblyCacheItem_Impl,
    {
        unsafe extern "system" fn CreateStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pszstreamname: windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut core::ffi::c_void, pulimaxsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAssemblyCacheItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCacheItem_Impl::CreateStream(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszstreamname), core::mem::transmute_copy(&dwformat), core::mem::transmute_copy(&dwformatflags), core::mem::transmute_copy(&ppistream), core::mem::transmute_copy(&pulimaxsize)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyCacheItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCacheItem_Impl::Commit(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&puldisposition)).into()
        }
        unsafe extern "system" fn AbortItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssemblyCacheItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyCacheItem_Impl::AbortItem(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            AbortItem: AbortItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyCacheItem as windows_core::Interface>::IID
    }
}
pub trait IAssemblyName_Impl: Sized {
    fn SetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, cbproperty: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: u32, pvproperty: *mut core::ffi::c_void, pcbproperty: *mut u32) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn GetDisplayName(&self, szdisplayname: windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> windows_core::Result<()>;
    fn Reserved(&self, refiid: *const windows_core::GUID, punkreserved1: Option<&windows_core::IUnknown>, punkreserved2: Option<&windows_core::IUnknown>, szreserved: &windows_core::PCWSTR, llreserved: i64, pvreserved: *mut core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetName(&self, lpcwbuffer: *mut u32, pwzname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> windows_core::Result<()>;
    fn IsEqual(&self, pname: Option<&IAssemblyName>, dwcmpflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAssemblyName>;
}
impl windows_core::RuntimeName for IAssemblyName {}
impl IAssemblyName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAssemblyName_Vtbl
    where
        Identity: IAssemblyName_Impl,
    {
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: u32, pvproperty: *mut core::ffi::c_void, cbproperty: u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pvproperty), core::mem::transmute_copy(&cbproperty)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: u32, pvproperty: *mut core::ffi::c_void, pcbproperty: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pvproperty), core::mem::transmute_copy(&pcbproperty)).into()
        }
        unsafe extern "system" fn Finalize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::Finalize(this).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdisplayname: windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::GetDisplayName(this, core::mem::transmute_copy(&szdisplayname), core::mem::transmute_copy(&pccdisplayname), core::mem::transmute_copy(&dwdisplayflags)).into()
        }
        unsafe extern "system" fn Reserved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, punkreserved1: *mut core::ffi::c_void, punkreserved2: *mut core::ffi::c_void, szreserved: windows_core::PCWSTR, llreserved: i64, pvreserved: *mut core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::Reserved(this, core::mem::transmute_copy(&refiid), windows_core::from_raw_borrowed(&punkreserved1), windows_core::from_raw_borrowed(&punkreserved2), core::mem::transmute(&szreserved), core::mem::transmute_copy(&llreserved), core::mem::transmute_copy(&pvreserved), core::mem::transmute_copy(&cbreserved), core::mem::transmute_copy(&ppreserved)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::GetName(this, core::mem::transmute_copy(&lpcwbuffer), core::mem::transmute_copy(&pwzname)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::GetVersion(this, core::mem::transmute_copy(&pdwversionhi), core::mem::transmute_copy(&pdwversionlow)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::ffi::c_void, dwcmpflags: u32) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAssemblyName_Impl::IsEqual(this, windows_core::from_raw_borrowed(&pname), core::mem::transmute_copy(&dwcmpflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAssemblyName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAssemblyName_Impl::Clone(this) {
                Ok(ok__) => {
                    pname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            Reserved: Reserved::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyName as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmdependencies: *mut Option<IMsmDependency>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumMsmDependency_Vtbl
    where
        Identity: IEnumMsmDependency_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmDependency_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgmsmdependencies), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmDependency_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmDependency_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumMsmDependency_Impl::Clone(this) {
                Ok(ok__) => {
                    pemsmdependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmDependency as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: Sized {
    fn Next(&self, cfetch: u32, rgmsmerrors: *mut Option<IMsmError>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumMsmError {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMsmError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumMsmError_Vtbl
    where
        Identity: IEnumMsmError_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmError_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgmsmerrors), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmError_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmError_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumMsmError_Impl::Clone(this) {
                Ok(ok__) => {
                    pemsmerrors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmError as windows_core::Interface>::IID
    }
}
pub trait IEnumMsmString_Impl: Sized {
    fn Next(&self, cfetch: u32, rgbstrstrings: *mut windows_core::BSTR, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cskip: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMsmString>;
}
impl windows_core::RuntimeName for IEnumMsmString {}
impl IEnumMsmString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumMsmString_Vtbl
    where
        Identity: IEnumMsmString_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmString_Impl::Next(this, core::mem::transmute_copy(&cfetch), core::mem::transmute_copy(&rgbstrstrings), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskip: u32) -> windows_core::HRESULT
        where
            Identity: IEnumMsmString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmString_Impl::Skip(this, core::mem::transmute_copy(&cskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumMsmString_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemsmstrings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumMsmString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumMsmString_Impl::Clone(this) {
                Ok(ok__) => {
                    pemsmstrings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMsmString as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependencies_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> windows_core::Result<IMsmDependency>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmDependencies {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependencies_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmDependencies_Vtbl
    where
        Identity: IMsmDependencies_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmDependencies_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                Ok(ok__) => {
                    r#return.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMsmDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmDependencies_Impl::Count(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmDependencies_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmDependencies as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmDependency_Impl: Sized + super::Com::IDispatch_Impl {
    fn Module(&self, module: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Language(&self, language: *mut i16) -> windows_core::Result<()>;
    fn Version(&self, version: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmDependency_Vtbl
    where
        Identity: IMsmDependency_Impl,
    {
        unsafe extern "system" fn Module<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmDependency_Impl::Module(this, core::mem::transmute_copy(&module)).into()
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmDependency_Impl::Language(this, core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmDependency_Impl::Version(this, core::mem::transmute_copy(&version)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Module: Module::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmDependency as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmError_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self, errortype: *mut msmErrorType) -> windows_core::Result<()>;
    fn Path(&self, errorpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Language(&self, errorlanguage: *mut i16) -> windows_core::Result<()>;
    fn DatabaseTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DatabaseKeys(&self) -> windows_core::Result<IMsmStrings>;
    fn ModuleTable(&self, errortable: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ModuleKeys(&self) -> windows_core::Result<IMsmStrings>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmError {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmError_Vtbl
    where
        Identity: IMsmError_Impl,
    {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortype: *mut msmErrorType) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmError_Impl::Type(this, core::mem::transmute_copy(&errortype)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmError_Impl::Path(this, core::mem::transmute_copy(&errorpath)).into()
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorlanguage: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmError_Impl::Language(this, core::mem::transmute_copy(&errorlanguage)).into()
        }
        unsafe extern "system" fn DatabaseTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortable: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmError_Impl::DatabaseTable(this, core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn DatabaseKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmError_Impl::DatabaseKeys(this) {
                Ok(ok__) => {
                    errorkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortable: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmError_Impl::ModuleTable(this, core::mem::transmute_copy(&errortable)).into()
        }
        unsafe extern "system" fn ModuleKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmError_Impl::ModuleKeys(this) {
                Ok(ok__) => {
                    errorkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            DatabaseTable: DatabaseTable::<Identity, OFFSET>,
            DatabaseKeys: DatabaseKeys::<Identity, OFFSET>,
            ModuleTable: ModuleTable::<Identity, OFFSET>,
            ModuleKeys: ModuleKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmError as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmErrors_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32) -> windows_core::Result<IMsmError>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmErrors {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrors_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmErrors_Vtbl
    where
        Identity: IMsmErrors_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmErrors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmErrors_Impl::get_Item(this, core::mem::transmute_copy(&item)) {
                Ok(ok__) => {
                    r#return.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMsmErrors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmErrors_Impl::Count(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmErrors_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmErrors_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmErrors as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmGetFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn ModuleFiles(&self) -> windows_core::Result<IMsmStrings>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmGetFiles {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmGetFiles_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmGetFiles_Vtbl
    where
        Identity: IMsmGetFiles_Impl,
    {
        unsafe extern "system" fn ModuleFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmGetFiles_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmGetFiles_Impl::ModuleFiles(this) {
                Ok(ok__) => {
                    files.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ModuleFiles: ModuleFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmGetFiles as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmMerge_Impl: Sized + super::Com::IDispatch_Impl {
    fn OpenDatabase(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OpenModule(&self, path: &windows_core::BSTR, language: i16) -> windows_core::Result<()>;
    fn CloseDatabase(&self, commit: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CloseModule(&self) -> windows_core::Result<()>;
    fn OpenLog(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CloseLog(&self) -> windows_core::Result<()>;
    fn Log(&self, message: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Errors(&self) -> windows_core::Result<IMsmErrors>;
    fn Dependencies(&self) -> windows_core::Result<IMsmDependencies>;
    fn Merge(&self, feature: &windows_core::BSTR, redirectdir: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Connect(&self, feature: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtractCAB(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtractFiles(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmMerge {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmMerge_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmMerge_Vtbl
    where
        Identity: IMsmMerge_Impl,
    {
        unsafe extern "system" fn OpenDatabase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::OpenDatabase(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn OpenModule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, language: i16) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::OpenModule(this, core::mem::transmute(&path), core::mem::transmute_copy(&language)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commit: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::CloseDatabase(this, core::mem::transmute_copy(&commit)).into()
        }
        unsafe extern "system" fn CloseModule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::CloseModule(this).into()
        }
        unsafe extern "system" fn OpenLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::OpenLog(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn CloseLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::CloseLog(this).into()
        }
        unsafe extern "system" fn Log<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::Log(this, core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn Errors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmMerge_Impl::Errors(this) {
                Ok(ok__) => {
                    errors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmMerge_Impl::Dependencies(this) {
                Ok(ok__) => {
                    dependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: core::mem::MaybeUninit<windows_core::BSTR>, redirectdir: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::Merge(this, core::mem::transmute(&feature), core::mem::transmute(&redirectdir)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::Connect(this, core::mem::transmute(&feature)).into()
        }
        unsafe extern "system" fn ExtractCAB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::ExtractCAB(this, core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn ExtractFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmMerge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmMerge_Impl::ExtractFiles(this, core::mem::transmute(&path)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, OFFSET>,
            OpenModule: OpenModule::<Identity, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, OFFSET>,
            CloseModule: CloseModule::<Identity, OFFSET>,
            OpenLog: OpenLog::<Identity, OFFSET>,
            CloseLog: CloseLog::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
            Errors: Errors::<Identity, OFFSET>,
            Dependencies: Dependencies::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            ExtractCAB: ExtractCAB::<Identity, OFFSET>,
            ExtractFiles: ExtractFiles::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmMerge as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMsmStrings_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, item: i32, r#return: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Count(&self, count: *mut i32) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMsmStrings {}
#[cfg(feature = "Win32_System_Com")]
impl IMsmStrings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMsmStrings_Vtbl
    where
        Identity: IMsmStrings_Impl,
    {
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: i32, r#return: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMsmStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmStrings_Impl::get_Item(this, core::mem::transmute_copy(&item), core::mem::transmute_copy(&r#return)).into()
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMsmStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMsmStrings_Impl::Count(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMsmStrings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMsmStrings_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMsmStrings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IPMApplicationInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn InstanceID(&self) -> windows_core::Result<windows_core::GUID>;
    fn OfferID(&self) -> windows_core::Result<windows_core::GUID>;
    fn DefaultTask(&self, pdefaulttask: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppTitle(&self, papptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IconPath(&self, pappiconpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NotificationState(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn AppInstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(&self) -> windows_core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn UpdateAvailable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InstallDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsThemable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsTrial(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn InstallPath(&self, pinstallpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DataRoot(&self, pdataroot: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Genre(&self) -> windows_core::Result<PM_APP_GENRE>;
    fn Publisher(&self, ppublisher: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppPlatMajorVersion(&self) -> windows_core::Result<u8>;
    fn AppPlatMinorVersion(&self) -> windows_core::Result<u8>;
    fn PublisherID(&self) -> windows_core::Result<windows_core::GUID>;
    fn IsMultiCore(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SID(&self, psid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AppPlatMajorVersionLightUp(&self) -> windows_core::Result<u8>;
    fn AppPlatMinorVersionLightUp(&self) -> windows_core::Result<u8>;
    fn set_UpdateAvailable(&self, isupdateavailable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_NotificationState(&self, isnotified: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_IconPath(&self, appiconpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn set_UninstallableState(&self, isuninstallable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsPinableOnKidZone(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOriginallyPreInstalled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInstallOnSD(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutOnSD(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutBackupRestore(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn set_EnterpriseDisabled(&self, isdisabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_EnterpriseUninstallable(&self, isuninstallable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnterpriseDisabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EnterpriseUninstallable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsVisibleOnAppList(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInboxApp(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn StorageID(&self) -> windows_core::Result<windows_core::GUID>;
    fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> windows_core::Result<()>;
    fn IsMovable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DeploymentAppEnumerationHubFilter(&self) -> windows_core::Result<PM_TILE_HUBTYPE>;
    fn ModifiedDate(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsOriginallyRestored(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShouldDeferMdilBind(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsFullyPreInstall(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsMdilMaintenanceNeeded(&self, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_Title(&self, apptitle: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMApplicationInfo {}
impl IPMApplicationInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMApplicationInfo_Vtbl
    where
        Identity: IPMApplicationInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::InstanceID(this) {
                Ok(ok__) => {
                    pinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfferID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pofferid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::OfferID(this) {
                Ok(ok__) => {
                    pofferid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefaulttask: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::DefaultTask(this, core::mem::transmute_copy(&pdefaulttask)).into()
        }
        unsafe extern "system" fn AppTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papptitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::AppTitle(this, core::mem::transmute_copy(&papptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappiconpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::IconPath(this, core::mem::transmute_copy(&pappiconpath)).into()
        }
        unsafe extern "system" fn NotificationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::NotificationState(this) {
                Ok(ok__) => {
                    pisnotified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppInstallType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::AppInstallType(this) {
                Ok(ok__) => {
                    pappinstalltype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::State(this) {
                Ok(ok__) => {
                    pstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRevoked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsRevoked(this) {
                Ok(ok__) => {
                    pisrevoked.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::UpdateAvailable(this) {
                Ok(ok__) => {
                    pisupdateavailable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::InstallDate(this) {
                Ok(ok__) => {
                    pinstalldate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsUninstallable(this) {
                Ok(ok__) => {
                    pisuninstallable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsThemable(this) {
                Ok(ok__) => {
                    pisthemable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsTrial(this) {
                Ok(ok__) => {
                    pistrial.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::InstallPath(this, core::mem::transmute_copy(&pinstallpath)).into()
        }
        unsafe extern "system" fn DataRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataroot: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::DataRoot(this, core::mem::transmute_copy(&pdataroot)).into()
        }
        unsafe extern "system" fn Genre<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::Genre(this) {
                Ok(ok__) => {
                    pgenre.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublisher: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::Publisher(this, core::mem::transmute_copy(&ppublisher)).into()
        }
        unsafe extern "system" fn Author<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthor: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::Author(this, core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::Version(this, core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajorver: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::AppPlatMajorVersion(this) {
                Ok(ok__) => {
                    pmajorver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminorver: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::AppPlatMinorVersion(this) {
                Ok(ok__) => {
                    pminorver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublisherid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::PublisherID(this) {
                Ok(ok__) => {
                    ppublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiCore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsMultiCore(this) {
                Ok(ok__) => {
                    pismulticore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::SID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajorver: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::AppPlatMajorVersionLightUp(this) {
                Ok(ok__) => {
                    pmajorver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminorver: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::AppPlatMinorVersionLightUp(this) {
                Ok(ok__) => {
                    pminorver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_UpdateAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_UpdateAvailable(this, core::mem::transmute_copy(&isupdateavailable)).into()
        }
        unsafe extern "system" fn set_NotificationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_NotificationState(this, core::mem::transmute_copy(&isnotified)).into()
        }
        unsafe extern "system" fn set_IconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appiconpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_IconPath(this, core::mem::transmute(&appiconpath)).into()
        }
        unsafe extern "system" fn set_UninstallableState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_UninstallableState(this, core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn IsPinableOnKidZone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsPinableOnKidZone(this) {
                Ok(ok__) => {
                    pispinable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsOriginallyPreInstalled(this) {
                Ok(ok__) => {
                    pispreinstalled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstallOnSD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsInstallOnSD(this) {
                Ok(ok__) => {
                    pisinstallonsd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutOnSD<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsOptoutOnSD(this) {
                Ok(ok__) => {
                    pisoptoutonsd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsOptoutBackupRestore(this) {
                Ok(ok__) => {
                    pisoptoutbackuprestore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_EnterpriseDisabled(this, core::mem::transmute_copy(&isdisabled)).into()
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_EnterpriseUninstallable(this, core::mem::transmute_copy(&isuninstallable)).into()
        }
        unsafe extern "system" fn EnterpriseDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::EnterpriseDisabled(this) {
                Ok(ok__) => {
                    isdisabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseUninstallable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::EnterpriseUninstallable(this) {
                Ok(ok__) => {
                    isuninstallable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisibleOnAppList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsVisibleOnAppList(this) {
                Ok(ok__) => {
                    pisvisible.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInboxApp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsInboxApp(this) {
                Ok(ok__) => {
                    pisinboxapp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorageid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::StorageID(this) {
                Ok(ok__) => {
                    pstorageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::StartAppBlob(this, core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsMovable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsMovable(this) {
                Ok(ok__) => {
                    pismovable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::DeploymentAppEnumerationHubFilter(this) {
                Ok(ok__) => {
                    hubtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::ModifiedDate(this) {
                Ok(ok__) => {
                    pmodifieddate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOriginallyRestored<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsOriginallyRestored(this) {
                Ok(ok__) => {
                    pisrestored.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::ShouldDeferMdilBind(this) {
                Ok(ok__) => {
                    pfdefermdilbind.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullyPreInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfo_Impl::IsFullyPreInstall(this) {
                Ok(ok__) => {
                    pfisfullypreinstall.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_IsMdilMaintenanceNeeded(this, core::mem::transmute_copy(&fismdilmaintenanceneeded)).into()
        }
        unsafe extern "system" fn set_Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, apptitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMApplicationInfo_Impl::set_Title(this, core::mem::transmute(&apptitle)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            InstanceID: InstanceID::<Identity, OFFSET>,
            OfferID: OfferID::<Identity, OFFSET>,
            DefaultTask: DefaultTask::<Identity, OFFSET>,
            AppTitle: AppTitle::<Identity, OFFSET>,
            IconPath: IconPath::<Identity, OFFSET>,
            NotificationState: NotificationState::<Identity, OFFSET>,
            AppInstallType: AppInstallType::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            IsRevoked: IsRevoked::<Identity, OFFSET>,
            UpdateAvailable: UpdateAvailable::<Identity, OFFSET>,
            InstallDate: InstallDate::<Identity, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, OFFSET>,
            IsThemable: IsThemable::<Identity, OFFSET>,
            IsTrial: IsTrial::<Identity, OFFSET>,
            InstallPath: InstallPath::<Identity, OFFSET>,
            DataRoot: DataRoot::<Identity, OFFSET>,
            Genre: Genre::<Identity, OFFSET>,
            Publisher: Publisher::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            AppPlatMajorVersion: AppPlatMajorVersion::<Identity, OFFSET>,
            AppPlatMinorVersion: AppPlatMinorVersion::<Identity, OFFSET>,
            PublisherID: PublisherID::<Identity, OFFSET>,
            IsMultiCore: IsMultiCore::<Identity, OFFSET>,
            SID: SID::<Identity, OFFSET>,
            AppPlatMajorVersionLightUp: AppPlatMajorVersionLightUp::<Identity, OFFSET>,
            AppPlatMinorVersionLightUp: AppPlatMinorVersionLightUp::<Identity, OFFSET>,
            set_UpdateAvailable: set_UpdateAvailable::<Identity, OFFSET>,
            set_NotificationState: set_NotificationState::<Identity, OFFSET>,
            set_IconPath: set_IconPath::<Identity, OFFSET>,
            set_UninstallableState: set_UninstallableState::<Identity, OFFSET>,
            IsPinableOnKidZone: IsPinableOnKidZone::<Identity, OFFSET>,
            IsOriginallyPreInstalled: IsOriginallyPreInstalled::<Identity, OFFSET>,
            IsInstallOnSD: IsInstallOnSD::<Identity, OFFSET>,
            IsOptoutOnSD: IsOptoutOnSD::<Identity, OFFSET>,
            IsOptoutBackupRestore: IsOptoutBackupRestore::<Identity, OFFSET>,
            set_EnterpriseDisabled: set_EnterpriseDisabled::<Identity, OFFSET>,
            set_EnterpriseUninstallable: set_EnterpriseUninstallable::<Identity, OFFSET>,
            EnterpriseDisabled: EnterpriseDisabled::<Identity, OFFSET>,
            EnterpriseUninstallable: EnterpriseUninstallable::<Identity, OFFSET>,
            IsVisibleOnAppList: IsVisibleOnAppList::<Identity, OFFSET>,
            IsInboxApp: IsInboxApp::<Identity, OFFSET>,
            StorageID: StorageID::<Identity, OFFSET>,
            StartAppBlob: StartAppBlob::<Identity, OFFSET>,
            IsMovable: IsMovable::<Identity, OFFSET>,
            DeploymentAppEnumerationHubFilter: DeploymentAppEnumerationHubFilter::<Identity, OFFSET>,
            ModifiedDate: ModifiedDate::<Identity, OFFSET>,
            IsOriginallyRestored: IsOriginallyRestored::<Identity, OFFSET>,
            ShouldDeferMdilBind: ShouldDeferMdilBind::<Identity, OFFSET>,
            IsFullyPreInstall: IsFullyPreInstall::<Identity, OFFSET>,
            set_IsMdilMaintenanceNeeded: set_IsMdilMaintenanceNeeded::<Identity, OFFSET>,
            set_Title: set_Title::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMApplicationInfo as windows_core::Interface>::IID
    }
}
pub trait IPMApplicationInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMApplicationInfo>;
}
impl windows_core::RuntimeName for IPMApplicationInfoEnumerator {}
impl IPMApplicationInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMApplicationInfoEnumerator_Vtbl
    where
        Identity: IPMApplicationInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMApplicationInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMApplicationInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    ppappinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMApplicationInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMBackgroundServiceAgentInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BSAID(&self) -> windows_core::Result<u32>;
    fn BGSpecifier(&self, pbgspecifier: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGSource(&self, pbgsource: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGType(&self, pbgtype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsPeriodic(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsScheduled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsScheduleAllowed(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsLaunchOnBoot(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsScheduled(&self, isscheduled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_IsScheduleAllowed(&self, isscheduleallowed: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMBackgroundServiceAgentInfo {}
impl IPMBackgroundServiceAgentInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfo_Vtbl
    where
        Identity: IPMBackgroundServiceAgentInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BSAID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsaid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::BSAID(this) {
                Ok(ok__) => {
                    pbsaid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BGSpecifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgspecifier: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::BGSpecifier(this, core::mem::transmute_copy(&pbgspecifier)).into()
        }
        unsafe extern "system" fn BGName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::BGName(this, core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn BGSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgsource: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::BGSource(this, core::mem::transmute_copy(&pbgsource)).into()
        }
        unsafe extern "system" fn BGType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::BGType(this, core::mem::transmute_copy(&pbgtype)).into()
        }
        unsafe extern "system" fn IsPeriodic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::IsPeriodic(this) {
                Ok(ok__) => {
                    pisperiodic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::IsScheduled(this) {
                Ok(ok__) => {
                    pisscheduled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScheduleAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::IsScheduleAllowed(this) {
                Ok(ok__) => {
                    pisscheduleallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsLaunchOnBoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfo_Impl::IsLaunchOnBoot(this) {
                Ok(ok__) => {
                    plaunchonboot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsScheduled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::set_IsScheduled(this, core::mem::transmute_copy(&isscheduled)).into()
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundServiceAgentInfo_Impl::set_IsScheduleAllowed(this, core::mem::transmute_copy(&isscheduleallowed)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            BSAID: BSAID::<Identity, OFFSET>,
            BGSpecifier: BGSpecifier::<Identity, OFFSET>,
            BGName: BGName::<Identity, OFFSET>,
            BGSource: BGSource::<Identity, OFFSET>,
            BGType: BGType::<Identity, OFFSET>,
            IsPeriodic: IsPeriodic::<Identity, OFFSET>,
            IsScheduled: IsScheduled::<Identity, OFFSET>,
            IsScheduleAllowed: IsScheduleAllowed::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            IsLaunchOnBoot: IsLaunchOnBoot::<Identity, OFFSET>,
            set_IsScheduled: set_IsScheduled::<Identity, OFFSET>,
            set_IsScheduleAllowed: set_IsScheduleAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfo as windows_core::Interface>::IID
    }
}
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMBackgroundServiceAgentInfo>;
}
impl windows_core::RuntimeName for IPMBackgroundServiceAgentInfoEnumerator {}
impl IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMBackgroundServiceAgentInfoEnumerator_Vtbl
    where
        Identity: IPMBackgroundServiceAgentInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbsainfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundServiceAgentInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundServiceAgentInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    ppbsainfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundServiceAgentInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMBackgroundWorkerInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn BGName(&self, pbgname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxStartupLatency(&self) -> windows_core::Result<u32>;
    fn ExpectedRuntime(&self) -> windows_core::Result<u32>;
    fn IsBootWorker(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMBackgroundWorkerInfo {}
impl IPMBackgroundWorkerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMBackgroundWorkerInfo_Vtbl
    where
        Identity: IPMBackgroundWorkerInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundWorkerInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundWorkerInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn BGName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbgname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMBackgroundWorkerInfo_Impl::BGName(this, core::mem::transmute_copy(&pbgname)).into()
        }
        unsafe extern "system" fn MaxStartupLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxstartuplatency: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundWorkerInfo_Impl::MaxStartupLatency(this) {
                Ok(ok__) => {
                    pmaxstartuplatency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpectedruntime: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundWorkerInfo_Impl::ExpectedRuntime(this) {
                Ok(ok__) => {
                    pexpectedruntime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBootWorker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundWorkerInfo_Impl::IsBootWorker(this) {
                Ok(ok__) => {
                    pisbootworker.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            BGName: BGName::<Identity, OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Identity, OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Identity, OFFSET>,
            IsBootWorker: IsBootWorker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfo as windows_core::Interface>::IID
    }
}
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMBackgroundWorkerInfo>;
}
impl windows_core::RuntimeName for IPMBackgroundWorkerInfoEnumerator {}
impl IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMBackgroundWorkerInfoEnumerator_Vtbl
    where
        Identity: IPMBackgroundWorkerInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbwinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMBackgroundWorkerInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMBackgroundWorkerInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    ppbwinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMBackgroundWorkerInfoEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPMDeploymentManager_Impl: Sized {
    fn ReportDownloadBegin(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ReportDownloadProgress(&self, productid: &windows_core::GUID, usprogress: u16) -> windows_core::Result<()>;
    fn ReportDownloadComplete(&self, productid: &windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> windows_core::Result<()>;
    fn BeginUninstall(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::Result<()>;
    fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn BeginUpdateLicense(&self, productid: &windows_core::GUID, offerid: &windows_core::GUID, pblicense: *const u8, cblicense: u32) -> windows_core::Result<()>;
    fn GetLicenseChallenge(&self, packagepath: &windows_core::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::Result<()>;
    fn GetLicenseChallengeByProductID(&self, productid: &windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> windows_core::Result<()>;
    fn GetLicenseChallengeByProductID2(&self, productid: &windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::Result<()>;
    fn RevokeLicense(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RebindMdilBinaries(&self, productid: &windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RebindAllMdilBinaries(&self, productid: &windows_core::GUID, instanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RegenerateXbf(&self, productid: &windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GenerateXbfForCurrentLocale(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginProvision(&self, productid: &windows_core::GUID, xmlpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BeginDeprovision(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ReindexSQLCEDatabases(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> windows_core::Result<u32>;
    fn UpdateChamberProfile(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(&self, productid: &windows_core::GUID, publishername: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::Result<()>;
    fn ReportRestoreCancelled(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn ResolveResourceString(&self, resourcestring: &windows_core::PCWSTR, presolvedresourcestring: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateCapabilitiesForModernApps(&self) -> windows_core::Result<()>;
    fn ReportDownloadStatusUpdate(&self, productid: &windows_core::GUID) -> windows_core::Result<()>;
    fn BeginUninstallWithOptions(&self, productid: &windows_core::GUID, removaloptions: u32) -> windows_core::Result<()>;
    fn BindDeferredMdilBinaries(&self) -> windows_core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(&self, packagefamilyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddLicenseForAppx(&self, productid: &windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> windows_core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPMDeploymentManager {}
#[cfg(feature = "Win32_System_Com")]
impl IPMDeploymentManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMDeploymentManager_Vtbl
    where
        Identity: IPMDeploymentManager_Impl,
    {
        unsafe extern "system" fn ReportDownloadBegin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReportDownloadBegin(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReportDownloadProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, usprogress: u16) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReportDownloadProgress(this, core::mem::transmute(&productid), core::mem::transmute_copy(&usprogress)).into()
        }
        unsafe extern "system" fn ReportDownloadComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReportDownloadComplete(this, core::mem::transmute(&productid), core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginInstall(this, core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUpdate(this, core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginDeployPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginDeployPackage(this, core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUpdateDeployedPackageLegacy(this, core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUninstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUninstall(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginEnterpriseAppInstall(this, core::mem::transmute_copy(&pinstallinfo)).into()
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginEnterpriseAppUpdate(this, core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn BeginUpdateLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, offerid: windows_core::GUID, pblicense: *const u8, cblicense: u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUpdateLicense(this, core::mem::transmute(&productid), core::mem::transmute(&offerid), core::mem::transmute_copy(&pblicense), core::mem::transmute_copy(&cblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallenge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagepath: core::mem::MaybeUninit<windows_core::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::GetLicenseChallenge(this, core::mem::transmute(&packagepath), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcbchallenge), core::mem::transmute_copy(&ppbkid), core::mem::transmute_copy(&pcbkid), core::mem::transmute_copy(&ppbdeviceid), core::mem::transmute_copy(&pcbdeviceid), core::mem::transmute_copy(&ppbsaltvalue), core::mem::transmute_copy(&pcbsaltvalue), core::mem::transmute_copy(&ppbkgvvalue), core::mem::transmute_copy(&pcbkgvvalue)).into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::GetLicenseChallengeByProductID(this, core::mem::transmute(&productid), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcblicense)).into()
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::GetLicenseChallengeByProductID2(this, core::mem::transmute(&productid), core::mem::transmute_copy(&ppbchallenge), core::mem::transmute_copy(&pcblicense), core::mem::transmute_copy(&ppbkid), core::mem::transmute_copy(&pcbkid), core::mem::transmute_copy(&ppbdeviceid), core::mem::transmute_copy(&pcbdeviceid), core::mem::transmute_copy(&ppbsaltvalue), core::mem::transmute_copy(&pcbsaltvalue), core::mem::transmute_copy(&ppbkgvvalue), core::mem::transmute_copy(&pcbkgvvalue)).into()
        }
        unsafe extern "system" fn RevokeLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::RevokeLicense(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn RebindMdilBinaries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::RebindMdilBinaries(this, core::mem::transmute(&productid), core::mem::transmute_copy(&filenames)).into()
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, instanceid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::RebindAllMdilBinaries(this, core::mem::transmute(&productid), core::mem::transmute(&instanceid)).into()
        }
        unsafe extern "system" fn RegenerateXbf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::RegenerateXbf(this, core::mem::transmute(&productid), core::mem::transmute_copy(&assemblypaths)).into()
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::GenerateXbfForCurrentLocale(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginProvision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, xmlpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginProvision(this, core::mem::transmute(&productid), core::mem::transmute(&xmlpath)).into()
        }
        unsafe extern "system" fn BeginDeprovision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginDeprovision(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReindexSQLCEDatabases(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMDeploymentManager_Impl::SetApplicationsNeedMaintenance(this, core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                Ok(ok__) => {
                    pcapplications.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateChamberProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::UpdateChamberProfile(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, publishername: windows_core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMDeploymentManager_Impl::EnterprisePolicyIsApplicationAllowed(this, core::mem::transmute(&productid), core::mem::transmute(&publishername)) {
                Ok(ok__) => {
                    pisallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUpdateDeployedPackage(this, core::mem::transmute_copy(&pupdateinfo)).into()
        }
        unsafe extern "system" fn ReportRestoreCancelled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReportRestoreCancelled(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn ResolveResourceString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcestring: windows_core::PCWSTR, presolvedresourcestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ResolveResourceString(this, core::mem::transmute(&resourcestring), core::mem::transmute_copy(&presolvedresourcestring)).into()
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::UpdateCapabilitiesForModernApps(this).into()
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::ReportDownloadStatusUpdate(this, core::mem::transmute(&productid)).into()
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, removaloptions: u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BeginUninstallWithOptions(this, core::mem::transmute(&productid), core::mem::transmute_copy(&removaloptions)).into()
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::BindDeferredMdilBinaries(this).into()
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::GenerateXamlLightupXbfForCurrentLocale(this, core::mem::transmute(&packagefamilyname)).into()
        }
        unsafe extern "system" fn AddLicenseForAppx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::AddLicenseForAppx(this, core::mem::transmute(&productid), core::mem::transmute_copy(&pblicense), core::mem::transmute_copy(&cblicense), core::mem::transmute_copy(&pbplayreadyheader), core::mem::transmute_copy(&cbplayreadyheader)).into()
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMDeploymentManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMDeploymentManager_Impl::FixJunctionsForAppsOnSDCard(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportDownloadBegin: ReportDownloadBegin::<Identity, OFFSET>,
            ReportDownloadProgress: ReportDownloadProgress::<Identity, OFFSET>,
            ReportDownloadComplete: ReportDownloadComplete::<Identity, OFFSET>,
            BeginInstall: BeginInstall::<Identity, OFFSET>,
            BeginUpdate: BeginUpdate::<Identity, OFFSET>,
            BeginDeployPackage: BeginDeployPackage::<Identity, OFFSET>,
            BeginUpdateDeployedPackageLegacy: BeginUpdateDeployedPackageLegacy::<Identity, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, OFFSET>,
            BeginEnterpriseAppInstall: BeginEnterpriseAppInstall::<Identity, OFFSET>,
            BeginEnterpriseAppUpdate: BeginEnterpriseAppUpdate::<Identity, OFFSET>,
            BeginUpdateLicense: BeginUpdateLicense::<Identity, OFFSET>,
            GetLicenseChallenge: GetLicenseChallenge::<Identity, OFFSET>,
            GetLicenseChallengeByProductID: GetLicenseChallengeByProductID::<Identity, OFFSET>,
            GetLicenseChallengeByProductID2: GetLicenseChallengeByProductID2::<Identity, OFFSET>,
            RevokeLicense: RevokeLicense::<Identity, OFFSET>,
            RebindMdilBinaries: RebindMdilBinaries::<Identity, OFFSET>,
            RebindAllMdilBinaries: RebindAllMdilBinaries::<Identity, OFFSET>,
            RegenerateXbf: RegenerateXbf::<Identity, OFFSET>,
            GenerateXbfForCurrentLocale: GenerateXbfForCurrentLocale::<Identity, OFFSET>,
            BeginProvision: BeginProvision::<Identity, OFFSET>,
            BeginDeprovision: BeginDeprovision::<Identity, OFFSET>,
            ReindexSQLCEDatabases: ReindexSQLCEDatabases::<Identity, OFFSET>,
            SetApplicationsNeedMaintenance: SetApplicationsNeedMaintenance::<Identity, OFFSET>,
            UpdateChamberProfile: UpdateChamberProfile::<Identity, OFFSET>,
            EnterprisePolicyIsApplicationAllowed: EnterprisePolicyIsApplicationAllowed::<Identity, OFFSET>,
            BeginUpdateDeployedPackage: BeginUpdateDeployedPackage::<Identity, OFFSET>,
            ReportRestoreCancelled: ReportRestoreCancelled::<Identity, OFFSET>,
            ResolveResourceString: ResolveResourceString::<Identity, OFFSET>,
            UpdateCapabilitiesForModernApps: UpdateCapabilitiesForModernApps::<Identity, OFFSET>,
            ReportDownloadStatusUpdate: ReportDownloadStatusUpdate::<Identity, OFFSET>,
            BeginUninstallWithOptions: BeginUninstallWithOptions::<Identity, OFFSET>,
            BindDeferredMdilBinaries: BindDeferredMdilBinaries::<Identity, OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale: GenerateXamlLightupXbfForCurrentLocale::<Identity, OFFSET>,
            AddLicenseForAppx: AddLicenseForAppx::<Identity, OFFSET>,
            FixJunctionsForAppsOnSDCard: FixJunctionsForAppsOnSDCard::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMDeploymentManager as windows_core::Interface>::IID
    }
}
pub trait IPMEnumerationManager_Impl: Sized {
    fn get_AllApplications(&self, ppappenum: *mut Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllTiles(&self, pptileenum: *mut Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllTasks(&self, pptaskenum: *mut Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllExtensions(&self, ppextensionenum: *mut Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllBackgroundServiceAgents(&self, ppbsaenum: *mut Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_AllBackgroundWorkers(&self, ppbswenum: *mut Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> windows_core::Result<()>;
    fn get_ApplicationInfo(&self, productid: &windows_core::GUID) -> windows_core::Result<IPMApplicationInfo>;
    fn get_TileInfo(&self, productid: &windows_core::GUID, tileid: &windows_core::BSTR) -> windows_core::Result<IPMTileInfo>;
    fn get_TaskInfo(&self, productid: &windows_core::GUID, taskid: &windows_core::BSTR) -> windows_core::Result<IPMTaskInfo>;
    fn get_TaskInfoEx(&self, productid: &windows_core::GUID, taskid: &windows_core::PCWSTR) -> windows_core::Result<IPMTaskInfo>;
    fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> windows_core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(&self) -> windows_core::Result<IPMLiveTileJobInfoEnumerator>;
    fn get_LiveTileJob(&self, productid: &windows_core::GUID, tileid: &windows_core::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> windows_core::Result<IPMLiveTileJobInfo>;
    fn get_ApplicationInfoExternal(&self, productid: &windows_core::GUID) -> windows_core::Result<IPMApplicationInfo>;
    fn get_FileHandlerGenericLogo(&self, filetype: &windows_core::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ApplicationInfoFromAccessClaims(&self, sysappid0: &windows_core::BSTR, sysappid1: &windows_core::BSTR) -> windows_core::Result<IPMApplicationInfo>;
    fn get_StartTileEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> windows_core::Result<()>;
    fn get_StartAppEnumeratorBlob(&self, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMEnumerationManager {}
impl IPMEnumerationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMEnumerationManager_Vtbl
    where
        Identity: IPMEnumerationManager_Impl,
    {
        unsafe extern "system" fn get_AllApplications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppappenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllApplications(this, core::mem::transmute_copy(&ppappenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllTiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptileenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllTiles(this, core::mem::transmute_copy(&pptileenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllTasks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllTasks(this, core::mem::transmute_copy(&pptaskenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensionenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllExtensions(this, core::mem::transmute_copy(&ppextensionenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllBackgroundServiceAgents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbsaenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllBackgroundServiceAgents(this, core::mem::transmute_copy(&ppbsaenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_AllBackgroundWorkers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbswenum: *mut *mut core::ffi::c_void, filter: PM_ENUM_FILTER) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_AllBackgroundWorkers(this, core::mem::transmute_copy(&ppbswenum), core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn get_ApplicationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_ApplicationInfo(this, core::mem::transmute(&productid)) {
                Ok(ok__) => {
                    ppappinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TileInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, tileid: core::mem::MaybeUninit<windows_core::BSTR>, pptileinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_TileInfo(this, core::mem::transmute(&productid), core::mem::transmute(&tileid)) {
                Ok(ok__) => {
                    pptileinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TaskInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, taskid: core::mem::MaybeUninit<windows_core::BSTR>, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_TaskInfo(this, core::mem::transmute(&productid), core::mem::transmute(&taskid)) {
                Ok(ok__) => {
                    pptaskinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TaskInfoEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, taskid: windows_core::PCWSTR, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_TaskInfoEx(this, core::mem::transmute(&productid), core::mem::transmute(&taskid)) {
                Ok(ok__) => {
                    pptaskinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_BackgroundServiceAgentInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_BackgroundServiceAgentInfo(this, core::mem::transmute_copy(&bsaid)) {
                Ok(ok__) => {
                    pptaskinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllLiveTileJobs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplivetilejobenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::AllLiveTileJobs(this) {
                Ok(ok__) => {
                    pplivetilejobenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_LiveTileJob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, tileid: core::mem::MaybeUninit<windows_core::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_LiveTileJob(this, core::mem::transmute(&productid), core::mem::transmute(&tileid), core::mem::transmute_copy(&recurrencetype)) {
                Ok(ok__) => {
                    pplivetilejobinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ApplicationInfoExternal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, productid: windows_core::GUID, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_ApplicationInfoExternal(this, core::mem::transmute(&productid)) {
                Ok(ok__) => {
                    ppappinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_FileHandlerGenericLogo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: core::mem::MaybeUninit<windows_core::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_FileHandlerGenericLogo(this, core::mem::transmute(&filetype), core::mem::transmute_copy(&logosize), core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn get_ApplicationInfoFromAccessClaims<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sysappid0: core::mem::MaybeUninit<windows_core::BSTR>, sysappid1: core::mem::MaybeUninit<windows_core::BSTR>, ppappinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMEnumerationManager_Impl::get_ApplicationInfoFromAccessClaims(this, core::mem::transmute(&sysappid0), core::mem::transmute(&sysappid1)) {
                Ok(ok__) => {
                    ppappinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_StartTileEnumeratorBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_StartTileEnumeratorBlob(this, core::mem::transmute(&filter), core::mem::transmute_copy(&pctiles), core::mem::transmute_copy(&pptileblobs)).into()
        }
        unsafe extern "system" fn get_StartAppEnumeratorBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> windows_core::HRESULT
        where
            Identity: IPMEnumerationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMEnumerationManager_Impl::get_StartAppEnumeratorBlob(this, core::mem::transmute(&filter), core::mem::transmute_copy(&pcapps), core::mem::transmute_copy(&ppappblobs)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllApplications: get_AllApplications::<Identity, OFFSET>,
            get_AllTiles: get_AllTiles::<Identity, OFFSET>,
            get_AllTasks: get_AllTasks::<Identity, OFFSET>,
            get_AllExtensions: get_AllExtensions::<Identity, OFFSET>,
            get_AllBackgroundServiceAgents: get_AllBackgroundServiceAgents::<Identity, OFFSET>,
            get_AllBackgroundWorkers: get_AllBackgroundWorkers::<Identity, OFFSET>,
            get_ApplicationInfo: get_ApplicationInfo::<Identity, OFFSET>,
            get_TileInfo: get_TileInfo::<Identity, OFFSET>,
            get_TaskInfo: get_TaskInfo::<Identity, OFFSET>,
            get_TaskInfoEx: get_TaskInfoEx::<Identity, OFFSET>,
            get_BackgroundServiceAgentInfo: get_BackgroundServiceAgentInfo::<Identity, OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Identity, OFFSET>,
            get_LiveTileJob: get_LiveTileJob::<Identity, OFFSET>,
            get_ApplicationInfoExternal: get_ApplicationInfoExternal::<Identity, OFFSET>,
            get_FileHandlerGenericLogo: get_FileHandlerGenericLogo::<Identity, OFFSET>,
            get_ApplicationInfoFromAccessClaims: get_ApplicationInfoFromAccessClaims::<Identity, OFFSET>,
            get_StartTileEnumeratorBlob: get_StartTileEnumeratorBlob::<Identity, OFFSET>,
            get_StartAppEnumeratorBlob: get_StartAppEnumeratorBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMEnumerationManager as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: Sized {
    fn SupportsUpdates(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMExtensionCachedFileUpdaterInfo {}
impl IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionCachedFileUpdaterInfo_Vtbl
    where
        Identity: IPMExtensionCachedFileUpdaterInfo_Impl,
    {
        unsafe extern "system" fn SupportsUpdates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMExtensionCachedFileUpdaterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionCachedFileUpdaterInfo_Impl::SupportsUpdates(this) {
                Ok(ok__) => {
                    psupportsupdates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SupportsUpdates: SupportsUpdates::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionCachedFileUpdaterInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionContractInfo_Impl: Sized {
    fn get_InvocationInfo(&self, paumid: *mut windows_core::BSTR, pargs: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMExtensionContractInfo {}
impl IPMExtensionContractInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionContractInfo_Vtbl
    where
        Identity: IPMExtensionContractInfo_Impl,
    {
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paumid: *mut core::mem::MaybeUninit<windows_core::BSTR>, pargs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionContractInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionContractInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&paumid), core::mem::transmute_copy(&pargs)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionContractInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionFileExtensionInfo_Impl: Sized {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self, pdisplayname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ContentType(&self, filetype: &windows_core::BSTR, pcontenttype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_FileType(&self, contenttype: &windows_core::BSTR, pfiletype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMExtensionFileExtensionInfo {}
impl IPMExtensionFileExtensionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionFileExtensionInfo_Vtbl
    where
        Identity: IPMExtensionFileExtensionInfo_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::DisplayName(this, core::mem::transmute_copy(&pdisplayname)).into()
        }
        unsafe extern "system" fn get_Logo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::get_Logo(this, core::mem::transmute_copy(&logosize), core::mem::transmute_copy(&plogo)).into()
        }
        unsafe extern "system" fn get_ContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: core::mem::MaybeUninit<windows_core::BSTR>, pcontenttype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::get_ContentType(this, core::mem::transmute(&filetype), core::mem::transmute_copy(&pcontenttype)).into()
        }
        unsafe extern "system" fn get_FileType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: core::mem::MaybeUninit<windows_core::BSTR>, pfiletype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::get_FileType(this, core::mem::transmute(&contenttype), core::mem::transmute_copy(&pfiletype)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn get_AllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileExtensionInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pcbtypes), core::mem::transmute_copy(&pptypes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            get_Logo: get_Logo::<Identity, OFFSET>,
            get_ContentType: get_ContentType::<Identity, OFFSET>,
            get_FileType: get_FileType::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileExtensionInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionFileOpenPickerInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMExtensionFileOpenPickerInfo {}
impl IPMExtensionFileOpenPickerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionFileOpenPickerInfo_Vtbl
    where
        Identity: IPMExtensionFileOpenPickerInfo_Impl,
    {
        unsafe extern "system" fn get_AllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileOpenPickerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileOpenPickerInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileOpenPickerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionFileOpenPickerInfo_Impl::SupportsAllFileTypes(this) {
                Ok(ok__) => {
                    psupportsalltypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileOpenPickerInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionFileSavePickerInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMExtensionFileSavePickerInfo {}
impl IPMExtensionFileSavePickerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionFileSavePickerInfo_Vtbl
    where
        Identity: IPMExtensionFileSavePickerInfo_Impl,
    {
        unsafe extern "system" fn get_AllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileSavePickerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionFileSavePickerInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMExtensionFileSavePickerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionFileSavePickerInfo_Impl::SupportsAllFileTypes(this) {
                Ok(ok__) => {
                    psupportsalltypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionFileSavePickerInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionInfo_Impl: Sized {
    fn SupplierPID(&self) -> windows_core::Result<windows_core::GUID>;
    fn SupplierTaskID(&self, psuppliertid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IconPath(&self, piconpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ExtraFile(&self, pfilepath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMExtensionInfo {}
impl IPMExtensionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionInfo_Vtbl
    where
        Identity: IPMExtensionInfo_Impl,
    {
        unsafe extern "system" fn SupplierPID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupplierpid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionInfo_Impl::SupplierPID(this) {
                Ok(ok__) => {
                    psupplierpid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupplierTaskID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psuppliertid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionInfo_Impl::SupplierTaskID(this, core::mem::transmute_copy(&psuppliertid)).into()
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionInfo_Impl::Title(this, core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn IconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piconpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionInfo_Impl::IconPath(this, core::mem::transmute_copy(&piconpath)).into()
        }
        unsafe extern "system" fn ExtraFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionInfo_Impl::ExtraFile(this, core::mem::transmute_copy(&pfilepath)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupplierPID: SupplierPID::<Identity, OFFSET>,
            SupplierTaskID: SupplierTaskID::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            IconPath: IconPath::<Identity, OFFSET>,
            ExtraFile: ExtraFile::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMExtensionInfo>;
}
impl windows_core::RuntimeName for IPMExtensionInfoEnumerator {}
impl IPMExtensionInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionInfoEnumerator_Vtbl
    where
        Identity: IPMExtensionInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMExtensionInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    ppextensioninfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionProtocolInfo_Impl: Sized {
    fn Protocol(&self, pprotocol: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMExtensionProtocolInfo {}
impl IPMExtensionProtocolInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionProtocolInfo_Vtbl
    where
        Identity: IPMExtensionProtocolInfo_Impl,
    {
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionProtocolInfo_Impl::Protocol(this, core::mem::transmute_copy(&pprotocol)).into()
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMExtensionProtocolInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionProtocolInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Protocol: Protocol::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionProtocolInfo as windows_core::Interface>::IID
    }
}
pub trait IPMExtensionShareTargetInfo_Impl: Sized {
    fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SupportsAllFileTypes(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMExtensionShareTargetInfo {}
impl IPMExtensionShareTargetInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMExtensionShareTargetInfo_Vtbl
    where
        Identity: IPMExtensionShareTargetInfo_Impl,
    {
        unsafe extern "system" fn get_AllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPMExtensionShareTargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionShareTargetInfo_Impl::get_AllFileTypes(this, core::mem::transmute_copy(&pctypes), core::mem::transmute_copy(&pptypes)).into()
        }
        unsafe extern "system" fn get_AllDataFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut windows_core::BSTR) -> windows_core::HRESULT
        where
            Identity: IPMExtensionShareTargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMExtensionShareTargetInfo_Impl::get_AllDataFormats(this, core::mem::transmute_copy(&pcdataformats), core::mem::transmute_copy(&ppdataformats)).into()
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMExtensionShareTargetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMExtensionShareTargetInfo_Impl::SupportsAllFileTypes(this) {
                Ok(ok__) => {
                    psupportsalltypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_AllFileTypes: get_AllFileTypes::<Identity, OFFSET>,
            get_AllDataFormats: get_AllDataFormats::<Identity, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMExtensionShareTargetInfo as windows_core::Interface>::IID
    }
}
pub trait IPMLiveTileJobInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NextSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_NextSchedule(&self, ftnextschedule: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn StartSchedule(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_StartSchedule(&self, ftstartschedule: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn IntervalDuration(&self) -> windows_core::Result<u32>;
    fn set_IntervalDuration(&self, ulintervalduration: u32) -> windows_core::Result<()>;
    fn RunForever(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn set_RunForever(&self, frunforever: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn MaxRunCount(&self) -> windows_core::Result<u32>;
    fn set_MaxRunCount(&self, ulmaxruncount: u32) -> windows_core::Result<()>;
    fn RunCount(&self) -> windows_core::Result<u32>;
    fn set_RunCount(&self, ulruncount: u32) -> windows_core::Result<()>;
    fn RecurrenceType(&self) -> windows_core::Result<u32>;
    fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> windows_core::Result<()>;
    fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> windows_core::Result<()>;
    fn set_TileXML(&self, ptilexml: *const u8, cbtilexml: u32) -> windows_core::Result<()>;
    fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> windows_core::Result<()>;
    fn set_UrlXML(&self, purlxml: *const u8, cburlxml: u32) -> windows_core::Result<()>;
    fn AttemptCount(&self) -> windows_core::Result<u32>;
    fn set_AttemptCount(&self, ulattemptcount: u32) -> windows_core::Result<()>;
    fn DownloadState(&self) -> windows_core::Result<u32>;
    fn set_DownloadState(&self, uldownloadstate: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMLiveTileJobInfo {}
impl IPMLiveTileJobInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMLiveTileJobInfo_Vtbl
    where
        Identity: IPMLiveTileJobInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptileid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::TileID(this, core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn NextSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::NextSchedule(this) {
                Ok(ok__) => {
                    pnextschedule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_NextSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_NextSchedule(this, core::mem::transmute(&ftnextschedule)).into()
        }
        unsafe extern "system" fn StartSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::StartSchedule(this) {
                Ok(ok__) => {
                    pstartschedule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_StartSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_StartSchedule(this, core::mem::transmute(&ftstartschedule)).into()
        }
        unsafe extern "system" fn IntervalDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintervalduration: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::IntervalDuration(this) {
                Ok(ok__) => {
                    pintervalduration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IntervalDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulintervalduration: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_IntervalDuration(this, core::mem::transmute_copy(&ulintervalduration)).into()
        }
        unsafe extern "system" fn RunForever<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::RunForever(this) {
                Ok(ok__) => {
                    isrunforever.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunForever<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_RunForever(this, core::mem::transmute_copy(&frunforever)).into()
        }
        unsafe extern "system" fn MaxRunCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxruncount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::MaxRunCount(this) {
                Ok(ok__) => {
                    pmaxruncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_MaxRunCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxruncount: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_MaxRunCount(this, core::mem::transmute_copy(&ulmaxruncount)).into()
        }
        unsafe extern "system" fn RunCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruncount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::RunCount(this) {
                Ok(ok__) => {
                    pruncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RunCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulruncount: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_RunCount(this, core::mem::transmute_copy(&ulruncount)).into()
        }
        unsafe extern "system" fn RecurrenceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precurrencetype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::RecurrenceType(this) {
                Ok(ok__) => {
                    precurrencetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_RecurrenceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecurrencetype: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_RecurrenceType(this, core::mem::transmute_copy(&ulrecurrencetype)).into()
        }
        unsafe extern "system" fn get_TileXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::get_TileXML(this, core::mem::transmute_copy(&ptilexml), core::mem::transmute_copy(&pcbtilexml)).into()
        }
        unsafe extern "system" fn set_TileXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_TileXML(this, core::mem::transmute_copy(&ptilexml), core::mem::transmute_copy(&cbtilexml)).into()
        }
        unsafe extern "system" fn get_UrlXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::get_UrlXML(this, core::mem::transmute_copy(&purlxml), core::mem::transmute_copy(&pcburlxml)).into()
        }
        unsafe extern "system" fn set_UrlXML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_UrlXML(this, core::mem::transmute_copy(&purlxml), core::mem::transmute_copy(&cburlxml)).into()
        }
        unsafe extern "system" fn AttemptCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattemptcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::AttemptCount(this) {
                Ok(ok__) => {
                    pattemptcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_AttemptCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulattemptcount: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_AttemptCount(this, core::mem::transmute_copy(&ulattemptcount)).into()
        }
        unsafe extern "system" fn DownloadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdownloadstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfo_Impl::DownloadState(this) {
                Ok(ok__) => {
                    pdownloadstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_DownloadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uldownloadstate: u32) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMLiveTileJobInfo_Impl::set_DownloadState(this, core::mem::transmute_copy(&uldownloadstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TileID: TileID::<Identity, OFFSET>,
            NextSchedule: NextSchedule::<Identity, OFFSET>,
            set_NextSchedule: set_NextSchedule::<Identity, OFFSET>,
            StartSchedule: StartSchedule::<Identity, OFFSET>,
            set_StartSchedule: set_StartSchedule::<Identity, OFFSET>,
            IntervalDuration: IntervalDuration::<Identity, OFFSET>,
            set_IntervalDuration: set_IntervalDuration::<Identity, OFFSET>,
            RunForever: RunForever::<Identity, OFFSET>,
            set_RunForever: set_RunForever::<Identity, OFFSET>,
            MaxRunCount: MaxRunCount::<Identity, OFFSET>,
            set_MaxRunCount: set_MaxRunCount::<Identity, OFFSET>,
            RunCount: RunCount::<Identity, OFFSET>,
            set_RunCount: set_RunCount::<Identity, OFFSET>,
            RecurrenceType: RecurrenceType::<Identity, OFFSET>,
            set_RecurrenceType: set_RecurrenceType::<Identity, OFFSET>,
            get_TileXML: get_TileXML::<Identity, OFFSET>,
            set_TileXML: set_TileXML::<Identity, OFFSET>,
            get_UrlXML: get_UrlXML::<Identity, OFFSET>,
            set_UrlXML: set_UrlXML::<Identity, OFFSET>,
            AttemptCount: AttemptCount::<Identity, OFFSET>,
            set_AttemptCount: set_AttemptCount::<Identity, OFFSET>,
            DownloadState: DownloadState::<Identity, OFFSET>,
            set_DownloadState: set_DownloadState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfo as windows_core::Interface>::IID
    }
}
pub trait IPMLiveTileJobInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMLiveTileJobInfo>;
}
impl windows_core::RuntimeName for IPMLiveTileJobInfoEnumerator {}
impl IPMLiveTileJobInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMLiveTileJobInfoEnumerator_Vtbl
    where
        Identity: IPMLiveTileJobInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplivetilejobinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMLiveTileJobInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMLiveTileJobInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    pplivetilejobinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMLiveTileJobInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMTaskInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn NavigationPage(&self, pnavigationpage: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskTransition(&self) -> windows_core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(&self) -> windows_core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(&self) -> windows_core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(&self) -> windows_core::Result<PM_TASK_TYPE>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ImagePath(&self, pimagepath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ImageParams(&self, pimageparams: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallRootFolder(&self, pinstallrootfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn DataRootFolder(&self, pdatarootfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSingleInstanceHost(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInteropEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ApplicationState(&self) -> windows_core::Result<PM_APPLICATION_STATE>;
    fn InstallType(&self) -> windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> windows_core::Result<()>;
    fn BitsPerPixel(&self) -> windows_core::Result<u16>;
    fn SuppressesDehydration(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsOptedForExtendedMem(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IPMTaskInfo {}
impl IPMTaskInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTaskInfo_Vtbl
    where
        Identity: IPMTaskInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn NavigationPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnavigationpage: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::NavigationPage(this, core::mem::transmute_copy(&pnavigationpage)).into()
        }
        unsafe extern "system" fn TaskTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::TaskTransition(this) {
                Ok(ok__) => {
                    ptasktransition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuntimeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::RuntimeType(this) {
                Ok(ok__) => {
                    pruntimetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::ActivationPolicy(this) {
                Ok(ok__) => {
                    pactivationpolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::TaskType(this) {
                Ok(ok__) => {
                    ptasktype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn ImagePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimagepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::ImagePath(this, core::mem::transmute_copy(&pimagepath)).into()
        }
        unsafe extern "system" fn ImageParams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageparams: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::ImageParams(this, core::mem::transmute_copy(&pimageparams)).into()
        }
        unsafe extern "system" fn InstallRootFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstallrootfolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::InstallRootFolder(this, core::mem::transmute_copy(&pinstallrootfolder)).into()
        }
        unsafe extern "system" fn DataRootFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatarootfolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::DataRootFolder(this, core::mem::transmute_copy(&pdatarootfolder)).into()
        }
        unsafe extern "system" fn IsSingleInstanceHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::IsSingleInstanceHost(this) {
                Ok(ok__) => {
                    pissingleinstancehost.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInteropEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::IsInteropEnabled(this) {
                Ok(ok__) => {
                    pisinteropenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::ApplicationState(this) {
                Ok(ok__) => {
                    papplicationstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::InstallType(this) {
                Ok(ok__) => {
                    pinstalltype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::get_Version(this, core::mem::transmute_copy(&ptargetmajorversion), core::mem::transmute_copy(&ptargetminorversion)).into()
        }
        unsafe extern "system" fn BitsPerPixel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitsperpixel: *mut u16) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::BitsPerPixel(this) {
                Ok(ok__) => {
                    pbitsperpixel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressesDehydration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::SuppressesDehydration(this) {
                Ok(ok__) => {
                    psuppressesdehydration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbackgroundexecutionabilities: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTaskInfo_Impl::BackgroundExecutionAbilities(this, core::mem::transmute_copy(&pbackgroundexecutionabilities)).into()
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfo_Impl::IsOptedForExtendedMem(this) {
                Ok(ok__) => {
                    pisoptedin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            NavigationPage: NavigationPage::<Identity, OFFSET>,
            TaskTransition: TaskTransition::<Identity, OFFSET>,
            RuntimeType: RuntimeType::<Identity, OFFSET>,
            ActivationPolicy: ActivationPolicy::<Identity, OFFSET>,
            TaskType: TaskType::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            ImagePath: ImagePath::<Identity, OFFSET>,
            ImageParams: ImageParams::<Identity, OFFSET>,
            InstallRootFolder: InstallRootFolder::<Identity, OFFSET>,
            DataRootFolder: DataRootFolder::<Identity, OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Identity, OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Identity, OFFSET>,
            ApplicationState: ApplicationState::<Identity, OFFSET>,
            InstallType: InstallType::<Identity, OFFSET>,
            get_Version: get_Version::<Identity, OFFSET>,
            BitsPerPixel: BitsPerPixel::<Identity, OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Identity, OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Identity, OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTaskInfo as windows_core::Interface>::IID
    }
}
pub trait IPMTaskInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMTaskInfo>;
}
impl windows_core::RuntimeName for IPMTaskInfoEnumerator {}
impl IPMTaskInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTaskInfoEnumerator_Vtbl
    where
        Identity: IPMTaskInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptaskinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMTaskInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTaskInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    pptaskinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTaskInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMTileInfo_Impl: Sized {
    fn ProductID(&self) -> windows_core::Result<windows_core::GUID>;
    fn TileID(&self, ptileid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateType(&self) -> windows_core::Result<TILE_TEMPLATE_TYPE>;
    fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<u32>;
    fn IsNotified(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn TaskID(&self, ptaskid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn TileType(&self) -> windows_core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn get_PropertyById(&self, propid: u32) -> windows_core::Result<IPMTilePropertyInfo>;
    fn get_InvocationInfo(&self, pimageurn: *mut windows_core::BSTR, pparameters: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn PropertyEnum(&self) -> windows_core::Result<IPMTilePropertyEnumerator>;
    fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> windows_core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> windows_core::Result<()>;
    fn set_NotifiedState(&self, notified: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> windows_core::Result<()>;
    fn set_InvocationInfo(&self, taskname: &windows_core::BSTR, taskparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> windows_core::Result<()>;
    fn IsRestoring(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsAutoRestoreDisabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsRestoring(&self, restoring: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn set_IsAutoRestoreDisabled(&self, autorestoredisabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMTileInfo {}
impl IPMTileInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTileInfo_Vtbl
    where
        Identity: IPMTileInfo_Impl,
    {
        unsafe extern "system" fn ProductID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproductid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::ProductID(this) {
                Ok(ok__) => {
                    pproductid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TileID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptileid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::TileID(this, core::mem::transmute_copy(&ptileid)).into()
        }
        unsafe extern "system" fn TemplateType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::TemplateType(this) {
                Ok(ok__) => {
                    ptemplatetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubPinnedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::get_HubPinnedState(this, core::mem::transmute_copy(&hubtype)) {
                Ok(ok__) => {
                    ppinned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::get_HubPosition(this, core::mem::transmute_copy(&hubtype)) {
                Ok(ok__) => {
                    pposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::IsNotified(this) {
                Ok(ok__) => {
                    pisnotified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::IsDefault(this) {
                Ok(ok__) => {
                    pisdefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptaskid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::TaskID(this, core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn TileType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::TileType(this) {
                Ok(ok__) => {
                    pstarttiletype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThemable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::IsThemable(this) {
                Ok(ok__) => {
                    pisthemable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_PropertyById<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: u32, pppropinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::get_PropertyById(this, core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    pppropinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimageurn: *mut core::mem::MaybeUninit<windows_core::BSTR>, pparameters: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::get_InvocationInfo(this, core::mem::transmute_copy(&pimageurn), core::mem::transmute_copy(&pparameters)).into()
        }
        unsafe extern "system" fn PropertyEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptilepropenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::PropertyEnum(this) {
                Ok(ok__) => {
                    pptilepropenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_HubTileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::get_HubTileSize(this, core::mem::transmute_copy(&hubtype)) {
                Ok(ok__) => {
                    psize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_HubPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_HubPosition(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn set_NotifiedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notified: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_NotifiedState(this, core::mem::transmute_copy(&notified)).into()
        }
        unsafe extern "system" fn set_HubPinnedState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_HubPinnedState(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&pinned)).into()
        }
        unsafe extern "system" fn set_HubTileSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_HubTileSize(this, core::mem::transmute_copy(&hubtype), core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn set_InvocationInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: core::mem::MaybeUninit<windows_core::BSTR>, taskparameters: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_InvocationInfo(this, core::mem::transmute(&taskname), core::mem::transmute(&taskparameters)).into()
        }
        unsafe extern "system" fn StartTileBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::StartTileBlob(this, core::mem::transmute_copy(&pblob)).into()
        }
        unsafe extern "system" fn IsRestoring<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::IsRestoring(this) {
                Ok(ok__) => {
                    pisrestoring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfo_Impl::IsAutoRestoreDisabled(this) {
                Ok(ok__) => {
                    pisautorestoredisabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn set_IsRestoring<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_IsRestoring(this, core::mem::transmute_copy(&restoring)).into()
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IPMTileInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTileInfo_Impl::set_IsAutoRestoreDisabled(this, core::mem::transmute_copy(&autorestoredisabled)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProductID: ProductID::<Identity, OFFSET>,
            TileID: TileID::<Identity, OFFSET>,
            TemplateType: TemplateType::<Identity, OFFSET>,
            get_HubPinnedState: get_HubPinnedState::<Identity, OFFSET>,
            get_HubPosition: get_HubPosition::<Identity, OFFSET>,
            IsNotified: IsNotified::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            TaskID: TaskID::<Identity, OFFSET>,
            TileType: TileType::<Identity, OFFSET>,
            IsThemable: IsThemable::<Identity, OFFSET>,
            get_PropertyById: get_PropertyById::<Identity, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, OFFSET>,
            PropertyEnum: PropertyEnum::<Identity, OFFSET>,
            get_HubTileSize: get_HubTileSize::<Identity, OFFSET>,
            set_HubPosition: set_HubPosition::<Identity, OFFSET>,
            set_NotifiedState: set_NotifiedState::<Identity, OFFSET>,
            set_HubPinnedState: set_HubPinnedState::<Identity, OFFSET>,
            set_HubTileSize: set_HubTileSize::<Identity, OFFSET>,
            set_InvocationInfo: set_InvocationInfo::<Identity, OFFSET>,
            StartTileBlob: StartTileBlob::<Identity, OFFSET>,
            IsRestoring: IsRestoring::<Identity, OFFSET>,
            IsAutoRestoreDisabled: IsAutoRestoreDisabled::<Identity, OFFSET>,
            set_IsRestoring: set_IsRestoring::<Identity, OFFSET>,
            set_IsAutoRestoreDisabled: set_IsAutoRestoreDisabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTileInfo as windows_core::Interface>::IID
    }
}
pub trait IPMTileInfoEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMTileInfo>;
}
impl windows_core::RuntimeName for IPMTileInfoEnumerator {}
impl IPMTileInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTileInfoEnumerator_Vtbl
    where
        Identity: IPMTileInfoEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptileinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMTileInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTileInfoEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    pptileinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTileInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMTilePropertyEnumerator_Impl: Sized {
    fn Next(&self) -> windows_core::Result<IPMTilePropertyInfo>;
}
impl windows_core::RuntimeName for IPMTilePropertyEnumerator {}
impl IPMTilePropertyEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTilePropertyEnumerator_Vtbl
    where
        Identity: IPMTilePropertyEnumerator_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPMTilePropertyEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTilePropertyEnumerator_Impl::Next(this) {
                Ok(ok__) => {
                    pppropinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTilePropertyEnumerator as windows_core::Interface>::IID
    }
}
pub trait IPMTilePropertyInfo_Impl: Sized {
    fn PropertyID(&self) -> windows_core::Result<u32>;
    fn PropertyValue(&self, ppropvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn set_Property(&self, propvalue: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPMTilePropertyInfo {}
impl IPMTilePropertyInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPMTilePropertyInfo_Vtbl
    where
        Identity: IPMTilePropertyInfo_Impl,
    {
        unsafe extern "system" fn PropertyID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPMTilePropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPMTilePropertyInfo_Impl::PropertyID(this) {
                Ok(ok__) => {
                    ppropid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTilePropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTilePropertyInfo_Impl::PropertyValue(this, core::mem::transmute_copy(&ppropvalue)).into()
        }
        unsafe extern "system" fn set_Property<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPMTilePropertyInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPMTilePropertyInfo_Impl::set_Property(this, core::mem::transmute(&propvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropertyID: PropertyID::<Identity, OFFSET>,
            PropertyValue: PropertyValue::<Identity, OFFSET>,
            set_Property: set_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPMTilePropertyInfo as windows_core::Interface>::IID
    }
}
pub trait IValidate_Impl: Sized {
    fn OpenDatabase(&self, szdatabase: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OpenCUB(&self, szcubfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CloseDatabase(&self) -> windows_core::Result<()>;
    fn CloseCUB(&self) -> windows_core::Result<()>;
    fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Validate(&self, wzices: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IValidate {}
impl IValidate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IValidate_Vtbl
    where
        Identity: IValidate_Impl,
    {
        unsafe extern "system" fn OpenDatabase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szdatabase: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::OpenDatabase(this, core::mem::transmute(&szdatabase)).into()
        }
        unsafe extern "system" fn OpenCUB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szcubfile: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::OpenCUB(this, core::mem::transmute(&szcubfile)).into()
        }
        unsafe extern "system" fn CloseDatabase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::CloseDatabase(this).into()
        }
        unsafe extern "system" fn CloseCUB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::CloseCUB(this).into()
        }
        unsafe extern "system" fn SetDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::SetDisplay(this, core::mem::transmute_copy(&pdisplayfunction), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::SetStatus(this, core::mem::transmute_copy(&pstatusfunction), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Validate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzices: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IValidate_Impl::Validate(this, core::mem::transmute(&wzices)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenDatabase: OpenDatabase::<Identity, OFFSET>,
            OpenCUB: OpenCUB::<Identity, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, OFFSET>,
            CloseCUB: CloseCUB::<Identity, OFFSET>,
            SetDisplay: SetDisplay::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValidate as windows_core::Interface>::IID
    }
}
