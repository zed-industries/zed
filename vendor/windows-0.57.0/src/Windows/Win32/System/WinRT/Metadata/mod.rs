#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("rometadata.dll" "system" fn MetaDataGetDispenser(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    MetaDataGetDispenser(rclsid, riid, ppv).ok()
}
#[cfg(feature = "Foundation_Collections")]
#[inline]
pub unsafe fn RoCreateNonAgilePropertySet() -> windows_core::Result<super::super::super::super::Foundation::Collections::IPropertySet> {
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-1.dll" "system" fn RoCreateNonAgilePropertySet(pppropertyset : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoCreateNonAgilePropertySet(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Storage_Streams")]
#[inline]
pub unsafe fn RoCreatePropertySetSerializer() -> windows_core::Result<super::super::super::super::Storage::Streams::IPropertySetSerializer> {
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-1.dll" "system" fn RoCreatePropertySetSerializer(pppropertysetserializer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoCreatePropertySetSerializer(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<P0>(extra: P0)
where
    P0: windows_core::Param<super::ROPARAMIIDHANDLE>,
{
    windows_targets::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoFreeParameterizedTypeExtra(extra : super:: ROPARAMIIDHANDLE));
    RoFreeParameterizedTypeExtra(extra.param().abi())
}
#[inline]
pub unsafe fn RoGetMetaDataFile<P0>(name: &windows_core::HSTRING, metadatadispenser: P0, metadatafilepath: Option<*mut windows_core::HSTRING>, metadataimport: Option<*mut Option<IMetaDataImport2>>, typedeftoken: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMetaDataDispenserEx>,
{
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-0.dll" "system" fn RoGetMetaDataFile(name : core::mem::MaybeUninit < windows_core::HSTRING >, metadatadispenser : * mut core::ffi::c_void, metadatafilepath : *mut core::mem::MaybeUninit < windows_core::HSTRING >, metadataimport : *mut * mut core::ffi::c_void, typedeftoken : *mut u32) -> windows_core::HRESULT);
    RoGetMetaDataFile(core::mem::transmute_copy(name), metadatadispenser.param().abi(), core::mem::transmute(metadatafilepath.unwrap_or(std::ptr::null_mut())), core::mem::transmute(metadataimport.unwrap_or(std::ptr::null_mut())), core::mem::transmute(typedeftoken.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<P0>(nameelements: &[windows_core::PCWSTR], metadatalocator: P0, iid: *mut windows_core::GUID, pextra: Option<*mut super::ROPARAMIIDHANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRoMetaDataLocator>,
{
    windows_targets::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoGetParameterizedTypeInstanceIID(nameelementcount : u32, nameelements : *const windows_core::PCWSTR, metadatalocator : * mut core::ffi::c_void, iid : *mut windows_core::GUID, pextra : *mut super:: ROPARAMIIDHANDLE) -> windows_core::HRESULT);
    RoGetParameterizedTypeInstanceIID(nameelements.len().try_into().unwrap(), core::mem::transmute(nameelements.as_ptr()), metadatalocator.param().abi(), iid, core::mem::transmute(pextra.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RoIsApiContractMajorVersionPresent<P0>(name: P0, majorversion: u16) -> windows_core::Result<super::super::super::Foundation::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-1.dll" "system" fn RoIsApiContractMajorVersionPresent(name : windows_core::PCWSTR, majorversion : u16, present : *mut super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoIsApiContractMajorVersionPresent(name.param().abi(), majorversion, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoIsApiContractPresent<P0>(name: P0, majorversion: u16, minorversion: u16) -> windows_core::Result<super::super::super::Foundation::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-1.dll" "system" fn RoIsApiContractPresent(name : windows_core::PCWSTR, majorversion : u16, minorversion : u16, present : *mut super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoIsApiContractPresent(name.param().abi(), majorversion, minorversion, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<P0>(extra: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<super::ROPARAMIIDHANDLE>,
{
    windows_targets::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoParameterizedTypeExtraGetTypeSignature(extra : super:: ROPARAMIIDHANDLE) -> windows_core::PCSTR);
    RoParameterizedTypeExtraGetTypeSignature(extra.param().abi())
}
#[inline]
pub unsafe fn RoParseTypeName(typename: &windows_core::HSTRING, partscount: *mut u32, typenameparts: *mut *mut windows_core::HSTRING) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-0.dll" "system" fn RoParseTypeName(typename : core::mem::MaybeUninit < windows_core::HSTRING >, partscount : *mut u32, typenameparts : *mut *mut windows_core::HSTRING) -> windows_core::HRESULT);
    RoParseTypeName(core::mem::transmute_copy(typename), partscount, typenameparts).ok()
}
#[inline]
pub unsafe fn RoResolveNamespace(name: &windows_core::HSTRING, windowsmetadatadir: &windows_core::HSTRING, packagegraphdirs: Option<&[windows_core::HSTRING]>, metadatafilepathscount: Option<*mut u32>, metadatafilepaths: Option<*mut *mut windows_core::HSTRING>, subnamespacescount: Option<*mut u32>, subnamespaces: Option<*mut *mut windows_core::HSTRING>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-ro-typeresolution-l1-1-0.dll" "system" fn RoResolveNamespace(name : core::mem::MaybeUninit < windows_core::HSTRING >, windowsmetadatadir : core::mem::MaybeUninit < windows_core::HSTRING >, packagegraphdirscount : u32, packagegraphdirs : *const core::mem::MaybeUninit < windows_core::HSTRING >, metadatafilepathscount : *mut u32, metadatafilepaths : *mut *mut windows_core::HSTRING, subnamespacescount : *mut u32, subnamespaces : *mut *mut windows_core::HSTRING) -> windows_core::HRESULT);
    RoResolveNamespace(
        core::mem::transmute_copy(name),
        core::mem::transmute_copy(windowsmetadatadir),
        packagegraphdirs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(packagegraphdirs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        core::mem::transmute(metadatafilepathscount.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(metadatafilepaths.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(subnamespacescount.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(subnamespaces.unwrap_or(std::ptr::null_mut())),
    )
    .ok()
}
windows_core::imp::define_interface!(ICeeGen, ICeeGen_Vtbl, 0x7ed1bdff_8e36_11d2_9c56_00a0c9b7cc45);
impl core::ops::Deref for ICeeGen {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICeeGen, windows_core::IUnknown);
impl ICeeGen {
    pub unsafe fn EmitString<P0>(&self, lpstring: P0, rva: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EmitString)(windows_core::Interface::as_raw(self), lpstring.param().abi(), rva).ok()
    }
    pub unsafe fn GetString(&self, rva: u32, lpstring: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), rva, core::mem::transmute(lpstring.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AllocateMethodBuffer(&self, cchbuffer: u32, lpbuffer: *mut *mut u8, rva: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllocateMethodBuffer)(windows_core::Interface::as_raw(self), cchbuffer, lpbuffer, rva).ok()
    }
    pub unsafe fn GetMethodBuffer(&self, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMethodBuffer)(windows_core::Interface::as_raw(self), rva, lpbuffer).ok()
    }
    pub unsafe fn GetIMapTokenIface(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIMapTokenIface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GenerateCeeFile(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateCeeFile)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetIlSection(&self, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIlSection)(windows_core::Interface::as_raw(self), section).ok()
    }
    pub unsafe fn GetStringSection(&self, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringSection)(windows_core::Interface::as_raw(self), section).ok()
    }
    pub unsafe fn AddSectionReloc(&self, section: *mut core::ffi::c_void, offset: u32, relativeto: *mut core::ffi::c_void, reloctype: CeeSectionRelocType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSectionReloc)(windows_core::Interface::as_raw(self), section, offset, relativeto, reloctype).ok()
    }
    pub unsafe fn GetSectionCreate<P0>(&self, name: P0, flags: u32, section: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetSectionCreate)(windows_core::Interface::as_raw(self), name.param().abi(), flags, section).ok()
    }
    pub unsafe fn GetSectionDataLen(&self, section: *mut core::ffi::c_void, datalen: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSectionDataLen)(windows_core::Interface::as_raw(self), section, datalen).ok()
    }
    pub unsafe fn GetSectionBlock(&self, section: *mut core::ffi::c_void, len: u32, align: u32, ppbytes: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSectionBlock)(windows_core::Interface::as_raw(self), section, len, align, ppbytes).ok()
    }
    pub unsafe fn TruncateSection(&self, section: *mut core::ffi::c_void, len: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TruncateSection)(windows_core::Interface::as_raw(self), section, len).ok()
    }
    pub unsafe fn GenerateCeeMemoryImage(&self, ppimage: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GenerateCeeMemoryImage)(windows_core::Interface::as_raw(self), ppimage).ok()
    }
    pub unsafe fn ComputePointer(&self, section: *mut core::ffi::c_void, rva: u32, lpbuffer: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ComputePointer)(windows_core::Interface::as_raw(self), section, rva, lpbuffer).ok()
    }
}
#[repr(C)]
pub struct ICeeGen_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EmitString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub AllocateMethodBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetMethodBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetIMapTokenIface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateCeeFile: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIlSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSectionReloc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, CeeSectionRelocType) -> windows_core::HRESULT,
    pub GetSectionCreate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSectionDataLen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSectionBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TruncateSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GenerateCeeMemoryImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComputePointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostFilter, IHostFilter_Vtbl, 0xd0e80dd3_12d4_11d3_b39d_00c04ff81795);
impl core::ops::Deref for IHostFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHostFilter, windows_core::IUnknown);
impl IHostFilter {
    pub unsafe fn MarkToken(&self, tk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MarkToken)(windows_core::Interface::as_raw(self), tk).ok()
    }
}
#[repr(C)]
pub struct IHostFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MarkToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMapToken, IMapToken_Vtbl, 0x06a3ea8b_0225_11d1_bf72_00c04fc31e12);
impl core::ops::Deref for IMapToken {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMapToken, windows_core::IUnknown);
impl IMapToken {
    pub unsafe fn Map(&self, tkimp: u32, tkemit: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), tkimp, tkemit).ok()
    }
}
#[repr(C)]
pub struct IMapToken_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataAssemblyEmit, IMetaDataAssemblyEmit_Vtbl, 0x211ef15b_5317_4438_b196_dec87b887693);
impl core::ops::Deref for IMetaDataAssemblyEmit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataAssemblyEmit, windows_core::IUnknown);
impl IMetaDataAssemblyEmit {
    pub unsafe fn DefineAssembly<P0>(&self, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32, pma: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineAssembly)(windows_core::Interface::as_raw(self), pbpublickey, cbpublickey, ulhashalgid, szname.param().abi(), pmetadata, dwassemblyflags, pma).ok()
    }
    pub unsafe fn DefineAssemblyRef<P0>(&self, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32, pmdar: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineAssemblyRef)(windows_core::Interface::as_raw(self), pbpublickeyortoken, cbpublickeyortoken, szname.param().abi(), pmetadata, pbhashvalue, cbhashvalue, dwassemblyrefflags, pmdar).ok()
    }
    pub unsafe fn DefineFile<P0>(&self, szname: P0, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32, pmdf: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineFile)(windows_core::Interface::as_raw(self), szname.param().abi(), pbhashvalue, cbhashvalue, dwfileflags, pmdf).ok()
    }
    pub unsafe fn DefineExportedType<P0>(&self, szname: P0, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32, pmdct: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineExportedType)(windows_core::Interface::as_raw(self), szname.param().abi(), tkimplementation, tktypedef, dwexportedtypeflags, pmdct).ok()
    }
    pub unsafe fn DefineManifestResource<P0>(&self, szname: P0, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32, pmdmr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineManifestResource)(windows_core::Interface::as_raw(self), szname.param().abi(), tkimplementation, dwoffset, dwresourceflags, pmdmr).ok()
    }
    pub unsafe fn SetAssemblyProps<P0>(&self, pma: u32, pbpublickey: *const core::ffi::c_void, cbpublickey: u32, ulhashalgid: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, dwassemblyflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAssemblyProps)(windows_core::Interface::as_raw(self), pma, pbpublickey, cbpublickey, ulhashalgid, szname.param().abi(), pmetadata, dwassemblyflags).ok()
    }
    pub unsafe fn SetAssemblyRefProps<P0>(&self, ar: u32, pbpublickeyortoken: *const core::ffi::c_void, cbpublickeyortoken: u32, szname: P0, pmetadata: *const ASSEMBLYMETADATA, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwassemblyrefflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAssemblyRefProps)(windows_core::Interface::as_raw(self), ar, pbpublickeyortoken, cbpublickeyortoken, szname.param().abi(), pmetadata, pbhashvalue, cbhashvalue, dwassemblyrefflags).ok()
    }
    pub unsafe fn SetFileProps(&self, file: u32, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, dwfileflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileProps)(windows_core::Interface::as_raw(self), file, pbhashvalue, cbhashvalue, dwfileflags).ok()
    }
    pub unsafe fn SetExportedTypeProps(&self, ct: u32, tkimplementation: u32, tktypedef: u32, dwexportedtypeflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExportedTypeProps)(windows_core::Interface::as_raw(self), ct, tkimplementation, tktypedef, dwexportedtypeflags).ok()
    }
    pub unsafe fn SetManifestResourceProps(&self, mr: u32, tkimplementation: u32, dwoffset: u32, dwresourceflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetManifestResourceProps)(windows_core::Interface::as_raw(self), mr, tkimplementation, dwoffset, dwresourceflags).ok()
    }
}
#[repr(C)]
pub struct IMetaDataAssemblyEmit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DefineAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32, windows_core::PCWSTR, *const ASSEMBLYMETADATA, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineAssemblyRef: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, windows_core::PCWSTR, *const ASSEMBLYMETADATA, *const core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineExportedType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineManifestResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetAssemblyProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32, windows_core::PCWSTR, *const ASSEMBLYMETADATA, u32) -> windows_core::HRESULT,
    pub SetAssemblyRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, windows_core::PCWSTR, *const ASSEMBLYMETADATA, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetFileProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetExportedTypeProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub SetManifestResourceProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataAssemblyImport, IMetaDataAssemblyImport_Vtbl, 0xee62470b_e94b_424e_9b7c_2f00c9249f93);
impl core::ops::Deref for IMetaDataAssemblyImport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataAssemblyImport, windows_core::IUnknown);
impl IMetaDataAssemblyImport {
    pub unsafe fn GetAssemblyProps(&self, mda: u32, ppbpublickey: *const *const core::ffi::c_void, pcbpublickey: *mut u32, pulhashalgid: *mut u32, szname: Option<&mut [u16]>, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, pdwassemblyflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAssemblyProps)(windows_core::Interface::as_raw(self), mda, ppbpublickey, pcbpublickey, pulhashalgid, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, pmetadata, pdwassemblyflags).ok()
    }
    pub unsafe fn GetAssemblyRefProps(&self, mdar: u32, ppbpublickeyortoken: *const *const core::ffi::c_void, pcbpublickeyortoken: *mut u32, szname: Option<&mut [u16]>, pchname: *mut u32, pmetadata: *mut ASSEMBLYMETADATA, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwassemblyrefflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAssemblyRefProps)(windows_core::Interface::as_raw(self), mdar, ppbpublickeyortoken, pcbpublickeyortoken, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, pmetadata, ppbhashvalue, pcbhashvalue, pdwassemblyrefflags).ok()
    }
    pub unsafe fn GetFileProps(&self, mdf: u32, szname: Option<&mut [u16]>, pchname: *mut u32, ppbhashvalue: *const *const core::ffi::c_void, pcbhashvalue: *mut u32, pdwfileflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileProps)(windows_core::Interface::as_raw(self), mdf, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, ppbhashvalue, pcbhashvalue, pdwfileflags).ok()
    }
    pub unsafe fn GetExportedTypeProps(&self, mdct: u32, szname: Option<&mut [u16]>, pchname: *mut u32, ptkimplementation: *mut u32, ptktypedef: *mut u32, pdwexportedtypeflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetExportedTypeProps)(windows_core::Interface::as_raw(self), mdct, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, ptkimplementation, ptktypedef, pdwexportedtypeflags).ok()
    }
    pub unsafe fn GetManifestResourceProps(&self, mdmr: u32, szname: Option<&mut [u16]>, pchname: *mut u32, ptkimplementation: *mut u32, pdwoffset: *mut u32, pdwresourceflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetManifestResourceProps)(windows_core::Interface::as_raw(self), mdmr, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, ptkimplementation, pdwoffset, pdwresourceflags).ok()
    }
    pub unsafe fn EnumAssemblyRefs(&self, phenum: *mut *mut core::ffi::c_void, rassemblyrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumAssemblyRefs)(windows_core::Interface::as_raw(self), phenum, rassemblyrefs, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFiles(&self, phenum: *mut *mut core::ffi::c_void, rfiles: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumFiles)(windows_core::Interface::as_raw(self), phenum, rfiles, cmax, pctokens).ok()
    }
    pub unsafe fn EnumExportedTypes(&self, phenum: *mut *mut core::ffi::c_void, rexportedtypes: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumExportedTypes)(windows_core::Interface::as_raw(self), phenum, rexportedtypes, cmax, pctokens).ok()
    }
    pub unsafe fn EnumManifestResources(&self, phenum: *mut *mut core::ffi::c_void, rmanifestresources: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumManifestResources)(windows_core::Interface::as_raw(self), phenum, rmanifestresources, cmax, pctokens).ok()
    }
    pub unsafe fn GetAssemblyFromScope(&self, ptkassembly: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAssemblyFromScope)(windows_core::Interface::as_raw(self), ptkassembly).ok()
    }
    pub unsafe fn FindExportedTypeByName<P0>(&self, szname: P0, mdtexportedtype: u32, ptkexportedtype: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindExportedTypeByName)(windows_core::Interface::as_raw(self), szname.param().abi(), mdtexportedtype, ptkexportedtype).ok()
    }
    pub unsafe fn FindManifestResourceByName<P0>(&self, szname: P0, ptkmanifestresource: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindManifestResourceByName)(windows_core::Interface::as_raw(self), szname.param().abi(), ptkmanifestresource).ok()
    }
    pub unsafe fn CloseEnum(&self, henum: *mut core::ffi::c_void) {
        (windows_core::Interface::vtable(self).CloseEnum)(windows_core::Interface::as_raw(self), henum)
    }
    pub unsafe fn FindAssembliesByName<P0, P1, P2>(&self, szappbase: P0, szprivatebin: P1, szassemblyname: P2, ppiunk: *mut Option<windows_core::IUnknown>, cmax: u32, pcassemblies: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindAssembliesByName)(windows_core::Interface::as_raw(self), szappbase.param().abi(), szprivatebin.param().abi(), szassemblyname.param().abi(), core::mem::transmute(ppiunk), cmax, pcassemblies).ok()
    }
}
#[repr(C)]
pub struct IMetaDataAssemblyImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAssemblyProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const core::ffi::c_void, *mut u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut ASSEMBLYMETADATA, *mut u32) -> windows_core::HRESULT,
    pub GetAssemblyRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const core::ffi::c_void, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut ASSEMBLYMETADATA, *const *const core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetFileProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32, *const *const core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetExportedTypeProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetManifestResourceProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EnumAssemblyRefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumExportedTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumManifestResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetAssemblyFromScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FindExportedTypeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub FindManifestResourceByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub CloseEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub FindAssembliesByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataDispenser, IMetaDataDispenser_Vtbl, 0x809c652e_7396_11d2_9771_00a0c9b4d50c);
