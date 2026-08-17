#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> windows_core::Result<IVssExpressWriter> {
    windows_targets::link!("vssapi.dll" "system" fn CreateVssExpressWriterInternal(ppwriter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateVssExpressWriterInternal(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(IVssAdmin, IVssAdmin_Vtbl, 0x77ed5996_2f63_11d3_8a39_00c04f72d8e3);
impl core::ops::Deref for IVssAdmin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssAdmin, windows_core::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider(&self, pproviderid: windows_core::GUID, classid: windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(pproviderid), core::mem::transmute(classid), pwszprovidername, eprovidertype, pwszproviderversion, core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> windows_core::Result<IVssEnumObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortAllSnapshotsInProgress)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IVssAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, VSS_PROVIDER_TYPE, *const u16, windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AbortAllSnapshotsInProgress: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssAdminEx, IVssAdminEx_Vtbl, 0x7858a9f8_b1fa_41a6_964f_b9b36b8cd8d8);
impl core::ops::Deref for IVssAdminEx {
    type Target = IVssAdmin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssAdminEx, windows_core::IUnknown, IVssAdmin);
impl IVssAdminEx {
    pub unsafe fn GetProviderCapability(&self, pproviderid: windows_core::GUID) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderCapability)(windows_core::Interface::as_raw(self), core::mem::transmute(pproviderid), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProviderContext(&self, providerid: windows_core::GUID) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderContext)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProviderContext(&self, providerid: windows_core::GUID, lcontext: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProviderContext)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), lcontext).ok()
    }
}
#[repr(C)]
pub struct IVssAdminEx_Vtbl {
    pub base__: IVssAdmin_Vtbl,
    pub GetProviderCapability: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u64) -> windows_core::HRESULT,
    pub GetProviderContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut i32) -> windows_core::HRESULT,
    pub SetProviderContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssAsync, IVssAsync_Vtbl, 0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
impl core::ops::Deref for IVssAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssAsync, windows_core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), phrresult, preserved).ok()
    }
}
#[repr(C)]
pub struct IVssAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssComponent, IVssComponent_Vtbl, 0xd2c72c96_c121_4518_b627_e5a93d010ead);
impl core::ops::Deref for IVssComponent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponent, windows_core::IUnknown);
impl IVssComponent {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentType)(windows_core::Interface::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackupSucceeded)(windows_core::Interface::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAlternateLocationMappingCount)(windows_core::Interface::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetBackupMetadata)(windows_core::Interface::as_raw(self), wszdata.param().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackupMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddPartialFile)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilename.param().abi(), wszranges.param().abi(), wszmetadata.param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPartialFileCount)(windows_core::Interface::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPartialFile)(windows_core::Interface::as_raw(self), ipartialfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilename), core::mem::transmute(pbstrrange), core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSelectedForRestore)(windows_core::Interface::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAdditionalRestores)(windows_core::Interface::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNewTargetCount)(windows_core::Interface::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNewTarget)(windows_core::Interface::as_raw(self), inewtarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddDirectedTarget)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilename.param().abi(), wszsourcerangelist.param().abi(), wszdestinationpath.param().abi(), wszdestinationfilename.param().abi(), wszdestinationrangelist.param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDirectedTargetCount)(windows_core::Interface::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDirectedTarget)(windows_core::Interface::as_raw(self), idirectedtarget, core::mem::transmute(pbstrsourcepath), core::mem::transmute(pbstrsourcefilename), core::mem::transmute(pbstrsourcerangelist), core::mem::transmute(pbstrdestinationpath), core::mem::transmute(pbstrdestinationfilename), core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRestoreMetadata)(windows_core::Interface::as_raw(self), wszrestoremetadata.param().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestoreMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRestoreTarget)(windows_core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestoreTarget)(windows_core::Interface::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszprerestorefailuremsg.param().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.param().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetBackupStamp)(windows_core::Interface::as_raw(self), wszbackupstamp.param().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackupStamp)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPreviousBackupStamp)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBackupOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestoreOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestoreSubcomponentCount)(windows_core::Interface::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRestoreSubcomponent)(windows_core::Interface::as_raw(self), icomponent, core::mem::transmute(pbstrlogicalpath), core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFileRestoreStatus)(windows_core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyTime)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.param().abi(), core::mem::transmute(ftlastmodifytime)).ok()
    }
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyLSN)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.param().abi(), bstrlsnstring.param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDifferencedFilesCount)(windows_core::Interface::as_raw(self), pcdifferencedfiles).ok()
    }
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDifferencedFile)(windows_core::Interface::as_raw(self), idifferencedfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilespec), pbrecursive, core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
    }
}
#[repr(C)]
pub struct IVssComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBackupSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsSelectedForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPreviousBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBackupOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRestoreOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRestoreSubcomponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut bool) -> windows_core::HRESULT,
    pub GetFileRestoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT,
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDifferencedFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssComponentEx, IVssComponentEx_Vtbl, 0x156c8b5e_f131_4bd7_9c97_d1923be7e1fa);
impl core::ops::Deref for IVssComponentEx {
    type Target = IVssComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponentEx, windows_core::IUnknown, IVssComponent);
