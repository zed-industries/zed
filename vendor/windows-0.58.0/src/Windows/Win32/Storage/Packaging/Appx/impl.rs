#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IAppxAppInstallerReader_Impl: Sized {
    fn GetXmlDom(&self) -> windows_core::Result<super::super::super::Data::Xml::MsXml::IXMLDOMDocument>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IAppxAppInstallerReader {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IAppxAppInstallerReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxAppInstallerReader_Vtbl
    where
        Identity: IAppxAppInstallerReader_Impl,
    {
        unsafe extern "system" fn GetXmlDom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxAppInstallerReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxAppInstallerReader_Impl::GetXmlDom(this) {
                Ok(ok__) => {
                    dom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXmlDom: GetXmlDom::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxAppInstallerReader as windows_core::Interface>::IID
    }
}
pub trait IAppxBlockMapBlock_Impl: Sized {
    fn GetHash(&self, buffersize: *mut u32) -> windows_core::Result<*mut u8>;
    fn GetCompressedSize(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAppxBlockMapBlock {}
impl IAppxBlockMapBlock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBlockMapBlock_Vtbl
    where
        Identity: IAppxBlockMapBlock_Impl,
    {
        unsafe extern "system" fn GetHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapBlock_Impl::GetHash(this, core::mem::transmute_copy(&buffersize)) {
                Ok(ok__) => {
                    buffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressedSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapBlock_Impl::GetCompressedSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHash: GetHash::<Identity, OFFSET>,
            GetCompressedSize: GetCompressedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapBlock as windows_core::Interface>::IID
    }
}
pub trait IAppxBlockMapBlocksEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapBlock>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBlockMapBlocksEnumerator {}
impl IAppxBlockMapBlocksEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBlockMapBlocksEnumerator_Vtbl
    where
        Identity: IAppxBlockMapBlocksEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapBlocksEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapBlocksEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    block.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapBlocksEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapBlocksEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapBlocksEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapBlocksEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapBlocksEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapFile_Impl: Sized {
    fn GetBlocks(&self) -> windows_core::Result<IAppxBlockMapBlocksEnumerator>;
    fn GetLocalFileHeaderSize(&self) -> windows_core::Result<u32>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUncompressedSize(&self) -> windows_core::Result<u64>;
    fn ValidateFileHash(&self, filestream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBlockMapFile {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBlockMapFile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBlockMapFile_Vtbl
    where
        Identity: IAppxBlockMapFile_Impl,
    {
        unsafe extern "system" fn GetBlocks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFile_Impl::GetBlocks(this) {
                Ok(ok__) => {
                    blocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfhsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFile_Impl::GetLocalFileHeaderSize(this) {
                Ok(ok__) => {
                    lfhsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFile_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUncompressedSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFile_Impl::GetUncompressedSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateFileHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filestream: *mut core::ffi::c_void, isvalid: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFile_Impl::ValidateFileHash(this, windows_core::from_raw_borrowed(&filestream)) {
                Ok(ok__) => {
                    isvalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlocks: GetBlocks::<Identity, OFFSET>,
            GetLocalFileHeaderSize: GetLocalFileHeaderSize::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetUncompressedSize: GetUncompressedSize::<Identity, OFFSET>,
            ValidateFileHash: ValidateFileHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapFile as windows_core::Interface>::IID
    }
}
pub trait IAppxBlockMapFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapFile>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBlockMapFilesEnumerator {}
impl IAppxBlockMapFilesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBlockMapFilesEnumerator_Vtbl
    where
        Identity: IAppxBlockMapFilesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFilesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFilesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapFilesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapFilesEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapReader_Impl: Sized {
    fn GetFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxBlockMapFile>;
    fn GetFiles(&self) -> windows_core::Result<IAppxBlockMapFilesEnumerator>;
    fn GetHashMethod(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBlockMapReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBlockMapReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBlockMapReader_Vtbl
    where
        Identity: IAppxBlockMapReader_Impl,
    {
        unsafe extern "system" fn GetFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapReader_Impl::GetFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapReader_Impl::GetFiles(this) {
                Ok(ok__) => {
                    enumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapReader_Impl::GetHashMethod(this) {
                Ok(ok__) => {
                    hashmethod.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBlockMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBlockMapReader_Impl::GetStream(this) {
                Ok(ok__) => {
                    blockmapstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFile: GetFile::<Identity, OFFSET>,
            GetFiles: GetFiles::<Identity, OFFSET>,
            GetHashMethod: GetHashMethod::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory_Impl: Sized {
    fn CreateBundleWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, bundleversion: u64) -> windows_core::Result<IAppxBundleWriter>;
    fn CreateBundleReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBundleReader>;
    fn CreateBundleManifestReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBundleManifestReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleFactory_Vtbl
    where
        Identity: IAppxBundleFactory_Impl,
    {
        unsafe extern "system" fn CreateBundleWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleFactory_Impl::CreateBundleWriter(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&bundleversion)) {
                Ok(ok__) => {
                    bundlewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleFactory_Impl::CreateBundleReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    bundlereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBundleManifestReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleFactory_Impl::CreateBundleManifestReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    manifestreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBundleWriter: CreateBundleWriter::<Identity, OFFSET>,
            CreateBundleReader: CreateBundleReader::<Identity, OFFSET>,
            CreateBundleManifestReader: CreateBundleManifestReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory2_Impl: Sized {
    fn CreateBundleReader2(&self, inputstream: Option<&super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleFactory2_Vtbl
    where
        Identity: IAppxBundleFactory2_Impl,
    {
        unsafe extern "system" fn CreateBundleReader2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleFactory2_Impl::CreateBundleReader2(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    bundlereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateBundleReader2: CreateBundleReader2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleFactory2 as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestOptionalBundleInfo_Impl: Sized {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
}
impl windows_core::RuntimeName for IAppxBundleManifestOptionalBundleInfo {}
impl IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfo_Vtbl
    where
        Identity: IAppxBundleManifestOptionalBundleInfo_Impl,
    {
        unsafe extern "system" fn GetPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfo_Impl::GetPackageId(this) {
                Ok(ok__) => {
                    packageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfo_Impl::GetFileName(this) {
                Ok(ok__) => {
                    filename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfoitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfo_Impl::GetPackageInfoItems(this) {
                Ok(ok__) => {
                    packageinfoitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfo as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestOptionalBundleInfoEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfo>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBundleManifestOptionalBundleInfoEnumerator {}
impl IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl
    where
        Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalbundle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    optionalbundle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfoEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfo_Impl: Sized {
    fn GetPackageType(&self) -> windows_core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>;
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOffset(&self) -> windows_core::Result<u64>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo {}
impl IAppxBundleManifestPackageInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo_Vtbl
    where
        Identity: IAppxBundleManifestPackageInfo_Impl,
    {
        unsafe extern "system" fn GetPackageType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetPackageType(this) {
                Ok(ok__) => {
                    packagetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetPackageId(this) {
                Ok(ok__) => {
                    packageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetFileName(this) {
                Ok(ok__) => {
                    filename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetOffset(this) {
                Ok(ok__) => {
                    offset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo_Impl::GetResources(this) {
                Ok(ok__) => {
                    resources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageType: GetPackageType::<Identity, OFFSET>,
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetOffset: GetOffset::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetResources: GetResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfo2_Impl: Sized {
    fn GetIsPackageReference(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsDefaultApplicablePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo2 {}
impl IAppxBundleManifestPackageInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo2_Vtbl
    where
        Identity: IAppxBundleManifestPackageInfo2_Impl,
    {
        unsafe extern "system" fn GetIsPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo2_Impl::GetIsPackageReference(this) {
                Ok(ok__) => {
                    ispackagereference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo2_Impl::GetIsNonQualifiedResourcePackage(this) {
                Ok(ok__) => {
                    isnonqualifiedresourcepackage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo2_Impl::GetIsDefaultApplicablePackage(this) {
                Ok(ok__) => {
                    isdefaultapplicablepackage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsPackageReference: GetIsPackageReference::<Identity, OFFSET>,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, OFFSET>,
            GetIsDefaultApplicablePackage: GetIsDefaultApplicablePackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo2 as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfo3_Impl: Sized {
    fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo3 {}
impl IAppxBundleManifestPackageInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo3_Vtbl
    where
        Identity: IAppxBundleManifestPackageInfo3_Impl,
    {
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamilies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo3_Impl::GetTargetDeviceFamilies(this) {
                Ok(ok__) => {
                    targetdevicefamilies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo3 as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfo4_Impl: Sized {
    fn GetIsStub(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo4 {}
impl IAppxBundleManifestPackageInfo4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfo4_Vtbl
    where
        Identity: IAppxBundleManifestPackageInfo4_Impl,
    {
        unsafe extern "system" fn GetIsStub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfo4_Impl::GetIsStub(this) {
                Ok(ok__) => {
                    isstub.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsStub: GetIsStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo4 as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestPackageInfoEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestPackageInfo>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfoEnumerator {}
impl IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestPackageInfoEnumerator_Vtbl
    where
        Identity: IAppxBundleManifestPackageInfoEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfoEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    packageinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfoEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestPackageInfoEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestPackageInfoEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfoEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleManifestReader_Impl: Sized {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleManifestReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleManifestReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestReader_Vtbl
    where
        Identity: IAppxBundleManifestReader_Impl,
    {
        unsafe extern "system" fn GetPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestReader_Impl::GetPackageId(this) {
                Ok(ok__) => {
                    packageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfoitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestReader_Impl::GetPackageInfoItems(this) {
                Ok(ok__) => {
                    packageinfoitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifeststream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestReader_Impl::GetStream(this) {
                Ok(ok__) => {
                    manifeststream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleManifestReader2_Impl: Sized {
    fn GetOptionalBundles(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator>;
}
impl windows_core::RuntimeName for IAppxBundleManifestReader2 {}
impl IAppxBundleManifestReader2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleManifestReader2_Vtbl
    where
        Identity: IAppxBundleManifestReader2_Impl,
    {
        unsafe extern "system" fn GetOptionalBundles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalbundles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleManifestReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleManifestReader2_Impl::GetOptionalBundles(this) {
                Ok(ok__) => {
                    optionalbundles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOptionalBundles: GetOptionalBundles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader2 as windows_core::Interface>::IID
    }
}
pub trait IAppxBundleReader_Impl: Sized {
    fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile>;
    fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader>;
    fn GetManifest(&self) -> windows_core::Result<IAppxBundleManifestReader>;
    fn GetPayloadPackages(&self) -> windows_core::Result<IAppxFilesEnumerator>;
    fn GetPayloadPackage(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxFile>;
}
impl windows_core::RuntimeName for IAppxBundleReader {}
impl IAppxBundleReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleReader_Vtbl
    where
        Identity: IAppxBundleReader_Impl,
    {
        unsafe extern "system" fn GetFootprintFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleReader_Impl::GetFootprintFile(this, core::mem::transmute_copy(&filetype)) {
                Ok(ok__) => {
                    footprintfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlockMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleReader_Impl::GetBlockMap(this) {
                Ok(ok__) => {
                    blockmapreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleReader_Impl::GetManifest(this) {
                Ok(ok__) => {
                    manifestreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, payloadpackages: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleReader_Impl::GetPayloadPackages(this) {
                Ok(ok__) => {
                    payloadpackages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, payloadpackage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxBundleReader_Impl::GetPayloadPackage(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    payloadpackage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFootprintFile: GetFootprintFile::<Identity, OFFSET>,
            GetBlockMap: GetBlockMap::<Identity, OFFSET>,
            GetManifest: GetManifest::<Identity, OFFSET>,
            GetPayloadPackages: GetPayloadPackages::<Identity, OFFSET>,
            GetPayloadPackage: GetPayloadPackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter_Impl: Sized {
    fn AddPayloadPackage(&self, filename: &windows_core::PCWSTR, packagestream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleWriter_Vtbl
    where
        Identity: IAppxBundleWriter_Impl,
    {
        unsafe extern "system" fn AddPayloadPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter_Impl::AddPayloadPackage(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleWriter2_Vtbl
    where
        Identity: IAppxBundleWriter2_Impl,
    {
        unsafe extern "system" fn AddExternalPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter2_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter3_Impl: Sized {
    fn AddPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self, hashmethodstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleWriter3_Vtbl
    where
        Identity: IAppxBundleWriter3_Impl,
    {
        unsafe extern "system" fn AddPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter3_Impl::AddPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashmethodstring: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter3_Impl::Close(this, core::mem::transmute(&hashmethodstring)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPackageReference: AddPackageReference::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter4_Impl: Sized {
    fn AddPayloadPackage(&self, filename: &windows_core::PCWSTR, packagestream: Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter4 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxBundleWriter4_Vtbl
    where
        Identity: IAppxBundleWriter4_Impl,
    {
        unsafe extern "system" fn AddPayloadPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter4_Impl::AddPayloadPackage(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&packagestream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter4_Impl::AddPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxBundleWriter4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxBundleWriter4_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, OFFSET>,
            AddPackageReference: AddPackageReference::<Identity, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter4 as windows_core::Interface>::IID
    }
}
pub trait IAppxContentGroup_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFiles(&self) -> windows_core::Result<IAppxContentGroupFilesEnumerator>;
}
impl windows_core::RuntimeName for IAppxContentGroup {}
impl IAppxContentGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxContentGroup_Vtbl
    where
        Identity: IAppxContentGroup_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroup_Impl::GetName(this) {
                Ok(ok__) => {
                    groupname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroup_Impl::GetFiles(this) {
                Ok(ok__) => {
                    enumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetFiles: GetFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroup as windows_core::Interface>::IID
    }
}
pub trait IAppxContentGroupFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxContentGroupFilesEnumerator {}
impl IAppxContentGroupFilesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxContentGroupFilesEnumerator_Vtbl
    where
        Identity: IAppxContentGroupFilesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupFilesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupFilesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupFilesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupFilesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl windows_core::RuntimeName for IAppxContentGroupMapReader {}
impl IAppxContentGroupMapReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxContentGroupMapReader_Vtbl
    where
        Identity: IAppxContentGroupMapReader_Impl,
    {
        unsafe extern "system" fn GetRequiredGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupMapReader_Impl::GetRequiredGroup(this) {
                Ok(ok__) => {
                    requiredgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticgroupsenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupMapReader_Impl::GetAutomaticGroups(this) {
                Ok(ok__) => {
                    automaticgroupsenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupMapReader as windows_core::Interface>::IID
    }
}
pub trait IAppxContentGroupMapWriter_Impl: Sized {
    fn AddAutomaticGroup(&self, groupname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddAutomaticFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppxContentGroupMapWriter {}
impl IAppxContentGroupMapWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxContentGroupMapWriter_Vtbl
    where
        Identity: IAppxContentGroupMapWriter_Impl,
    {
        unsafe extern "system" fn AddAutomaticGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupMapWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxContentGroupMapWriter_Impl::AddAutomaticGroup(this, core::mem::transmute(&groupname)).into()
        }
        unsafe extern "system" fn AddAutomaticFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupMapWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxContentGroupMapWriter_Impl::AddAutomaticFile(this, core::mem::transmute(&filename)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupMapWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxContentGroupMapWriter_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAutomaticGroup: AddAutomaticGroup::<Identity, OFFSET>,
            AddAutomaticFile: AddAutomaticFile::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupMapWriter as windows_core::Interface>::IID
    }
}
pub trait IAppxContentGroupsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxContentGroupsEnumerator {}
impl IAppxContentGroupsEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxContentGroupsEnumerator_Vtbl
    where
        Identity: IAppxContentGroupsEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupsEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupsEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxContentGroupsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxContentGroupsEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupsEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxDigestProvider_Impl: Sized {
    fn GetDigest(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxDigestProvider {}
impl IAppxDigestProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxDigestProvider_Vtbl
    where
        Identity: IAppxDigestProvider_Impl,
    {
        unsafe extern "system" fn GetDigest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, digest: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxDigestProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxDigestProvider_Impl::GetDigest(this) {
                Ok(ok__) => {
                    digest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDigest: GetDigest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxDigestProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter_Impl: Sized {
    fn AddPayloadPackageEncrypted(&self, filename: &windows_core::PCWSTR, packagestream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter_Vtbl
    where
        Identity: IAppxEncryptedBundleWriter_Impl,
    {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedBundleWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedBundleWriter_Impl::AddPayloadPackageEncrypted(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&packagestream)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedBundleWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedBundleWriter_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter2_Impl: Sized {
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter2_Vtbl
    where
        Identity: IAppxEncryptedBundleWriter2_Impl,
    {
        unsafe extern "system" fn AddExternalPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedBundleWriter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedBundleWriter2_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter3_Impl: Sized {
    fn AddPayloadPackageEncrypted(&self, filename: &windows_core::PCWSTR, packagestream: Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptedBundleWriter3_Vtbl
    where
        Identity: IAppxEncryptedBundleWriter3_Impl,
    {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedBundleWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedBundleWriter3_Impl::AddPayloadPackageEncrypted(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&packagestream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedBundleWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedBundleWriter3_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter_Impl: Sized {
    fn AddPayloadFileEncrypted(&self, filename: &windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedPackageWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptedPackageWriter_Vtbl
    where
        Identity: IAppxEncryptedPackageWriter_Impl,
    {
        unsafe extern "system" fn AddPayloadFileEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedPackageWriter_Impl::AddPayloadFileEncrypted(this, core::mem::transmute(&filename), core::mem::transmute_copy(&compressionoption), windows_core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedPackageWriter_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFileEncrypted: AddPayloadFileEncrypted::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter2_Impl: Sized {
    fn AddPayloadFilesEncrypted(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedPackageWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptedPackageWriter2_Vtbl
    where
        Identity: IAppxEncryptedPackageWriter2_Impl,
    {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptedPackageWriter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptedPackageWriter2_Impl::AddPayloadFilesEncrypted(this, core::mem::transmute_copy(&filecount), core::mem::transmute_copy(&payloadfiles), core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory_Impl: Sized {
    fn EncryptPackage(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn DecryptPackage(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, manifeststream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
    fn CreateEncryptedPackageReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxPackageReader>;
    fn EncryptBundle(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn DecryptBundle(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>;
    fn CreateEncryptedBundleReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptionFactory_Vtbl
    where
        Identity: IAppxEncryptionFactory_Impl,
    {
        unsafe extern "system" fn EncryptPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory_Impl::EncryptPackage(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory_Impl::DecryptPackage(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory_Impl::CreateEncryptedPackageWriter(this, windows_core::from_raw_borrowed(&outputstream), windows_core::from_raw_borrowed(&manifeststream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory_Impl::CreateEncryptedPackageReader(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&keyinfo)) {
                Ok(ok__) => {
                    packagereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory_Impl::EncryptBundle(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn DecryptBundle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory_Impl::DecryptBundle(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory_Impl::CreateEncryptedBundleWriter(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&bundleversion), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                Ok(ok__) => {
                    bundlewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory_Impl::CreateEncryptedBundleReader(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&keyinfo)) {
                Ok(ok__) => {
                    bundlereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, OFFSET>,
            DecryptPackage: DecryptPackage::<Identity, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET>,
            CreateEncryptedPackageReader: CreateEncryptedPackageReader::<Identity, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, OFFSET>,
            DecryptBundle: DecryptBundle::<Identity, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, OFFSET>,
            CreateEncryptedBundleReader: CreateEncryptedBundleReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory2_Impl: Sized {
    fn CreateEncryptedPackageWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, manifeststream: Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptionFactory2_Vtbl
    where
        Identity: IAppxEncryptionFactory2_Impl,
    {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, contentgroupmapstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory2_Impl::CreateEncryptedPackageWriter(this, windows_core::from_raw_borrowed(&outputstream), windows_core::from_raw_borrowed(&manifeststream), windows_core::from_raw_borrowed(&contentgroupmapstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory3_Impl: Sized {
    fn EncryptPackage(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, manifeststream: Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
    fn EncryptBundle(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptionFactory3_Vtbl
    where
        Identity: IAppxEncryptionFactory3_Impl,
    {
        unsafe extern "system" fn EncryptPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory3_Impl::EncryptPackage(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, contentgroupmapstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory3_Impl::CreateEncryptedPackageWriter(this, windows_core::from_raw_borrowed(&outputstream), windows_core::from_raw_borrowed(&manifeststream), windows_core::from_raw_borrowed(&contentgroupmapstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory3_Impl::EncryptBundle(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory3_Impl::CreateEncryptedBundleWriter(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&bundleversion), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                Ok(ok__) => {
                    bundlewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory4_Impl: Sized {
    fn EncryptPackage(&self, inputstream: Option<&super::super::super::System::Com::IStream>, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory4 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptionFactory4_Vtbl
    where
        Identity: IAppxEncryptionFactory4_Impl,
    {
        unsafe extern "system" fn EncryptPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxEncryptionFactory4_Impl::EncryptPackage(this, windows_core::from_raw_borrowed(&inputstream), windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles), core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EncryptPackage: EncryptPackage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory5_Impl: Sized {
    fn CreateEncryptedPackageReader2(&self, inputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxPackageReader>;
    fn CreateEncryptedBundleReader2(&self, inputstream: Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory5 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxEncryptionFactory5_Vtbl
    where
        Identity: IAppxEncryptionFactory5_Impl,
    {
        unsafe extern "system" fn CreateEncryptedPackageReader2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: windows_core::PCWSTR, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory5_Impl::CreateEncryptedPackageReader2(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&keyinfo), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    packagereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: windows_core::PCWSTR, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxEncryptionFactory5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxEncryptionFactory5_Impl::CreateEncryptedBundleReader2(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute_copy(&keyinfo), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    bundlereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEncryptedPackageReader2: CreateEncryptedPackageReader2::<Identity, OFFSET>,
            CreateEncryptedBundleReader2: CreateEncryptedBundleReader2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory_Impl: Sized {
    fn CreatePackageWriter(&self, outputstream: Option<&super::super::super::System::Com::IStream>, settings: *const APPX_PACKAGE_SETTINGS) -> windows_core::Result<IAppxPackageWriter>;
    fn CreatePackageReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxManifestReader>;
    fn CreateBlockMapReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBlockMapReader>;
    fn CreateValidatedBlockMapReader(&self, blockmapstream: Option<&super::super::super::System::Com::IStream>, signaturefilename: &windows_core::PCWSTR) -> windows_core::Result<IAppxBlockMapReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxFactory_Vtbl
    where
        Identity: IAppxFactory_Impl,
    {
        unsafe extern "system" fn CreatePackageWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory_Impl::CreatePackageWriter(this, windows_core::from_raw_borrowed(&outputstream), core::mem::transmute_copy(&settings)) {
                Ok(ok__) => {
                    packagewriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePackageReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory_Impl::CreatePackageReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    packagereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManifestReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory_Impl::CreateManifestReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    manifestreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlockMapReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory_Impl::CreateBlockMapReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    blockmapreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapstream: *mut core::ffi::c_void, signaturefilename: windows_core::PCWSTR, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory_Impl::CreateValidatedBlockMapReader(this, windows_core::from_raw_borrowed(&blockmapstream), core::mem::transmute(&signaturefilename)) {
                Ok(ok__) => {
                    blockmapreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageWriter: CreatePackageWriter::<Identity, OFFSET>,
            CreatePackageReader: CreatePackageReader::<Identity, OFFSET>,
            CreateManifestReader: CreateManifestReader::<Identity, OFFSET>,
            CreateBlockMapReader: CreateBlockMapReader::<Identity, OFFSET>,
            CreateValidatedBlockMapReader: CreateValidatedBlockMapReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory2_Impl: Sized {
    fn CreateContentGroupMapReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxContentGroupMapReader>;
    fn CreateSourceContentGroupMapReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxSourceContentGroupMapReader>;
    fn CreateContentGroupMapWriter(&self, stream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxContentGroupMapWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxFactory2_Vtbl
    where
        Identity: IAppxFactory2_Impl,
    {
        unsafe extern "system" fn CreateContentGroupMapReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, contentgroupmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory2_Impl::CreateContentGroupMapReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    contentgroupmapreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, reader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory2_Impl::CreateSourceContentGroupMapReader(this, windows_core::from_raw_borrowed(&inputstream)) {
                Ok(ok__) => {
                    reader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, contentgroupmapwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory2_Impl::CreateContentGroupMapWriter(this, windows_core::from_raw_borrowed(&stream)) {
                Ok(ok__) => {
                    contentgroupmapwriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContentGroupMapReader: CreateContentGroupMapReader::<Identity, OFFSET>,
            CreateSourceContentGroupMapReader: CreateSourceContentGroupMapReader::<Identity, OFFSET>,
            CreateContentGroupMapWriter: CreateContentGroupMapWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory3_Impl: Sized {
    fn CreatePackageReader2(&self, inputstream: Option<&super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader2(&self, inputstream: Option<&super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxManifestReader>;
    fn CreateAppInstallerReader(&self, inputstream: Option<&super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxAppInstallerReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxFactory3_Vtbl
    where
        Identity: IAppxFactory3_Impl,
    {
        unsafe extern "system" fn CreatePackageReader2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory3_Impl::CreatePackageReader2(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    packagereader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManifestReader2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory3_Impl::CreateManifestReader2(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    manifestreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAppInstallerReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, appinstallerreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFactory3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFactory3_Impl::CreateAppInstallerReader(this, windows_core::from_raw_borrowed(&inputstream), core::mem::transmute(&expecteddigest)) {
                Ok(ok__) => {
                    appinstallerreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageReader2: CreatePackageReader2::<Identity, OFFSET>,
            CreateManifestReader2: CreateManifestReader2::<Identity, OFFSET>,
            CreateAppInstallerReader: CreateAppInstallerReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFile_Impl: Sized {
    fn GetCompressionOption(&self) -> windows_core::Result<APPX_COMPRESSION_OPTION>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFile {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxFile_Vtbl
    where
        Identity: IAppxFile_Impl,
    {
        unsafe extern "system" fn GetCompressionOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> windows_core::HRESULT
        where
            Identity: IAppxFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFile_Impl::GetCompressionOption(this) {
                Ok(ok__) => {
                    compressionoption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFile_Impl::GetContentType(this) {
                Ok(ok__) => {
                    contenttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFile_Impl::GetName(this) {
                Ok(ok__) => {
                    filename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFile_Impl::GetSize(this) {
                Ok(ok__) => {
                    size.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFile_Impl::GetStream(this) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompressionOption: GetCompressionOption::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFile as windows_core::Interface>::IID
    }
}
pub trait IAppxFilesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxFile>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxFilesEnumerator {}
impl IAppxFilesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxFilesEnumerator_Vtbl
    where
        Identity: IAppxFilesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFilesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFilesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxFilesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxFilesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFilesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestApplication_Impl: Sized {
    fn GetStringValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAppUserModelId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestApplication {}
impl IAppxManifestApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestApplication_Vtbl
    where
        Identity: IAppxManifestApplication_Impl,
    {
        unsafe extern "system" fn GetStringValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestApplication_Impl::GetStringValue(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppUserModelId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestApplication_Impl::GetAppUserModelId(this) {
                Ok(ok__) => {
                    appusermodelid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetAppUserModelId: GetAppUserModelId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestApplication as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestApplicationsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestApplication>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestApplicationsEnumerator {}
impl IAppxManifestApplicationsEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestApplicationsEnumerator_Vtbl
    where
        Identity: IAppxManifestApplicationsEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestApplicationsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestApplicationsEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    application.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestApplicationsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestApplicationsEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestApplicationsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestApplicationsEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestApplicationsEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestCapabilitiesEnumerator {}
impl IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestCapabilitiesEnumerator_Vtbl
    where
        Identity: IAppxManifestCapabilitiesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, capability: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestCapabilitiesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    capability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestCapabilitiesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestCapabilitiesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestCapabilitiesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestDeviceCapabilitiesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestDeviceCapabilitiesEnumerator {}
impl IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestDeviceCapabilitiesEnumerator_Vtbl
    where
        Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicecapability: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDeviceCapabilitiesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    devicecapability.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDeviceCapabilitiesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDeviceCapabilitiesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDeviceCapabilitiesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestDriverConstraint_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
    fn GetMinDate(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestDriverConstraint {}
impl IAppxManifestDriverConstraint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestDriverConstraint_Vtbl
    where
        Identity: IAppxManifestDriverConstraint_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraint_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraint_Impl::GetMinVersion(this) {
                Ok(ok__) => {
                    minversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mindate: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraint_Impl::GetMinDate(this) {
                Ok(ok__) => {
                    mindate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
            GetMinDate: GetMinDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraint as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestDriverConstraintsEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverConstraint>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestDriverConstraintsEnumerator {}
impl IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestDriverConstraintsEnumerator_Vtbl
    where
        Identity: IAppxManifestDriverConstraintsEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverconstraint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraintsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraintsEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    driverconstraint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraintsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraintsEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverConstraintsEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverConstraintsEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraintsEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestDriverDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestDriverDependenciesEnumerator {}
impl IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestDriverDependenciesEnumerator_Vtbl
    where
        Identity: IAppxManifestDriverDependenciesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverdependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverDependenciesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    driverdependency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverDependenciesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverDependenciesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependenciesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestDriverDependency_Impl: Sized {
    fn GetDriverConstraints(&self) -> windows_core::Result<IAppxManifestDriverConstraintsEnumerator>;
}
impl windows_core::RuntimeName for IAppxManifestDriverDependency {}
impl IAppxManifestDriverDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestDriverDependency_Vtbl
    where
        Identity: IAppxManifestDriverDependency_Impl,
    {
        unsafe extern "system" fn GetDriverConstraints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverconstraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestDriverDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestDriverDependency_Impl::GetDriverConstraints(this) {
                Ok(ok__) => {
                    driverconstraints.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDriverConstraints: GetDriverConstraints::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestHostRuntimeDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependenciesEnumerator {}
impl IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl
    where
        Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostruntimedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    hostruntimedependency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependenciesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestHostRuntimeDependency_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependency {}
impl IAppxManifestHostRuntimeDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependency_Vtbl
    where
        Identity: IAppxManifestHostRuntimeDependency_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependency_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependency_Impl::GetPublisher(this) {
                Ok(ok__) => {
                    publisher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependency_Impl::GetMinVersion(this) {
                Ok(ok__) => {
                    minversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestHostRuntimeDependency2_Impl: Sized {
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependency2 {}
impl IAppxManifestHostRuntimeDependency2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestHostRuntimeDependency2_Vtbl
    where
        Identity: IAppxManifestHostRuntimeDependency2_Impl,
    {
        unsafe extern "system" fn GetPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestHostRuntimeDependency2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestHostRuntimeDependency2_Impl::GetPackageFamilyName(this) {
                Ok(ok__) => {
                    packagefamilyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency2 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestMainPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestMainPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestMainPackageDependenciesEnumerator {}
impl IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestMainPackageDependenciesEnumerator_Vtbl
    where
        Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    mainpackagedependency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependenciesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestMainPackageDependency_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestMainPackageDependency {}
impl IAppxManifestMainPackageDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestMainPackageDependency_Vtbl
    where
        Identity: IAppxManifestMainPackageDependency_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependency_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependency_Impl::GetPublisher(this) {
                Ok(ok__) => {
                    publisher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestMainPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestMainPackageDependency_Impl::GetPackageFamilyName(this) {
                Ok(ok__) => {
                    packagefamilyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestOSPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestOSPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestOSPackageDependenciesEnumerator {}
impl IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestOSPackageDependenciesEnumerator_Vtbl
    where
        Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ospackagedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOSPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    ospackagedependency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOSPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOSPackageDependenciesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestOSPackageDependency_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetVersion(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IAppxManifestOSPackageDependency {}
impl IAppxManifestOSPackageDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestOSPackageDependency_Vtbl
    where
        Identity: IAppxManifestOSPackageDependency_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOSPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOSPackageDependency_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOSPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOSPackageDependency_Impl::GetVersion(this) {
                Ok(ok__) => {
                    version.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetVersion: GetVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestOptionalPackageInfo_Impl: Sized {
    fn GetIsOptionalPackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetMainPackageName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestOptionalPackageInfo {}
impl IAppxManifestOptionalPackageInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestOptionalPackageInfo_Vtbl
    where
        Identity: IAppxManifestOptionalPackageInfo_Impl,
    {
        unsafe extern "system" fn GetIsOptionalPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOptionalPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOptionalPackageInfo_Impl::GetIsOptionalPackage(this) {
                Ok(ok__) => {
                    isoptionalpackage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainPackageName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestOptionalPackageInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestOptionalPackageInfo_Impl::GetMainPackageName(this) {
                Ok(ok__) => {
                    mainpackagename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsOptionalPackage: GetIsOptionalPackage::<Identity, OFFSET>,
            GetMainPackageName: GetMainPackageName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOptionalPackageInfo as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageDependenciesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestPackageDependenciesEnumerator {}
impl IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageDependenciesEnumerator_Vtbl
    where
        Identity: IAppxManifestPackageDependenciesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    dependency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependenciesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependenciesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageDependency_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency {}
impl IAppxManifestPackageDependency_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageDependency_Vtbl
    where
        Identity: IAppxManifestPackageDependency_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependency_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependency_Impl::GetPublisher(this) {
                Ok(ok__) => {
                    publisher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependency_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependency_Impl::GetMinVersion(this) {
                Ok(ok__) => {
                    minversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageDependency2_Impl: Sized + IAppxManifestPackageDependency_Impl {
    fn GetMaxMajorVersionTested(&self) -> windows_core::Result<u16>;
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency2 {}
impl IAppxManifestPackageDependency2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageDependency2_Vtbl
    where
        Identity: IAppxManifestPackageDependency2_Impl,
    {
        unsafe extern "system" fn GetMaxMajorVersionTested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxmajorversiontested: *mut u16) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependency2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependency2_Impl::GetMaxMajorVersionTested(this) {
                Ok(ok__) => {
                    maxmajorversiontested.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestPackageDependency_Vtbl::new::<Identity, OFFSET>(), GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency2 as windows_core::Interface>::IID || iid == &<IAppxManifestPackageDependency as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageDependency3_Impl: Sized {
    fn GetIsOptional(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency3 {}
impl IAppxManifestPackageDependency3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageDependency3_Vtbl
    where
        Identity: IAppxManifestPackageDependency3_Impl,
    {
        unsafe extern "system" fn GetIsOptional<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageDependency3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageDependency3_Impl::GetIsOptional(this) {
                Ok(ok__) => {
                    isoptional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsOptional: GetIsOptional::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency3 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageId_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetArchitecture(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetVersion(&self) -> windows_core::Result<u64>;
    fn GetResourceId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn ComparePublisher(&self, other: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetPackageFullName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestPackageId {}
impl IAppxManifestPackageId_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageId_Vtbl
    where
        Identity: IAppxManifestPackageId_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchitecture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetArchitecture(this) {
                Ok(ok__) => {
                    architecture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetPublisher(this) {
                Ok(ok__) => {
                    publisher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetVersion(this) {
                Ok(ok__) => {
                    packageversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetResourceId(this) {
                Ok(ok__) => {
                    resourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComparePublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, other: windows_core::PCWSTR, issame: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::ComparePublisher(this, core::mem::transmute(&other)) {
                Ok(ok__) => {
                    issame.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFullName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefullname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetPackageFullName(this) {
                Ok(ok__) => {
                    packagefullname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId_Impl::GetPackageFamilyName(this) {
                Ok(ok__) => {
                    packagefamilyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetArchitecture: GetArchitecture::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetResourceId: GetResourceId::<Identity, OFFSET>,
            ComparePublisher: ComparePublisher::<Identity, OFFSET>,
            GetPackageFullName: GetPackageFullName::<Identity, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageId as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestPackageId2_Impl: Sized + IAppxManifestPackageId_Impl {
    fn GetArchitecture2(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE2>;
}
impl windows_core::RuntimeName for IAppxManifestPackageId2 {}
impl IAppxManifestPackageId2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestPackageId2_Vtbl
    where
        Identity: IAppxManifestPackageId2_Impl,
    {
        unsafe extern "system" fn GetArchitecture2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> windows_core::HRESULT
        where
            Identity: IAppxManifestPackageId2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestPackageId2_Impl::GetArchitecture2(this) {
                Ok(ok__) => {
                    architecture.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestPackageId_Vtbl::new::<Identity, OFFSET>(), GetArchitecture2: GetArchitecture2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageId2 as windows_core::Interface>::IID || iid == &<IAppxManifestPackageId as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestProperties_Impl: Sized {
    fn GetBoolValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetStringValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAppxManifestProperties {}
impl IAppxManifestProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestProperties_Vtbl
    where
        Identity: IAppxManifestProperties_Impl,
    {
        unsafe extern "system" fn GetBoolValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestProperties_Impl::GetBoolValue(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestProperties_Impl::GetStringValue(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoolValue: GetBoolValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestProperties as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestQualifiedResource_Impl: Sized {
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetScale(&self) -> windows_core::Result<u32>;
    fn GetDXFeatureLevel(&self) -> windows_core::Result<DX_FEATURE_LEVEL>;
}
impl windows_core::RuntimeName for IAppxManifestQualifiedResource {}
impl IAppxManifestQualifiedResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestQualifiedResource_Vtbl
    where
        Identity: IAppxManifestQualifiedResource_Impl,
    {
        unsafe extern "system" fn GetLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResource_Impl::GetLanguage(this) {
                Ok(ok__) => {
                    language.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scale: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResource_Impl::GetScale(this) {
                Ok(ok__) => {
                    scale.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDXFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResource_Impl::GetDXFeatureLevel(this) {
                Ok(ok__) => {
                    dxfeaturelevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetScale: GetScale::<Identity, OFFSET>,
            GetDXFeatureLevel: GetDXFeatureLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResource as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestQualifiedResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestQualifiedResource>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestQualifiedResourcesEnumerator {}
impl IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestQualifiedResourcesEnumerator_Vtbl
    where
        Identity: IAppxManifestQualifiedResourcesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResourcesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    resource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResourcesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestQualifiedResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestQualifiedResourcesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResourcesEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader_Impl: Sized {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetProperties(&self) -> windows_core::Result<IAppxManifestProperties>;
    fn GetPackageDependencies(&self) -> windows_core::Result<IAppxManifestPackageDependenciesEnumerator>;
    fn GetCapabilities(&self) -> windows_core::Result<APPX_CAPABILITIES>;
    fn GetResources(&self) -> windows_core::Result<IAppxManifestResourcesEnumerator>;
    fn GetDeviceCapabilities(&self) -> windows_core::Result<IAppxManifestDeviceCapabilitiesEnumerator>;
    fn GetPrerequisite(&self, name: &windows_core::PCWSTR) -> windows_core::Result<u64>;
    fn GetApplications(&self) -> windows_core::Result<IAppxManifestApplicationsEnumerator>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader_Vtbl
    where
        Identity: IAppxManifestReader_Impl,
    {
        unsafe extern "system" fn GetPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetPackageId(this) {
                Ok(ok__) => {
                    packageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetProperties(this) {
                Ok(ok__) => {
                    packageproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPackageDependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetPackageDependencies(this) {
                Ok(ok__) => {
                    dependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetCapabilities(this) {
                Ok(ok__) => {
                    capabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetResources(this) {
                Ok(ok__) => {
                    resources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicecapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetDeviceCapabilities(this) {
                Ok(ok__) => {
                    devicecapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetPrerequisite(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applications: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetApplications(this) {
                Ok(ok__) => {
                    applications.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifeststream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader_Impl::GetStream(this) {
                Ok(ok__) => {
                    manifeststream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPackageDependencies: GetPackageDependencies::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetResources: GetResources::<Identity, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, OFFSET>,
            GetPrerequisite: GetPrerequisite::<Identity, OFFSET>,
            GetApplications: GetApplications::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader2_Impl: Sized + IAppxManifestReader_Impl {
    fn GetQualifiedResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader2_Vtbl
    where
        Identity: IAppxManifestReader2_Impl,
    {
        unsafe extern "system" fn GetQualifiedResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader2_Impl::GetQualifiedResources(this) {
                Ok(ok__) => {
                    resources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestReader_Vtbl::new::<Identity, OFFSET>(), GetQualifiedResources: GetQualifiedResources::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader2 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader3_Impl: Sized + IAppxManifestReader2_Impl {
    fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> windows_core::Result<IAppxManifestCapabilitiesEnumerator>;
    fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader3_Vtbl
    where
        Identity: IAppxManifestReader3_Impl,
    {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader3_Impl::GetCapabilitiesByCapabilityClass(this, core::mem::transmute_copy(&capabilityclass)) {
                Ok(ok__) => {
                    capabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamilies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader3_Impl::GetTargetDeviceFamilies(this) {
                Ok(ok__) => {
                    targetdevicefamilies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IAppxManifestReader2_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Identity, OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader3 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID || iid == &<IAppxManifestReader2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader4_Impl: Sized + IAppxManifestReader3_Impl {
    fn GetOptionalPackageInfo(&self) -> windows_core::Result<IAppxManifestOptionalPackageInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader4 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader4_Vtbl
    where
        Identity: IAppxManifestReader4_Impl,
    {
        unsafe extern "system" fn GetOptionalPackageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalpackageinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader4_Impl::GetOptionalPackageInfo(this) {
                Ok(ok__) => {
                    optionalpackageinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IAppxManifestReader3_Vtbl::new::<Identity, OFFSET>(), GetOptionalPackageInfo: GetOptionalPackageInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader4 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID || iid == &<IAppxManifestReader2 as windows_core::Interface>::IID || iid == &<IAppxManifestReader3 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestReader5_Impl: Sized {
    fn GetMainPackageDependencies(&self) -> windows_core::Result<IAppxManifestMainPackageDependenciesEnumerator>;
}
impl windows_core::RuntimeName for IAppxManifestReader5 {}
impl IAppxManifestReader5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader5_Vtbl
    where
        Identity: IAppxManifestReader5_Impl,
    {
        unsafe extern "system" fn GetMainPackageDependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader5_Impl::GetMainPackageDependencies(this) {
                Ok(ok__) => {
                    mainpackagedependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMainPackageDependencies: GetMainPackageDependencies::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader5 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestReader6_Impl: Sized {
    fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestReader6 {}
impl IAppxManifestReader6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader6_Vtbl
    where
        Identity: IAppxManifestReader6_Impl,
    {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader6_Impl::GetIsNonQualifiedResourcePackage(this) {
                Ok(ok__) => {
                    isnonqualifiedresourcepackage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader6 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestReader7_Impl: Sized {
    fn GetDriverDependencies(&self) -> windows_core::Result<IAppxManifestDriverDependenciesEnumerator>;
    fn GetOSPackageDependencies(&self) -> windows_core::Result<IAppxManifestOSPackageDependenciesEnumerator>;
    fn GetHostRuntimeDependencies(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependenciesEnumerator>;
}
impl windows_core::RuntimeName for IAppxManifestReader7 {}
impl IAppxManifestReader7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestReader7_Vtbl
    where
        Identity: IAppxManifestReader7_Impl,
    {
        unsafe extern "system" fn GetDriverDependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader7_Impl::GetDriverDependencies(this) {
                Ok(ok__) => {
                    driverdependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSPackageDependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ospackagedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader7_Impl::GetOSPackageDependencies(this) {
                Ok(ok__) => {
                    ospackagedependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostruntimedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestReader7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestReader7_Impl::GetHostRuntimeDependencies(this) {
                Ok(ok__) => {
                    hostruntimedependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDriverDependencies: GetDriverDependencies::<Identity, OFFSET>,
            GetOSPackageDependencies: GetOSPackageDependencies::<Identity, OFFSET>,
            GetHostRuntimeDependencies: GetHostRuntimeDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader7 as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestResourcesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestResourcesEnumerator {}
impl IAppxManifestResourcesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestResourcesEnumerator_Vtbl
    where
        Identity: IAppxManifestResourcesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestResourcesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    resource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestResourcesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestResourcesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestResourcesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestResourcesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestTargetDeviceFamiliesEnumerator_Impl: Sized {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamily>;
    fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAppxManifestTargetDeviceFamiliesEnumerator {}
impl IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl
    where
        Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl,
    {
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    targetdevicefamily.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::GetHasCurrent(this) {
                Ok(ok__) => {
                    hascurrent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    hasnext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamiliesEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAppxManifestTargetDeviceFamily_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
    fn GetMaxVersionTested(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IAppxManifestTargetDeviceFamily {}
impl IAppxManifestTargetDeviceFamily_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxManifestTargetDeviceFamily_Vtbl
    where
        Identity: IAppxManifestTargetDeviceFamily_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamily_Impl::GetName(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamily_Impl::GetMinVersion(this) {
                Ok(ok__) => {
                    minversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxVersionTested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxversiontested: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAppxManifestTargetDeviceFamily_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxManifestTargetDeviceFamily_Impl::GetMaxVersionTested(this) {
                Ok(ok__) => {
                    maxversiontested.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
            GetMaxVersionTested: GetMaxVersionTested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamily as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageEditor_Impl: Sized {
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateDeltaPackage(&self, updatedpackagestream: Option<&super::super::super::System::Com::IStream>, baselinepackagestream: Option<&super::super::super::System::Com::IStream>, deltapackagestream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDeltaPackageUsingBaselineBlockMap(&self, updatedpackagestream: Option<&super::super::super::System::Com::IStream>, baselineblockmapstream: Option<&super::super::super::System::Com::IStream>, baselinepackagefullname: &windows_core::PCWSTR, deltapackagestream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn UpdatePackage(&self, baselinepackagestream: Option<&super::super::super::System::Com::IStream>, deltapackagestream: Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::Result<()>;
    fn UpdateEncryptedPackage(&self, baselineencryptedpackagestream: Option<&super::super::super::System::Com::IStream>, deltapackagestream: Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn UpdatePackageManifest(&self, packagestream: Option<&super::super::super::System::Com::IStream>, updatedmanifeststream: Option<&super::super::super::System::Com::IStream>, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageEditor {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageEditor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackageEditor_Vtbl
    where
        Identity: IAppxPackageEditor_Impl,
    {
        unsafe extern "system" fn SetWorkingDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
        }
        unsafe extern "system" fn CreateDeltaPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatedpackagestream: *mut core::ffi::c_void, baselinepackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::CreateDeltaPackage(this, windows_core::from_raw_borrowed(&updatedpackagestream), windows_core::from_raw_borrowed(&baselinepackagestream), windows_core::from_raw_borrowed(&deltapackagestream)).into()
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatedpackagestream: *mut core::ffi::c_void, baselineblockmapstream: *mut core::ffi::c_void, baselinepackagefullname: windows_core::PCWSTR, deltapackagestream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::CreateDeltaPackageUsingBaselineBlockMap(this, windows_core::from_raw_borrowed(&updatedpackagestream), windows_core::from_raw_borrowed(&baselineblockmapstream), core::mem::transmute(&baselinepackagefullname), windows_core::from_raw_borrowed(&deltapackagestream)).into()
        }
        unsafe extern "system" fn UpdatePackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselinepackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::UpdatePackage(this, windows_core::from_raw_borrowed(&baselinepackagestream), windows_core::from_raw_borrowed(&deltapackagestream), core::mem::transmute_copy(&updateoption)).into()
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineencryptedpackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::UpdateEncryptedPackage(this, windows_core::from_raw_borrowed(&baselineencryptedpackagestream), windows_core::from_raw_borrowed(&deltapackagestream), core::mem::transmute_copy(&updateoption), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo)).into()
        }
        unsafe extern "system" fn UpdatePackageManifest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagestream: *mut core::ffi::c_void, updatedmanifeststream: *mut core::ffi::c_void, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::HRESULT
        where
            Identity: IAppxPackageEditor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageEditor_Impl::UpdatePackageManifest(this, windows_core::from_raw_borrowed(&packagestream), windows_core::from_raw_borrowed(&updatedmanifeststream), core::mem::transmute_copy(&ispackageencrypted), core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            CreateDeltaPackage: CreateDeltaPackage::<Identity, OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap: CreateDeltaPackageUsingBaselineBlockMap::<Identity, OFFSET>,
            UpdatePackage: UpdatePackage::<Identity, OFFSET>,
            UpdateEncryptedPackage: UpdateEncryptedPackage::<Identity, OFFSET>,
            UpdatePackageManifest: UpdatePackageManifest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageEditor as windows_core::Interface>::IID
    }
}
pub trait IAppxPackageReader_Impl: Sized {
    fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader>;
    fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile>;
    fn GetPayloadFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxFile>;
    fn GetPayloadFiles(&self) -> windows_core::Result<IAppxFilesEnumerator>;
    fn GetManifest(&self) -> windows_core::Result<IAppxManifestReader>;
}
impl windows_core::RuntimeName for IAppxPackageReader {}
impl IAppxPackageReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackageReader_Vtbl
    where
        Identity: IAppxPackageReader_Impl,
    {
        unsafe extern "system" fn GetBlockMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxPackageReader_Impl::GetBlockMap(this) {
                Ok(ok__) => {
                    blockmapreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFootprintFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxPackageReader_Impl::GetFootprintFile(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxPackageReader_Impl::GetPayloadFile(this, core::mem::transmute(&filename)) {
                Ok(ok__) => {
                    file.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPayloadFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxPackageReader_Impl::GetPayloadFiles(this) {
                Ok(ok__) => {
                    filesenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManifest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxPackageReader_Impl::GetManifest(this) {
                Ok(ok__) => {
                    manifestreader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlockMap: GetBlockMap::<Identity, OFFSET>,
            GetFootprintFile: GetFootprintFile::<Identity, OFFSET>,
            GetPayloadFile: GetPayloadFile::<Identity, OFFSET>,
            GetPayloadFiles: GetPayloadFiles::<Identity, OFFSET>,
            GetManifest: GetManifest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter_Impl: Sized {
    fn AddPayloadFile(&self, filename: &windows_core::PCWSTR, contenttype: &windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self, manifest: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackageWriter_Vtbl
    where
        Identity: IAppxPackageWriter_Impl,
    {
        unsafe extern "system" fn AddPayloadFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, contenttype: windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageWriter_Impl::AddPayloadFile(this, core::mem::transmute(&filename), core::mem::transmute(&contenttype), core::mem::transmute_copy(&compressionoption), windows_core::from_raw_borrowed(&inputstream)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifest: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageWriter_Impl::Close(this, windows_core::from_raw_borrowed(&manifest)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFile: AddPayloadFile::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter2_Impl: Sized {
    fn Close(&self, manifest: Option<&super::super::super::System::Com::IStream>, contentgroupmap: Option<&super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackageWriter2_Vtbl
    where
        Identity: IAppxPackageWriter2_Impl,
    {
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifest: *mut core::ffi::c_void, contentgroupmap: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackageWriter2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageWriter2_Impl::Close(this, windows_core::from_raw_borrowed(&manifest), windows_core::from_raw_borrowed(&contentgroupmap)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter3_Impl: Sized {
    fn AddPayloadFiles(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackageWriter3_Vtbl
    where
        Identity: IAppxPackageWriter3_Impl,
    {
        unsafe extern "system" fn AddPayloadFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::HRESULT
        where
            Identity: IAppxPackageWriter3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackageWriter3_Impl::AddPayloadFiles(this, core::mem::transmute_copy(&filecount), core::mem::transmute_copy(&payloadfiles), core::mem::transmute_copy(&memorylimit)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFiles: AddPayloadFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter3 as windows_core::Interface>::IID
    }
}
pub trait IAppxPackagingDiagnosticEventSink_Impl: Sized {
    fn ReportContextChange(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: &windows_core::PCSTR, contextmessage: &windows_core::PCWSTR, detailsmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReportError(&self, errormessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppxPackagingDiagnosticEventSink {}
impl IAppxPackagingDiagnosticEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSink_Vtbl
    where
        Identity: IAppxPackagingDiagnosticEventSink_Impl,
    {
        unsafe extern "system" fn ReportContextChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: windows_core::PCSTR, contextmessage: windows_core::PCWSTR, detailsmessage: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxPackagingDiagnosticEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackagingDiagnosticEventSink_Impl::ReportContextChange(this, core::mem::transmute_copy(&changetype), core::mem::transmute_copy(&contextid), core::mem::transmute(&contextname), core::mem::transmute(&contextmessage), core::mem::transmute(&detailsmessage)).into()
        }
        unsafe extern "system" fn ReportError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, errormessage: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAppxPackagingDiagnosticEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackagingDiagnosticEventSink_Impl::ReportError(this, core::mem::transmute(&errormessage)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportContextChange: ReportContextChange::<Identity, OFFSET>,
            ReportError: ReportError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSink as windows_core::Interface>::IID
    }
}
pub trait IAppxPackagingDiagnosticEventSinkManager_Impl: Sized {
    fn SetSinkForProcess(&self, sink: Option<&IAppxPackagingDiagnosticEventSink>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppxPackagingDiagnosticEventSinkManager {}
impl IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxPackagingDiagnosticEventSinkManager_Vtbl
    where
        Identity: IAppxPackagingDiagnosticEventSinkManager_Impl,
    {
        unsafe extern "system" fn SetSinkForProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxPackagingDiagnosticEventSinkManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppxPackagingDiagnosticEventSinkManager_Impl::SetSinkForProcess(this, windows_core::from_raw_borrowed(&sink)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSinkForProcess: SetSinkForProcess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSinkManager as windows_core::Interface>::IID
    }
}
pub trait IAppxSourceContentGroupMapReader_Impl: Sized {
    fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl windows_core::RuntimeName for IAppxSourceContentGroupMapReader {}
impl IAppxSourceContentGroupMapReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppxSourceContentGroupMapReader_Vtbl
    where
        Identity: IAppxSourceContentGroupMapReader_Impl,
    {
        unsafe extern "system" fn GetRequiredGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxSourceContentGroupMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxSourceContentGroupMapReader_Impl::GetRequiredGroup(this) {
                Ok(ok__) => {
                    requiredgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticgroupsenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppxSourceContentGroupMapReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAppxSourceContentGroupMapReader_Impl::GetAutomaticGroups(this) {
                Ok(ok__) => {
                    automaticgroupsenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxSourceContentGroupMapReader as windows_core::Interface>::IID
    }
}