impl core::ops::Deref for IMetaDataDispenser {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataDispenser, windows_core::IUnknown);
impl IMetaDataDispenser {
    pub unsafe fn DefineScope(&self, rclsid: *const windows_core::GUID, dwcreateflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefineScope)(windows_core::Interface::as_raw(self), rclsid, dwcreateflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenScope<P0>(&self, szscope: P0, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenScope)(windows_core::Interface::as_raw(self), szscope.param().abi(), dwopenflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenScopeOnMemory(&self, pdata: *const core::ffi::c_void, cbdata: u32, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenScopeOnMemory)(windows_core::Interface::as_raw(self), pdata, cbdata, dwopenflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMetaDataDispenser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DefineScope: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenScope: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenScopeOnMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataDispenserEx, IMetaDataDispenserEx_Vtbl, 0x31bcfce2_dafb_11d2_9f81_00c04f79a0a3);
impl core::ops::Deref for IMetaDataDispenserEx {
    type Target = IMetaDataDispenser;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataDispenserEx, windows_core::IUnknown, IMetaDataDispenser);
impl IMetaDataDispenserEx {
    pub unsafe fn SetOption(&self, optionid: *const windows_core::GUID, value: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), optionid, core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetOption(&self, optionid: *const windows_core::GUID, pvalue: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), optionid, core::mem::transmute(pvalue)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScopeOnITypeInfo<P0>(&self, piti: P0, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::super::Com::ITypeInfo>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenScopeOnITypeInfo)(windows_core::Interface::as_raw(self), piti.param().abi(), dwopenflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCORSystemDirectory(&self, szbuffer: Option<&mut [u16]>, pchbuffer: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCORSystemDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(szbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchbuffer).ok()
    }
    pub unsafe fn FindAssembly<P0, P1, P2, P3, P4>(&self, szappbase: P0, szprivatebin: P1, szglobalbin: P2, szassemblyname: P3, szname: P4, cchname: u32, pcname: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindAssembly)(windows_core::Interface::as_raw(self), szappbase.param().abi(), szprivatebin.param().abi(), szglobalbin.param().abi(), szassemblyname.param().abi(), szname.param().abi(), cchname, pcname).ok()
    }
    pub unsafe fn FindAssemblyModule<P0, P1, P2, P3, P4>(&self, szappbase: P0, szprivatebin: P1, szglobalbin: P2, szassemblyname: P3, szmodulename: P4, szname: Option<&mut [u16]>, pcname: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindAssemblyModule)(windows_core::Interface::as_raw(self), szappbase.param().abi(), szprivatebin.param().abi(), szglobalbin.param().abi(), szassemblyname.param().abi(), szmodulename.param().abi(), core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcname).ok()
    }
}
#[repr(C)]
pub struct IMetaDataDispenserEx_Vtbl {
    pub base__: IMetaDataDispenser_Vtbl,
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScopeOnITypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScopeOnITypeInfo: usize,
    pub GetCORSystemDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub FindAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub FindAssemblyModule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataEmit, IMetaDataEmit_Vtbl, 0xba3fee4c_ecb9_4e41_83b7_183fa41cd859);
impl core::ops::Deref for IMetaDataEmit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataEmit, windows_core::IUnknown);
impl IMetaDataEmit {
    pub unsafe fn SetModuleProps<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetModuleProps)(windows_core::Interface::as_raw(self), szname.param().abi()).ok()
    }
    pub unsafe fn Save<P0>(&self, szfile: P0, dwsaveflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), szfile.param().abi(), dwsaveflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveToStream<P0>(&self, pistream: P0, dwsaveflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveToStream)(windows_core::Interface::as_raw(self), pistream.param().abi(), dwsaveflags).ok()
    }
    pub unsafe fn GetSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSaveSize)(windows_core::Interface::as_raw(self), fsave, pdwsavesize).ok()
    }
    pub unsafe fn DefineTypeDef<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, ptd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineTypeDef)(windows_core::Interface::as_raw(self), sztypedef.param().abi(), dwtypedefflags, tkextends, rtkimplements, ptd).ok()
    }
    pub unsafe fn DefineNestedType<P0>(&self, sztypedef: P0, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32, tdencloser: u32, ptd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineNestedType)(windows_core::Interface::as_raw(self), sztypedef.param().abi(), dwtypedefflags, tkextends, rtkimplements, tdencloser, ptd).ok()
    }
    pub unsafe fn SetHandler<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetHandler)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn DefineMethod<P0>(&self, td: u32, szname: P0, dwmethodflags: u32, pvsigblob: *mut u8, cbsigblob: u32, ulcoderva: u32, dwimplflags: u32, pmd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineMethod)(windows_core::Interface::as_raw(self), td, szname.param().abi(), dwmethodflags, pvsigblob, cbsigblob, ulcoderva, dwimplflags, pmd).ok()
    }
    pub unsafe fn DefineMethodImpl(&self, td: u32, tkbody: u32, tkdecl: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefineMethodImpl)(windows_core::Interface::as_raw(self), td, tkbody, tkdecl).ok()
    }
    pub unsafe fn DefineTypeRefByName<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineTypeRefByName)(windows_core::Interface::as_raw(self), tkresolutionscope, szname.param().abi(), ptr).ok()
    }
    pub unsafe fn DefineImportType<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: P1, tdimport: u32, passememit: P2, ptr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMetaDataAssemblyImport>,
        P1: windows_core::Param<IMetaDataImport>,
        P2: windows_core::Param<IMetaDataAssemblyEmit>,
    {
        (windows_core::Interface::vtable(self).DefineImportType)(windows_core::Interface::as_raw(self), passemimport.param().abi(), pbhashvalue, cbhashvalue, pimport.param().abi(), tdimport, passememit.param().abi(), ptr).ok()
    }
    pub unsafe fn DefineMemberRef<P0>(&self, tkimport: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineMemberRef)(windows_core::Interface::as_raw(self), tkimport, szname.param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn DefineImportMember<P0, P1, P2>(&self, passemimport: P0, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, pimport: P1, mbmember: u32, passememit: P2, tkparent: u32, pmr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMetaDataAssemblyImport>,
        P1: windows_core::Param<IMetaDataImport>,
        P2: windows_core::Param<IMetaDataAssemblyEmit>,
    {
        (windows_core::Interface::vtable(self).DefineImportMember)(windows_core::Interface::as_raw(self), passemimport.param().abi(), pbhashvalue, cbhashvalue, pimport.param().abi(), mbmember, passememit.param().abi(), tkparent, pmr).ok()
    }
    pub unsafe fn DefineEvent<P0>(&self, td: u32, szevent: P0, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32, pmdevent: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineEvent)(windows_core::Interface::as_raw(self), td, szevent.param().abi(), dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods, pmdevent).ok()
    }
    pub unsafe fn SetClassLayout(&self, td: u32, dwpacksize: u32, rfieldoffsets: *mut COR_FIELD_OFFSET, ulclasssize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClassLayout)(windows_core::Interface::as_raw(self), td, dwpacksize, rfieldoffsets, ulclasssize).ok()
    }
    pub unsafe fn DeleteClassLayout(&self, td: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteClassLayout)(windows_core::Interface::as_raw(self), td).ok()
    }
    pub unsafe fn SetFieldMarshal(&self, tk: u32, pvnativetype: *mut u8, cbnativetype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFieldMarshal)(windows_core::Interface::as_raw(self), tk, pvnativetype, cbnativetype).ok()
    }
    pub unsafe fn DeleteFieldMarshal(&self, tk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteFieldMarshal)(windows_core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefinePermissionSet(&self, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefinePermissionSet)(windows_core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn SetRVA(&self, md: u32, ulrva: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRVA)(windows_core::Interface::as_raw(self), md, ulrva).ok()
    }
    pub unsafe fn GetTokenFromSig(&self, pvsig: *mut u8, cbsig: u32, pmsig: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTokenFromSig)(windows_core::Interface::as_raw(self), pvsig, cbsig, pmsig).ok()
    }
    pub unsafe fn DefineModuleRef<P0>(&self, szname: P0, pmur: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineModuleRef)(windows_core::Interface::as_raw(self), szname.param().abi(), pmur).ok()
    }
    pub unsafe fn SetParent(&self, mr: u32, tk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParent)(windows_core::Interface::as_raw(self), mr, tk).ok()
    }
    pub unsafe fn GetTokenFromTypeSpec(&self, pvsig: *mut u8, cbsig: u32, ptypespec: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTokenFromTypeSpec)(windows_core::Interface::as_raw(self), pvsig, cbsig, ptypespec).ok()
    }
    pub unsafe fn SaveToMemory(&self, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveToMemory)(windows_core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn DefineUserString<P0>(&self, szstring: P0, cchstring: u32, pstk: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineUserString)(windows_core::Interface::as_raw(self), szstring.param().abi(), cchstring, pstk).ok()
    }
    pub unsafe fn DeleteToken(&self, tkobj: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteToken)(windows_core::Interface::as_raw(self), tkobj).ok()
    }
    pub unsafe fn SetMethodProps(&self, md: u32, dwmethodflags: u32, ulcoderva: u32, dwimplflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMethodProps)(windows_core::Interface::as_raw(self), md, dwmethodflags, ulcoderva, dwimplflags).ok()
    }
    pub unsafe fn SetTypeDefProps(&self, td: u32, dwtypedefflags: u32, tkextends: u32, rtkimplements: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTypeDefProps)(windows_core::Interface::as_raw(self), td, dwtypedefflags, tkextends, rtkimplements).ok()
    }
    pub unsafe fn SetEventProps(&self, ev: u32, dweventflags: u32, tkeventtype: u32, mdaddon: u32, mdremoveon: u32, mdfire: u32, rmdothermethods: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventProps)(windows_core::Interface::as_raw(self), ev, dweventflags, tkeventtype, mdaddon, mdremoveon, mdfire, rmdothermethods).ok()
    }
    pub unsafe fn SetPermissionSetProps(&self, tk: u32, dwaction: u32, pvpermission: *const core::ffi::c_void, cbpermission: u32, ppm: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPermissionSetProps)(windows_core::Interface::as_raw(self), tk, dwaction, pvpermission, cbpermission, ppm).ok()
    }
    pub unsafe fn DefinePinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefinePinvokeMap)(windows_core::Interface::as_raw(self), tk, dwmappingflags, szimportname.param().abi(), mrimportdll).ok()
    }
    pub unsafe fn SetPinvokeMap<P0>(&self, tk: u32, dwmappingflags: u32, szimportname: P0, mrimportdll: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPinvokeMap)(windows_core::Interface::as_raw(self), tk, dwmappingflags, szimportname.param().abi(), mrimportdll).ok()
    }
    pub unsafe fn DeletePinvokeMap(&self, tk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeletePinvokeMap)(windows_core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn DefineCustomAttribute(&self, tkowner: u32, tkctor: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32, pcv: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefineCustomAttribute)(windows_core::Interface::as_raw(self), tkowner, tkctor, pcustomattribute, cbcustomattribute, pcv).ok()
    }
    pub unsafe fn SetCustomAttributeValue(&self, pcv: u32, pcustomattribute: *const core::ffi::c_void, cbcustomattribute: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCustomAttributeValue)(windows_core::Interface::as_raw(self), pcv, pcustomattribute, cbcustomattribute).ok()
    }
    pub unsafe fn DefineField<P0>(&self, td: u32, szname: P0, dwfieldflags: u32, pvsigblob: *mut u8, cbsigblob: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, pmd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineField)(windows_core::Interface::as_raw(self), td, szname.param().abi(), dwfieldflags, pvsigblob, cbsigblob, dwcplustypeflag, pvalue, cchvalue, pmd).ok()
    }
    pub unsafe fn DefineProperty<P0>(&self, td: u32, szproperty: P0, dwpropflags: u32, pvsig: *mut u8, cbsig: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32, pmdprop: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineProperty)(windows_core::Interface::as_raw(self), td, szproperty.param().abi(), dwpropflags, pvsig, cbsig, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods, pmdprop).ok()
    }
    pub unsafe fn DefineParam<P0>(&self, md: u32, ulparamseq: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, ppd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineParam)(windows_core::Interface::as_raw(self), md, ulparamseq, szname.param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue, ppd).ok()
    }
    pub unsafe fn SetFieldProps(&self, fd: u32, dwfieldflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFieldProps)(windows_core::Interface::as_raw(self), fd, dwfieldflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn SetPropertyProps(&self, pr: u32, dwpropflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32, mdsetter: u32, mdgetter: u32, rmdothermethods: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertyProps)(windows_core::Interface::as_raw(self), pr, dwpropflags, dwcplustypeflag, pvalue, cchvalue, mdsetter, mdgetter, rmdothermethods).ok()
    }
    pub unsafe fn SetParamProps<P0>(&self, pd: u32, szname: P0, dwparamflags: u32, dwcplustypeflag: u32, pvalue: *const core::ffi::c_void, cchvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetParamProps)(windows_core::Interface::as_raw(self), pd, szname.param().abi(), dwparamflags, dwcplustypeflag, pvalue, cchvalue).ok()
    }
    pub unsafe fn DefineSecurityAttributeSet(&self, tkobj: u32, rsecattrs: *mut COR_SECATTR, csecattrs: u32, pulerrorattr: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefineSecurityAttributeSet)(windows_core::Interface::as_raw(self), tkobj, rsecattrs, csecattrs, pulerrorattr).ok()
    }
    pub unsafe fn ApplyEditAndContinue<P0>(&self, pimport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ApplyEditAndContinue)(windows_core::Interface::as_raw(self), pimport.param().abi()).ok()
    }
    pub unsafe fn TranslateSigWithScope<P0, P1, P2, P3>(&self, passemimport: P0, pbhashvalue: *const core::ffi::c_void, cbhashvalue: u32, import: P1, pbsigblob: *mut u8, cbsigblob: u32, passememit: P2, emit: P3, pvtranslatedsig: *mut u8, cbtranslatedsigmax: u32, pcbtranslatedsig: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMetaDataAssemblyImport>,
        P1: windows_core::Param<IMetaDataImport>,
        P2: windows_core::Param<IMetaDataAssemblyEmit>,
        P3: windows_core::Param<IMetaDataEmit>,
    {
        (windows_core::Interface::vtable(self).TranslateSigWithScope)(windows_core::Interface::as_raw(self), passemimport.param().abi(), pbhashvalue, cbhashvalue, import.param().abi(), pbsigblob, cbsigblob, passememit.param().abi(), emit.param().abi(), pvtranslatedsig, cbtranslatedsigmax, pcbtranslatedsig).ok()
    }
    pub unsafe fn SetMethodImplFlags(&self, md: u32, dwimplflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMethodImplFlags)(windows_core::Interface::as_raw(self), md, dwimplflags).ok()
    }
    pub unsafe fn SetFieldRVA(&self, fd: u32, ulrva: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFieldRVA)(windows_core::Interface::as_raw(self), fd, ulrva).ok()
    }
    pub unsafe fn Merge<P0, P1, P2>(&self, pimport: P0, phostmaptoken: P1, phandler: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMetaDataImport>,
        P1: windows_core::Param<IMapToken>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Merge)(windows_core::Interface::as_raw(self), pimport.param().abi(), phostmaptoken.param().abi(), phandler.param().abi()).ok()
    }
    pub unsafe fn MergeEnd(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MergeEnd)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMetaDataEmit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetModuleProps: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveToStream: usize,
    pub GetSaveSize: unsafe extern "system" fn(*mut core::ffi::c_void, CorSaveSize, *mut u32) -> windows_core::HRESULT,
    pub DefineTypeDef: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub DefineNestedType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefineMethod: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut u8, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineMethodImpl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub DefineTypeRefByName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub DefineImportType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DefineMemberRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineImportMember: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, u32, u32, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut COR_FIELD_OFFSET, u32) -> windows_core::HRESULT,
    pub DeleteClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFieldMarshal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32) -> windows_core::HRESULT,
    pub DeleteFieldMarshal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefinePermissionSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetRVA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetTokenFromSig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineModuleRef: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub SetParent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetTokenFromTypeSpec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub SaveToMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefineUserString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub DeleteToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetMethodProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub SetTypeDefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetEventProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetPermissionSetProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DefinePinvokeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetPinvokeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub DeletePinvokeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefineCustomAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetCustomAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefineField: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut u8, u32, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DefineProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut u8, u32, u32, *const core::ffi::c_void, u32, u32, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub DefineParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetFieldProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetPropertyProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *const core::ffi::c_void, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub SetParamProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefineSecurityAttributeSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut COR_SECATTR, u32, *mut u32) -> windows_core::HRESULT,
    pub ApplyEditAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TranslateSigWithScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub SetMethodImplFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetFieldRVA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MergeEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataEmit2, IMetaDataEmit2_Vtbl, 0xf5dd9950_f693_42e6_830e_7b833e8146a9);
impl core::ops::Deref for IMetaDataEmit2 {
    type Target = IMetaDataEmit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataEmit2, windows_core::IUnknown, IMetaDataEmit);