impl IVssComponentEx {
    pub unsafe fn SetPrepareForBackupFailureMsg<P0>(&self, wszfailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()).ok()
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAuthoritativeRestore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRollForward)(windows_core::Interface::as_raw(self), prolltype, core::mem::transmute(pbstrpoint)).ok()
    }
    pub unsafe fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestoreName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssComponentEx_Vtbl {
    pub base__: IVssComponent_Vtbl,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetRollForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_ROLLFORWARD_TYPE, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRestoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssComponentEx2, IVssComponentEx2_Vtbl, 0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
impl core::ops::Deref for IVssComponentEx2 {
    type Target = IVssComponentEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponentEx2, windows_core::IUnknown, IVssComponent, IVssComponentEx);
impl IVssComponentEx2 {
    pub unsafe fn SetFailure<P0>(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: P0, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFailure)(windows_core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.param().abi(), dwreserved).ok()
    }
    pub unsafe fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFailure)(windows_core::Interface::as_raw(self), phr, phrapplication, core::mem::transmute(pbstrapplicationmessage), pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IVssComponentEx2_Vtbl {
    pub base__: IVssComponentEx_Vtbl,
    pub SetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::HRESULT, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut windows_core::HRESULT, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssCreateExpressWriterMetadata, IVssCreateExpressWriterMetadata_Vtbl, 0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
impl core::ops::Deref for IVssCreateExpressWriterMetadata {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssCreateExpressWriterMetadata, windows_core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive).ok()
    }
    pub unsafe fn AddComponent<P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddComponentDependency<P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: windows_core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask).ok()
    }
    pub unsafe fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssCreateExpressWriterMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddExcludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u8) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, u8, u8, u8, u8, u32) -> windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u8, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTOREMETHOD_ENUM, windows_core::PCWSTR, windows_core::PCWSTR, VSS_WRITERRESTORE_ENUM, u8) -> windows_core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssCreateWriterMetadata, IVssCreateWriterMetadata_Vtbl);
impl IVssCreateWriterMetadata {
    pub unsafe fn AddIncludeFiles<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8, wszalternatelocation: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddIncludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi()).ok()
    }
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive).ok()
    }
    pub unsafe fn AddComponent<P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddDatabaseFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddDatabaseFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddDatabaseLogFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddDatabaseLogFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddAlternateLocationMapping<P0, P1, P2>(&self, wszsourcepath: P0, wszsourcefilespec: P1, brecursive: u8, wszdestination: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAlternateLocationMapping)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilespec.param().abi(), brecursive, wszdestination.param().abi()).ok()
    }
    pub unsafe fn AddComponentDependency<P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: windows_core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask).ok()
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocument(&self) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrxml)).ok()
    }
}
#[repr(C)]
pub struct IVssCreateWriterMetadata_Vtbl {
    pub AddIncludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u8, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddExcludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u8) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, u8, u8, u8, u8, u32) -> windows_core::HRESULT,
    pub AddDatabaseFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddDatabaseLogFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u8, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTOREMETHOD_ENUM, windows_core::PCWSTR, windows_core::PCWSTR, VSS_WRITERRESTORE_ENUM, u8) -> windows_core::HRESULT,
    pub AddAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u8, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetDocument: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssDifferentialSoftwareSnapshotMgmt, IVssDifferentialSoftwareSnapshotMgmt_Vtbl, 0x214a0f28_b737_4026_b847_4f9e37d79529);
impl core::ops::Deref for IVssDifferentialSoftwareSnapshotMgmt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt, windows_core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddDiffArea)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSize)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryVolumesSupportedForDiffAreas)(windows_core::Interface::as_raw(self), pwszoriginalvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDiffAreasForVolume)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDiffAreasOnVolume)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: windows_core::GUID) -> windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryDiffAreasForSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddDiffArea: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64) -> windows_core::HRESULT,
    pub ChangeDiffAreaMaximumSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64) -> windows_core::HRESULT,
    pub QueryVolumesSupportedForDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasForVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasOnVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasForSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssDifferentialSoftwareSnapshotMgmt2, IVssDifferentialSoftwareSnapshotMgmt2_Vtbl, 0x949d7353_675f_4275_8969_f044c6277815);
