#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> windows_core::Result<IVssExpressWriter> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssExpressWriterInternal(ppwriter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssExpressWriterInternal(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IVssAdmin, IVssAdmin_Vtbl, 0x77ed5996_2f63_11d3_8a39_00c04f72d8e3);
windows_core::imp::interface_hierarchy!(IVssAdmin, windows_core::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider(&self, pproviderid: windows_core::GUID, classid: windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(pproviderid), core::mem::transmute(classid), pwszprovidername, eprovidertype, pwszproviderversion, core::mem::transmute(providerversionid)).ok() }
    }
    pub unsafe fn UnregisterProvider(&self, providerid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterProvider)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid)).ok() }
    }
    pub unsafe fn QueryProviders(&self) -> windows_core::Result<IVssEnumObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortAllSnapshotsInProgress)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdmin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, VSS_PROVIDER_TYPE, *const u16, windows_core::GUID) -> windows_core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AbortAllSnapshotsInProgress: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssAdmin_Impl: windows_core::IUnknownImpl {
    fn RegisterProvider(&self, pproviderid: &windows_core::GUID, classid: &windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, providerid: &windows_core::GUID) -> windows_core::Result<()>;
    fn QueryProviders(&self) -> windows_core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(&self) -> windows_core::Result<()>;
}
impl IVssAdmin_Vtbl {
    pub const fn new<Identity: IVssAdmin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterProvider<Identity: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, classid: windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAdmin_Impl::RegisterProvider(this, core::mem::transmute(&pproviderid), core::mem::transmute(&classid), core::mem::transmute_copy(&pwszprovidername), core::mem::transmute_copy(&eprovidertype), core::mem::transmute_copy(&pwszproviderversion), core::mem::transmute(&providerversionid)).into()
            }
        }
        unsafe extern "system" fn UnregisterProvider<Identity: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAdmin_Impl::UnregisterProvider(this, core::mem::transmute(&providerid)).into()
            }
        }
        unsafe extern "system" fn QueryProviders<Identity: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssAdmin_Impl::QueryProviders(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Identity: IVssAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAdmin_Impl::AbortAllSnapshotsInProgress(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, OFFSET>,
            QueryProviders: QueryProviders::<Identity, OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAdmin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssAdmin {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderCapability)(windows_core::Interface::as_raw(self), core::mem::transmute(pproviderid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderContext(&self, providerid: windows_core::GUID) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderContext)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProviderContext(&self, providerid: windows_core::GUID, lcontext: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProviderContext)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), lcontext).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdminEx_Vtbl {
    pub base__: IVssAdmin_Vtbl,
    pub GetProviderCapability: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u64) -> windows_core::HRESULT,
    pub GetProviderContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut i32) -> windows_core::HRESULT,
    pub SetProviderContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32) -> windows_core::HRESULT,
}
pub trait IVssAdminEx_Impl: IVssAdmin_Impl {
    fn GetProviderCapability(&self, pproviderid: &windows_core::GUID) -> windows_core::Result<u64>;
    fn GetProviderContext(&self, providerid: &windows_core::GUID) -> windows_core::Result<i32>;
    fn SetProviderContext(&self, providerid: &windows_core::GUID, lcontext: i32) -> windows_core::Result<()>;
}
impl IVssAdminEx_Vtbl {
    pub const fn new<Identity: IVssAdminEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProviderCapability<Identity: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderid: windows_core::GUID, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssAdminEx_Impl::GetProviderCapability(this, core::mem::transmute(&pproviderid)) {
                    Ok(ok__) => {
                        plloriginalcapabilitymask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderContext<Identity: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, plcontext: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssAdminEx_Impl::GetProviderContext(this, core::mem::transmute(&providerid)) {
                    Ok(ok__) => {
                        plcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProviderContext<Identity: IVssAdminEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAdminEx_Impl::SetProviderContext(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)).into()
            }
        }
        Self {
            base__: IVssAdmin_Vtbl::new::<Identity, OFFSET>(),
            GetProviderCapability: GetProviderCapability::<Identity, OFFSET>,
            GetProviderContext: GetProviderContext::<Identity, OFFSET>,
            SetProviderContext: SetProviderContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAdminEx as windows_core::Interface>::IID || iid == &<IVssAdmin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssAdminEx {}
windows_core::imp::define_interface!(IVssAsync, IVssAsync_Vtbl, 0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
windows_core::imp::interface_hierarchy!(IVssAsync, windows_core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds).ok() }
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), phrresult as _, preserved as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut i32) -> windows_core::HRESULT,
}
pub trait IVssAsync_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::Result<()>;
}
impl IVssAsync_Vtbl {
    pub const fn new<Identity: IVssAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Wait<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn QueryStatus<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::QueryStatus(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&preserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            QueryStatus: QueryStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssAsync {}
windows_core::imp::define_interface!(IVssComponent, IVssComponent_Vtbl, 0xd2c72c96_c121_4518_b627_e5a93d010ead);
windows_core::imp::interface_hierarchy!(IVssComponent, windows_core::IUnknown);
impl IVssComponent {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrpath)).ok() }
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentType)(windows_core::Interface::as_raw(self), pct as _).ok() }
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrname)).ok() }
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackupSucceeded)(windows_core::Interface::as_raw(self), pbsucceeded as _).ok() }
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAlternateLocationMappingCount)(windows_core::Interface::as_raw(self), pcmappings as _).ok() }
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupMetadata)(windows_core::Interface::as_raw(self), wszdata.param().abi()).ok() }
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackupMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdata)).ok() }
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPartialFile)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilename.param().abi(), wszranges.param().abi(), wszmetadata.param().abi()).ok() }
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPartialFileCount)(windows_core::Interface::as_raw(self), pcpartialfiles as _).ok() }
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPartialFile)(windows_core::Interface::as_raw(self), ipartialfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilename), core::mem::transmute(pbstrrange), core::mem::transmute(pbstrmetadata)).ok() }
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsSelectedForRestore)(windows_core::Interface::as_raw(self), pbselectedforrestore as _).ok() }
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdditionalRestores)(windows_core::Interface::as_raw(self), pbadditionalrestores as _).ok() }
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNewTargetCount)(windows_core::Interface::as_raw(self), pcnewtarget as _).ok() }
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewTarget)(windows_core::Interface::as_raw(self), inewtarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
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
        unsafe { (windows_core::Interface::vtable(self).AddDirectedTarget)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilename.param().abi(), wszsourcerangelist.param().abi(), wszdestinationpath.param().abi(), wszdestinationfilename.param().abi(), wszdestinationrangelist.param().abi()).ok() }
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDirectedTargetCount)(windows_core::Interface::as_raw(self), pcdirectedtarget as _).ok() }
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDirectedTarget)(windows_core::Interface::as_raw(self), idirectedtarget, core::mem::transmute(pbstrsourcepath), core::mem::transmute(pbstrsourcefilename), core::mem::transmute(pbstrsourcerangelist), core::mem::transmute(pbstrdestinationpath), core::mem::transmute(pbstrdestinationfilename), core::mem::transmute(pbstrdestinationrangelist)).ok() }
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMetadata)(windows_core::Interface::as_raw(self), wszrestoremetadata.param().abi()).ok() }
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrrestoremetadata)).ok() }
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreTarget)(windows_core::Interface::as_raw(self), target).ok() }
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreTarget)(windows_core::Interface::as_raw(self), ptarget as _).ok() }
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszprerestorefailuremsg.param().abi()).ok() }
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrprerestorefailuremsg)).ok() }
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.param().abi()).ok() }
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrpostrestorefailuremsg)).ok() }
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupStamp)(windows_core::Interface::as_raw(self), wszbackupstamp.param().abi()).ok() }
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackupStamp)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupstamp)).ok() }
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPreviousBackupStamp)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupstamp)).ok() }
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBackupOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrbackupoptions)).ok() }
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrrestoreoptions)).ok() }
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreSubcomponentCount)(windows_core::Interface::as_raw(self), pcrestoresubcomponent as _).ok() }
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreSubcomponent)(windows_core::Interface::as_raw(self), icomponent, core::mem::transmute(pbstrlogicalpath), core::mem::transmute(pbstrcomponentname), pbrepair as _).ok() }
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFileRestoreStatus)(windows_core::Interface::as_raw(self), pstatus as _).ok() }
    }
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool, ftlastmodifytime: super::super::Foundation::FILETIME) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyTime)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.into(), core::mem::transmute(ftlastmodifytime)).ok() }
    }
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool, bstrlsnstring: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyLSN)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.into(), core::mem::transmute_copy(bstrlsnstring)).ok() }
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDifferencedFilesCount)(windows_core::Interface::as_raw(self), pcdifferencedfiles as _).ok() }
    }
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDifferencedFile)(windows_core::Interface::as_raw(self), idifferencedfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilespec), pbrecursive as _, core::mem::transmute(pbstrlsnstring), pftlastmodifytime as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSelectedForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestoreOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRestoreSubcomponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetFileRestoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT,
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDifferencedFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
pub trait IVssComponent_Impl: windows_core::IUnknownImpl {
    fn GetLogicalPath(&self, pbstrpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::Result<()>;
    fn GetComponentName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> windows_core::Result<()>;
    fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> windows_core::Result<()>;
    fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn SetBackupMetadata(&self, wszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupMetadata(&self, pbstrdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddPartialFile(&self, wszpath: &windows_core::PCWSTR, wszfilename: &windows_core::PCWSTR, wszranges: &windows_core::PCWSTR, wszmetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> windows_core::Result<()>;
    fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> windows_core::Result<()>;
    fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> windows_core::Result<()>;
    fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> windows_core::Result<()>;
    fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn AddDirectedTarget(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilename: &windows_core::PCWSTR, wszsourcerangelist: &windows_core::PCWSTR, wszdestinationpath: &windows_core::PCWSTR, wszdestinationfilename: &windows_core::PCWSTR, wszdestinationrangelist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> windows_core::Result<()>;
    fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreMetadata(&self, wszrestoremetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::Result<()>;
    fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::Result<()>;
    fn SetPreRestoreFailureMsg(&self, wszprerestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPostRestoreFailureMsg(&self, wszpostrestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBackupStamp(&self, wszbackupstamp: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBackupOptions(&self, pbstrbackupoptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> windows_core::Result<()>;
    fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::Result<()>;
    fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyTime(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: windows_core::BOOL, ftlastmodifytime: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: windows_core::BOOL, bstrlsnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> windows_core::Result<()>;
    fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
}
impl IVssComponent_Vtbl {
    pub const fn new<Identity: IVssComponent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogicalPath<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrpath)).into()
            }
        }
        unsafe extern "system" fn GetComponentType<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetComponentType(this, core::mem::transmute_copy(&pct)).into()
            }
        }
        unsafe extern "system" fn GetComponentName<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrname)).into()
            }
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsucceeded: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetBackupSucceeded(this, core::mem::transmute_copy(&pbsucceeded)).into()
            }
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmappings: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetAlternateLocationMappingCount(this, core::mem::transmute_copy(&pcmappings)).into()
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetBackupMetadata(this, core::mem::transmute(&wszdata)).into()
            }
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetBackupMetadata(this, core::mem::transmute_copy(&pbstrdata)).into()
            }
        }
        unsafe extern "system" fn AddPartialFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, wszranges: windows_core::PCWSTR, wszmetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddPartialFile(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute(&wszranges), core::mem::transmute(&wszmetadata)).into()
            }
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpartialfiles: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPartialFileCount(this, core::mem::transmute_copy(&pcpartialfiles)).into()
            }
        }
        unsafe extern "system" fn GetPartialFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void, pbstrrange: *mut *mut core::ffi::c_void, pbstrmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPartialFile(this, core::mem::transmute_copy(&ipartialfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pbstrrange), core::mem::transmute_copy(&pbstrmetadata)).into()
            }
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbselectedforrestore: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::IsSelectedForRestore(this, core::mem::transmute_copy(&pbselectedforrestore)).into()
            }
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbadditionalrestores: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetAdditionalRestores(this, core::mem::transmute_copy(&pbadditionalrestores)).into()
            }
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnewtarget: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetNewTargetCount(this, core::mem::transmute_copy(&pcnewtarget)).into()
            }
        }
        unsafe extern "system" fn GetNewTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetNewTarget(this, core::mem::transmute_copy(&inewtarget)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilename: windows_core::PCWSTR, wszsourcerangelist: windows_core::PCWSTR, wszdestinationpath: windows_core::PCWSTR, wszdestinationfilename: windows_core::PCWSTR, wszdestinationrangelist: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDirectedTarget(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilename), core::mem::transmute(&wszsourcerangelist), core::mem::transmute(&wszdestinationpath), core::mem::transmute(&wszdestinationfilename), core::mem::transmute(&wszdestinationrangelist)).into()
            }
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdirectedtarget: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDirectedTargetCount(this, core::mem::transmute_copy(&pcdirectedtarget)).into()
            }
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut *mut core::ffi::c_void, pbstrsourcefilename: *mut *mut core::ffi::c_void, pbstrsourcerangelist: *mut *mut core::ffi::c_void, pbstrdestinationpath: *mut *mut core::ffi::c_void, pbstrdestinationfilename: *mut *mut core::ffi::c_void, pbstrdestinationrangelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDirectedTarget(this, core::mem::transmute_copy(&idirectedtarget), core::mem::transmute_copy(&pbstrsourcepath), core::mem::transmute_copy(&pbstrsourcefilename), core::mem::transmute_copy(&pbstrsourcerangelist), core::mem::transmute_copy(&pbstrdestinationpath), core::mem::transmute_copy(&pbstrdestinationfilename), core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszrestoremetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetRestoreMetadata(this, core::mem::transmute(&wszrestoremetadata)).into()
            }
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoremetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreMetadata(this, core::mem::transmute_copy(&pbstrrestoremetadata)).into()
            }
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetRestoreTarget(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreTarget(this, core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszprerestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetPreRestoreFailureMsg(this, core::mem::transmute(&wszprerestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprerestorefailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPreRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpostrestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetPostRestoreFailureMsg(this, core::mem::transmute(&wszpostrestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostrestorefailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPostRestoreFailureMsg(this, core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn SetBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetBackupStamp(this, core::mem::transmute(&wszbackupstamp)).into()
            }
        }
        unsafe extern "system" fn GetBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
            }
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPreviousBackupStamp(this, core::mem::transmute_copy(&pbstrbackupstamp)).into()
            }
        }
        unsafe extern "system" fn GetBackupOptions<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetBackupOptions(this, core::mem::transmute_copy(&pbstrbackupoptions)).into()
            }
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoreoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreOptions(this, core::mem::transmute_copy(&pbstrrestoreoptions)).into()
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreSubcomponentCount(this, core::mem::transmute_copy(&pcrestoresubcomponent)).into()
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut *mut core::ffi::c_void, pbstrcomponentname: *mut *mut core::ffi::c_void, pbrepair: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreSubcomponent(this, core::mem::transmute_copy(&icomponent), core::mem::transmute_copy(&pbstrlogicalpath), core::mem::transmute_copy(&pbstrcomponentname), core::mem::transmute_copy(&pbrepair)).into()
            }
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetFileRestoreStatus(this, core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: windows_core::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDifferencedFilesByLastModifyTime(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&ftlastmodifytime)).into()
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: windows_core::BOOL, bstrlsnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDifferencedFilesByLastModifyLSN(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&bstrlsnstring)).into()
            }
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdifferencedfiles: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDifferencedFilesCount(this, core::mem::transmute_copy(&pcdifferencedfiles)).into()
            }
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut *mut core::ffi::c_void, pbstrfilespec: *mut *mut core::ffi::c_void, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut *mut core::ffi::c_void, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDifferencedFile(this, core::mem::transmute_copy(&idifferencedfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilespec), core::mem::transmute_copy(&pbrecursive), core::mem::transmute_copy(&pbstrlsnstring), core::mem::transmute_copy(&pftlastmodifytime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentType: GetComponentType::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssComponent {}
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
        unsafe { (windows_core::Interface::vtable(self).SetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()).ok() }
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()).ok() }
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthoritativeRestore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRollForward)(windows_core::Interface::as_raw(self), prolltype as _, core::mem::transmute(pbstrpoint)).ok() }
    }
    pub unsafe fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_Vtbl {
    pub base__: IVssComponent_Vtbl,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetRollForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_ROLLFORWARD_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssComponentEx_Impl: IVssComponent_Impl {
    fn SetPrepareForBackupFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPostSnapshotFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool>;
    fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IVssComponentEx_Vtbl {
    pub const fn new<Identity: IVssComponentEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::SetPrepareForBackupFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
            }
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::SetPostSnapshotFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetPrepareForBackupFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrfailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetPostSnapshotFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrfailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbauth: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetAuthoritativeRestore(this) {
                    Ok(ok__) => {
                        pbauth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRollForward<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::GetRollForward(this, core::mem::transmute_copy(&prolltype), core::mem::transmute_copy(&pbstrpoint)).into()
            }
        }
        unsafe extern "system" fn GetRestoreName<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetRestoreName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVssComponent_Vtbl::new::<Identity, OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, OFFSET>,
            GetRollForward: GetRollForward::<Identity, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssComponentEx {}
windows_core::imp::define_interface!(IVssComponentEx2, IVssComponentEx2_Vtbl, 0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
impl core::ops::Deref for IVssComponentEx2 {
    type Target = IVssComponentEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponentEx2, windows_core::IUnknown, IVssComponent, IVssComponentEx);
impl IVssComponentEx2 {
    pub unsafe fn SetFailure<P2>(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: P2, dwreserved: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFailure)(windows_core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.param().abi(), dwreserved).ok() }
    }
    pub unsafe fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFailure)(windows_core::Interface::as_raw(self), phr as _, phrapplication as _, core::mem::transmute(pbstrapplicationmessage), pdwreserved as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_Vtbl {
    pub base__: IVssComponentEx_Vtbl,
    pub SetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::HRESULT, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IVssComponentEx2_Impl: IVssComponentEx_Impl {
    fn SetFailure(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
impl IVssComponentEx2_Vtbl {
    pub const fn new<Identity: IVssComponentEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFailure<Identity: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx2_Impl::SetFailure(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&hrapplication), core::mem::transmute(&wszapplicationmessage), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetFailure<Identity: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut *mut core::ffi::c_void, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx2_Impl::GetFailure(this, core::mem::transmute_copy(&phr), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IVssComponentEx_Vtbl::new::<Identity, OFFSET>(), SetFailure: SetFailure::<Identity, OFFSET>, GetFailure: GetFailure::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx2 as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID || iid == &<IVssComponentEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssComponentEx2 {}
windows_core::imp::define_interface!(IVssCreateExpressWriterMetadata, IVssCreateExpressWriterMetadata_Vtbl, 0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
windows_core::imp::interface_hierarchy!(IVssCreateExpressWriterMetadata, windows_core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive).ok() }
    }
    pub unsafe fn AddComponent<P1, P2, P3>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P1, wszcomponentname: P2, wszcaption: P3, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok() }
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P5>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P5, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask).ok() }
    }
    pub unsafe fn SetRestoreMethod<P1, P2>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P1, wszuserprocedure: P2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired).ok() }
    }
    pub unsafe fn AddComponentDependency<P0, P1, P3, P4>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: windows_core::GUID, wszonlogicalpath: P3, wszoncomponentname: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()).ok() }
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask).ok() }
    }
    pub unsafe fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateExpressWriterMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddExcludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u8) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, u8, u8, u8, u8, u32) -> windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u8, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTOREMETHOD_ENUM, windows_core::PCWSTR, windows_core::PCWSTR, VSS_WRITERRESTORE_ENUM, u8) -> windows_core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssCreateExpressWriterMetadata_Impl: windows_core::IUnknownImpl {
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &windows_core::GUID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IVssCreateExpressWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddExcludeFiles<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
            }
        }
        unsafe extern "system" fn AddComponent<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
            }
        }
        unsafe extern "system" fn AddComponentDependency<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: windows_core::GUID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
            }
        }
        unsafe extern "system" fn SetBackupSchema<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssCreateExpressWriterMetadata_Impl::SaveAsXML(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Identity, OFFSET>,
            AddComponent: AddComponent::<Identity, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssCreateExpressWriterMetadata {}
windows_core::imp::define_interface!(IVssCreateWriterMetadata, IVssCreateWriterMetadata_Vtbl);
impl IVssCreateWriterMetadata {
    pub unsafe fn AddIncludeFiles<P0, P1, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8, wszalternatelocation: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddIncludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi()).ok() }
    }
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive).ok() }
    }
    pub unsafe fn AddComponent<P1, P2, P3>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P1, wszcomponentname: P2, wszcaption: P3, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok() }
    }
    pub unsafe fn AddDatabaseFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDatabaseFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask).ok() }
    }
    pub unsafe fn AddDatabaseLogFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDatabaseLogFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask).ok() }
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P5>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P5, dwbackuptypemask: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask).ok() }
    }
    pub unsafe fn SetRestoreMethod<P1, P2>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P1, wszuserprocedure: P2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired).ok() }
    }
    pub unsafe fn AddAlternateLocationMapping<P0, P1, P3>(&self, wszsourcepath: P0, wszsourcefilespec: P1, brecursive: u8, wszdestination: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAlternateLocationMapping)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilespec.param().abi(), brecursive, wszdestination.param().abi()).ok() }
    }
    pub unsafe fn AddComponentDependency<P0, P1, P3, P4>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: windows_core::GUID, wszonlogicalpath: P3, wszoncomponentname: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()).ok() }
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask).ok() }
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocument(&self) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrxml)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IVssCreateWriterMetadata_Impl {
    fn AddIncludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddDatabaseFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddDatabaseLogFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::Result<()>;
    fn AddAlternateLocationMapping(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilespec: &windows_core::PCWSTR, brecursive: u8, wszdestination: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &windows_core::GUID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn GetDocument(&self) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument>;
    fn SaveAsXML(&self, pbstrxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssCreateWriterMetadata_Impl>() -> Self {
        unsafe extern "system" fn AddIncludeFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddIncludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation)).into()
            }
        }
        unsafe extern "system" fn AddExcludeFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
            }
        }
        unsafe extern "system" fn AddComponent<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
            }
        }
        unsafe extern "system" fn AddDatabaseFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddDatabaseFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddDatabaseLogFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: u8, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
            }
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilespec: windows_core::PCWSTR, brecursive: u8, wszdestination: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddAlternateLocationMapping(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszdestination)).into()
            }
        }
        unsafe extern "system" fn AddComponentDependency<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: windows_core::GUID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
            }
        }
        unsafe extern "system" fn SetBackupSchema<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssCreateWriterMetadata_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        pdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::SaveAsXML(this, core::mem::transmute_copy(&pbstrxml)).into()
            }
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Identity>,
            AddExcludeFiles: AddExcludeFiles::<Identity>,
            AddComponent: AddComponent::<Identity>,
            AddDatabaseFiles: AddDatabaseFiles::<Identity>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Identity>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity>,
            SetRestoreMethod: SetRestoreMethod::<Identity>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Identity>,
            AddComponentDependency: AddComponentDependency::<Identity>,
            SetBackupSchema: SetBackupSchema::<Identity>,
            GetDocument: GetDocument::<Identity>,
            SaveAsXML: SaveAsXML::<Identity>,
        }
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
struct IVssCreateWriterMetadata_ImplVtbl<T: IVssCreateWriterMetadata_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<T: IVssCreateWriterMetadata_Impl> IVssCreateWriterMetadata_ImplVtbl<T> {
    const VTABLE: IVssCreateWriterMetadata_Vtbl = IVssCreateWriterMetadata_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata {
    pub fn new<'a, T: IVssCreateWriterMetadata_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssCreateWriterMetadata_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IVssDifferentialSoftwareSnapshotMgmt, IVssDifferentialSoftwareSnapshotMgmt_Vtbl, 0x214a0f28_b737_4026_b847_4f9e37d79529);
windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt, windows_core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDiffArea)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok() }
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSize)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok() }
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryVolumesSupportedForDiffAreas)(windows_core::Interface::as_raw(self), pwszoriginalvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDiffAreasForVolume)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDiffAreasOnVolume)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: windows_core::GUID) -> windows_core::Result<IVssEnumMgmtObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryDiffAreasForSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddDiffArea: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64) -> windows_core::HRESULT,
    pub ChangeDiffAreaMaximumSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64) -> windows_core::HRESULT,
    pub QueryVolumesSupportedForDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasForVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasOnVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryDiffAreasForSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssDifferentialSoftwareSnapshotMgmt_Impl: windows_core::IUnknownImpl {
    fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()>;
    fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::Result<()>;
    fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForSnapshot(&self, snapshotid: &windows_core::GUID) -> windows_core::Result<IVssEnumMgmtObject>;
}
impl IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub const fn new<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddDiffArea<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt_Impl::AddDiffArea(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
            }
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt_Impl::ChangeDiffAreaMaximumSize(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace)).into()
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryVolumesSupportedForDiffAreas(this, core::mem::transmute_copy(&pwszoriginalvolumename)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasOnVolume(this, core::mem::transmute_copy(&pwszvolumename)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Identity: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssDifferentialSoftwareSnapshotMgmt_Impl::QueryDiffAreasForSnapshot(this, core::mem::transmute(&snapshotid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddDiffArea: AddDiffArea::<Identity, OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Identity, OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Identity, OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Identity, OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Identity, OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt {}
windows_core::imp::define_interface!(IVssDifferentialSoftwareSnapshotMgmt2, IVssDifferentialSoftwareSnapshotMgmt2_Vtbl, 0x949d7353_675f_4275_8969_f044c6277815);
impl core::ops::Deref for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Target = IVssDifferentialSoftwareSnapshotMgmt;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt2, windows_core::IUnknown, IVssDifferentialSoftwareSnapshotMgmt);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn ChangeDiffAreaMaximumSizeEx(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSizeEx)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, bvolatile.into()).ok() }
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MigrateDiffAreas)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename).ok() }
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryMigrationStatus)(windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: windows_core::GUID, priority: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSnapshotPriority)(windows_core::Interface::as_raw(self), core::mem::transmute(idsnapshot), priority).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl,
    pub ChangeDiffAreaMaximumSizeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, i64, windows_core::BOOL) -> windows_core::HRESULT,
    pub MigrateDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
    pub QueryMigrationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSnapshotPriority: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u8) -> windows_core::HRESULT,
}
pub trait IVssDifferentialSoftwareSnapshotMgmt2_Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl {
    fn ChangeDiffAreaMaximumSizeEx(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: windows_core::BOOL) -> windows_core::Result<()>;
    fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::Result<()>;
    fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> windows_core::Result<IVssAsync>;
    fn SetSnapshotPriority(&self, idsnapshot: &windows_core::GUID, priority: u8) -> windows_core::Result<()>;
}
impl IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt2_Impl::ChangeDiffAreaMaximumSizeEx(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&llmaximumdiffspace), core::mem::transmute_copy(&bvolatile)).into()
            }
        }
        unsafe extern "system" fn MigrateDiffAreas<Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt2_Impl::MigrateDiffAreas(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename), core::mem::transmute_copy(&pwsznewdiffareavolumename)).into()
            }
        }
        unsafe extern "system" fn QueryMigrationStatus<Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssDifferentialSoftwareSnapshotMgmt2_Impl::QueryMigrationStatus(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pwszdiffareavolumename)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSnapshotPriority<Identity: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshot: windows_core::GUID, priority: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt2_Impl::SetSnapshotPriority(this, core::mem::transmute(&idsnapshot), core::mem::transmute_copy(&priority)).into()
            }
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl::new::<Identity, OFFSET>(),
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Identity, OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Identity, OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Identity, OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt2 {}
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
        unsafe { (windows_core::Interface::vtable(self).SetVolumeProtectLevel)(windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel).ok() }
    }
    pub unsafe fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeProtectLevel)(windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel as _).ok() }
    }
    pub unsafe fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearVolumeProtectFault)(windows_core::Interface::as_raw(self), pwszvolumename).ok() }
    }
    pub unsafe fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteUnusedDiffAreas)(windows_core::Interface::as_raw(self), pwszdiffareavolumename).ok() }
    }
    pub unsafe fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: windows_core::GUID, idsnapshotyounger: windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QuerySnapshotDeltaBitmap)(windows_core::Interface::as_raw(self), core::mem::transmute(idsnapshotolder), core::mem::transmute(idsnapshotyounger), pcblocksizeperbit as _, pcbitmaplength as _, ppbbitmap as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl,
    pub SetVolumeProtectLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, VSS_PROTECTION_LEVEL) -> windows_core::HRESULT,
    pub GetVolumeProtectLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::HRESULT,
    pub ClearVolumeProtectFault: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub DeleteUnusedDiffAreas: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub QuerySnapshotDeltaBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *mut u32, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IVssDifferentialSoftwareSnapshotMgmt3_Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl {
    fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::Result<()>;
    fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::Result<()>;
    fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> windows_core::Result<()>;
    fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> windows_core::Result<()>;
    fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: &windows_core::GUID, idsnapshotyounger: &windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::Result<()>;
}
impl IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub const fn new<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVolumeProtectLevel<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt3_Impl::SetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
            }
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt3_Impl::GetVolumeProtectLevel(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&protectionlevel)).into()
            }
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt3_Impl::ClearVolumeProtectFault(this, core::mem::transmute_copy(&pwszvolumename)).into()
            }
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdiffareavolumename: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt3_Impl::DeleteUnusedDiffAreas(this, core::mem::transmute_copy(&pwszdiffareavolumename)).into()
            }
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Identity: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsnapshotolder: windows_core::GUID, idsnapshotyounger: windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssDifferentialSoftwareSnapshotMgmt3_Impl::QuerySnapshotDeltaBitmap(this, core::mem::transmute(&idsnapshotolder), core::mem::transmute(&idsnapshotyounger), core::mem::transmute_copy(&pcblocksizeperbit), core::mem::transmute_copy(&pcbitmaplength), core::mem::transmute_copy(&ppbbitmap)).into()
            }
        }
        Self {
            base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl::new::<Identity, OFFSET>(),
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Identity, OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Identity, OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Identity, OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Identity, OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssDifferentialSoftwareSnapshotMgmt3 as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt as windows_core::Interface>::IID || iid == &<IVssDifferentialSoftwareSnapshotMgmt2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssDifferentialSoftwareSnapshotMgmt3 {}
windows_core::imp::define_interface!(IVssEnumMgmtObject, IVssEnumMgmtObject_Vtbl, 0x01954e6b_9254_4e6e_808c_c9e05d007696);
windows_core::imp::interface_hierarchy!(IVssEnumMgmtObject, windows_core::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_MGMT_OBJECT_PROP], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<IVssEnumMgmtObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumMgmtObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VSS_MGMT_OBJECT_PROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssEnumMgmtObject_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IVssEnumMgmtObject>) -> windows_core::Result<()>;
}
impl IVssEnumMgmtObject_Vtbl {
    pub const fn new<Identity: IVssEnumMgmtObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumMgmtObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumMgmtObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumMgmtObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IVssEnumMgmtObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumMgmtObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
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
        iid == &<IVssEnumMgmtObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssEnumMgmtObject {}
windows_core::imp::define_interface!(IVssEnumObject, IVssEnumObject_Vtbl, 0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
windows_core::imp::interface_hierarchy!(IVssEnumObject, windows_core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_OBJECT_PROP], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<IVssEnumObject>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VSS_OBJECT_PROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssEnumObject_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IVssEnumObject>) -> windows_core::Result<()>;
}
impl IVssEnumObject_Vtbl {
    pub const fn new<Identity: IVssEnumObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
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
        iid == &<IVssEnumObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssEnumObject {}
windows_core::imp::define_interface!(IVssExpressWriter, IVssExpressWriter_Vtbl, 0xe33affdc_59c7_47b1_97d5_4266598f6235);
windows_core::imp::interface_hierarchy!(IVssExpressWriter, windows_core::IUnknown);
impl IVssExpressWriter {
    pub unsafe fn CreateMetadata<P1>(&self, writerid: windows_core::GUID, writername: P1, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), writername.param().abi(), usagetype, versionmajor, versionminor, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LoadMetadata<P0>(&self, metadata: P0, reserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadMetadata)(windows_core::Interface::as_raw(self), metadata.param().abi(), reserved).ok() }
    }
    pub unsafe fn Register(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unregister(&self, writerid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExpressWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::PCWSTR, VSS_USAGE_TYPE, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IVssExpressWriter_Impl: windows_core::IUnknownImpl {
    fn CreateMetadata(&self, writerid: &windows_core::GUID, writername: &windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(&self, metadata: &windows_core::PCWSTR, reserved: u32) -> windows_core::Result<()>;
    fn Register(&self) -> windows_core::Result<()>;
    fn Unregister(&self, writerid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IVssExpressWriter_Vtbl {
    pub const fn new<Identity: IVssExpressWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMetadata<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID, writername: windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExpressWriter_Impl::CreateMetadata(this, core::mem::transmute(&writerid), core::mem::transmute(&writername), core::mem::transmute_copy(&usagetype), core::mem::transmute_copy(&versionmajor), core::mem::transmute_copy(&versionminor), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        ppmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadMetadata<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: windows_core::PCWSTR, reserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::LoadMetadata(this, core::mem::transmute(&metadata), core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn Register<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::Register(this).into()
            }
        }
        unsafe extern "system" fn Unregister<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::Unregister(this, core::mem::transmute(&writerid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadata: CreateMetadata::<Identity, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExpressWriter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssExpressWriter {}
windows_core::imp::define_interface!(IVssFileShareSnapshotProvider, IVssFileShareSnapshotProvider_Vtbl, 0xc8636060_7c2e_11df_8c4a_0800200c9a66);
windows_core::imp::interface_hierarchy!(IVssFileShareSnapshotProvider, windows_core::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext).ok() }
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), pprop as _).ok() }
    }
    pub unsafe fn Query(&self, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteSnapshots(&self, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: bool, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into(), pldeletedsnapshots as _, pnondeletedsnapshotid as _).ok() }
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), pwszsharepath, lnewcontext, core::mem::transmute(providerid)).ok() }
    }
    pub unsafe fn IsPathSupported(&self, pwszsharepath: *const u16) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPathSupported)(windows_core::Interface::as_raw(self), pwszsharepath, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPathSnapshotted)(windows_core::Interface::as_raw(self), pwszsharepath, pbsnapshotspresent as _, plsnapshotcompatibility as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSnapshotProperty(&self, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSnapshotProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), esnapshotpropertyid, core::mem::transmute_copy(vproperty)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssFileShareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, VSS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, windows_core::BOOL, *mut i32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, i32, windows_core::GUID) -> windows_core::HRESULT,
    pub IsPathSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPathSnapshotted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_SNAPSHOT_PROPERTY_ID, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSnapshotProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVssFileShareSnapshotProvider_Impl: windows_core::IUnknownImpl {
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&self, sourceobjectid: &windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: &windows_core::GUID) -> windows_core::Result<()>;
    fn IsPathSupported(&self, pwszsharepath: *const u16) -> windows_core::Result<windows_core::BOOL>;
    fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()>;
    fn SetSnapshotProperty(&self, snapshotid: &windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVssFileShareSnapshotProvider_Vtbl {
    pub const fn new<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContext<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssFileShareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&lnewcontext), core::mem::transmute(&providerid)).into()
            }
        }
        unsafe extern "system" fn IsPathSupported<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssFileShareSnapshotProvider_Impl::IsPathSupported(this, core::mem::transmute_copy(&pwszsharepath)) {
                    Ok(ok__) => {
                        pbsupportedbythisprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPathSnapshotted<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::IsPathSnapshotted(this, core::mem::transmute_copy(&pwszsharepath), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
            }
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: IVssFileShareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssFileShareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            IsPathSupported: IsPathSupported::<Identity, OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Identity, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssFileShareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IVssFileShareSnapshotProvider {}
windows_core::imp::define_interface!(IVssHardwareSnapshotProvider, IVssHardwareSnapshotProvider_Vtbl, 0x9593a157_44e9_4344_bbeb_44fbf9b06b10);
windows_core::imp::interface_hierarchy!(IVssHardwareSnapshotProvider, windows_core::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AreLunsSupported)(windows_core::Interface::as_raw(self), lluncount, lcontext, rgwszdevices, pluninformation as _, pbissupported as _).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FillInLunInfo)(windows_core::Interface::as_raw(self), wszdevicename, pluninfo as _, pbissupported as _).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), lcontext, lluncount, rgdevicenames, rgluninformation as _).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTargetLuns)(windows_core::Interface::as_raw(self), lluncount, rgdevicenames, rgsourceluns, rgdestinationluns as _).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LocateLuns)(windows_core::Interface::as_raw(self), rgsourceluns.len().try_into().unwrap(), core::mem::transmute(rgsourceluns.as_ptr())).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLunEmpty)(windows_core::Interface::as_raw(self), wszdevicename, pinformation).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub AreLunsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const *const u16, *mut super::VirtualDiskService::VDS_LUN_INFORMATION, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_VirtualDiskService"))]
    AreLunsSupported: usize,
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub FillInLunInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::VirtualDiskService::VDS_LUN_INFORMATION, *mut windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
pub trait IVssHardwareSnapshotProvider_Impl: windows_core::IUnknownImpl {
    fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn LocateLuns(&self, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl IVssHardwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AreLunsSupported<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::AreLunsSupported(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&rgwszdevices), core::mem::transmute_copy(&pluninformation), core::mem::transmute_copy(&pbissupported)).into()
            }
        }
        unsafe extern "system" fn FillInLunInfo<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::FillInLunInfo(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pluninfo), core::mem::transmute_copy(&pbissupported)).into()
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&lcontext), core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgluninformation)).into()
            }
        }
        unsafe extern "system" fn GetTargetLuns<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::GetTargetLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgdevicenames), core::mem::transmute_copy(&rgsourceluns), core::mem::transmute_copy(&rgdestinationluns)).into()
            }
        }
        unsafe extern "system" fn LocateLuns<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::LocateLuns(this, core::mem::transmute_copy(&lluncount), core::mem::transmute_copy(&rgsourceluns)).into()
            }
        }
        unsafe extern "system" fn OnLunEmpty<Identity: IVssHardwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProvider_Impl::OnLunEmpty(this, core::mem::transmute_copy(&wszdevicename), core::mem::transmute_copy(&pinformation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AreLunsSupported: AreLunsSupported::<Identity, OFFSET>,
            FillInLunInfo: FillInLunInfo::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            GetTargetLuns: GetTargetLuns::<Identity, OFFSET>,
            LocateLuns: LocateLuns::<Identity, OFFSET>,
            OnLunEmpty: OnLunEmpty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl windows_core::RuntimeName for IVssHardwareSnapshotProvider {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLunStateChange)(windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount, dwflags).ok() }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResyncLuns)(windows_core::Interface::as_raw(self), psourceluns, ptargetluns, dwcount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Storage_VirtualDiskService")]
    pub unsafe fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnReuseLuns)(windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
pub trait IVssHardwareSnapshotProviderEx_Impl: IVssHardwareSnapshotProvider_Impl {
    fn GetProviderCapabilities(&self) -> windows_core::Result<u64>;
    fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::Result<()>;
    fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<IVssAsync>;
    fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl IVssHardwareSnapshotProviderEx_Vtbl {
    pub const fn new<Identity: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProviderCapabilities<Identity: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssHardwareSnapshotProviderEx_Impl::GetProviderCapabilities(this) {
                    Ok(ok__) => {
                        plloriginalcapabilitymask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnLunStateChange<Identity: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProviderEx_Impl::OnLunStateChange(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ResyncLuns<Identity: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssHardwareSnapshotProviderEx_Impl::ResyncLuns(this, core::mem::transmute_copy(&psourceluns), core::mem::transmute_copy(&ptargetluns), core::mem::transmute_copy(&dwcount)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnReuseLuns<Identity: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssHardwareSnapshotProviderEx_Impl::OnReuseLuns(this, core::mem::transmute_copy(&psnapshotluns), core::mem::transmute_copy(&poriginalluns), core::mem::transmute_copy(&dwcount)).into()
            }
        }
        Self {
            base__: IVssHardwareSnapshotProvider_Vtbl::new::<Identity, OFFSET>(),
            GetProviderCapabilities: GetProviderCapabilities::<Identity, OFFSET>,
            OnLunStateChange: OnLunStateChange::<Identity, OFFSET>,
            ResyncLuns: ResyncLuns::<Identity, OFFSET>,
            OnReuseLuns: OnReuseLuns::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssHardwareSnapshotProviderEx as windows_core::Interface>::IID || iid == &<IVssHardwareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
impl windows_core::RuntimeName for IVssHardwareSnapshotProviderEx {}
windows_core::imp::define_interface!(IVssProviderCreateSnapshotSet, IVssProviderCreateSnapshotSet_Vtbl, 0x5f894e5b_1e39_4778_8e23_9abad9f0e08c);
windows_core::imp::interface_hierarchy!(IVssProviderCreateSnapshotSet, windows_core::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndPrepareSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
    pub unsafe fn PreCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
    pub unsafe fn CommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
    pub unsafe fn PostCommitSnapshots(&self, snapshotsetid: windows_core::GUID, lsnapshotscount: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), lsnapshotscount).ok() }
    }
    pub unsafe fn PreFinalCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreFinalCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
    pub unsafe fn PostFinalCommitSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostFinalCommitSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
    pub unsafe fn AbortSnapshots(&self, snapshotsetid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AbortSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IVssProviderCreateSnapshotSet_Impl: windows_core::IUnknownImpl {
    fn EndPrepareSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PreCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn CommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PostCommitSnapshots(&self, snapshotsetid: &windows_core::GUID, lsnapshotscount: i32) -> windows_core::Result<()>;
    fn PreFinalCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn PostFinalCommitSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn AbortSnapshots(&self, snapshotsetid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IVssProviderCreateSnapshotSet_Vtbl {
    pub const fn new<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EndPrepareSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::EndPrepareSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn PreCommitSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::PreCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn CommitSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::CommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn PostCommitSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, lsnapshotscount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::PostCommitSnapshots(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&lsnapshotscount)).into()
            }
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::PreFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::PostFinalCommitSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn AbortSnapshots<Identity: IVssProviderCreateSnapshotSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderCreateSnapshotSet_Impl::AbortSnapshots(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EndPrepareSnapshots: EndPrepareSnapshots::<Identity, OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Identity, OFFSET>,
            CommitSnapshots: CommitSnapshots::<Identity, OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Identity, OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Identity, OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Identity, OFFSET>,
            AbortSnapshots: AbortSnapshots::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssProviderCreateSnapshotSet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssProviderCreateSnapshotSet {}
windows_core::imp::define_interface!(IVssProviderNotifications, IVssProviderNotifications_Vtbl, 0xe561901f_03a5_4afe_86d0_72baeece7004);
windows_core::imp::interface_hierarchy!(IVssProviderNotifications, windows_core::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn OnUnload(&self, bforceunload: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self), bforceunload.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderNotifications_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IVssProviderNotifications_Impl: windows_core::IUnknownImpl {
    fn OnLoad(&self, pcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnUnload(&self, bforceunload: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IVssProviderNotifications_Vtbl {
    pub const fn new<Identity: IVssProviderNotifications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLoad<Identity: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderNotifications_Impl::OnLoad(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn OnUnload<Identity: IVssProviderNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforceunload: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssProviderNotifications_Impl::OnUnload(this, core::mem::transmute_copy(&bforceunload)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLoad: OnLoad::<Identity, OFFSET>, OnUnload: OnUnload::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssProviderNotifications as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssProviderNotifications {}
windows_core::imp::define_interface!(IVssSnapshotMgmt, IVssSnapshotMgmt_Vtbl, 0xfa7df749_66e7_4986_a27f_e2f04ae53772);
windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt, windows_core::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface(&self, providerid: windows_core::GUID, interfaceid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderMgmtInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), interfaceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots(&self, providerid: windows_core::GUID, lcontext: i32) -> windows_core::Result<IVssEnumMgmtObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryVolumesSupportedForSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), lcontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: windows_core::GUID) -> windows_core::Result<IVssEnumObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySnapshotsByVolume)(windows_core::Interface::as_raw(self), pwszvolumename, core::mem::transmute(providerid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProviderMgmtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryVolumesSupportedForSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySnapshotsByVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssSnapshotMgmt_Impl: windows_core::IUnknownImpl {
    fn GetProviderMgmtInterface(&self, providerid: &windows_core::GUID, interfaceid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryVolumesSupportedForSnapshots(&self, providerid: &windows_core::GUID, lcontext: i32) -> windows_core::Result<IVssEnumMgmtObject>;
    fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: &windows_core::GUID) -> windows_core::Result<IVssEnumObject>;
}
impl IVssSnapshotMgmt_Vtbl {
    pub const fn new<Identity: IVssSnapshotMgmt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProviderMgmtInterface<Identity: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, interfaceid: *const windows_core::GUID, ppitf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSnapshotMgmt_Impl::GetProviderMgmtInterface(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&interfaceid)) {
                    Ok(ok__) => {
                        ppitf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Identity: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, lcontext: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSnapshotMgmt_Impl::QueryVolumesSupportedForSnapshots(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&lcontext)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Identity: IVssSnapshotMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, providerid: windows_core::GUID, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSnapshotMgmt_Impl::QuerySnapshotsByVolume(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute(&providerid)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Identity, OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Identity, OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssSnapshotMgmt {}
windows_core::imp::define_interface!(IVssSnapshotMgmt2, IVssSnapshotMgmt2_Vtbl, 0x0f61ec39_fe82_45f2_a3f0_768b5d427102);
windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt2, windows_core::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinDiffAreaSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMinDiffAreaSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IVssSnapshotMgmt2_Impl: windows_core::IUnknownImpl {
    fn GetMinDiffAreaSize(&self) -> windows_core::Result<i64>;
}
impl IVssSnapshotMgmt2_Vtbl {
    pub const fn new<Identity: IVssSnapshotMgmt2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMinDiffAreaSize<Identity: IVssSnapshotMgmt2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllmindiffareasize: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSnapshotMgmt2_Impl::GetMinDiffAreaSize(this) {
                    Ok(ok__) => {
                        pllmindiffareasize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMinDiffAreaSize: GetMinDiffAreaSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSnapshotMgmt2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssSnapshotMgmt2 {}
windows_core::imp::define_interface!(IVssSoftwareSnapshotProvider, IVssSoftwareSnapshotProvider_Vtbl, 0x609e123e_2c5a_44d3_8f01_0b1d9a47d1ff);
windows_core::imp::interface_hierarchy!(IVssSoftwareSnapshotProvider, windows_core::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext).ok() }
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), pprop as _).ok() }
    }
    pub unsafe fn Query(&self, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteSnapshots(&self, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: bool, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into(), pldeletedsnapshots as _, pnondeletedsnapshotid as _).ok() }
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPrepareSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), core::mem::transmute(snapshotid), pwszvolumename, lnewcontext).ok() }
    }
    pub unsafe fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVolumeSupported)(windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsVolumeSnapshotted)(windows_core::Interface::as_raw(self), pwszvolumename, pbsnapshotspresent as _, plsnapshotcompatibility as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSnapshotProperty(&self, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSnapshotProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), esnapshotpropertyid, core::mem::transmute_copy(vproperty)).ok() }
    }
    pub unsafe fn RevertToSnapshot(&self, snapshotid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RevertToSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid)).ok() }
    }
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: *const u16) -> windows_core::Result<IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryRevertStatus)(windows_core::Interface::as_raw(self), pwszvolume, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSoftwareSnapshotProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, VSS_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_OBJECT_TYPE, windows_core::BOOL, *mut i32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::GUID, *const u16, i32) -> windows_core::HRESULT,
    pub IsVolumeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsVolumeSnapshotted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut windows_core::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, VSS_SNAPSHOT_PROPERTY_ID, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSnapshotProperty: usize,
    pub RevertToSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVssSoftwareSnapshotProvider_Impl: windows_core::IUnknownImpl {
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(&self, sourceobjectid: &windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn BeginPrepareSnapshot(&self, snapshotsetid: &windows_core::GUID, snapshotid: &windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::Result<()>;
    fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> windows_core::Result<windows_core::BOOL>;
    fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::Result<()>;
    fn SetSnapshotProperty(&self, snapshotid: &windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn RevertToSnapshot(&self, snapshotid: &windows_core::GUID) -> windows_core::Result<()>;
    fn QueryRevertStatus(&self, pwszvolume: *const u16) -> windows_core::Result<IVssAsync>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVssSoftwareSnapshotProvider_Vtbl {
    pub const fn new<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContext<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSoftwareSnapshotProvider_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
            }
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: windows_core::GUID, snapshotid: windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::BeginPrepareSnapshot(this, core::mem::transmute(&snapshotsetid), core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&lnewcontext)).into()
            }
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSoftwareSnapshotProvider_Impl::IsVolumeSupported(this, core::mem::transmute_copy(&pwszvolumename)) {
                    Ok(ok__) => {
                        pbsupportedbythisprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcompatibility: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::IsVolumeSnapshotted(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pbsnapshotspresent), core::mem::transmute_copy(&plsnapshotcompatibility)).into()
            }
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::SetSnapshotProperty(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&esnapshotpropertyid), core::mem::transmute(&vproperty)).into()
            }
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssSoftwareSnapshotProvider_Impl::RevertToSnapshot(this, core::mem::transmute(&snapshotid)).into()
            }
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: IVssSoftwareSnapshotProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssSoftwareSnapshotProvider_Impl::QueryRevertStatus(this, core::mem::transmute_copy(&pwszvolume)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Identity, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssSoftwareSnapshotProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IVssSoftwareSnapshotProvider {}
windows_core::imp::define_interface!(IVssWMDependency, IVssWMDependency_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMDependency, windows_core::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(&self, pwriterid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWriterId)(windows_core::Interface::as_raw(self), pwriterid as _).ok() }
    }
    pub unsafe fn GetLogicalPath(&self, pbstrlogicalpath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrlogicalpath)).ok() }
    }
    pub unsafe fn GetComponentName(&self, pbstrcomponentname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrcomponentname)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWriterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssWMDependency_Impl: windows_core::IUnknownImpl {
    fn GetWriterId(&self, pwriterid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetLogicalPath(&self, pbstrlogicalpath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetComponentName(&self, pbstrcomponentname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IVssWMDependency_Vtbl {
    pub const fn new<Identity: IVssWMDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterId<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwriterid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMDependency_Impl::GetWriterId(this, core::mem::transmute_copy(&pwriterid)).into()
            }
        }
        unsafe extern "system" fn GetLogicalPath<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlogicalpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMDependency_Impl::GetLogicalPath(this, core::mem::transmute_copy(&pbstrlogicalpath)).into()
            }
        }
        unsafe extern "system" fn GetComponentName<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomponentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMDependency_Impl::GetComponentName(this, core::mem::transmute_copy(&pbstrcomponentname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterId: GetWriterId::<Identity, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssWMDependency {}
windows_core::imp::define_interface!(IVssWMFiledesc, IVssWMFiledesc_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMFiledesc, windows_core::IUnknown);
impl IVssWMFiledesc {
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilespec)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRecursive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecursive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBackupTypeMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMFiledesc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilespec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecursive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IVssWMFiledesc_Impl: windows_core::IUnknownImpl {
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRecursive(&self) -> windows_core::Result<bool>;
    fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackupTypeMask(&self) -> windows_core::Result<u32>;
}
impl IVssWMFiledesc_Vtbl {
    pub const fn new<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPath<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilespec<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilespec: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetFilespec(this) {
                    Ok(ok__) => {
                        pbstrfilespec.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRecursive<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrecursive: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetRecursive(this) {
                    Ok(ok__) => {
                        pbrecursive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstralternatelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetAlternateLocation(this) {
                    Ok(ok__) => {
                        pbstralternatelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtypemask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetBackupTypeMask(this) {
                    Ok(ok__) => {
                        pdwtypemask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPath: GetPath::<Identity, OFFSET>,
            GetFilespec: GetFilespec::<Identity, OFFSET>,
            GetRecursive: GetRecursive::<Identity, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMFiledesc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssWMFiledesc {}
windows_core::imp::define_interface!(IVssWriterComponents, IVssWriterComponents_Vtbl);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self, pccomponents: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentCount)(windows_core::Interface::as_raw(self), pccomponents as _).ok() }
    }
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWriterInfo)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _).ok() }
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponents_Vtbl {
    pub GetComponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWriterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssWriterComponents_Impl {
    fn GetComponentCount(&self, pccomponents: *mut u32) -> windows_core::Result<()>;
    fn GetWriterInfo(&self, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent>;
}
impl IVssWriterComponents_Vtbl {
    pub const fn new<Identity: IVssWriterComponents_Impl>() -> Self {
        unsafe extern "system" fn GetComponentCount<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pccomponents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssWriterComponents_Impl::GetComponentCount(this, core::mem::transmute_copy(&pccomponents)).into()
            }
        }
        unsafe extern "system" fn GetWriterInfo<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pidinstance: *mut windows_core::GUID, pidwriter: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssWriterComponents_Impl::GetWriterInfo(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter)).into()
            }
        }
        unsafe extern "system" fn GetComponent<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssWriterComponents_Impl::GetComponent(this, core::mem::transmute_copy(&icomponent)) {
                    Ok(ok__) => {
                        ppcomponent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { GetComponentCount: GetComponentCount::<Identity>, GetWriterInfo: GetWriterInfo::<Identity>, GetComponent: GetComponent::<Identity> }
    }
}
struct IVssWriterComponents_ImplVtbl<T: IVssWriterComponents_Impl>(core::marker::PhantomData<T>);
impl<T: IVssWriterComponents_Impl> IVssWriterComponents_ImplVtbl<T> {
    const VTABLE: IVssWriterComponents_Vtbl = IVssWriterComponents_Vtbl::new::<T>();
}
impl IVssWriterComponents {
    pub fn new<'a, T: IVssWriterComponents_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssWriterComponents_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub const VSSCoordinator: windows_core::GUID = windows_core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_APPLICATION_LEVEL(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_BACKUP_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_COMPONENT_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_COMPONENT_TYPE(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_HARDWARE_OPTIONS(pub i32);
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_OBJECT_TYPE(pub i32);
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1024i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(512i32);
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(256i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROTECTION_FAULT(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROTECTION_LEVEL(pub i32);
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: windows_core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: windows_core::GUID,
    pub m_ClassId: windows_core::GUID,
}
impl Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROVIDER_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_RECOVERY_OPTIONS(pub i32);
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_RESTORE_TARGET(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_RESTORE_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SOURCE_TYPE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SUBSCRIBE_MASK(pub i32);
pub const VSS_S_ASYNC_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x4230B_u32 as _);
pub const VSS_S_ASYNC_FINISHED: windows_core::HRESULT = windows_core::HRESULT(0x4230A_u32 as _);
pub const VSS_S_ASYNC_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x42309_u32 as _);
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: windows_core::HRESULT = windows_core::HRESULT(0x42321_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_USAGE_TYPE(pub i32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: windows_core::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: windows_core::BOOL,
    pub m_reserved: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_WRITER_STATE(pub i32);
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
pub const VssSnapshotMgmt: windows_core::GUID = windows_core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