impl IMetaDataEmit2 {
    pub unsafe fn DefineMethodSpec(&self, tkparent: u32, pvsigblob: *mut u8, cbsigblob: u32, pmi: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DefineMethodSpec)(windows_core::Interface::as_raw(self), tkparent, pvsigblob, cbsigblob, pmi).ok()
    }
    pub unsafe fn GetDeltaSaveSize(&self, fsave: CorSaveSize, pdwsavesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeltaSaveSize)(windows_core::Interface::as_raw(self), fsave, pdwsavesize).ok()
    }
    pub unsafe fn SaveDelta<P0>(&self, szfile: P0, dwsaveflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveDelta)(windows_core::Interface::as_raw(self), szfile.param().abi(), dwsaveflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveDeltaToStream<P0>(&self, pistream: P0, dwsaveflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveDeltaToStream)(windows_core::Interface::as_raw(self), pistream.param().abi(), dwsaveflags).ok()
    }
    pub unsafe fn SaveDeltaToMemory(&self, pbdata: *mut core::ffi::c_void, cbdata: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveDeltaToMemory)(windows_core::Interface::as_raw(self), pbdata, cbdata).ok()
    }
    pub unsafe fn DefineGenericParam<P0>(&self, tk: u32, ulparamseq: u32, dwparamflags: u32, szname: P0, reserved: u32, rtkconstraints: *mut u32, pgp: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineGenericParam)(windows_core::Interface::as_raw(self), tk, ulparamseq, dwparamflags, szname.param().abi(), reserved, rtkconstraints, pgp).ok()
    }
    pub unsafe fn SetGenericParamProps<P0>(&self, gp: u32, dwparamflags: u32, szname: P0, reserved: u32, rtkconstraints: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetGenericParamProps)(windows_core::Interface::as_raw(self), gp, dwparamflags, szname.param().abi(), reserved, rtkconstraints).ok()
    }
    pub unsafe fn ResetENCLog(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetENCLog)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMetaDataEmit2_Vtbl {
    pub base__: IMetaDataEmit_Vtbl,
    pub DefineMethodSpec: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetDeltaSaveSize: unsafe extern "system" fn(*mut core::ffi::c_void, CorSaveSize, *mut u32) -> windows_core::HRESULT,
    pub SaveDelta: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveDeltaToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveDeltaToStream: usize,
    pub SaveDeltaToMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DefineGenericParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, windows_core::PCWSTR, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetGenericParamProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub ResetENCLog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataError, IMetaDataError_Vtbl, 0xb81ff171_20f3_11d2_8dcc_00a0c9b09c19);
impl core::ops::Deref for IMetaDataError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataError, windows_core::IUnknown);
impl IMetaDataError {
    pub unsafe fn OnError(&self, hrerror: windows_core::HRESULT, token: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hrerror, token).ok()
    }
}
#[repr(C)]
pub struct IMetaDataError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataFilter, IMetaDataFilter_Vtbl, 0xd0e80dd1_12d4_11d3_b39d_00c04ff81795);
impl core::ops::Deref for IMetaDataFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataFilter, windows_core::IUnknown);
impl IMetaDataFilter {
    pub unsafe fn UnmarkAll(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnmarkAll)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MarkToken(&self, tk: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MarkToken)(windows_core::Interface::as_raw(self), tk).ok()
    }
    pub unsafe fn IsTokenMarked(&self, tk: u32, pismarked: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsTokenMarked)(windows_core::Interface::as_raw(self), tk, pismarked).ok()
    }
}
#[repr(C)]
pub struct IMetaDataFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UnmarkAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MarkToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsTokenMarked: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataImport, IMetaDataImport_Vtbl, 0x7dac8207_d3ae_4c75_9b67_92801a497d44);
impl core::ops::Deref for IMetaDataImport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataImport, windows_core::IUnknown);
impl IMetaDataImport {
    pub unsafe fn CloseEnum(&self, henum: *mut core::ffi::c_void) {
        (windows_core::Interface::vtable(self).CloseEnum)(windows_core::Interface::as_raw(self), henum)
    }
    pub unsafe fn CountEnum(&self, henum: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CountEnum)(windows_core::Interface::as_raw(self), henum, pulcount).ok()
    }
    pub unsafe fn ResetEnum(&self, henum: *mut core::ffi::c_void, ulpos: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetEnum)(windows_core::Interface::as_raw(self), henum, ulpos).ok()
    }
    pub unsafe fn EnumTypeDefs(&self, phenum: *mut *mut core::ffi::c_void, rtypedefs: *mut u32, cmax: u32, pctypedefs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumTypeDefs)(windows_core::Interface::as_raw(self), phenum, rtypedefs, cmax, pctypedefs).ok()
    }
    pub unsafe fn EnumInterfaceImpls(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rimpls: *mut u32, cmax: u32, pcimpls: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumInterfaceImpls)(windows_core::Interface::as_raw(self), phenum, td, rimpls, cmax, pcimpls).ok()
    }
    pub unsafe fn EnumTypeRefs(&self, phenum: *mut *mut core::ffi::c_void, rtyperefs: *mut u32, cmax: u32, pctyperefs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumTypeRefs)(windows_core::Interface::as_raw(self), phenum, rtyperefs, cmax, pctyperefs).ok()
    }
    pub unsafe fn FindTypeDefByName<P0>(&self, sztypedef: P0, tkenclosingclass: u32, ptd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindTypeDefByName)(windows_core::Interface::as_raw(self), sztypedef.param().abi(), tkenclosingclass, ptd).ok()
    }
    pub unsafe fn GetScopeProps(&self, szname: Option<&mut [u16]>, pchname: *mut u32, pmvid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScopeProps)(windows_core::Interface::as_raw(self), core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, pmvid).ok()
    }
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetModuleFromScope)(windows_core::Interface::as_raw(self), pmd).ok()
    }
    pub unsafe fn GetTypeDefProps(&self, td: u32, sztypedef: Option<&mut [u16]>, pchtypedef: *mut u32, pdwtypedefflags: *mut u32, ptkextends: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTypeDefProps)(windows_core::Interface::as_raw(self), td, core::mem::transmute(sztypedef.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), sztypedef.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchtypedef, pdwtypedefflags, ptkextends).ok()
    }
    pub unsafe fn GetInterfaceImplProps(&self, iiimpl: u32, pclass: *mut u32, ptkiface: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInterfaceImplProps)(windows_core::Interface::as_raw(self), iiimpl, pclass, ptkiface).ok()
    }
    pub unsafe fn GetTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: Option<&mut [u16]>, pchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTypeRefProps)(windows_core::Interface::as_raw(self), tr, ptkresolutionscope, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname).ok()
    }
    pub unsafe fn ResolveTypeRef(&self, tr: u32, riid: *const windows_core::GUID, ppiscope: *mut Option<windows_core::IUnknown>, ptd: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResolveTypeRef)(windows_core::Interface::as_raw(self), tr, riid, core::mem::transmute(ppiscope), ptd).ok()
    }
    pub unsafe fn EnumMembers(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMembers)(windows_core::Interface::as_raw(self), phenum, cl, rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMembersWithName<P0>(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: P0, rmembers: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumMembersWithName)(windows_core::Interface::as_raw(self), phenum, cl, szname.param().abi(), rmembers, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethods(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMethods)(windows_core::Interface::as_raw(self), phenum, cl, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodsWithName<P0>(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: P0, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumMethodsWithName)(windows_core::Interface::as_raw(self), phenum, cl, szname.param().abi(), rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFields(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumFields)(windows_core::Interface::as_raw(self), phenum, cl, rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumFieldsWithName<P0>(&self, phenum: *mut *mut core::ffi::c_void, cl: u32, szname: P0, rfields: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).EnumFieldsWithName)(windows_core::Interface::as_raw(self), phenum, cl, szname.param().abi(), rfields, cmax, pctokens).ok()
    }
    pub unsafe fn EnumParams(&self, phenum: *mut *mut core::ffi::c_void, mb: u32, rparams: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumParams)(windows_core::Interface::as_raw(self), phenum, mb, rparams, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMemberRefs(&self, phenum: *mut *mut core::ffi::c_void, tkparent: u32, rmemberrefs: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMemberRefs)(windows_core::Interface::as_raw(self), phenum, tkparent, rmemberrefs, cmax, pctokens).ok()
    }
    pub unsafe fn EnumMethodImpls(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rmethodbody: *mut u32, rmethoddecl: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMethodImpls)(windows_core::Interface::as_raw(self), phenum, td, rmethodbody, rmethoddecl, cmax, pctokens).ok()
    }
    pub unsafe fn EnumPermissionSets(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, dwactions: u32, rpermission: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumPermissionSets)(windows_core::Interface::as_raw(self), phenum, tk, dwactions, rpermission, cmax, pctokens).ok()
    }
    pub unsafe fn FindMember<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindMember)(windows_core::Interface::as_raw(self), td, szname.param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMethod<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindMethod)(windows_core::Interface::as_raw(self), td, szname.param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindField<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmb: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindField)(windows_core::Interface::as_raw(self), td, szname.param().abi(), pvsigblob, cbsigblob, pmb).ok()
    }
    pub unsafe fn FindMemberRef<P0>(&self, td: u32, szname: P0, pvsigblob: *mut u8, cbsigblob: u32, pmr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindMemberRef)(windows_core::Interface::as_raw(self), td, szname.param().abi(), pvsigblob, cbsigblob, pmr).ok()
    }
    pub unsafe fn GetMethodProps(&self, mb: u32, pclass: *mut u32, szmethod: Option<&mut [u16]>, pchmethod: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMethodProps)(windows_core::Interface::as_raw(self), mb, pclass, core::mem::transmute(szmethod.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szmethod.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchmethod, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetMemberRefProps(&self, mr: u32, ptk: *mut u32, szmember: Option<&mut [u16]>, pchmember: *mut u32, ppvsigblob: *mut *mut u8, pbsig: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMemberRefProps)(windows_core::Interface::as_raw(self), mr, ptk, core::mem::transmute(szmember.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchmember, ppvsigblob, pbsig).ok()
    }
    pub unsafe fn EnumProperties(&self, phenum: *mut *mut core::ffi::c_void, td: u32, rproperties: *mut u32, cmax: u32, pcproperties: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumProperties)(windows_core::Interface::as_raw(self), phenum, td, rproperties, cmax, pcproperties).ok()
    }
    pub unsafe fn EnumEvents(&self, phenum: *mut *mut core::ffi::c_void, td: u32, revents: *mut u32, cmax: u32, pcevents: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumEvents)(windows_core::Interface::as_raw(self), phenum, td, revents, cmax, pcevents).ok()
    }
    pub unsafe fn GetEventProps<P0>(&self, ev: u32, pclass: *mut u32, szevent: P0, cchevent: u32, pchevent: *mut u32, pdweventflags: *mut u32, ptkeventtype: *mut u32, pmdaddon: *mut u32, pmdremoveon: *mut u32, pmdfire: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetEventProps)(windows_core::Interface::as_raw(self), ev, pclass, szevent.param().abi(), cchevent, pchevent, pdweventflags, ptkeventtype, pmdaddon, pmdremoveon, pmdfire, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn EnumMethodSemantics(&self, phenum: *mut *mut core::ffi::c_void, mb: u32, reventprop: *mut u32, cmax: u32, pceventprop: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMethodSemantics)(windows_core::Interface::as_raw(self), phenum, mb, reventprop, cmax, pceventprop).ok()
    }
    pub unsafe fn GetMethodSemantics(&self, mb: u32, tkeventprop: u32, pdwsemanticsflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMethodSemantics)(windows_core::Interface::as_raw(self), mb, tkeventprop, pdwsemanticsflags).ok()
    }
    pub unsafe fn GetClassLayout(&self, td: u32, pdwpacksize: *mut u32, rfieldoffset: *mut COR_FIELD_OFFSET, cmax: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassLayout)(windows_core::Interface::as_raw(self), td, pdwpacksize, rfieldoffset, cmax, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetFieldMarshal(&self, tk: u32, ppvnativetype: *mut *mut u8, pcbnativetype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFieldMarshal)(windows_core::Interface::as_raw(self), tk, ppvnativetype, pcbnativetype).ok()
    }
    pub unsafe fn GetRVA(&self, tk: u32, pulcoderva: *mut u32, pdwimplflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRVA)(windows_core::Interface::as_raw(self), tk, pulcoderva, pdwimplflags).ok()
    }
    pub unsafe fn GetPermissionSetProps(&self, pm: u32, pdwaction: *mut u32, ppvpermission: *const *const core::ffi::c_void, pcbpermission: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPermissionSetProps)(windows_core::Interface::as_raw(self), pm, pdwaction, ppvpermission, pcbpermission).ok()
    }
    pub unsafe fn GetSigFromToken(&self, mdsig: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSigFromToken)(windows_core::Interface::as_raw(self), mdsig, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetModuleRefProps(&self, mur: u32, szname: Option<&mut [u16]>, pchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetModuleRefProps)(windows_core::Interface::as_raw(self), mur, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname).ok()
    }
    pub unsafe fn EnumModuleRefs(&self, phenum: *mut *mut core::ffi::c_void, rmodulerefs: *mut u32, cmax: u32, pcmodulerefs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumModuleRefs)(windows_core::Interface::as_raw(self), phenum, rmodulerefs, cmax, pcmodulerefs).ok()
    }
    pub unsafe fn GetTypeSpecFromToken(&self, typespec: u32, ppvsig: *mut *mut u8, pcbsig: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTypeSpecFromToken)(windows_core::Interface::as_raw(self), typespec, ppvsig, pcbsig).ok()
    }
    pub unsafe fn GetNameFromToken(&self, tk: u32, pszutf8nameptr: *mut *mut i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNameFromToken)(windows_core::Interface::as_raw(self), tk, pszutf8nameptr).ok()
    }
    pub unsafe fn EnumUnresolvedMethods(&self, phenum: *mut *mut core::ffi::c_void, rmethods: *mut u32, cmax: u32, pctokens: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumUnresolvedMethods)(windows_core::Interface::as_raw(self), phenum, rmethods, cmax, pctokens).ok()
    }
    pub unsafe fn GetUserString(&self, stk: u32, szstring: Option<&mut [u16]>, pchstring: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserString)(windows_core::Interface::as_raw(self), stk, core::mem::transmute(szstring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szstring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchstring).ok()
    }
    pub unsafe fn GetPinvokeMap(&self, tk: u32, pdwmappingflags: *mut u32, szimportname: Option<&mut [u16]>, pchimportname: *mut u32, pmrimportdll: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPinvokeMap)(windows_core::Interface::as_raw(self), tk, pdwmappingflags, core::mem::transmute(szimportname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szimportname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchimportname, pmrimportdll).ok()
    }
    pub unsafe fn EnumSignatures(&self, phenum: *mut *mut core::ffi::c_void, rsignatures: *mut u32, cmax: u32, pcsignatures: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumSignatures)(windows_core::Interface::as_raw(self), phenum, rsignatures, cmax, pcsignatures).ok()
    }
    pub unsafe fn EnumTypeSpecs(&self, phenum: *mut *mut core::ffi::c_void, rtypespecs: *mut u32, cmax: u32, pctypespecs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumTypeSpecs)(windows_core::Interface::as_raw(self), phenum, rtypespecs, cmax, pctypespecs).ok()
    }
    pub unsafe fn EnumUserStrings(&self, phenum: *mut *mut core::ffi::c_void, rstrings: *mut u32, cmax: u32, pcstrings: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumUserStrings)(windows_core::Interface::as_raw(self), phenum, rstrings, cmax, pcstrings).ok()
    }
    pub unsafe fn GetParamForMethodIndex(&self, md: u32, ulparamseq: u32, ppd: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParamForMethodIndex)(windows_core::Interface::as_raw(self), md, ulparamseq, ppd).ok()
    }
    pub unsafe fn EnumCustomAttributes(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, tktype: u32, rcustomattributes: *mut u32, cmax: u32, pccustomattributes: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumCustomAttributes)(windows_core::Interface::as_raw(self), phenum, tk, tktype, rcustomattributes, cmax, pccustomattributes).ok()
    }
    pub unsafe fn GetCustomAttributeProps(&self, cv: u32, ptkobj: *mut u32, ptktype: *mut u32, ppblob: *const *const core::ffi::c_void, pcbsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCustomAttributeProps)(windows_core::Interface::as_raw(self), cv, ptkobj, ptktype, ppblob, pcbsize).ok()
    }
    pub unsafe fn FindTypeRef<P0>(&self, tkresolutionscope: u32, szname: P0, ptr: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindTypeRef)(windows_core::Interface::as_raw(self), tkresolutionscope, szname.param().abi(), ptr).ok()
    }
    pub unsafe fn GetMemberProps(&self, mb: u32, pclass: *mut u32, szmember: Option<&mut [u16]>, pchmember: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pulcoderva: *mut u32, pdwimplflags: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMemberProps)(windows_core::Interface::as_raw(self), mb, pclass, core::mem::transmute(szmember.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szmember.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchmember, pdwattr, ppvsigblob, pcbsigblob, pulcoderva, pdwimplflags, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetFieldProps(&self, mb: u32, pclass: *mut u32, szfield: Option<&mut [u16]>, pchfield: *mut u32, pdwattr: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFieldProps)(windows_core::Interface::as_raw(self), mb, pclass, core::mem::transmute(szfield.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szfield.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchfield, pdwattr, ppvsigblob, pcbsigblob, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetPropertyProps<P0>(&self, prop: u32, pclass: *mut u32, szproperty: P0, cchproperty: u32, pchproperty: *mut u32, pdwpropflags: *mut u32, ppvsig: *mut *mut u8, pbsig: *mut u32, pdwcplustypeflag: *mut u32, ppdefaultvalue: *mut *mut core::ffi::c_void, pcchdefaultvalue: *mut u32, pmdsetter: *mut u32, pmdgetter: *mut u32, rmdothermethod: *mut u32, cmax: u32, pcothermethod: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyProps)(windows_core::Interface::as_raw(self), prop, pclass, szproperty.param().abi(), cchproperty, pchproperty, pdwpropflags, ppvsig, pbsig, pdwcplustypeflag, ppdefaultvalue, pcchdefaultvalue, pmdsetter, pmdgetter, rmdothermethod, cmax, pcothermethod).ok()
    }
    pub unsafe fn GetParamProps(&self, tk: u32, pmd: *mut u32, pulsequence: *mut u32, szname: Option<&mut [u16]>, pchname: *mut u32, pdwattr: *mut u32, pdwcplustypeflag: *mut u32, ppvalue: *mut *mut core::ffi::c_void, pcchvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParamProps)(windows_core::Interface::as_raw(self), tk, pmd, pulsequence, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname, pdwattr, pdwcplustypeflag, ppvalue, pcchvalue).ok()
    }
    pub unsafe fn GetCustomAttributeByName<P0>(&self, tkobj: u32, szname: P0, ppdata: *const *const core::ffi::c_void, pcbdata: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetCustomAttributeByName)(windows_core::Interface::as_raw(self), tkobj, szname.param().abi(), ppdata, pcbdata).ok()
    }
    pub unsafe fn IsValidToken(&self, tk: u32) -> super::super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsValidToken)(windows_core::Interface::as_raw(self), tk)
    }
    pub unsafe fn GetNestedClassProps(&self, tdnestedclass: u32, ptdenclosingclass: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNestedClassProps)(windows_core::Interface::as_raw(self), tdnestedclass, ptdenclosingclass).ok()
    }
    pub unsafe fn GetNativeCallConvFromSig(&self, pvsig: *const core::ffi::c_void, cbsig: u32, pcallconv: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNativeCallConvFromSig)(windows_core::Interface::as_raw(self), pvsig, cbsig, pcallconv).ok()
    }
    pub unsafe fn IsGlobal(&self, pd: u32, pbglobal: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsGlobal)(windows_core::Interface::as_raw(self), pd, pbglobal).ok()
    }
}
#[repr(C)]
pub struct IMetaDataImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CloseEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub CountEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ResetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumTypeDefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumInterfaceImpls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumTypeRefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub FindTypeDefByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetScopeProps: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetModuleFromScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeDefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetInterfaceImplProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetTypeRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub ResolveTypeRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMembersWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMethodsWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumFieldsWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMemberRefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMethodImpls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumPermissionSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub FindMember: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub FindMethod: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub FindField: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub FindMemberRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMethodProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut *mut u8, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetMemberRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetEventProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PCWSTR, u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMethodSemantics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMethodSemantics: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut COR_FIELD_OFFSET, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetFieldMarshal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetRVA: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPermissionSetProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *const *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSigFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetModuleRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumModuleRefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetTypeSpecFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetNameFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut i8) -> windows_core::HRESULT,
    pub EnumUnresolvedMethods: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetUserString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetPinvokeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EnumSignatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumTypeSpecs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumUserStrings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetParamForMethodIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumCustomAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetCustomAttributeProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *const *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FindTypeRef: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetMemberProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut *mut u8, *mut u32, *mut u32, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFieldProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut *mut u8, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PCWSTR, u32, *mut u32, *mut u32, *mut *mut u8, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetParamProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, windows_core::PWSTR, u32, *mut u32, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCustomAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsValidToken: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::super::super::Foundation::BOOL,
    pub GetNestedClassProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetNativeCallConvFromSig: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub IsGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataImport2, IMetaDataImport2_Vtbl, 0xfce5efa0_8bba_4f8e_a036_8f2022b08466);