impl core::ops::Deref for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Target = IVssDifferentialSoftwareSnapshotMgmt;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt2, windows_core::IUnknown, IVssDifferentialSoftwareSnapshotMgmt);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSizeEx)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, bvolatile.param().abi()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MigrateDiffAreas)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> windows_core::Result<IVssAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryMigrationStatus)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: windows_core::GUID, priority: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSnapshotPriority)(windows_core::Interface::as_raw(self), core::mem::transmute(idsnapshot), priority).ok()
    }
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl,
    pub ChangeDiffAreaMaximumSizeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MigrateDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
    pub QueryMigrationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSnapshotPriority: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssDifferentialSoftwareSnapshotMgmt3, IVssDifferentialSoftwareSnapshotMgmt3_Vtbl, 0x383f7e71_a4c5_401f_b27f_f826289f8458);
impl core::ops::Deref for IVssDifferentialSoftwareSnapshotMgmt3 {
    type Target = IVssDifferentialSoftwareSnapshotMgmt2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt3, windows_core::IUnknown, IVssDifferentialSoftwareSnapshotMgmt, IVssDifferentialSoftwareSnapshotMgmt2);
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolumeProtectLevel)(windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel).ok()
    }
    pub unsafe fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVolumeProtectLevel)(windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel).ok()
    }
    pub unsafe fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearVolumeProtectFault)(windows_core::Interface::as_raw(self), pwszvolumename).ok()
    }
    pub unsafe fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteUnusedDiffAreas)(windows_core::Interface::as_raw(self), pwszdiffareavolumename).ok()
    }
    pub unsafe fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: windows_core::GUID, idsnapshotyounger: windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QuerySnapshotDeltaBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(idsnapshotolder), core::mem::transmute(idsnapshotyounger), pcblocksizeperbit, pcbitmaplength, ppbbitmap).ok()
    }
}
#[repr(C)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl,
    pub SetVolumeProtectLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, VSS_PROTECTION_LEVEL) -> windows_core::HRESULT,
    pub GetVolumeProtectLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::HRESULT,
    pub ClearVolumeProtectFault: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub DeleteUnusedDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub QuerySnapshotDeltaBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *mut u32, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssEnumMgmtObject, IVssEnumMgmtObject_Vtbl, 0x01954e6b_9254_4e6e_808c_c9e05d007696);
impl core::ops::Deref for IVssEnumMgmtObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssEnumMgmtObject, windows_core::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_MGMT_OBJECT_PROP], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<IVssEnumMgmtObject>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)).ok()
    }
}
#[repr(C)]
pub struct IVssEnumMgmtObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VSS_MGMT_OBJECT_PROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssEnumObject, IVssEnumObject_Vtbl, 0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
impl core::ops::Deref for IVssEnumObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssEnumObject, windows_core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_OBJECT_PROP], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<IVssEnumObject>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)).ok()
    }
}
#[repr(C)]
pub struct IVssEnumObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VSS_OBJECT_PROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssExpressWriter, IVssExpressWriter_Vtbl, 0xe33affdc_59c7_47b1_97d5_4266598f6235);
impl core::ops::Deref for IVssExpressWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssExpressWriter, windows_core::IUnknown);
impl IVssExpressWriter {
    pub unsafe fn CreateMetadata<P0>(&self, writerid: windows_core::GUID, writername: P0, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), writername.param().abi(), usagetype, versionmajor, versionminor, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadMetadata<P0>(&self, metadata: P0, reserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadMetadata)(windows_core::Interface::as_raw(self), metadata.param().abi(), reserved).ok()
    }
    pub unsafe fn Register(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unregister(&self, writerid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid)).ok()
    }
}
#[repr(C)]
pub struct IVssExpressWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::PCWSTR, VSS_USAGE_TYPE, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssFileShareSnapshotProvider, IVssFileShareSnapshotProvider_Vtbl, 0xc8636060_7c2e_11df_8c4a_0800200c9a66);
impl core::ops::Deref for IVssFileShareSnapshotProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssFileShareSnapshotProvider, windows_core::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), pprop).ok()
    }
    pub unsafe fn Query(&self, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteSnapshots<P0>(&self, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.param().abi(), pldeletedsnapshots, pnondeletedsnapshotid).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), pwszsharepath, lnewcontext, core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn IsPathSupported(&self, pwszsharepath: *const u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPathSupported)(windows_core::Interface::as_raw(self), pwszsharepath, &mut result__).map(|| result__)
    }
    pub unsafe fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsPathSnapshotted)(windows_core::Interface::as_raw(self), pwszsharepath, pbsnapshotspresent, plsnapshotcompatibility).ok()
    }
    pub unsafe fn SetSnapshotProperty<P0>(&self, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSnapshotProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), esnapshotpropertyid, vproperty.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVssFileShareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, VSS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, super::super::Foundation::BOOL, *mut i32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, i32, windows_core::GUID) -> windows_core::HRESULT,
    pub IsPathSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsPathSnapshotted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    pub SetSnapshotProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_SNAPSHOT_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssHardwareSnapshotProvider, IVssHardwareSnapshotProvider_Vtbl, 0x9593a157_44e9_4344_bbeb_44fbf9b06b10);
impl core::ops::Deref for IVssHardwareSnapshotProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssHardwareSnapshotProvider, windows_core::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AreLunsSupported)(windows_core::Interface::as_raw(self), lluncount, lcontext, rgwszdevices, pluninformation, pbissupported).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FillInLunInfo)(windows_core::Interface::as_raw(self), wszdevicename, pluninfo, pbissupported).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), lcontext, lluncount, rgdevicenames, rgluninformation).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTargetLuns)(windows_core::Interface::as_raw(self), lluncount, rgdevicenames, rgsourceluns, rgdestinationluns).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LocateLuns)(windows_core::Interface::as_raw(self), rgsourceluns.len().try_into().unwrap(), core::mem::transmute(rgsourceluns.as_ptr())).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLunEmpty)(windows_core::Interface::as_raw(self), wszdevicename, pinformation).ok()
    }
}
#[repr(C)]
pub struct IVssHardwareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub AreLunsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const *const u16, *mut super::VirtualDiskService::VDS_LUN_INFORMATION, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    AreLunsSupported: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub FillInLunInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::VirtualDiskService::VDS_LUN_INFORMATION, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    FillInLunInfo: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub BeginPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, i32, i32, *const *const u16, *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    BeginPrepareSnapshot: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub GetTargetLuns: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const *const u16, *const super::VirtualDiskService::VDS_LUN_INFORMATION, *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    GetTargetLuns: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub LocateLuns: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    LocateLuns: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub OnLunEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    OnLunEmpty: usize,
}
windows_core::imp::define_interface!(IVssHardwareSnapshotProviderEx, IVssHardwareSnapshotProviderEx_Vtbl, 0x7f5ba925_cdb1_4d11_a71f_339eb7e709fd);
impl core::ops::Deref for IVssHardwareSnapshotProviderEx {
    type Target = IVssHardwareSnapshotProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssHardwareSnapshotProviderEx, windows_core::IUnknown, IVssHardwareSnapshotProvider);
impl IVssHardwareSnapshotProviderEx {
    pub unsafe fn GetProviderCapabilities(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLunStateChange)(windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount, dwflags).ok()
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<IVssAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResyncLuns)(windows_core::Interface::as_raw(self), psourceluns, ptargetluns, dwcount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnReuseLuns)(windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount).ok()
    }
}
#[repr(C)]
pub struct IVssHardwareSnapshotProviderEx_Vtbl {
    pub base__: IVssHardwareSnapshotProvider_Vtbl,
    pub GetProviderCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub OnLunStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VirtualDiskService::VDS_LUN_INFORMATION, *const super::VirtualDiskService::VDS_LUN_INFORMATION, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    OnLunStateChange: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub ResyncLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VirtualDiskService::VDS_LUN_INFORMATION, *const super::VirtualDiskService::VDS_LUN_INFORMATION, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    ResyncLuns: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub OnReuseLuns: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VirtualDiskService::VDS_LUN_INFORMATION, *const super::VirtualDiskService::VDS_LUN_INFORMATION, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    OnReuseLuns: usize,
}
windows_core::imp::define_interface!(IVssProviderCreateSnapshotSet, IVssProviderCreateSnapshotSet_Vtbl, 0x5f894e5b_1e39_4778_8e23_9abad9f0e08c);
impl core::ops::Deref for IVssProviderCreateSnapshotSet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssProviderCreateSnapshotSet, windows_core::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndPrepareSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PreCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn CommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostCommitSnapshots(&self, snapshotsetid: windows_core::GUID, lsnapshotscount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PostCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), lsnapshotscount).ok()
    }
    pub unsafe fn PreFinalCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreFinalCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostFinalCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PostFinalCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn AbortSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbortSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok()
    }
}
#[repr(C)]
pub struct IVssProviderCreateSnapshotSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EndPrepareSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PreCommitSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub CommitSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PostCommitSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32) -> windows_core::HRESULT,
    pub PreFinalCommitSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PostFinalCommitSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub AbortSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssProviderNotifications, IVssProviderNotifications_Vtbl, 0xe561901f_03a5_4afe_86d0_72baeece7004);