impl core::ops::Deref for IMetaDataImport2 {
    type Target = IMetaDataImport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataImport2, windows_core::IUnknown, IMetaDataImport);
impl IMetaDataImport2 {
    pub unsafe fn EnumGenericParams(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparams: *mut u32, cmax: u32, pcgenericparams: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumGenericParams)(windows_core::Interface::as_raw(self), phenum, tk, rgenericparams, cmax, pcgenericparams).ok()
    }
    pub unsafe fn GetGenericParamProps(&self, gp: u32, pulparamseq: *mut u32, pdwparamflags: *mut u32, ptowner: *mut u32, reserved: *mut u32, wzname: Option<&mut [u16]>, pchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenericParamProps)(windows_core::Interface::as_raw(self), gp, pulparamseq, pdwparamflags, ptowner, reserved, core::mem::transmute(wzname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), wzname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname).ok()
    }
    pub unsafe fn GetMethodSpecProps(&self, mi: u32, tkparent: *mut u32, ppvsigblob: *mut *mut u8, pcbsigblob: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMethodSpecProps)(windows_core::Interface::as_raw(self), mi, tkparent, ppvsigblob, pcbsigblob).ok()
    }
    pub unsafe fn EnumGenericParamConstraints(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rgenericparamconstraints: *mut u32, cmax: u32, pcgenericparamconstraints: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumGenericParamConstraints)(windows_core::Interface::as_raw(self), phenum, tk, rgenericparamconstraints, cmax, pcgenericparamconstraints).ok()
    }
    pub unsafe fn GetGenericParamConstraintProps(&self, gpc: u32, ptgenericparam: *mut u32, ptkconstrainttype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenericParamConstraintProps)(windows_core::Interface::as_raw(self), gpc, ptgenericparam, ptkconstrainttype).ok()
    }
    pub unsafe fn GetPEKind(&self, pdwpekind: *mut u32, pdwmachine: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPEKind)(windows_core::Interface::as_raw(self), pdwpekind, pdwmachine).ok()
    }
    pub unsafe fn GetVersionString(&self, pwzbuf: Option<&mut [u16]>, pccbufsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVersionString)(windows_core::Interface::as_raw(self), core::mem::transmute(pwzbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwzbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pccbufsize).ok()
    }
    pub unsafe fn EnumMethodSpecs(&self, phenum: *mut *mut core::ffi::c_void, tk: u32, rmethodspecs: *mut u32, cmax: u32, pcmethodspecs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumMethodSpecs)(windows_core::Interface::as_raw(self), phenum, tk, rmethodspecs, cmax, pcmethodspecs).ok()
    }
}
#[repr(C)]
pub struct IMetaDataImport2_Vtbl {
    pub base__: IMetaDataImport_Vtbl,
    pub EnumGenericParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetGenericParamProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32, *mut u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMethodSpecProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub EnumGenericParamConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetGenericParamConstraintProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPEKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub EnumMethodSpecs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataInfo, IMetaDataInfo_Vtbl, 0x7998ea64_7f95_48b8_86fc_17caf48bf5cb);
impl core::ops::Deref for IMetaDataInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataInfo, windows_core::IUnknown);
impl IMetaDataInfo {
    pub unsafe fn GetFileMapping(&self, ppvdata: *const *const core::ffi::c_void, pcbdata: *mut u64, pdwmappingtype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileMapping)(windows_core::Interface::as_raw(self), ppvdata, pcbdata, pdwmappingtype).ok()
    }
}
#[repr(C)]
pub struct IMetaDataInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void, *mut u64, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataTables, IMetaDataTables_Vtbl, 0xd8f579ab_402d_4b8e_82d9_5d63b1065c68);
impl core::ops::Deref for IMetaDataTables {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataTables, windows_core::IUnknown);
impl IMetaDataTables {
    pub unsafe fn GetStringHeapSize(&self, pcbstrings: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringHeapSize)(windows_core::Interface::as_raw(self), pcbstrings).ok()
    }
    pub unsafe fn GetBlobHeapSize(&self, pcbblobs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBlobHeapSize)(windows_core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetGuidHeapSize(&self, pcbguids: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGuidHeapSize)(windows_core::Interface::as_raw(self), pcbguids).ok()
    }
    pub unsafe fn GetUserStringHeapSize(&self, pcbblobs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserStringHeapSize)(windows_core::Interface::as_raw(self), pcbblobs).ok()
    }
    pub unsafe fn GetNumTables(&self, pctables: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNumTables)(windows_core::Interface::as_raw(self), pctables).ok()
    }
    pub unsafe fn GetTableIndex(&self, token: u32, pixtbl: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTableIndex)(windows_core::Interface::as_raw(self), token, pixtbl).ok()
    }
    pub unsafe fn GetTableInfo(&self, ixtbl: u32, pcbrow: *mut u32, pcrows: *mut u32, pccols: *mut u32, pikey: *mut u32, ppname: *const *const i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTableInfo)(windows_core::Interface::as_raw(self), ixtbl, pcbrow, pcrows, pccols, pikey, ppname).ok()
    }
    pub unsafe fn GetColumnInfo(&self, ixtbl: u32, ixcol: u32, pocol: *mut u32, pcbcol: *mut u32, ptype: *mut u32, ppname: *const *const i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnInfo)(windows_core::Interface::as_raw(self), ixtbl, ixcol, pocol, pcbcol, ptype, ppname).ok()
    }
    pub unsafe fn GetCodedTokenInfo(&self, ixcdtkn: u32, pctokens: *mut u32, pptokens: *mut *mut u32, ppname: *const *const i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodedTokenInfo)(windows_core::Interface::as_raw(self), ixcdtkn, pctokens, pptokens, ppname).ok()
    }
    pub unsafe fn GetRow(&self, ixtbl: u32, rid: u32, pprow: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRow)(windows_core::Interface::as_raw(self), ixtbl, rid, pprow).ok()
    }
    pub unsafe fn GetColumn(&self, ixtbl: u32, ixcol: u32, rid: u32, pval: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), ixtbl, ixcol, rid, pval).ok()
    }
    pub unsafe fn GetString(&self, ixstring: u32, ppstring: *const *const i8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), ixstring, ppstring).ok()
    }
    pub unsafe fn GetBlob(&self, ixblob: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBlob)(windows_core::Interface::as_raw(self), ixblob, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetGuid(&self, ixguid: u32, ppguid: *const *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), ixguid, ppguid).ok()
    }
    pub unsafe fn GetUserString(&self, ixuserstring: u32, pcbdata: *mut u32, ppdata: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserString)(windows_core::Interface::as_raw(self), ixuserstring, pcbdata, ppdata).ok()
    }
    pub unsafe fn GetNextString(&self, ixstring: u32, pnext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextString)(windows_core::Interface::as_raw(self), ixstring, pnext).ok()
    }
    pub unsafe fn GetNextBlob(&self, ixblob: u32, pnext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextBlob)(windows_core::Interface::as_raw(self), ixblob, pnext).ok()
    }
    pub unsafe fn GetNextGuid(&self, ixguid: u32, pnext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextGuid)(windows_core::Interface::as_raw(self), ixguid, pnext).ok()
    }
    pub unsafe fn GetNextUserString(&self, ixuserstring: u32, pnext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextUserString)(windows_core::Interface::as_raw(self), ixuserstring, pnext).ok()
    }
}
#[repr(C)]
pub struct IMetaDataTables_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStringHeapSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetBlobHeapSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGuidHeapSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetUserStringHeapSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNumTables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTableIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetTableInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32, *mut u32, *const *const i8) -> windows_core::HRESULT,
    pub GetColumnInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, *mut u32, *mut u32, *const *const i8) -> windows_core::HRESULT,
    pub GetCodedTokenInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut u32, *const *const i8) -> windows_core::HRESULT,
    pub GetRow: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const i8) -> windows_core::HRESULT,
    pub GetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetUserString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetNextBlob: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetNextGuid: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetNextUserString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataTables2, IMetaDataTables2_Vtbl, 0xbadb5f70_58da_43a9_a1c6_d74819f19b15);
impl core::ops::Deref for IMetaDataTables2 {
    type Target = IMetaDataTables;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataTables2, windows_core::IUnknown, IMetaDataTables);
impl IMetaDataTables2 {
    pub unsafe fn GetMetaDataStorage(&self, ppvmd: *const *const core::ffi::c_void, pcbmd: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMetaDataStorage)(windows_core::Interface::as_raw(self), ppvmd, pcbmd).ok()
    }
    pub unsafe fn GetMetaDataStreamInfo(&self, ix: u32, ppchname: *const *const i8, ppv: *const *const core::ffi::c_void, pcb: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMetaDataStreamInfo)(windows_core::Interface::as_raw(self), ix, ppchname, ppv, pcb).ok()
    }
}
#[repr(C)]
pub struct IMetaDataTables2_Vtbl {
    pub base__: IMetaDataTables_Vtbl,
    pub GetMetaDataStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMetaDataStreamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const i8, *const *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataValidate, IMetaDataValidate_Vtbl, 0x4709c9c6_81ff_11d3_9fc7_00c04f79a0a3);
impl core::ops::Deref for IMetaDataValidate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataValidate, windows_core::IUnknown);
impl IMetaDataValidate {
    pub unsafe fn ValidatorInit<P0>(&self, dwmoduletype: u32, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ValidatorInit)(windows_core::Interface::as_raw(self), dwmoduletype, punk.param().abi()).ok()
    }
    pub unsafe fn ValidateMetaData(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ValidateMetaData)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IMetaDataValidate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ValidatorInit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateMetaData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMetaDataWinMDImport, IMetaDataWinMDImport_Vtbl, 0x969ea0c5_964e_411b_a807_b0f3c2dfcbd4);
impl core::ops::Deref for IMetaDataWinMDImport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMetaDataWinMDImport, windows_core::IUnknown);
impl IMetaDataWinMDImport {
    pub unsafe fn GetUntransformedTypeRefProps(&self, tr: u32, ptkresolutionscope: *mut u32, szname: Option<&mut [u16]>, pchname: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUntransformedTypeRefProps)(windows_core::Interface::as_raw(self), tr, ptkresolutionscope, core::mem::transmute(szname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pchname).ok()
    }
}
#[repr(C)]
pub struct IMetaDataWinMDImport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUntransformedTypeRefProps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRoMetaDataLocator, IRoMetaDataLocator_Vtbl);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IRoSimpleMetaDataBuilder>,
    {
        (windows_core::Interface::vtable(self).Locate)(windows_core::Interface::as_raw(self), nameelement.param().abi(), metadatadestination.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRoSimpleMetaDataBuilder, IRoSimpleMetaDataBuilder_Vtbl);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWinRtInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetDelegate(&self, iid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)).ok()
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), core::mem::transmute(defaultinterfaceiid.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), core::mem::transmute(defaultinterfaceiid.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())).ok()
    }
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[windows_core::PCWSTR]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStruct)(windows_core::Interface::as_raw(self), name.param().abi(), fieldtypenames.len().try_into().unwrap(), core::mem::transmute(fieldtypenames.as_ptr())).ok()
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetEnum)(windows_core::Interface::as_raw(self), name.param().abi(), basetype.param().abi()).ok()
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameterizedInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs).ok()
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameterizedDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs).ok()
    }
}
#[repr(C)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub const ASSEMBLY_METADATA_TYPE: windows_core::PCSTR = windows_core::s!("System.Reflection.AssemblyMetadataAttribute");
pub const ASSEMBLY_METADATA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Reflection.AssemblyMetadataAttribute");
pub const CLSID_CLR_v1_MetaData: windows_core::GUID = windows_core::GUID::from_u128(0x005023ca_72b1_11d3_9fc4_00c04f79a0a3);
pub const CLSID_CLR_v2_MetaData: windows_core::GUID = windows_core::GUID::from_u128(0xefea471a_44fd_4862_9292_0c58d46e1f3a);
pub const CLSID_Cor: windows_core::GUID = windows_core::GUID::from_u128(0xbee00010_ee77_11d0_a015_00c04fbbb884);
pub const CLSID_CorMetaDataDispenser: windows_core::GUID = windows_core::GUID::from_u128(0xe5cb7a31_7512_11d2_89ce_0080c792e5d8);
pub const CLSID_CorMetaDataDispenserReg: windows_core::GUID = windows_core::GUID::from_u128(0x435755ff_7397_11d2_9771_00a0c9b4d50c);
pub const CLSID_CorMetaDataDispenserRuntime: windows_core::GUID = windows_core::GUID::from_u128(0x1ec2de53_75cc_11d2_9775_00a0c9b4d50c);
pub const CLSID_CorMetaDataReg: windows_core::GUID = windows_core::GUID::from_u128(0x87f3a1f5_7397_11d2_9771_00a0c9b4d50c);
pub const CMOD_CALLCONV_NAMESPACE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices");
pub const CMOD_CALLCONV_NAMESPACE_OLD: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices");
pub const CMOD_CALLCONV_NAME_CDECL: windows_core::PCSTR = windows_core::s!("CallConvCdecl");
pub const CMOD_CALLCONV_NAME_FASTCALL: windows_core::PCSTR = windows_core::s!("CallConvFastcall");
pub const CMOD_CALLCONV_NAME_STDCALL: windows_core::PCSTR = windows_core::s!("CallConvStdcall");
pub const CMOD_CALLCONV_NAME_THISCALL: windows_core::PCSTR = windows_core::s!("CallConvThiscall");
pub const COINITCOR_DEFAULT: COINITICOR = COINITICOR(0i32);
pub const COINITEE_DEFAULT: COINITIEE = COINITIEE(0i32);
pub const COINITEE_DLL: COINITIEE = COINITIEE(1i32);
pub const COINITEE_MAIN: COINITIEE = COINITIEE(2i32);
pub const COMPILATIONRELAXATIONS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
pub const COMPILATIONRELAXATIONS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS: windows_core::PCWSTR = windows_core::w!("System.Security.Permissions.SecurityAttribute");
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.Permissions.SecurityAttribute");
pub const COR_CCTOR_METHOD_NAME: windows_core::PCSTR = windows_core::s!(".cctor");
pub const COR_CCTOR_METHOD_NAME_W: windows_core::PCWSTR = windows_core::w!(".cctor");
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DiscardableAttribute");
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE_ASNI: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DiscardableAttribute");
pub const COR_CTOR_METHOD_NAME: windows_core::PCSTR = windows_core::s!(".ctor");
pub const COR_CTOR_METHOD_NAME_W: windows_core::PCWSTR = windows_core::w!(".ctor");
pub const COR_DELETED_NAME_A: windows_core::PCSTR = windows_core::s!("_Deleted");
pub const COR_DELETED_NAME_W: windows_core::PCWSTR = windows_core::w!("_Deleted");
pub const COR_ENUM_FIELD_NAME: windows_core::PCSTR = windows_core::s!("value__");
pub const COR_ENUM_FIELD_NAME_W: windows_core::PCWSTR = windows_core::w!("value__");
pub const COR_E_AMBIGUOUSMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8000211D_u32 as _);
pub const COR_E_ARGUMENT: i32 = -2147024809i32;
pub const COR_E_BADIMAGEFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8007000B_u32 as _);
pub const COR_E_DIVIDEBYZERO: windows_core::HRESULT = windows_core::HRESULT(0x80020012_u32 as _);
pub const COR_E_INVALIDCAST: i32 = -2147467262i32;
pub const COR_E_NULLREFERENCE: i32 = -2147467261i32;
pub const COR_E_OUTOFMEMORY: i32 = -2147024882i32;
pub const COR_E_TARGETPARAMCOUNT: windows_core::HRESULT = windows_core::HRESULT(0x8002000E_u32 as _);
pub const COR_E_UNAUTHORIZEDACCESS: i32 = -2147024891i32;
pub const COR_ILEXCEPTION_CLAUSE_DEPRECATED: CorExceptionFlag = CorExceptionFlag(0i32);
pub const COR_ILEXCEPTION_CLAUSE_DUPLICATED: CorExceptionFlag = CorExceptionFlag(8i32);
pub const COR_ILEXCEPTION_CLAUSE_FAULT: CorExceptionFlag = CorExceptionFlag(4i32);
pub const COR_ILEXCEPTION_CLAUSE_FILTER: CorExceptionFlag = CorExceptionFlag(1i32);
pub const COR_ILEXCEPTION_CLAUSE_FINALLY: CorExceptionFlag = CorExceptionFlag(2i32);
pub const COR_ILEXCEPTION_CLAUSE_NONE: CorExceptionFlag = CorExceptionFlag(0i32);
pub const COR_ILEXCEPTION_CLAUSE_OFFSETLEN: CorExceptionFlag = CorExceptionFlag(0i32);
pub const COR_NATIVE_LINK_CUSTOM_VALUE: windows_core::PCWSTR = windows_core::w!("COMPLUS_NativeLink");
pub const COR_NATIVE_LINK_CUSTOM_VALUE_ANSI: windows_core::PCSTR = windows_core::s!("COMPLUS_NativeLink");
pub const COR_NATIVE_LINK_CUSTOM_VALUE_CC: u32 = 18u32;
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.DynamicSecurityMethodAttribute");
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.DynamicSecurityMethodAttribute");
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
pub const COR_UNVER_CODE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.UnverifiableCodeAttribute");
pub const COR_UNVER_CODE_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.UnverifiableCodeAttribute");
pub const COR_VTABLEGAP_NAME_A: windows_core::PCSTR = windows_core::s!("_VtblGap");
pub const COR_VTABLEGAP_NAME_W: windows_core::PCWSTR = windows_core::w!("_VtblGap");
pub const COUNINITEE_DEFAULT: COUNINITIEE = COUNINITIEE(0i32);
pub const COUNINITEE_DLL: COUNINITIEE = COUNINITIEE(1i32);
pub const CompilationRelaxations_NoStringInterning: CompilationRelaxationsEnum = CompilationRelaxationsEnum(8i32);
pub const CorILMethod_CompressedIL: CorILMethodFlags = CorILMethodFlags(64i32);
pub const CorILMethod_FatFormat: CorILMethodFlags = CorILMethodFlags(3i32);
pub const CorILMethod_FormatMask: CorILMethodFlags = CorILMethodFlags(7i32);
pub const CorILMethod_FormatShift: CorILMethodFlags = CorILMethodFlags(3i32);
pub const CorILMethod_InitLocals: CorILMethodFlags = CorILMethodFlags(16i32);
pub const CorILMethod_MoreSects: CorILMethodFlags = CorILMethodFlags(8i32);
pub const CorILMethod_Sect_EHTable: CorILMethodSect = CorILMethodSect(1i32);
pub const CorILMethod_Sect_FatFormat: CorILMethodSect = CorILMethodSect(64i32);
pub const CorILMethod_Sect_KindMask: CorILMethodSect = CorILMethodSect(63i32);
pub const CorILMethod_Sect_MoreSects: CorILMethodSect = CorILMethodSect(128i32);
pub const CorILMethod_Sect_OptILTable: CorILMethodSect = CorILMethodSect(2i32);
pub const CorILMethod_Sect_Reserved: CorILMethodSect = CorILMethodSect(0i32);
pub const CorILMethod_SmallFormat: CorILMethodFlags = CorILMethodFlags(0i32);
pub const CorILMethod_TinyFormat: CorILMethodFlags = CorILMethodFlags(2i32);
pub const CorILMethod_TinyFormat1: CorILMethodFlags = CorILMethodFlags(6i32);
pub const DEFAULTDEPENDENCY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
pub const DEFAULTDEPENDENCY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE: windows_core::PCSTR = windows_core::s!("System.LoaderOptimizationAttribute");
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.LoaderOptimizationAttribute");
pub const DEFAULTDOMAIN_MTA_TYPE: windows_core::PCSTR = windows_core::s!("System.MTAThreadAttribute");
pub const DEFAULTDOMAIN_MTA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.MTAThreadAttribute");
pub const DEFAULTDOMAIN_STA_TYPE: windows_core::PCSTR = windows_core::s!("System.STAThreadAttribute");
pub const DEFAULTDOMAIN_STA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.STAThreadAttribute");
pub const DEPENDENCY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DependencyAttribute");
pub const DEPENDENCY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DependencyAttribute");
pub const DESCR_GROUP_METHODDEF: i32 = 0i32;
pub const DESCR_GROUP_METHODIMPL: i32 = 1i32;
pub const DISABLED_PRIVATE_REFLECTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
pub const DISABLED_PRIVATE_REFLECTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
pub const DropMemberRefCAs: MergeFlags = MergeFlags(2i32);
pub const ELEMENT_TYPE_ARRAY: CorElementType = CorElementType(20u8);
pub const ELEMENT_TYPE_BOOLEAN: CorElementType = CorElementType(2u8);
pub const ELEMENT_TYPE_BYREF: CorElementType = CorElementType(16u8);
pub const ELEMENT_TYPE_CHAR: CorElementType = CorElementType(3u8);
pub const ELEMENT_TYPE_CLASS: CorElementType = CorElementType(18u8);
pub const ELEMENT_TYPE_CMOD_OPT: CorElementType = CorElementType(32u8);
pub const ELEMENT_TYPE_CMOD_REQD: CorElementType = CorElementType(31u8);
pub const ELEMENT_TYPE_END: CorElementType = CorElementType(0u8);
pub const ELEMENT_TYPE_FNPTR: CorElementType = CorElementType(27u8);
pub const ELEMENT_TYPE_GENERICINST: CorElementType = CorElementType(21u8);
pub const ELEMENT_TYPE_I: CorElementType = CorElementType(24u8);
pub const ELEMENT_TYPE_I1: CorElementType = CorElementType(4u8);
pub const ELEMENT_TYPE_I2: CorElementType = CorElementType(6u8);
pub const ELEMENT_TYPE_I4: CorElementType = CorElementType(8u8);
pub const ELEMENT_TYPE_I8: CorElementType = CorElementType(10u8);
pub const ELEMENT_TYPE_INTERNAL: CorElementType = CorElementType(33u8);
pub const ELEMENT_TYPE_MAX: CorElementType = CorElementType(34u8);
pub const ELEMENT_TYPE_MODIFIER: CorElementType = CorElementType(64u8);
pub const ELEMENT_TYPE_MVAR: CorElementType = CorElementType(30u8);
pub const ELEMENT_TYPE_OBJECT: CorElementType = CorElementType(28u8);
pub const ELEMENT_TYPE_PINNED: CorElementType = CorElementType(69u8);
pub const ELEMENT_TYPE_PTR: CorElementType = CorElementType(15u8);
pub const ELEMENT_TYPE_R4: CorElementType = CorElementType(12u8);
pub const ELEMENT_TYPE_R8: CorElementType = CorElementType(13u8);
pub const ELEMENT_TYPE_SENTINEL: CorElementType = CorElementType(65u8);
pub const ELEMENT_TYPE_STRING: CorElementType = CorElementType(14u8);
pub const ELEMENT_TYPE_SZARRAY: CorElementType = CorElementType(29u8);
pub const ELEMENT_TYPE_TYPEDBYREF: CorElementType = CorElementType(22u8);
pub const ELEMENT_TYPE_U: CorElementType = CorElementType(25u8);
pub const ELEMENT_TYPE_U1: CorElementType = CorElementType(5u8);
pub const ELEMENT_TYPE_U2: CorElementType = CorElementType(7u8);
pub const ELEMENT_TYPE_U4: CorElementType = CorElementType(9u8);
pub const ELEMENT_TYPE_U8: CorElementType = CorElementType(11u8);
pub const ELEMENT_TYPE_VALUETYPE: CorElementType = CorElementType(17u8);
pub const ELEMENT_TYPE_VAR: CorElementType = CorElementType(19u8);
pub const ELEMENT_TYPE_VOID: CorElementType = CorElementType(1u8);
pub const FORWARD_INTEROP_STUB_METHOD_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
pub const FORWARD_INTEROP_STUB_METHOD_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
pub const FRAMEWORK_REGISTRY_KEY: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\.NETFramework");
pub const FRAMEWORK_REGISTRY_KEY_W: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\.NETFramework");
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
pub const FRIEND_ASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
pub const FRIEND_ASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
pub const GUID_DispIdOverride: windows_core::GUID = windows_core::GUID::from_u128(0xcd2bc5c9_f452_4326_b714_f9c539d4da58);
pub const GUID_ExportedFromComPlus: windows_core::GUID = windows_core::GUID::from_u128(0x90883f05_3d28_11d2_8f17_00a0c9a6186d);
pub const GUID_ForceIEnumerable: windows_core::GUID = windows_core::GUID::from_u128(0xb64784eb_d8d4_4d9b_9acd_0e30806426f7);
pub const GUID_Function2Getter: windows_core::GUID = windows_core::GUID::from_u128(0x54fc8f55_38de_4703_9c4e_250351302b1c);
pub const GUID_ManagedName: windows_core::GUID = windows_core::GUID::from_u128(0x0f21f359_ab84_41e8_9a78_36d110e6d2f9);
pub const GUID_PropGetCA: windows_core::GUID = windows_core::GUID::from_u128(0x2941ff83_88d8_4f73_b6a9_bdf8712d000d);
pub const GUID_PropPutCA: windows_core::GUID = windows_core::GUID::from_u128(0x29533527_3683_4364_abc0_db1add822fa2);
pub const IMAGE_CEE_CS_BYVALUE: CorArgType = CorArgType(10i32);
pub const IMAGE_CEE_CS_CALLCONV_C: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(1i32);
pub const IMAGE_CEE_CS_CALLCONV_DEFAULT: CorCallingConvention = CorCallingConvention(0i32);
pub const IMAGE_CEE_CS_CALLCONV_EXPLICITTHIS: CorCallingConvention = CorCallingConvention(64i32);
pub const IMAGE_CEE_CS_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(4i32);
pub const IMAGE_CEE_CS_CALLCONV_FIELD: CorCallingConvention = CorCallingConvention(6i32);
pub const IMAGE_CEE_CS_CALLCONV_GENERIC: CorCallingConvention = CorCallingConvention(16i32);
pub const IMAGE_CEE_CS_CALLCONV_GENERICINST: CorCallingConvention = CorCallingConvention(10i32);
pub const IMAGE_CEE_CS_CALLCONV_HASTHIS: CorCallingConvention = CorCallingConvention(32i32);
pub const IMAGE_CEE_CS_CALLCONV_LOCAL_SIG: CorCallingConvention = CorCallingConvention(7i32);
pub const IMAGE_CEE_CS_CALLCONV_MASK: CorCallingConvention = CorCallingConvention(15i32);
pub const IMAGE_CEE_CS_CALLCONV_MAX: CorCallingConvention = CorCallingConvention(12i32);
pub const IMAGE_CEE_CS_CALLCONV_NATIVEVARARG: CorCallingConvention = CorCallingConvention(11i32);
pub const IMAGE_CEE_CS_CALLCONV_PROPERTY: CorCallingConvention = CorCallingConvention(8i32);
pub const IMAGE_CEE_CS_CALLCONV_STDCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(2i32);
pub const IMAGE_CEE_CS_CALLCONV_THISCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(3i32);
pub const IMAGE_CEE_CS_CALLCONV_UNMGD: CorCallingConvention = CorCallingConvention(9i32);
pub const IMAGE_CEE_CS_CALLCONV_VARARG: CorCallingConvention = CorCallingConvention(5i32);
pub const IMAGE_CEE_CS_END: CorArgType = CorArgType(0i32);
pub const IMAGE_CEE_CS_I4: CorArgType = CorArgType(2i32);
pub const IMAGE_CEE_CS_I8: CorArgType = CorArgType(3i32);
pub const IMAGE_CEE_CS_OBJECT: CorArgType = CorArgType(7i32);
pub const IMAGE_CEE_CS_PTR: CorArgType = CorArgType(6i32);
pub const IMAGE_CEE_CS_R4: CorArgType = CorArgType(4i32);
pub const IMAGE_CEE_CS_R8: CorArgType = CorArgType(5i32);
pub const IMAGE_CEE_CS_STRUCT32: CorArgType = CorArgType(9i32);
pub const IMAGE_CEE_CS_STRUCT4: CorArgType = CorArgType(8i32);
pub const IMAGE_CEE_CS_VOID: CorArgType = CorArgType(1i32);
pub const IMAGE_CEE_UNMANAGED_CALLCONV_C: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(1i32);
pub const IMAGE_CEE_UNMANAGED_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(4i32);
pub const IMAGE_CEE_UNMANAGED_CALLCONV_STDCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(2i32);
pub const IMAGE_CEE_UNMANAGED_CALLCONV_THISCALL: CorUnmanagedCallingConvention = CorUnmanagedCallingConvention(3i32);
pub const IMAGE_DIRECTORY_ENTRY_COMHEADER: ReplacesGeneralNumericDefines = ReplacesGeneralNumericDefines(14i32);
pub const INTEROP_AUTOPROXY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.AutomationProxyAttribute");
pub const INTEROP_AUTOPROXY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.AutomationProxyAttribute");
pub const INTEROP_BESTFITMAPPING_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.BestFitMappingAttribute");
pub const INTEROP_BESTFITMAPPING_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.BestFitMappingAttribute");
pub const INTEROP_CLASSINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ClassInterfaceAttribute");
pub const INTEROP_CLASSINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ClassInterfaceAttribute");
pub const INTEROP_COCLASS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.CoClassAttribute");
pub const INTEROP_COCLASS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.CoClassAttribute");
pub const INTEROP_COMALIASNAME_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComAliasNameAttribute");
pub const INTEROP_COMALIASNAME_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComAliasNameAttribute");
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
pub const INTEROP_COMCONVERSIONLOSS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComConversionLossAttribute");
pub const INTEROP_COMCONVERSIONLOSS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComConversionLossAttribute");
pub const INTEROP_COMDEFAULTINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
pub const INTEROP_COMDEFAULTINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
pub const INTEROP_COMEMULATE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComEmulateAttribute");
pub const INTEROP_COMEMULATE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComEmulateAttribute");
pub const INTEROP_COMEVENTINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
pub const INTEROP_COMEVENTINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
pub const INTEROP_COMIMPORT_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComImportAttribute");
pub const INTEROP_COMIMPORT_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComImportAttribute");
pub const INTEROP_COMREGISTERFUNCTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
pub const INTEROP_COMREGISTERFUNCTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
pub const INTEROP_COMSOURCEINTERFACES_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
pub const INTEROP_COMSOURCEINTERFACES_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
pub const INTEROP_COMVISIBLE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComVisibleAttribute");
pub const INTEROP_COMVISIBLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComVisibleAttribute");
pub const INTEROP_DATETIMEVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
pub const INTEROP_DATETIMEVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
pub const INTEROP_DECIMALVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DecimalConstantAttribute");
pub const INTEROP_DECIMALVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DecimalConstantAttribute");
pub const INTEROP_DEFAULTMEMBER_TYPE: windows_core::PCSTR = windows_core::s!("System.Reflection.DefaultMemberAttribute");
pub const INTEROP_DEFAULTMEMBER_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Reflection.DefaultMemberAttribute");
pub const INTEROP_DISPID_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.DispIdAttribute");
pub const INTEROP_DISPID_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.DispIdAttribute");
pub const INTEROP_GUID_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.GuidAttribute");
pub const INTEROP_GUID_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.GuidAttribute");
pub const INTEROP_IDISPATCHIMPL_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.IDispatchImplAttribute");
pub const INTEROP_IDISPATCHIMPL_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.IDispatchImplAttribute");
pub const INTEROP_IDISPATCHVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
pub const INTEROP_IDISPATCHVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
pub const INTEROP_INTERFACETYPE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.InterfaceTypeAttribute");
pub const INTEROP_INTERFACETYPE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.InterfaceTypeAttribute");
pub const INTEROP_IN_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.InAttribute");
pub const INTEROP_IN_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.InAttribute");
pub const INTEROP_IUNKNOWNVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
pub const INTEROP_IUNKNOWNVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
pub const INTEROP_LCIDCONVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.LCIDConversionAttribute");
pub const INTEROP_LCIDCONVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.LCIDConversionAttribute");
pub const INTEROP_MARSHALAS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.MarshalAsAttribute");
pub const INTEROP_MARSHALAS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.MarshalAsAttribute");
pub const INTEROP_OUT_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.OutAttribute");
pub const INTEROP_OUT_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.OutAttribute");
pub const INTEROP_PARAMARRAY_TYPE: windows_core::PCSTR = windows_core::s!("System.ParamArrayAttribute");
pub const INTEROP_PARAMARRAY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.ParamArrayAttribute");
pub const INTEROP_PRESERVESIG_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.PreserveSigAttribure");
pub const INTEROP_PRESERVESIG_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.PreserveSigAttribure");
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
pub const INTEROP_SERIALIZABLE_TYPE: windows_core::PCSTR = windows_core::s!("System.SerializableAttribute");
pub const INTEROP_SERIALIZABLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.SerializableAttribute");
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
pub const INTEROP_TYPELIBFUNC_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibFuncAttribute");
pub const INTEROP_TYPELIBFUNC_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibFuncAttribute");
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
pub const INTEROP_TYPELIBTYPE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibTypeAttribute");
pub const INTEROP_TYPELIBTYPE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibTypeAttribute");
pub const INTEROP_TYPELIBVAR_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibVarAttribute");
pub const INTEROP_TYPELIBVAR_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibVarAttribute");
pub const INTEROP_TYPELIBVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibVersionAttribute");
pub const INTEROP_TYPELIBVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibVersionAttribute");
pub const INVALID_CONNECTION_ID: u32 = 0u32;
pub const INVALID_TASK_ID: u32 = 0u32;
pub const LIBID_ComPlusRuntime: windows_core::GUID = windows_core::GUID::from_u128(0xbed7f4ea_1a96_11d2_8f08_00a0c9a6186d);
pub const LoadAlways: LoadHintEnum = LoadHintEnum(1i32);
pub const LoadDefault: LoadHintEnum = LoadHintEnum(0i32);
pub const LoadNever: LoadHintEnum = LoadHintEnum(3i32);
pub const LoadSometimes: LoadHintEnum = LoadHintEnum(2i32);
pub const MAIN_CLR_MODULE_NAME_A: windows_core::PCSTR = windows_core::s!("coreclr");
pub const MAIN_CLR_MODULE_NAME_W: windows_core::PCWSTR = windows_core::w!("coreclr");
pub const MAX_CONNECTION_NAME: u32 = 260u32;
pub const MDAssembly: CorLinkerOptions = CorLinkerOptions(0i32);
pub const MDDupAll: CorCheckDuplicatesFor = CorCheckDuplicatesFor(-1i32);
pub const MDDupAssembly: CorCheckDuplicatesFor = CorCheckDuplicatesFor(268435456i32);
pub const MDDupAssemblyRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(32768i32);
pub const MDDupCustomAttribute: CorCheckDuplicatesFor = CorCheckDuplicatesFor(32i32);
pub const MDDupDefault: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1058840i32);
pub const MDDupENC: CorCheckDuplicatesFor = CorCheckDuplicatesFor(-1i32);
pub const MDDupEvent: CorCheckDuplicatesFor = CorCheckDuplicatesFor(512i32);
pub const MDDupExportedType: CorCheckDuplicatesFor = CorCheckDuplicatesFor(131072i32);
pub const MDDupFieldDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1024i32);
pub const MDDupFile: CorCheckDuplicatesFor = CorCheckDuplicatesFor(65536i32);
pub const MDDupGenericParam: CorCheckDuplicatesFor = CorCheckDuplicatesFor(524288i32);
pub const MDDupGenericParamConstraint: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2097152i32);
pub const MDDupImplMap: CorCheckDuplicatesFor = CorCheckDuplicatesFor(16384i32);
pub const MDDupInterfaceImpl: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2i32);
pub const MDDupManifestResource: CorCheckDuplicatesFor = CorCheckDuplicatesFor(262144i32);
pub const MDDupMemberRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(16i32);
pub const MDDupMethodDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(4i32);
pub const MDDupMethodSpec: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1048576i32);
pub const MDDupModuleRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(4096i32);
pub const MDDupParamDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(64i32);
pub const MDDupPermission: CorCheckDuplicatesFor = CorCheckDuplicatesFor(128i32);
pub const MDDupProperty: CorCheckDuplicatesFor = CorCheckDuplicatesFor(256i32);
pub const MDDupSignature: CorCheckDuplicatesFor = CorCheckDuplicatesFor(2048i32);
pub const MDDupTypeDef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(1i32);
pub const MDDupTypeRef: CorCheckDuplicatesFor = CorCheckDuplicatesFor(8i32);
pub const MDDupTypeSpec: CorCheckDuplicatesFor = CorCheckDuplicatesFor(8192i32);
pub const MDErrorOutOfOrderAll: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(-1i32);
pub const MDErrorOutOfOrderDefault: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(0i32);
pub const MDErrorOutOfOrderNone: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(0i32);
pub const MDEventOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(16i32);
pub const MDFieldOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(2i32);
pub const MDImportOptionAll: CorImportOptions = CorImportOptions(-1i32);
pub const MDImportOptionAllCustomAttributes: CorImportOptions = CorImportOptions(32i32);
pub const MDImportOptionAllEvents: CorImportOptions = CorImportOptions(16i32);
pub const MDImportOptionAllExportedTypes: CorImportOptions = CorImportOptions(64i32);
pub const MDImportOptionAllFieldDefs: CorImportOptions = CorImportOptions(4i32);
pub const MDImportOptionAllMethodDefs: CorImportOptions = CorImportOptions(2i32);
pub const MDImportOptionAllProperties: CorImportOptions = CorImportOptions(8i32);
pub const MDImportOptionAllTypeDefs: CorImportOptions = CorImportOptions(1i32);
pub const MDImportOptionDefault: CorImportOptions = CorImportOptions(0i32);
pub const MDMemberRefToDef: CorRefToDefCheck = CorRefToDefCheck(2i32);
pub const MDMethodOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(1i32);
pub const MDNetModule: CorLinkerOptions = CorLinkerOptions(1i32);
pub const MDNoDupChecks: CorCheckDuplicatesFor = CorCheckDuplicatesFor(0i32);
pub const MDNotifyAll: CorNotificationForTokenMovement = CorNotificationForTokenMovement(-1i32);
pub const MDNotifyAssemblyRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16777216i32);
pub const MDNotifyCustomAttribute: CorNotificationForTokenMovement = CorNotificationForTokenMovement(2048i32);
pub const MDNotifyDefault: CorNotificationForTokenMovement = CorNotificationForTokenMovement(15i32);
pub const MDNotifyEvent: CorNotificationForTokenMovement = CorNotificationForTokenMovement(256i32);
pub const MDNotifyExportedType: CorNotificationForTokenMovement = CorNotificationForTokenMovement(67108864i32);
pub const MDNotifyFieldDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(4i32);
pub const MDNotifyFile: CorNotificationForTokenMovement = CorNotificationForTokenMovement(33554432i32);
pub const MDNotifyInterfaceImpl: CorNotificationForTokenMovement = CorNotificationForTokenMovement(64i32);
pub const MDNotifyMemberRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(2i32);
pub const MDNotifyMethodDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(1i32);
pub const MDNotifyModuleRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16384i32);
pub const MDNotifyNameSpace: CorNotificationForTokenMovement = CorNotificationForTokenMovement(32768i32);
pub const MDNotifyNone: CorNotificationForTokenMovement = CorNotificationForTokenMovement(0i32);
pub const MDNotifyParamDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(32i32);
pub const MDNotifyPermission: CorNotificationForTokenMovement = CorNotificationForTokenMovement(8192i32);
pub const MDNotifyProperty: CorNotificationForTokenMovement = CorNotificationForTokenMovement(128i32);
pub const MDNotifyResource: CorNotificationForTokenMovement = CorNotificationForTokenMovement(134217728i32);
pub const MDNotifySecurityValue: CorNotificationForTokenMovement = CorNotificationForTokenMovement(4096i32);
pub const MDNotifySignature: CorNotificationForTokenMovement = CorNotificationForTokenMovement(512i32);
pub const MDNotifyTypeDef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(16i32);
pub const MDNotifyTypeRef: CorNotificationForTokenMovement = CorNotificationForTokenMovement(8i32);
pub const MDNotifyTypeSpec: CorNotificationForTokenMovement = CorNotificationForTokenMovement(1024i32);
pub const MDParamOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(4i32);
pub const MDPreserveLocalMemberRef: CorLocalRefPreservation = CorLocalRefPreservation(2i32);
pub const MDPreserveLocalRefsNone: CorLocalRefPreservation = CorLocalRefPreservation(0i32);
pub const MDPreserveLocalTypeRef: CorLocalRefPreservation = CorLocalRefPreservation(1i32);
pub const MDPropertyOutOfOrder: CorErrorIfEmitOutOfOrder = CorErrorIfEmitOutOfOrder(8i32);
pub const MDRefToDefAll: CorRefToDefCheck = CorRefToDefCheck(-1i32);
pub const MDRefToDefDefault: CorRefToDefCheck = CorRefToDefCheck(3i32);
pub const MDRefToDefNone: CorRefToDefCheck = CorRefToDefCheck(0i32);
pub const MDSetENCOff: CorSetENC = CorSetENC(2i32);
pub const MDSetENCOn: CorSetENC = CorSetENC(1i32);
pub const MDThreadSafetyDefault: CorThreadSafetyOptions = CorThreadSafetyOptions(0i32);
pub const MDThreadSafetyOff: CorThreadSafetyOptions = CorThreadSafetyOptions(0i32);
pub const MDThreadSafetyOn: CorThreadSafetyOptions = CorThreadSafetyOptions(1i32);
pub const MDTypeRefToDef: CorRefToDefCheck = CorRefToDefCheck(1i32);
pub const MDUpdateDelta: CorSetENC = CorSetENC(5i32);
pub const MDUpdateENC: CorSetENC = CorSetENC(1i32);
pub const MDUpdateExtension: CorSetENC = CorSetENC(3i32);
pub const MDUpdateFull: CorSetENC = CorSetENC(2i32);
pub const MDUpdateIncremental: CorSetENC = CorSetENC(4i32);
pub const MDUpdateMask: CorSetENC = CorSetENC(7i32);
pub const MSCOREE_SHIM_A: windows_core::PCSTR = windows_core::s!("mscoree.dll");
pub const MSCOREE_SHIM_W: windows_core::PCWSTR = windows_core::w!("mscoree.dll");
pub const MergeExportedTypes: MergeFlags = MergeFlags(8i32);
pub const MergeFlagsNone: MergeFlags = MergeFlags(0i32);
pub const MergeManifest: MergeFlags = MergeFlags(1i32);
pub const MetaDataCheckDuplicatesFor: windows_core::GUID = windows_core::GUID::from_u128(0x30fe7be8_d7d9_11d2_9f80_00c04f79a0a3);
pub const MetaDataErrorIfEmitOutOfOrder: windows_core::GUID = windows_core::GUID::from_u128(0x1547872d_dc03_11d2_9420_0000f8083460);
pub const MetaDataGenerateTCEAdapters: windows_core::GUID = windows_core::GUID::from_u128(0xdcc9de90_4151_11d3_88d6_00902754c43a);
pub const MetaDataImportOption: windows_core::GUID = windows_core::GUID::from_u128(0x79700f36_4aac_11d3_84c3_009027868cb1);
pub const MetaDataLinkerOptions: windows_core::GUID = windows_core::GUID::from_u128(0x47e099b6_ae7c_4797_8317_b48aa645b8f9);
pub const MetaDataMergerOptions: windows_core::GUID = windows_core::GUID::from_u128(0x132d3a6e_b35d_464e_951a_42efb9fb6601);
pub const MetaDataNotificationForTokenMovement: windows_core::GUID = windows_core::GUID::from_u128(0xe5d71a4c_d7da_11d2_9f80_00c04f79a0a3);
pub const MetaDataPreserveLocalRefs: windows_core::GUID = windows_core::GUID::from_u128(0xa55c0354_e91b_468b_8648_7cc31035d533);
pub const MetaDataRefToDefCheck: windows_core::GUID = windows_core::GUID::from_u128(0xde3856f8_d7d9_11d2_9f80_00c04f79a0a3);
pub const MetaDataRuntimeVersion: windows_core::GUID = windows_core::GUID::from_u128(0x47e099b7_ae7c_4797_8317_b48aa645b8f9);
pub const MetaDataSetUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x2eee315c_d7db_11d2_9f80_00c04f79a0a3);
pub const MetaDataThreadSafetyOptions: windows_core::GUID = windows_core::GUID::from_u128(0xf7559806_f266_42ea_8c63_0adb45e8b234);
pub const MetaDataTypeLibImportNamespace: windows_core::GUID = windows_core::GUID::from_u128(0xf17ff889_5a63_11d3_9ff2_00c04ff7431a);
pub const NATIVE_TYPE_ANSIBSTR: CorNativeType = CorNativeType(35i32);
pub const NATIVE_TYPE_ARRAY: CorNativeType = CorNativeType(42i32);
pub const NATIVE_TYPE_ASANY: CorNativeType = CorNativeType(40i32);
pub const NATIVE_TYPE_BOOLEAN: CorNativeType = CorNativeType(2i32);
pub const NATIVE_TYPE_BSTR: CorNativeType = CorNativeType(19i32);
pub const NATIVE_TYPE_BYVALSTR: CorNativeType = CorNativeType(34i32);
pub const NATIVE_TYPE_CURRENCY: CorNativeType = CorNativeType(15i32);
pub const NATIVE_TYPE_CUSTOMMARSHALER: CorNativeType = CorNativeType(44i32);
pub const NATIVE_TYPE_DATE: CorNativeType = CorNativeType(18i32);
pub const NATIVE_TYPE_DECIMAL: CorNativeType = CorNativeType(17i32);
pub const NATIVE_TYPE_END: CorNativeType = CorNativeType(0i32);
pub const NATIVE_TYPE_ERROR: CorNativeType = CorNativeType(45i32);
pub const NATIVE_TYPE_FIXEDARRAY: CorNativeType = CorNativeType(30i32);
pub const NATIVE_TYPE_FIXEDSYSSTRING: CorNativeType = CorNativeType(23i32);
pub const NATIVE_TYPE_FUNC: CorNativeType = CorNativeType(38i32);
pub const NATIVE_TYPE_HSTRING: CorNativeType = CorNativeType(47i32);
pub const NATIVE_TYPE_I1: CorNativeType = CorNativeType(3i32);
pub const NATIVE_TYPE_I2: CorNativeType = CorNativeType(5i32);
pub const NATIVE_TYPE_I4: CorNativeType = CorNativeType(7i32);
pub const NATIVE_TYPE_I8: CorNativeType = CorNativeType(9i32);
pub const NATIVE_TYPE_IDISPATCH: CorNativeType = CorNativeType(26i32);
pub const NATIVE_TYPE_IINSPECTABLE: CorNativeType = CorNativeType(46i32);
pub const NATIVE_TYPE_INT: CorNativeType = CorNativeType(31i32);
pub const NATIVE_TYPE_INTF: CorNativeType = CorNativeType(28i32);
pub const NATIVE_TYPE_IUNKNOWN: CorNativeType = CorNativeType(25i32);
pub const NATIVE_TYPE_LPSTR: CorNativeType = CorNativeType(20i32);
pub const NATIVE_TYPE_LPSTRUCT: CorNativeType = CorNativeType(43i32);
pub const NATIVE_TYPE_LPTSTR: CorNativeType = CorNativeType(22i32);
pub const NATIVE_TYPE_LPUTF8STR: CorNativeType = CorNativeType(48i32);
pub const NATIVE_TYPE_LPWSTR: CorNativeType = CorNativeType(21i32);
pub const NATIVE_TYPE_MAX: CorNativeType = CorNativeType(80i32);
pub const NATIVE_TYPE_NESTEDSTRUCT: CorNativeType = CorNativeType(33i32);
pub const NATIVE_TYPE_OBJECTREF: CorNativeType = CorNativeType(24i32);
pub const NATIVE_TYPE_PTR: CorNativeType = CorNativeType(16i32);
pub const NATIVE_TYPE_R4: CorNativeType = CorNativeType(11i32);
pub const NATIVE_TYPE_R8: CorNativeType = CorNativeType(12i32);
pub const NATIVE_TYPE_SAFEARRAY: CorNativeType = CorNativeType(29i32);
pub const NATIVE_TYPE_STRUCT: CorNativeType = CorNativeType(27i32);
pub const NATIVE_TYPE_SYSCHAR: CorNativeType = CorNativeType(13i32);
pub const NATIVE_TYPE_TBSTR: CorNativeType = CorNativeType(36i32);
pub const NATIVE_TYPE_U1: CorNativeType = CorNativeType(4i32);
pub const NATIVE_TYPE_U2: CorNativeType = CorNativeType(6i32);
pub const NATIVE_TYPE_U4: CorNativeType = CorNativeType(8i32);
pub const NATIVE_TYPE_U8: CorNativeType = CorNativeType(10i32);
pub const NATIVE_TYPE_UINT: CorNativeType = CorNativeType(32i32);
pub const NATIVE_TYPE_VARIANT: CorNativeType = CorNativeType(14i32);
pub const NATIVE_TYPE_VARIANTBOOL: CorNativeType = CorNativeType(37i32);
pub const NATIVE_TYPE_VOID: CorNativeType = CorNativeType(1i32);
pub const NGenDefault: NGenHintEnum = NGenHintEnum(0i32);
pub const NGenEager: NGenHintEnum = NGenHintEnum(1i32);
pub const NGenLazy: NGenHintEnum = NGenHintEnum(2i32);
pub const NGenNever: NGenHintEnum = NGenHintEnum(3i32);
pub const NONVERSIONABLE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.Versioning.NonVersionableAttribute");
pub const NONVERSIONABLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.Versioning.NonVersionableAttribute");
pub const NoDupCheck: MergeFlags = MergeFlags(4i32);
pub const RUNTIMECOMPATIBILITY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
pub const RUNTIMECOMPATIBILITY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
pub const SERIALIZATION_TYPE_BOOLEAN: CorSerializationType = CorSerializationType(2i32);
pub const SERIALIZATION_TYPE_CHAR: CorSerializationType = CorSerializationType(3i32);
pub const SERIALIZATION_TYPE_ENUM: CorSerializationType = CorSerializationType(85i32);
pub const SERIALIZATION_TYPE_FIELD: CorSerializationType = CorSerializationType(83i32);
pub const SERIALIZATION_TYPE_I1: CorSerializationType = CorSerializationType(4i32);
pub const SERIALIZATION_TYPE_I2: CorSerializationType = CorSerializationType(6i32);
pub const SERIALIZATION_TYPE_I4: CorSerializationType = CorSerializationType(8i32);
pub const SERIALIZATION_TYPE_I8: CorSerializationType = CorSerializationType(10i32);
pub const SERIALIZATION_TYPE_PROPERTY: CorSerializationType = CorSerializationType(84i32);
pub const SERIALIZATION_TYPE_R4: CorSerializationType = CorSerializationType(12i32);
pub const SERIALIZATION_TYPE_R8: CorSerializationType = CorSerializationType(13i32);
pub const SERIALIZATION_TYPE_STRING: CorSerializationType = CorSerializationType(14i32);
pub const SERIALIZATION_TYPE_SZARRAY: CorSerializationType = CorSerializationType(29i32);
pub const SERIALIZATION_TYPE_TAGGED_OBJECT: CorSerializationType = CorSerializationType(81i32);
pub const SERIALIZATION_TYPE_TYPE: CorSerializationType = CorSerializationType(80i32);
pub const SERIALIZATION_TYPE_U1: CorSerializationType = CorSerializationType(5i32);
pub const SERIALIZATION_TYPE_U2: CorSerializationType = CorSerializationType(7i32);
pub const SERIALIZATION_TYPE_U4: CorSerializationType = CorSerializationType(9i32);
pub const SERIALIZATION_TYPE_U8: CorSerializationType = CorSerializationType(11i32);
pub const SERIALIZATION_TYPE_UNDEFINED: CorSerializationType = CorSerializationType(0i32);
pub const SIGN_MASK_FOURBYTE: i32 = -268435456i32;
pub const SIGN_MASK_ONEBYTE: i32 = -64i32;
pub const SIGN_MASK_TWOBYTE: i32 = -8192i32;
pub const SUBJECT_ASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
pub const SUBJECT_ASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
pub const TARGET_FRAMEWORK_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.Versioning.TargetFrameworkAttribute");
pub const TARGET_FRAMEWORK_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.Versioning.TargetFrameworkAttribute");
pub const USER_FRAMEWORK_REGISTRY_KEY: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\.NETFramework64");
pub const USER_FRAMEWORK_REGISTRY_KEY_W: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\.NETFramework64");
pub const ValidatorModuleTypeEnc: CorValidatorModuleType = CorValidatorModuleType(3i32);
pub const ValidatorModuleTypeIncr: CorValidatorModuleType = CorValidatorModuleType(4i32);
pub const ValidatorModuleTypeInvalid: CorValidatorModuleType = CorValidatorModuleType(0i32);
pub const ValidatorModuleTypeMax: CorValidatorModuleType = CorValidatorModuleType(4i32);
pub const ValidatorModuleTypeMin: CorValidatorModuleType = CorValidatorModuleType(1i32);
pub const ValidatorModuleTypeObj: CorValidatorModuleType = CorValidatorModuleType(2i32);
pub const ValidatorModuleTypePE: CorValidatorModuleType = CorValidatorModuleType(1i32);
pub const afContentType_Default: CorAssemblyFlags = CorAssemblyFlags(0i32);
pub const afContentType_Mask: CorAssemblyFlags = CorAssemblyFlags(3584i32);
pub const afContentType_WindowsRuntime: CorAssemblyFlags = CorAssemblyFlags(512i32);
pub const afDisableJITcompileOptimizer: CorAssemblyFlags = CorAssemblyFlags(16384i32);
pub const afEnableJITcompileTracking: CorAssemblyFlags = CorAssemblyFlags(32768i32);
pub const afPA_AMD64: CorAssemblyFlags = CorAssemblyFlags(64i32);
pub const afPA_ARM: CorAssemblyFlags = CorAssemblyFlags(80i32);
pub const afPA_FullMask: CorAssemblyFlags = CorAssemblyFlags(240i32);
pub const afPA_IA64: CorAssemblyFlags = CorAssemblyFlags(48i32);
pub const afPA_MSIL: CorAssemblyFlags = CorAssemblyFlags(16i32);
pub const afPA_Mask: CorAssemblyFlags = CorAssemblyFlags(112i32);
pub const afPA_NoPlatform: CorAssemblyFlags = CorAssemblyFlags(112i32);
pub const afPA_None: CorAssemblyFlags = CorAssemblyFlags(0i32);
pub const afPA_Shift: CorAssemblyFlags = CorAssemblyFlags(4i32);
pub const afPA_Specified: CorAssemblyFlags = CorAssemblyFlags(128i32);
pub const afPA_x86: CorAssemblyFlags = CorAssemblyFlags(32i32);
pub const afPublicKey: CorAssemblyFlags = CorAssemblyFlags(1i32);
pub const afRetargetable: CorAssemblyFlags = CorAssemblyFlags(256i32);
pub const catAll: CorAttributeTargets = CorAttributeTargets(24575i32);
pub const catAssembly: CorAttributeTargets = CorAttributeTargets(1i32);
pub const catClass: CorAttributeTargets = CorAttributeTargets(4i32);
pub const catClassMembers: CorAttributeTargets = CorAttributeTargets(6140i32);
pub const catConstructor: CorAttributeTargets = CorAttributeTargets(32i32);
pub const catDelegate: CorAttributeTargets = CorAttributeTargets(4096i32);
pub const catEnum: CorAttributeTargets = CorAttributeTargets(16i32);
pub const catEvent: CorAttributeTargets = CorAttributeTargets(512i32);
pub const catField: CorAttributeTargets = CorAttributeTargets(256i32);
pub const catGenericParameter: CorAttributeTargets = CorAttributeTargets(16384i32);
pub const catInterface: CorAttributeTargets = CorAttributeTargets(1024i32);
pub const catMethod: CorAttributeTargets = CorAttributeTargets(64i32);
pub const catModule: CorAttributeTargets = CorAttributeTargets(2i32);
pub const catParameter: CorAttributeTargets = CorAttributeTargets(2048i32);
pub const catProperty: CorAttributeTargets = CorAttributeTargets(128i32);
pub const catStruct: CorAttributeTargets = CorAttributeTargets(8i32);
pub const cssAccurate: CorSaveSize = CorSaveSize(0i32);
pub const cssDiscardTransientCAs: CorSaveSize = CorSaveSize(2i32);
pub const cssQuick: CorSaveSize = CorSaveSize(1i32);
pub const dclActionMask: CorDeclSecurity = CorDeclSecurity(31i32);
pub const dclActionNil: CorDeclSecurity = CorDeclSecurity(0i32);
pub const dclAssert: CorDeclSecurity = CorDeclSecurity(3i32);
pub const dclDemand: CorDeclSecurity = CorDeclSecurity(2i32);
pub const dclDeny: CorDeclSecurity = CorDeclSecurity(4i32);
pub const dclInheritanceCheck: CorDeclSecurity = CorDeclSecurity(7i32);
pub const dclLinktimeCheck: CorDeclSecurity = CorDeclSecurity(6i32);
pub const dclMaximumValue: CorDeclSecurity = CorDeclSecurity(15i32);
pub const dclNonCasDemand: CorDeclSecurity = CorDeclSecurity(13i32);
pub const dclNonCasInheritance: CorDeclSecurity = CorDeclSecurity(15i32);
pub const dclNonCasLinkDemand: CorDeclSecurity = CorDeclSecurity(14i32);
pub const dclPermitOnly: CorDeclSecurity = CorDeclSecurity(5i32);
pub const dclPrejitDenied: CorDeclSecurity = CorDeclSecurity(12i32);
pub const dclPrejitGrant: CorDeclSecurity = CorDeclSecurity(11i32);
pub const dclRequest: CorDeclSecurity = CorDeclSecurity(1i32);
pub const dclRequestMinimum: CorDeclSecurity = CorDeclSecurity(8i32);
pub const dclRequestOptional: CorDeclSecurity = CorDeclSecurity(9i32);
pub const dclRequestRefuse: CorDeclSecurity = CorDeclSecurity(10i32);
pub const evRTSpecialName: CorEventAttr = CorEventAttr(1024i32);
pub const evReservedMask: CorEventAttr = CorEventAttr(1024i32);
pub const evSpecialName: CorEventAttr = CorEventAttr(512i32);
pub const fdAssembly: CorFieldAttr = CorFieldAttr(3i32);
pub const fdFamANDAssem: CorFieldAttr = CorFieldAttr(2i32);
pub const fdFamORAssem: CorFieldAttr = CorFieldAttr(5i32);
pub const fdFamily: CorFieldAttr = CorFieldAttr(4i32);
pub const fdFieldAccessMask: CorFieldAttr = CorFieldAttr(7i32);
pub const fdHasDefault: CorFieldAttr = CorFieldAttr(32768i32);
pub const fdHasFieldMarshal: CorFieldAttr = CorFieldAttr(4096i32);
pub const fdHasFieldRVA: CorFieldAttr = CorFieldAttr(256i32);
pub const fdInitOnly: CorFieldAttr = CorFieldAttr(32i32);
pub const fdLiteral: CorFieldAttr = CorFieldAttr(64i32);
pub const fdNotSerialized: CorFieldAttr = CorFieldAttr(128i32);
pub const fdPinvokeImpl: CorFieldAttr = CorFieldAttr(8192i32);
pub const fdPrivate: CorFieldAttr = CorFieldAttr(1i32);
pub const fdPrivateScope: CorFieldAttr = CorFieldAttr(0i32);
pub const fdPublic: CorFieldAttr = CorFieldAttr(6i32);
pub const fdRTSpecialName: CorFieldAttr = CorFieldAttr(1024i32);
pub const fdReservedMask: CorFieldAttr = CorFieldAttr(38144i32);
pub const fdSpecialName: CorFieldAttr = CorFieldAttr(512i32);
pub const fdStatic: CorFieldAttr = CorFieldAttr(16i32);
pub const ffContainsMetaData: CorFileFlags = CorFileFlags(0i32);
pub const ffContainsNoMetaData: CorFileFlags = CorFileFlags(1i32);
pub const fmExecutableImage: CorFileMapping = CorFileMapping(1i32);
pub const fmFlat: CorFileMapping = CorFileMapping(0i32);
pub const gpContravariant: CorGenericParamAttr = CorGenericParamAttr(2i32);
pub const gpCovariant: CorGenericParamAttr = CorGenericParamAttr(1i32);
pub const gpDefaultConstructorConstraint: CorGenericParamAttr = CorGenericParamAttr(16i32);
pub const gpNoSpecialConstraint: CorGenericParamAttr = CorGenericParamAttr(0i32);
pub const gpNonVariant: CorGenericParamAttr = CorGenericParamAttr(0i32);
pub const gpNotNullableValueTypeConstraint: CorGenericParamAttr = CorGenericParamAttr(8i32);
pub const gpReferenceTypeConstraint: CorGenericParamAttr = CorGenericParamAttr(4i32);
pub const gpSpecialConstraintMask: CorGenericParamAttr = CorGenericParamAttr(28i32);
pub const gpVarianceMask: CorGenericParamAttr = CorGenericParamAttr(3i32);
pub const mdAbstract: CorMethodAttr = CorMethodAttr(1024i32);
pub const mdAssem: CorMethodAttr = CorMethodAttr(3i32);
pub const mdCheckAccessOnOverride: CorMethodAttr = CorMethodAttr(512i32);
pub const mdFamANDAssem: CorMethodAttr = CorMethodAttr(2i32);
pub const mdFamORAssem: CorMethodAttr = CorMethodAttr(5i32);
pub const mdFamily: CorMethodAttr = CorMethodAttr(4i32);
pub const mdFinal: CorMethodAttr = CorMethodAttr(32i32);
pub const mdHasSecurity: CorMethodAttr = CorMethodAttr(16384i32);
pub const mdHideBySig: CorMethodAttr = CorMethodAttr(128i32);
pub const mdMemberAccessMask: CorMethodAttr = CorMethodAttr(7i32);
pub const mdNewSlot: CorMethodAttr = CorMethodAttr(256i32);
pub const mdPinvokeImpl: CorMethodAttr = CorMethodAttr(8192i32);
pub const mdPrivate: CorMethodAttr = CorMethodAttr(1i32);
pub const mdPrivateScope: CorMethodAttr = CorMethodAttr(0i32);
pub const mdPublic: CorMethodAttr = CorMethodAttr(6i32);
pub const mdRTSpecialName: CorMethodAttr = CorMethodAttr(4096i32);
pub const mdRequireSecObject: CorMethodAttr = CorMethodAttr(32768i32);
pub const mdReservedMask: CorMethodAttr = CorMethodAttr(53248i32);
pub const mdReuseSlot: CorMethodAttr = CorMethodAttr(0i32);
pub const mdSpecialName: CorMethodAttr = CorMethodAttr(2048i32);
pub const mdStatic: CorMethodAttr = CorMethodAttr(16i32);
pub const mdUnmanagedExport: CorMethodAttr = CorMethodAttr(8i32);
pub const mdVirtual: CorMethodAttr = CorMethodAttr(64i32);
pub const mdVtableLayoutMask: CorMethodAttr = CorMethodAttr(256i32);
pub const mdtAssembly: CorTokenType = CorTokenType(536870912i32);
pub const mdtAssemblyRef: CorTokenType = CorTokenType(587202560i32);
pub const mdtBaseType: CorTokenType = CorTokenType(1912602624i32);
pub const mdtCustomAttribute: CorTokenType = CorTokenType(201326592i32);
pub const mdtEvent: CorTokenType = CorTokenType(335544320i32);
pub const mdtExportedType: CorTokenType = CorTokenType(654311424i32);
pub const mdtFieldDef: CorTokenType = CorTokenType(67108864i32);
pub const mdtFile: CorTokenType = CorTokenType(637534208i32);
pub const mdtGenericParam: CorTokenType = CorTokenType(704643072i32);
pub const mdtGenericParamConstraint: CorTokenType = CorTokenType(738197504i32);
pub const mdtInterfaceImpl: CorTokenType = CorTokenType(150994944i32);
pub const mdtManifestResource: CorTokenType = CorTokenType(671088640i32);
pub const mdtMemberRef: CorTokenType = CorTokenType(167772160i32);
pub const mdtMethodDef: CorTokenType = CorTokenType(100663296i32);
pub const mdtMethodImpl: CorTokenType = CorTokenType(419430400i32);
pub const mdtMethodSpec: CorTokenType = CorTokenType(721420288i32);
pub const mdtModule: CorTokenType = CorTokenType(0i32);
pub const mdtModuleRef: CorTokenType = CorTokenType(436207616i32);
pub const mdtName: CorTokenType = CorTokenType(1895825408i32);
pub const mdtParamDef: CorTokenType = CorTokenType(134217728i32);
pub const mdtPermission: CorTokenType = CorTokenType(234881024i32);
pub const mdtProperty: CorTokenType = CorTokenType(385875968i32);
pub const mdtSignature: CorTokenType = CorTokenType(285212672i32);
pub const mdtString: CorTokenType = CorTokenType(1879048192i32);
pub const mdtTypeDef: CorTokenType = CorTokenType(33554432i32);
pub const mdtTypeRef: CorTokenType = CorTokenType(16777216i32);
pub const mdtTypeSpec: CorTokenType = CorTokenType(452984832i32);
pub const miAggressiveInlining: CorMethodImpl = CorMethodImpl(256i32);
pub const miCodeTypeMask: CorMethodImpl = CorMethodImpl(3i32);
pub const miForwardRef: CorMethodImpl = CorMethodImpl(16i32);
pub const miIL: CorMethodImpl = CorMethodImpl(0i32);
pub const miInternalCall: CorMethodImpl = CorMethodImpl(4096i32);
pub const miManaged: CorMethodImpl = CorMethodImpl(0i32);
pub const miManagedMask: CorMethodImpl = CorMethodImpl(4i32);
pub const miMaxMethodImplVal: CorMethodImpl = CorMethodImpl(65535i32);
pub const miNative: CorMethodImpl = CorMethodImpl(1i32);
pub const miNoInlining: CorMethodImpl = CorMethodImpl(8i32);
pub const miNoOptimization: CorMethodImpl = CorMethodImpl(64i32);
pub const miOPTIL: CorMethodImpl = CorMethodImpl(2i32);
pub const miPreserveSig: CorMethodImpl = CorMethodImpl(128i32);
pub const miRuntime: CorMethodImpl = CorMethodImpl(3i32);
pub const miSecurityMitigations: CorMethodImpl = CorMethodImpl(1024i32);
pub const miSynchronized: CorMethodImpl = CorMethodImpl(32i32);
pub const miUnmanaged: CorMethodImpl = CorMethodImpl(4i32);
pub const miUserMask: CorMethodImpl = CorMethodImpl(5628i32);
pub const mrPrivate: CorManifestResourceFlags = CorManifestResourceFlags(2i32);
pub const mrPublic: CorManifestResourceFlags = CorManifestResourceFlags(1i32);
pub const mrVisibilityMask: CorManifestResourceFlags = CorManifestResourceFlags(7i32);
pub const msAddOn: CorMethodSemanticsAttr = CorMethodSemanticsAttr(8i32);
pub const msFire: CorMethodSemanticsAttr = CorMethodSemanticsAttr(32i32);
pub const msGetter: CorMethodSemanticsAttr = CorMethodSemanticsAttr(2i32);
pub const msOther: CorMethodSemanticsAttr = CorMethodSemanticsAttr(4i32);
pub const msRemoveOn: CorMethodSemanticsAttr = CorMethodSemanticsAttr(16i32);
pub const msSetter: CorMethodSemanticsAttr = CorMethodSemanticsAttr(1i32);
pub const nlfLastError: CorNativeLinkFlags = CorNativeLinkFlags(1i32);
pub const nlfMaxValue: CorNativeLinkFlags = CorNativeLinkFlags(3i32);
pub const nlfNoMangle: CorNativeLinkFlags = CorNativeLinkFlags(2i32);
pub const nlfNone: CorNativeLinkFlags = CorNativeLinkFlags(0i32);
pub const nltAnsi: CorNativeLinkType = CorNativeLinkType(2i32);
pub const nltAuto: CorNativeLinkType = CorNativeLinkType(4i32);
pub const nltMaxValue: CorNativeLinkType = CorNativeLinkType(7i32);
pub const nltNone: CorNativeLinkType = CorNativeLinkType(1i32);
pub const nltOle: CorNativeLinkType = CorNativeLinkType(5i32);
pub const nltUnicode: CorNativeLinkType = CorNativeLinkType(3i32);
pub const ntaReserved: NativeTypeArrayFlags = NativeTypeArrayFlags(65534i32);
pub const ntaSizeParamIndexSpecified: NativeTypeArrayFlags = NativeTypeArrayFlags(1i32);
pub const ofCheckIntegrity: CorOpenFlags = CorOpenFlags(2048i32);
pub const ofCopyMemory: CorOpenFlags = CorOpenFlags(2i32);
pub const ofNoTransform: CorOpenFlags = CorOpenFlags(4096i32);
pub const ofNoTypeLib: CorOpenFlags = CorOpenFlags(128i32);
pub const ofRead: CorOpenFlags = CorOpenFlags(0i32);
pub const ofReadOnly: CorOpenFlags = CorOpenFlags(16i32);
pub const ofReadWriteMask: CorOpenFlags = CorOpenFlags(1i32);
pub const ofReserved: CorOpenFlags = CorOpenFlags(-6336i32);
pub const ofReserved1: CorOpenFlags = CorOpenFlags(256i32);
pub const ofReserved2: CorOpenFlags = CorOpenFlags(512i32);
pub const ofReserved3: CorOpenFlags = CorOpenFlags(1024i32);
pub const ofTakeOwnership: CorOpenFlags = CorOpenFlags(32i32);
pub const ofWrite: CorOpenFlags = CorOpenFlags(1i32);
pub const pdHasDefault: CorParamAttr = CorParamAttr(4096i32);
pub const pdHasFieldMarshal: CorParamAttr = CorParamAttr(8192i32);
pub const pdIn: CorParamAttr = CorParamAttr(1i32);
pub const pdOptional: CorParamAttr = CorParamAttr(16i32);
pub const pdOut: CorParamAttr = CorParamAttr(2i32);
pub const pdReservedMask: CorParamAttr = CorParamAttr(61440i32);
pub const pdUnused: CorParamAttr = CorParamAttr(53216i32);
pub const pe32BitPreferred: CorPEKind = CorPEKind(16i32);
pub const pe32BitRequired: CorPEKind = CorPEKind(2i32);
pub const pe32Plus: CorPEKind = CorPEKind(4i32);
pub const pe32Unmanaged: CorPEKind = CorPEKind(8i32);
pub const peILonly: CorPEKind = CorPEKind(1i32);
pub const peNot: CorPEKind = CorPEKind(0i32);
pub const pmBestFitDisabled: CorPinvokeMap = CorPinvokeMap(32i32);
pub const pmBestFitEnabled: CorPinvokeMap = CorPinvokeMap(16i32);
pub const pmBestFitMask: CorPinvokeMap = CorPinvokeMap(48i32);
pub const pmBestFitUseAssem: CorPinvokeMap = CorPinvokeMap(0i32);
pub const pmCallConvCdecl: CorPinvokeMap = CorPinvokeMap(512i32);
pub const pmCallConvFastcall: CorPinvokeMap = CorPinvokeMap(1280i32);
pub const pmCallConvMask: CorPinvokeMap = CorPinvokeMap(1792i32);
pub const pmCallConvStdcall: CorPinvokeMap = CorPinvokeMap(768i32);
pub const pmCallConvThiscall: CorPinvokeMap = CorPinvokeMap(1024i32);
pub const pmCallConvWinapi: CorPinvokeMap = CorPinvokeMap(256i32);
pub const pmCharSetAnsi: CorPinvokeMap = CorPinvokeMap(2i32);
pub const pmCharSetAuto: CorPinvokeMap = CorPinvokeMap(6i32);
pub const pmCharSetMask: CorPinvokeMap = CorPinvokeMap(6i32);
pub const pmCharSetNotSpec: CorPinvokeMap = CorPinvokeMap(0i32);
pub const pmCharSetUnicode: CorPinvokeMap = CorPinvokeMap(4i32);
pub const pmMaxValue: CorPinvokeMap = CorPinvokeMap(65535i32);
pub const pmNoMangle: CorPinvokeMap = CorPinvokeMap(1i32);
pub const pmSupportsLastError: CorPinvokeMap = CorPinvokeMap(64i32);
pub const pmThrowOnUnmappableCharDisabled: CorPinvokeMap = CorPinvokeMap(8192i32);
pub const pmThrowOnUnmappableCharEnabled: CorPinvokeMap = CorPinvokeMap(4096i32);
pub const pmThrowOnUnmappableCharMask: CorPinvokeMap = CorPinvokeMap(12288i32);
pub const pmThrowOnUnmappableCharUseAssem: CorPinvokeMap = CorPinvokeMap(0i32);
pub const prHasDefault: CorPropertyAttr = CorPropertyAttr(4096i32);
pub const prRTSpecialName: CorPropertyAttr = CorPropertyAttr(1024i32);
pub const prReservedMask: CorPropertyAttr = CorPropertyAttr(62464i32);
pub const prSpecialName: CorPropertyAttr = CorPropertyAttr(512i32);
pub const prUnused: CorPropertyAttr = CorPropertyAttr(59903i32);
pub const regConfig: CorRegFlags = CorRegFlags(2i32);
pub const regHasRefs: CorRegFlags = CorRegFlags(4i32);
pub const regNoCopy: CorRegFlags = CorRegFlags(1i32);
pub const sdExecute: CeeSectionAttr = CeeSectionAttr(1610612768i64);
pub const sdNone: CeeSectionAttr = CeeSectionAttr(0i64);
pub const sdReadOnly: CeeSectionAttr = CeeSectionAttr(1073741888i64);
pub const sdReadWrite: CeeSectionAttr = CeeSectionAttr(3221225536i64);
pub const srNoBaseReloc: CeeSectionRelocType = CeeSectionRelocType(16384i32);
pub const srRelocAbsolute: CeeSectionRelocType = CeeSectionRelocType(0i32);
pub const srRelocAbsolutePtr: CeeSectionRelocType = CeeSectionRelocType(32768i32);
pub const srRelocAbsoluteTagged: CeeSectionRelocType = CeeSectionRelocType(13i32);
pub const srRelocCodeRelative: CeeSectionRelocType = CeeSectionRelocType(8i32);
pub const srRelocDir64: CeeSectionRelocType = CeeSectionRelocType(10i32);
pub const srRelocDir64Ptr: CeeSectionRelocType = CeeSectionRelocType(32778i32);
pub const srRelocFilePos: CeeSectionRelocType = CeeSectionRelocType(7i32);
pub const srRelocHighAdj: CeeSectionRelocType = CeeSectionRelocType(4i32);
pub const srRelocHighLow: CeeSectionRelocType = CeeSectionRelocType(3i32);
pub const srRelocHighLowPtr: CeeSectionRelocType = CeeSectionRelocType(32771i32);
pub const srRelocIA64Imm64: CeeSectionRelocType = CeeSectionRelocType(9i32);
pub const srRelocIA64Imm64Ptr: CeeSectionRelocType = CeeSectionRelocType(32777i32);
pub const srRelocIA64PcRel25: CeeSectionRelocType = CeeSectionRelocType(11i32);
pub const srRelocIA64PcRel64: CeeSectionRelocType = CeeSectionRelocType(12i32);
pub const srRelocMapToken: CeeSectionRelocType = CeeSectionRelocType(5i32);
pub const srRelocPtr: CeeSectionRelocType = CeeSectionRelocType(32768i32);
pub const srRelocRelative: CeeSectionRelocType = CeeSectionRelocType(6i32);
pub const srRelocRelativePtr: CeeSectionRelocType = CeeSectionRelocType(32774i32);
pub const srRelocSentinel: CeeSectionRelocType = CeeSectionRelocType(14i32);
pub const tdAbstract: CorTypeAttr = CorTypeAttr(128i32);
pub const tdAnsiClass: CorTypeAttr = CorTypeAttr(0i32);
pub const tdAutoClass: CorTypeAttr = CorTypeAttr(131072i32);
pub const tdAutoLayout: CorTypeAttr = CorTypeAttr(0i32);
pub const tdBeforeFieldInit: CorTypeAttr = CorTypeAttr(1048576i32);
pub const tdClass: CorTypeAttr = CorTypeAttr(0i32);
pub const tdClassSemanticsMask: CorTypeAttr = CorTypeAttr(32i32);
pub const tdCustomFormatClass: CorTypeAttr = CorTypeAttr(196608i32);
pub const tdCustomFormatMask: CorTypeAttr = CorTypeAttr(12582912i32);
pub const tdExplicitLayout: CorTypeAttr = CorTypeAttr(16i32);
pub const tdForwarder: CorTypeAttr = CorTypeAttr(2097152i32);
pub const tdHasSecurity: CorTypeAttr = CorTypeAttr(262144i32);
pub const tdImport: CorTypeAttr = CorTypeAttr(4096i32);
pub const tdInterface: CorTypeAttr = CorTypeAttr(32i32);
pub const tdLayoutMask: CorTypeAttr = CorTypeAttr(24i32);
pub const tdNestedAssembly: CorTypeAttr = CorTypeAttr(5i32);
pub const tdNestedFamANDAssem: CorTypeAttr = CorTypeAttr(6i32);
pub const tdNestedFamORAssem: CorTypeAttr = CorTypeAttr(7i32);
pub const tdNestedFamily: CorTypeAttr = CorTypeAttr(4i32);
pub const tdNestedPrivate: CorTypeAttr = CorTypeAttr(3i32);
pub const tdNestedPublic: CorTypeAttr = CorTypeAttr(2i32);
pub const tdNotPublic: CorTypeAttr = CorTypeAttr(0i32);
pub const tdPublic: CorTypeAttr = CorTypeAttr(1i32);
pub const tdRTSpecialName: CorTypeAttr = CorTypeAttr(2048i32);
pub const tdReservedMask: CorTypeAttr = CorTypeAttr(264192i32);
pub const tdSealed: CorTypeAttr = CorTypeAttr(256i32);
pub const tdSequentialLayout: CorTypeAttr = CorTypeAttr(8i32);
pub const tdSerializable: CorTypeAttr = CorTypeAttr(8192i32);
pub const tdSpecialName: CorTypeAttr = CorTypeAttr(1024i32);
pub const tdStringFormatMask: CorTypeAttr = CorTypeAttr(196608i32);
pub const tdUnicodeClass: CorTypeAttr = CorTypeAttr(65536i32);
pub const tdVisibilityMask: CorTypeAttr = CorTypeAttr(7i32);
pub const tdWindowsRuntime: CorTypeAttr = CorTypeAttr(16384i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COINITICOR(pub i32);
impl windows_core::TypeKind for COINITICOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COINITICOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COINITICOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COINITIEE(pub i32);
impl windows_core::TypeKind for COINITIEE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COINITIEE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COINITIEE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COUNINITIEE(pub i32);
impl windows_core::TypeKind for COUNINITIEE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COUNINITIEE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COUNINITIEE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CeeSectionAttr(pub i64);
impl windows_core::TypeKind for CeeSectionAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CeeSectionAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CeeSectionAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CeeSectionRelocType(pub i32);
impl windows_core::TypeKind for CeeSectionRelocType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CeeSectionRelocType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CeeSectionRelocType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompilationRelaxationsEnum(pub i32);
impl windows_core::TypeKind for CompilationRelaxationsEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompilationRelaxationsEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompilationRelaxationsEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorArgType(pub i32);
impl windows_core::TypeKind for CorArgType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorArgType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorArgType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorAssemblyFlags(pub i32);
impl windows_core::TypeKind for CorAssemblyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorAssemblyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorAssemblyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorAttributeTargets(pub i32);
impl windows_core::TypeKind for CorAttributeTargets {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorAttributeTargets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorAttributeTargets").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorCallingConvention(pub i32);
impl windows_core::TypeKind for CorCallingConvention {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorCallingConvention {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorCallingConvention").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorCheckDuplicatesFor(pub i32);
impl windows_core::TypeKind for CorCheckDuplicatesFor {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorCheckDuplicatesFor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorCheckDuplicatesFor").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorDeclSecurity(pub i32);
impl windows_core::TypeKind for CorDeclSecurity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorDeclSecurity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorDeclSecurity").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorElementType(pub u8);
impl windows_core::TypeKind for CorElementType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorElementType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorErrorIfEmitOutOfOrder(pub i32);
impl windows_core::TypeKind for CorErrorIfEmitOutOfOrder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorErrorIfEmitOutOfOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorErrorIfEmitOutOfOrder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorEventAttr(pub i32);
impl windows_core::TypeKind for CorEventAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorEventAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorEventAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorExceptionFlag(pub i32);
impl windows_core::TypeKind for CorExceptionFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorExceptionFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorExceptionFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorFieldAttr(pub i32);
impl windows_core::TypeKind for CorFieldAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorFieldAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorFieldAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorFileFlags(pub i32);
impl windows_core::TypeKind for CorFileFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorFileFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorFileFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorFileMapping(pub i32);
impl windows_core::TypeKind for CorFileMapping {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorFileMapping {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorFileMapping").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorGenericParamAttr(pub i32);
impl windows_core::TypeKind for CorGenericParamAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorGenericParamAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorGenericParamAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorILMethodFlags(pub i32);
impl windows_core::TypeKind for CorILMethodFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorILMethodFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorILMethodFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorILMethodSect(pub i32);
impl windows_core::TypeKind for CorILMethodSect {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorILMethodSect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorILMethodSect").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorImportOptions(pub i32);
impl windows_core::TypeKind for CorImportOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorImportOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorImportOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorLinkerOptions(pub i32);
impl windows_core::TypeKind for CorLinkerOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorLinkerOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorLinkerOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorLocalRefPreservation(pub i32);
impl windows_core::TypeKind for CorLocalRefPreservation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorLocalRefPreservation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorLocalRefPreservation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorManifestResourceFlags(pub i32);
impl windows_core::TypeKind for CorManifestResourceFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorManifestResourceFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorManifestResourceFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorMethodAttr(pub i32);
impl windows_core::TypeKind for CorMethodAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorMethodAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorMethodAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorMethodImpl(pub i32);
impl windows_core::TypeKind for CorMethodImpl {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorMethodImpl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorMethodImpl").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorMethodSemanticsAttr(pub i32);
impl windows_core::TypeKind for CorMethodSemanticsAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorMethodSemanticsAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorMethodSemanticsAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorNativeLinkFlags(pub i32);
impl windows_core::TypeKind for CorNativeLinkFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorNativeLinkFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorNativeLinkFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorNativeLinkType(pub i32);
impl windows_core::TypeKind for CorNativeLinkType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorNativeLinkType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorNativeLinkType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorNativeType(pub i32);
impl windows_core::TypeKind for CorNativeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorNativeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorNativeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorNotificationForTokenMovement(pub i32);
impl windows_core::TypeKind for CorNotificationForTokenMovement {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorNotificationForTokenMovement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorNotificationForTokenMovement").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorOpenFlags(pub i32);
impl windows_core::TypeKind for CorOpenFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorOpenFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorOpenFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorPEKind(pub i32);
impl windows_core::TypeKind for CorPEKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorPEKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorPEKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorParamAttr(pub i32);
impl windows_core::TypeKind for CorParamAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorParamAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorParamAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorPinvokeMap(pub i32);
impl windows_core::TypeKind for CorPinvokeMap {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorPinvokeMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorPinvokeMap").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorPropertyAttr(pub i32);
impl windows_core::TypeKind for CorPropertyAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorPropertyAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorPropertyAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorRefToDefCheck(pub i32);
impl windows_core::TypeKind for CorRefToDefCheck {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorRefToDefCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorRefToDefCheck").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorRegFlags(pub i32);
impl windows_core::TypeKind for CorRegFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorRegFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorRegFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorSaveSize(pub i32);
impl windows_core::TypeKind for CorSaveSize {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorSaveSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorSaveSize").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorSerializationType(pub i32);
impl windows_core::TypeKind for CorSerializationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorSerializationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorSerializationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorSetENC(pub i32);
impl windows_core::TypeKind for CorSetENC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorSetENC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorSetENC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorThreadSafetyOptions(pub i32);
impl windows_core::TypeKind for CorThreadSafetyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorThreadSafetyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorThreadSafetyOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorTokenType(pub i32);
impl windows_core::TypeKind for CorTokenType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorTokenType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorTypeAttr(pub i32);
impl windows_core::TypeKind for CorTypeAttr {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorTypeAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorTypeAttr").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorUnmanagedCallingConvention(pub i32);
impl windows_core::TypeKind for CorUnmanagedCallingConvention {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorUnmanagedCallingConvention {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorUnmanagedCallingConvention").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorValidatorModuleType(pub i32);
impl windows_core::TypeKind for CorValidatorModuleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorValidatorModuleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorValidatorModuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LoadHintEnum(pub i32);
impl windows_core::TypeKind for LoadHintEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LoadHintEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LoadHintEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MergeFlags(pub i32);
impl windows_core::TypeKind for MergeFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MergeFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MergeFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NGenHintEnum(pub i32);
impl windows_core::TypeKind for NGenHintEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NGenHintEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NGenHintEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NativeTypeArrayFlags(pub i32);
impl windows_core::TypeKind for NativeTypeArrayFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NativeTypeArrayFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NativeTypeArrayFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ReplacesGeneralNumericDefines(pub i32);
impl windows_core::TypeKind for ReplacesGeneralNumericDefines {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ReplacesGeneralNumericDefines {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ReplacesGeneralNumericDefines").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ASSEMBLYMETADATA {
    pub usMajorVersion: u16,
    pub usMinorVersion: u16,
    pub usBuildNumber: u16,
    pub usRevisionNumber: u16,
    pub szLocale: windows_core::PWSTR,
    pub cbLocale: u32,
    pub rProcessor: *mut u32,
    pub ulProcessor: u32,
    pub rOS: *mut OSINFO,
    pub ulOS: u32,
}
impl windows_core::TypeKind for ASSEMBLYMETADATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ASSEMBLYMETADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_FIELD_OFFSET {
    pub ridOfField: u32,
    pub ulOffset: u32,
}
impl windows_core::TypeKind for COR_FIELD_OFFSET {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_FIELD_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct COR_NATIVE_LINK {
    pub m_linkType: u8,
    pub m_flags: u8,
    pub m_entryPoint: u32,
}
impl windows_core::TypeKind for COR_NATIVE_LINK {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_NATIVE_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_SECATTR {
    pub tkCtor: u32,
    pub pCustomAttribute: *const core::ffi::c_void,
    pub cbCustomAttribute: u32,
}
impl windows_core::TypeKind for COR_SECATTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_SECATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CVStruct {
    pub Major: i16,
    pub Minor: i16,
    pub Sub: i16,
    pub Build: i16,
}
impl windows_core::TypeKind for CVStruct {
    type TypeKind = windows_core::CopyType;
}
impl Default for CVStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CeeSectionRelocExtra {
    pub highAdj: u16,
}
impl windows_core::TypeKind for CeeSectionRelocExtra {
    type TypeKind = windows_core::CopyType;
}
impl Default for CeeSectionRelocExtra {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR_ILMETHOD {
    pub Tiny: IMAGE_COR_ILMETHOD_TINY,
    pub Fat: IMAGE_COR_ILMETHOD_FAT,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_FAT {
    pub _bitfield: u32,
    pub CodeSize: u32,
    pub LocalVarSigTok: u32,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_FAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR_ILMETHOD_SECT_EH {
    pub Small: IMAGE_COR_ILMETHOD_SECT_EH_SMALL,
    pub Fat: IMAGE_COR_ILMETHOD_SECT_EH_FAT,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    pub Flags: CorExceptionFlag,
    pub TryOffset: u32,
    pub TryLength: u32,
    pub HandlerOffset: u32,
    pub HandlerLength: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: i32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    pub SectFat: IMAGE_COR_ILMETHOD_SECT_FAT,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT; 1],
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    pub SectSmall: IMAGE_COR_ILMETHOD_SECT_SMALL,
    pub Reserved: u16,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL; 1],
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_FAT {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_FAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_SMALL {
    pub Kind: u8,
    pub DataSize: u8,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_SMALL {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_TINY {
    pub Flags_CodeSize: u8,
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_TINY {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_ILMETHOD_TINY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMAGE_COR_VTABLEFIXUP {
    pub RVA: u32,
    pub Count: u16,
    pub Type: u16,
}
impl windows_core::TypeKind for IMAGE_COR_VTABLEFIXUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMAGE_COR_VTABLEFIXUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OSINFO {
    pub dwOSPlatformId: u32,
    pub dwOSMajorVersion: u32,
    pub dwOSMinorVersion: u32,
}
impl windows_core::TypeKind for OSINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for OSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