impl core::ops::Deref for IVssProviderNotifications {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssProviderNotifications, windows_core::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn OnUnload<P0>(&self, bforceunload: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self), bforceunload.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IVssProviderNotifications_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssSnapshotMgmt, IVssSnapshotMgmt_Vtbl, 0xfa7df749_66e7_4986_a27f_e2f04ae53772);
impl core::ops::Deref for IVssSnapshotMgmt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt, windows_core::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface(&self, providerid: windows_core::GUID, interfaceid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProviderMgmtInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), interfaceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots(&self, providerid: windows_core::GUID, lcontext: i32) -> windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryVolumesSupportedForSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), lcontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: windows_core::GUID) -> windows_core::Result<IVssEnumObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySnapshotsByVolume)(windows_core::Interface::as_raw(self), pwszvolumename, core::mem::transmute(providerid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssSnapshotMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProviderMgmtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryVolumesSupportedForSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySnapshotsByVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssSnapshotMgmt2, IVssSnapshotMgmt2_Vtbl, 0x0f61ec39_fe82_45f2_a3f0_768b5d427102);
impl core::ops::Deref for IVssSnapshotMgmt2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt2, windows_core::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinDiffAreaSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVssSnapshotMgmt2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMinDiffAreaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssSoftwareSnapshotProvider, IVssSoftwareSnapshotProvider_Vtbl, 0x609e123e_2c5a_44d3_8f01_0b1d9a47d1ff);
impl core::ops::Deref for IVssSoftwareSnapshotProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssSoftwareSnapshotProvider, windows_core::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), pprop).ok()
    }
    pub unsafe fn Query(&self, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteSnapshots<P0>(&self, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.param().abi(), pldeletedsnapshots, pnondeletedsnapshotid).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), pwszvolumename, lnewcontext).ok()
    }
    pub unsafe fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsVolumeSupported)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).map(|| result__)
    }
    pub unsafe fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsVolumeSnapshotted)(windows_core::Interface::as_raw(self), pwszvolumename, pbsnapshotspresent, plsnapshotcompatibility).ok()
    }
    pub unsafe fn SetSnapshotProperty<P0>(&self, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSnapshotProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), esnapshotpropertyid, vproperty.param().abi()).ok()
    }
    pub unsafe fn RevertToSnapshot(&self, snapshotid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevertToSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid)).ok()
    }
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: *const u16) -> windows_core::Result<IVssAsync> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryRevertStatus)(windows_core::Interface::as_raw(self), pwszvolume, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssSoftwareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, VSS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, super::super::Foundation::BOOL, *mut i32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, i32) -> windows_core::HRESULT,
    pub IsVolumeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsVolumeSnapshotted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    pub SetSnapshotProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_SNAPSHOT_PROPERTY_ID, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RevertToSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssWMDependency, IVssWMDependency_Vtbl, 0);
impl core::ops::Deref for IVssWMDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssWMDependency, windows_core::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(&self, pwriterid: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWriterId)(windows_core::Interface::as_raw(self), pwriterid).ok()
    }
    pub unsafe fn GetLogicalPath(&self, pbstrlogicalpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrlogicalpath)).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrcomponentname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcomponentname)).ok()
    }
}
#[repr(C)]
pub struct IVssWMDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWriterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssWMFiledesc, IVssWMFiledesc_Vtbl, 0);
impl core::ops::Deref for IVssWMFiledesc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssWMFiledesc, windows_core::IUnknown);
impl IVssWMFiledesc {
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilespec)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRecursive(&self) -> windows_core::Result<bool> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecursive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAlternateLocation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBackupTypeMask(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBackupTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IVssWMFiledesc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFilespec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRecursive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetBackupTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVssWriterComponents, IVssWriterComponents_Vtbl);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self, pccomponents: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComponentCount)(windows_core::Interface::as_raw(self), pccomponents).ok()
    }
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWriterInfo)(windows_core::Interface::as_raw(self), pidinstance, pidwriter).ok()
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVssWriterComponents_Vtbl {
    pub GetComponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWriterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(-1i32);
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(2i32);
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(3i32);
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(1i32);
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(4i32);
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(0i32);
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(2i32);
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(1i32);
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(3i32);
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(0i32);
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2i32);
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(4i32);
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(8i32);
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16384i32);
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16i32);
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1i32);
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4i32);
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(2i32);
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1024i32);
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(64i32);
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8i32);
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(128i32);
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8192i32);
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4096i32);
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32i32);
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(0i32);
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(256i32);
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32768i32);
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(512i32);
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(5i32);
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(3i32);
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(1i32);
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(2i32);
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(4i32);
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(6i32);
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(0i32);
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(2i32);
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(1i32);
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(4i32);
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(-1i32);
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(9i32);
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(0i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(29i32);
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(13i32);
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(16i32);
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(25i32);
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(1i32);
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(2i32);
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(0i32);
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x80042408_u32 as _);
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x80042415_u32 as _);
pub const VSS_E_ASRERROR_DATADISK_RDISK0: windows_core::HRESULT = windows_core::HRESULT(0x80042406_u32 as _);
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042401_u32 as _);
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80042402_u32 as _);
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004240A_u32 as _);
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: windows_core::HRESULT = windows_core::HRESULT(0x80042414_u32 as _);
pub const VSS_E_ASRERROR_MISSING_DYNDISK: windows_core::HRESULT = windows_core::HRESULT(0x80042404_u32 as _);
pub const VSS_E_ASRERROR_NO_ARCPATH: windows_core::HRESULT = windows_core::HRESULT(0x80042403_u32 as _);
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80042413_u32 as _);
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x80042407_u32 as _);
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042412_u32 as _);
pub const VSS_E_ASRERROR_SHARED_CRIDISK: windows_core::HRESULT = windows_core::HRESULT(0x80042405_u32 as _);
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80042416_u32 as _);
pub const VSS_E_AUTORECOVERY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800423FB_u32 as _);
pub const VSS_E_BAD_STATE: windows_core::HRESULT = windows_core::HRESULT(0x80042301_u32 as _);
pub const VSS_E_BREAK_REVERT_ID_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800423F6_u32 as _);
pub const VSS_E_CANNOT_REVERT_DISKID: windows_core::HRESULT = windows_core::HRESULT(0x800423FE_u32 as _);
pub const VSS_E_CLUSTER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042400_u32 as _);
pub const VSS_E_CLUSTER_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004232E_u32 as _);
pub const VSS_E_CORRUPT_XML_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80042310_u32 as _);
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80042411_u32 as _);
pub const VSS_E_DYNAMIC_DISK_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x800423FC_u32 as _);
pub const VSS_E_FLUSH_WRITES_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042313_u32 as _);
pub const VSS_E_FSS_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042417_u32 as _);
pub const VSS_E_HOLD_WRITES_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042314_u32 as _);
pub const VSS_E_INSUFFICIENT_STORAGE: windows_core::HRESULT = windows_core::HRESULT(0x8004231F_u32 as _);
pub const VSS_E_INVALID_XML_DOCUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80042311_u32 as _);
pub const VSS_E_LEGACY_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x800423F7_u32 as _);
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x8004231E_u32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042322_u32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042317_u32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80042312_u32 as _);
pub const VSS_E_MISSING_DISK: windows_core::HRESULT = windows_core::HRESULT(0x800423F8_u32 as _);
pub const VSS_E_MISSING_HIDDEN_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x800423F9_u32 as _);
pub const VSS_E_MISSING_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x800423FA_u32 as _);
pub const VSS_E_NESTED_VOLUME_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x8004232C_u32 as _);
pub const VSS_E_NONTRANSPORTABLE_BCD: windows_core::HRESULT = windows_core::HRESULT(0x800423FD_u32 as _);
pub const VSS_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004232F_u32 as _);
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042320_u32 as _);
pub const VSS_E_OBJECT_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8004230D_u32 as _);
pub const VSS_E_OBJECT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80042308_u32 as _);
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80042303_u32 as _);
pub const VSS_E_PROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80042307_u32 as _);
pub const VSS_E_PROVIDER_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80042304_u32 as _);
pub const VSS_E_PROVIDER_VETO: windows_core::HRESULT = windows_core::HRESULT(0x80042306_u32 as _);
pub const VSS_E_REBOOT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80042327_u32 as _);
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80042323_u32 as _);
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042324_u32 as _);
pub const VSS_E_RESYNC_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x800423FF_u32 as _);
pub const VSS_E_REVERT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80042325_u32 as _);
pub const VSS_E_REVERT_VOLUME_LOST: windows_core::HRESULT = windows_core::HRESULT(0x80042326_u32 as _);
pub const VSS_E_SNAPSHOT_NOT_IN_SET: windows_core::HRESULT = windows_core::HRESULT(0x8004232B_u32 as _);
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80042316_u32 as _);
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80042321_u32 as _);
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042328_u32 as _);
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80042329_u32 as _);
pub const VSS_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80042302_u32 as _);
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8004230F_u32 as _);
pub const VSS_E_UNEXPECTED_WRITER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80042315_u32 as _);
pub const VSS_E_UNSELECTED_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x8004232A_u32 as _);
pub const VSS_E_UNSUPPORTED_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x8004231B_u32 as _);
pub const VSS_E_VOLUME_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8004231D_u32 as _);
pub const VSS_E_VOLUME_NOT_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x8004232D_u32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8004230C_u32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x8004230E_u32 as _);
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: windows_core::HRESULT = windows_core::HRESULT(0x800423F0_u32 as _);
pub const VSS_E_WRITERERROR_NONRETRYABLE: windows_core::HRESULT = windows_core::HRESULT(0x800423F4_u32 as _);
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x800423F1_u32 as _);
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80042336_u32 as _);
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800423F5_u32 as _);
pub const VSS_E_WRITERERROR_RETRYABLE: windows_core::HRESULT = windows_core::HRESULT(0x800423F3_u32 as _);
pub const VSS_E_WRITERERROR_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x800423F2_u32 as _);
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: windows_core::HRESULT = windows_core::HRESULT(0x8004231A_u32 as _);
pub const VSS_E_WRITER_INFRASTRUCTURE: windows_core::HRESULT = windows_core::HRESULT(0x80042318_u32 as _);
pub const VSS_E_WRITER_NOT_RESPONDING: windows_core::HRESULT = windows_core::HRESULT(0x80042319_u32 as _);
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80042409_u32 as _);
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(15i32);
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(3840i32);
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(65536i32);
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2i32);
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(512i32);
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1i32);
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(256i32);
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(4i32);
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1024i32);
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(8i32);
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2048i32);
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1024i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(512i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(256i32);
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(6i32);
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(7i32);
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(11i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(8i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(1i32);
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(14i32);
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(15i32);
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(12i32);
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(10i32);
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(9i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(13i32);
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(2i32);
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(5i32);
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(4i32);
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(3i32);
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(16i32);
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(0i32);
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(4i32);
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(3i32);
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(2i32);
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(1i32);
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(0i32);
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(512i32);
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(2i32);
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(256i32);
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(1i32);
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(4i32);
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(8i32);
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(32i32);
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(16i32);
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(128i32);
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(64i32);
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(512i32);
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(2i32);
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(1i32);
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(3i32);
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(0i32);
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(7i32);
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(5i32);
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(6i32);
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(2i32);
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(1i32);
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(8i32);
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(4i32);
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(3i32);
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(0i32);
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(2i32);
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(3i32);
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(1i32);
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(0i32);
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(1i32);
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(2i32);
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(3i32);
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(0i32);
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(2i32);
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(3i32);
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(1i32);
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(4i32);
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(0i32);
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(2i32);
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(1i32);
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(-1i32);
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(2i32);
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(8i32);
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(1i32);
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(4i32);
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(12i32);
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(8i32);
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(9i32);
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(5i32);
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(6i32);
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(10i32);
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(7i32);
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(3i32);
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(11i32);
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(4i32);
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(1i32);
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(2i32);
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(13i32);
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(0i32);
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(13i32);
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(7i32);
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(16i32);
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(12i32);
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(14i32);
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(15i32);
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(5i32);
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(10i32);
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(3i32);
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(1i32);
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(6i32);
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(8i32);
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(11i32);
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(4i32);
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(9i32);
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(2i32);
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(0i32);
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(2i32);
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(3i32);
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(1i32);
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(0i32);
pub const VSS_S_ASYNC_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x4230B_u32 as _);
pub const VSS_S_ASYNC_FINISHED: windows_core::HRESULT = windows_core::HRESULT(0x4230A_u32 as _);
pub const VSS_S_ASYNC_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x42309_u32 as _);
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: windows_core::HRESULT = windows_core::HRESULT(0x42321_u32 as _);
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(1i32);
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = VSS_USAGE_TYPE(4i32);
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(2i32);
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = VSS_USAGE_TYPE(0i32);
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = VSS_USAGE_TYPE(3i32);
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4194304i32);
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4i32);
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16777216i32);
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(131072i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1048576i32);
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2097152i32);
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(67108864i32);
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(65536i32);
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(524288i32);
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(64i32);
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(128i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2i32);
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8i32);
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16i32);
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1i32);
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(262144i32);
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8388608i32);
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(32i32);
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(33554432i32);
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
pub const VSS_WS_COUNT: VSS_WRITER_STATE = VSS_WRITER_STATE(16i32);
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(15i32);
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(12i32);
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(9i32);
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = VSS_WRITER_STATE(6i32);
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(14i32);
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(11i32);
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = VSS_WRITER_STATE(7i32);
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(8i32);
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(13i32);
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(10i32);
pub const VSS_WS_STABLE: VSS_WRITER_STATE = VSS_WRITER_STATE(1i32);
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(0i32);
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(5i32);
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(2i32);
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(4i32);
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(3i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
impl windows_core::TypeKind for VSS_ALTERNATE_WRITER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_ALTERNATE_WRITER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_ALTERNATE_WRITER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_APPLICATION_LEVEL(pub i32);
impl windows_core::TypeKind for VSS_APPLICATION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_APPLICATION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_APPLICATION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
impl windows_core::TypeKind for VSS_BACKUP_SCHEMA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_BACKUP_SCHEMA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_SCHEMA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_BACKUP_TYPE(pub i32);
impl windows_core::TypeKind for VSS_BACKUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_BACKUP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_COMPONENT_FLAGS(pub i32);
impl windows_core::TypeKind for VSS_COMPONENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_COMPONENT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_COMPONENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_COMPONENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
impl windows_core::TypeKind for VSS_FILE_RESTORE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_FILE_RESTORE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_FILE_RESTORE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
impl windows_core::TypeKind for VSS_FILE_SPEC_BACKUP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_FILE_SPEC_BACKUP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_FILE_SPEC_BACKUP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_HARDWARE_OPTIONS(pub i32);
impl windows_core::TypeKind for VSS_HARDWARE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_HARDWARE_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_MGMT_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_MGMT_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_MGMT_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for VSS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_PROTECTION_FAULT(pub i32);
impl windows_core::TypeKind for VSS_PROTECTION_FAULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_PROTECTION_FAULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_FAULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_PROTECTION_LEVEL(pub i32);
impl windows_core::TypeKind for VSS_PROTECTION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
impl windows_core::TypeKind for VSS_PROVIDER_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_PROVIDER_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_PROVIDER_TYPE(pub i32);
impl windows_core::TypeKind for VSS_PROVIDER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_PROVIDER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_RECOVERY_OPTIONS(pub i32);
impl windows_core::TypeKind for VSS_RECOVERY_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_RECOVERY_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_RECOVERY_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
impl windows_core::TypeKind for VSS_RESTOREMETHOD_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_RESTOREMETHOD_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_RESTOREMETHOD_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_RESTORE_TARGET(pub i32);
impl windows_core::TypeKind for VSS_RESTORE_TARGET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_RESTORE_TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TARGET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_RESTORE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_RESTORE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_RESTORE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
impl windows_core::TypeKind for VSS_ROLLFORWARD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_ROLLFORWARD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_ROLLFORWARD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_COMPATIBILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SNAPSHOT_COMPATIBILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_COMPATIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SNAPSHOT_CONTEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_CONTEXT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_PROPERTY_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SNAPSHOT_PROPERTY_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SNAPSHOT_STATE(pub i32);
impl windows_core::TypeKind for VSS_SNAPSHOT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SNAPSHOT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SOURCE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_SOURCE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SOURCE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_SUBSCRIBE_MASK(pub i32);
impl windows_core::TypeKind for VSS_SUBSCRIBE_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_SUBSCRIBE_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_SUBSCRIBE_MASK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_USAGE_TYPE(pub i32);
impl windows_core::TypeKind for VSS_USAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_USAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_USAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_VOLUME_SNAPSHOT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
impl windows_core::TypeKind for VSS_WRITERRESTORE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_WRITERRESTORE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_WRITERRESTORE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VSS_WRITER_STATE(pub i32);
impl windows_core::TypeKind for VSS_WRITER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VSS_WRITER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VSS_WRITER_STATE").field(&self.0).finish()
    }
}
pub const VSSCoordinator: windows_core::GUID = windows_core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl windows_core::TypeKind for VSS_DIFF_AREA_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl windows_core::TypeKind for VSS_DIFF_VOLUME_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl windows_core::TypeKind for VSS_MGMT_OBJECT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl windows_core::TypeKind for VSS_MGMT_OBJECT_UNION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl windows_core::TypeKind for VSS_OBJECT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl windows_core::TypeKind for VSS_OBJECT_UNION {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: windows_core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: windows_core::GUID,
    pub m_ClassId: windows_core::GUID,
}
impl windows_core::TypeKind for VSS_PROVIDER_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: windows_core::GUID,
    pub m_SnapshotSetId: windows_core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: windows_core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl windows_core::TypeKind for VSS_SNAPSHOT_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl windows_core::TypeKind for VSS_VOLUME_PROP {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
impl windows_core::TypeKind for VSS_VOLUME_PROTECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VssSnapshotMgmt: windows_core::GUID = windows_core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
#[cfg(feature = "implement")]
core::include!("impl.rs");
