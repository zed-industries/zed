#[inline]
pub unsafe fn OfflineFilesEnable(benable: bool, pbrebootrequired: *mut windows_core::BOOL) -> u32 {
    windows_core::link!("cscapi.dll" "system" fn OfflineFilesEnable(benable : windows_core::BOOL, pbrebootrequired : *mut windows_core::BOOL) -> u32);
    unsafe { OfflineFilesEnable(benable.into(), pbrebootrequired as _) }
}
#[inline]
pub unsafe fn OfflineFilesQueryStatus(pbactive: Option<*mut windows_core::BOOL>, pbenabled: Option<*mut windows_core::BOOL>) -> u32 {
    windows_core::link!("cscapi.dll" "system" fn OfflineFilesQueryStatus(pbactive : *mut windows_core::BOOL, pbenabled : *mut windows_core::BOOL) -> u32);
    unsafe { OfflineFilesQueryStatus(pbactive.unwrap_or(core::mem::zeroed()) as _, pbenabled.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OfflineFilesQueryStatusEx(pbactive: Option<*mut windows_core::BOOL>, pbenabled: Option<*mut windows_core::BOOL>, pbavailable: Option<*mut windows_core::BOOL>) -> u32 {
    windows_core::link!("cscapi.dll" "system" fn OfflineFilesQueryStatusEx(pbactive : *mut windows_core::BOOL, pbenabled : *mut windows_core::BOOL, pbavailable : *mut windows_core::BOOL) -> u32);
    unsafe { OfflineFilesQueryStatusEx(pbactive.unwrap_or(core::mem::zeroed()) as _, pbenabled.unwrap_or(core::mem::zeroed()) as _, pbavailable.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OfflineFilesStart() -> u32 {
    windows_core::link!("cscapi.dll" "system" fn OfflineFilesStart() -> u32);
    unsafe { OfflineFilesStart() }
}
windows_core::imp::define_interface!(IEnumOfflineFilesItems, IEnumOfflineFilesItems_Vtbl, 0xda70e815_c361_4407_bc0b_0d7046e5f2cd);
windows_core::imp::interface_hierarchy!(IEnumOfflineFilesItems, windows_core::IUnknown);
impl IEnumOfflineFilesItems {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOfflineFilesItem>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOfflineFilesItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesItems>;
}
impl IEnumOfflineFilesItems_Vtbl {
    pub const fn new<Identity: IEnumOfflineFilesItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOfflineFilesItems_Impl::Clone(this) {
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
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOfflineFilesItems as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOfflineFilesItems {}
windows_core::imp::define_interface!(IEnumOfflineFilesSettings, IEnumOfflineFilesSettings_Vtbl, 0x729680c4_1a38_47bc_9e5c_02c51562ac30);
windows_core::imp::interface_hierarchy!(IEnumOfflineFilesSettings, windows_core::IUnknown);
impl IEnumOfflineFilesSettings {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOfflineFilesSetting>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOfflineFilesSettings_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOfflineFilesSettings>;
}
impl IEnumOfflineFilesSettings_Vtbl {
    pub const fn new<Identity: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesSettings_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesSettings_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOfflineFilesSettings_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOfflineFilesSettings_Impl::Clone(this) {
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
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOfflineFilesSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOfflineFilesSettings {}
windows_core::imp::define_interface!(IOfflineFilesCache, IOfflineFilesCache_Vtbl, 0x855d6203_7914_48b9_8d40_4c56f5acffc5);
windows_core::imp::interface_hierarchy!(IOfflineFilesCache, windows_core::IUnknown);
impl IOfflineFilesCache {
    pub unsafe fn Synchronize<P5, P6>(&self, hwndparent: Option<super::super::Foundation::HWND>, rgpszpaths: &[windows_core::PCWSTR], basync: bool, dwsynccontrol: u32, pisyncconflicthandler: P5, piprogress: P6, psyncid: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P5: windows_core::Param<IOfflineFilesSyncConflictHandler>,
        P6: windows_core::Param<IOfflineFilesSyncProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Synchronize)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len().try_into().unwrap(), basync.into(), dwsynccontrol, pisyncconflicthandler.param().abi(), piprogress.param().abi(), psyncid.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DeleteItems<P4>(&self, rgpszpaths: &[windows_core::PCWSTR], dwflags: u32, basync: bool, piprogress: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IOfflineFilesSimpleProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteItems)(windows_core::Interface::as_raw(self), core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len().try_into().unwrap(), dwflags, basync.into(), piprogress.param().abi()).ok() }
    }
    pub unsafe fn DeleteItemsForUser<P0, P5>(&self, pszuser: P0, rgpszpaths: &[windows_core::PCWSTR], dwflags: u32, basync: bool, piprogress: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<IOfflineFilesSimpleProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteItemsForUser)(windows_core::Interface::as_raw(self), pszuser.param().abi(), core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len().try_into().unwrap(), dwflags, basync.into(), piprogress.param().abi()).ok() }
    }
    pub unsafe fn Pin<P6>(&self, hwndparent: Option<super::super::Foundation::HWND>, rgpszpaths: &[windows_core::PCWSTR], bdeep: bool, basync: bool, dwpincontrolflags: u32, piprogress: P6) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IOfflineFilesSyncProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Pin)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len().try_into().unwrap(), bdeep.into(), basync.into(), dwpincontrolflags, piprogress.param().abi()).ok() }
    }
    pub unsafe fn Unpin<P6>(&self, hwndparent: Option<super::super::Foundation::HWND>, rgpszpaths: &[windows_core::PCWSTR], bdeep: bool, basync: bool, dwpincontrolflags: u32, piprogress: P6) -> windows_core::Result<()>
    where
        P6: windows_core::Param<IOfflineFilesSyncProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unpin)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len().try_into().unwrap(), bdeep.into(), basync.into(), dwpincontrolflags, piprogress.param().abi()).ok() }
    }
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut windows_core::BOOL, pbpartial: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEncryptionStatus)(windows_core::Interface::as_raw(self), pbencrypted as _, pbpartial as _).ok() }
    }
    pub unsafe fn Encrypt<P4>(&self, hwndparent: Option<super::super::Foundation::HWND>, bencrypt: bool, dwencryptioncontrolflags: u32, basync: bool, piprogress: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IOfflineFilesSyncProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).Encrypt)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, bencrypt.into(), dwencryptioncontrolflags, basync.into(), piprogress.param().abi()).ok() }
    }
    pub unsafe fn FindItem<P0>(&self, pszpath: P0, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItem)(windows_core::Interface::as_raw(self), pszpath.param().abi(), dwqueryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindItemEx<P0, P1, P2, P3, P4>(&self, pszpath: P0, pincludefilefilter: P1, pincludedirfilter: P2, pexcludefilefilter: P3, pexcludedirfilter: P4, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IOfflineFilesItemFilter>,
        P2: windows_core::Param<IOfflineFilesItemFilter>,
        P3: windows_core::Param<IOfflineFilesItemFilter>,
        P4: windows_core::Param<IOfflineFilesItemFilter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemEx)(windows_core::Interface::as_raw(self), pszpath.param().abi(), pincludefilefilter.param().abi(), pincludedirfilter.param().abi(), pexcludefilefilter.param().abi(), pexcludedirfilter.param().abi(), dwqueryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RenameItem<P0, P1>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenameItem)(windows_core::Interface::as_raw(self), pszpathoriginal.param().abi(), pszpathnew.param().abi(), breplaceifexists.into()).ok() }
    }
    pub unsafe fn GetLocation(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDiskSpaceInformation)(windows_core::Interface::as_raw(self), pcbvolumetotal as _, pcblimit as _, pcbused as _, pcbunpinnedlimit as _, pcbunpinnedused as _).ok() }
    }
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDiskSpaceLimits)(windows_core::Interface::as_raw(self), cblimit, cbunpinnedlimit).ok() }
    }
    pub unsafe fn ProcessAdminPinPolicy<P0, P1>(&self, ppinprogress: P0, punpinprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOfflineFilesSyncProgress>,
        P1: windows_core::Param<IOfflineFilesSyncProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessAdminPinPolicy)(windows_core::Interface::as_raw(self), ppinprogress.param().abi(), punpinprogress.param().abi()).ok() }
    }
    pub unsafe fn GetSettingObject<P0>(&self, pszsettingname: P0) -> windows_core::Result<IOfflineFilesSetting>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettingObject)(windows_core::Interface::as_raw(self), pszsettingname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumSettingObjects(&self) -> windows_core::Result<IEnumOfflineFilesSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumSettingObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsPathCacheable<P0>(&self, pszpath: P0, pbcacheable: *mut windows_core::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsPathCacheable)(windows_core::Interface::as_raw(self), pszpath.param().abi(), pbcacheable as _, psharecachingmode as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Synchronize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::PCWSTR, u32, windows_core::BOOL, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteItems: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, u32, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteItemsForUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, u32, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::PCWSTR, u32, windows_core::BOOL, windows_core::BOOL, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unpin: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::PCWSTR, u32, windows_core::BOOL, windows_core::BOOL, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEncryptionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Encrypt: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::BOOL, u32, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItemEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RenameItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDiskSpaceInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64, *mut u64, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub SetDiskSpaceLimits: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    pub ProcessAdminPinPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSettingObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumSettingObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPathCacheable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL, *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT,
}
pub trait IOfflineFilesCache_Impl: windows_core::IUnknownImpl {
    fn Synchronize(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, basync: windows_core::BOOL, dwsynccontrol: u32, pisyncconflicthandler: windows_core::Ref<IOfflineFilesSyncConflictHandler>, piprogress: windows_core::Ref<IOfflineFilesSyncProgress>, psyncid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteItems(&self, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: windows_core::BOOL, piprogress: windows_core::Ref<IOfflineFilesSimpleProgress>) -> windows_core::Result<()>;
    fn DeleteItemsForUser(&self, pszuser: &windows_core::PCWSTR, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: windows_core::BOOL, piprogress: windows_core::Ref<IOfflineFilesSimpleProgress>) -> windows_core::Result<()>;
    fn Pin(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: windows_core::BOOL, basync: windows_core::BOOL, dwpincontrolflags: u32, piprogress: windows_core::Ref<IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn Unpin(&self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: windows_core::BOOL, basync: windows_core::BOOL, dwpincontrolflags: u32, piprogress: windows_core::Ref<IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn GetEncryptionStatus(&self, pbencrypted: *mut windows_core::BOOL, pbpartial: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Encrypt(&self, hwndparent: super::super::Foundation::HWND, bencrypt: windows_core::BOOL, dwencryptioncontrolflags: u32, basync: windows_core::BOOL, piprogress: windows_core::Ref<IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn FindItem(&self, pszpath: &windows_core::PCWSTR, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>;
    fn FindItemEx(&self, pszpath: &windows_core::PCWSTR, pincludefilefilter: windows_core::Ref<IOfflineFilesItemFilter>, pincludedirfilter: windows_core::Ref<IOfflineFilesItemFilter>, pexcludefilefilter: windows_core::Ref<IOfflineFilesItemFilter>, pexcludedirfilter: windows_core::Ref<IOfflineFilesItemFilter>, dwqueryflags: u32) -> windows_core::Result<IOfflineFilesItem>;
    fn RenameItem(&self, pszpathoriginal: &windows_core::PCWSTR, pszpathnew: &windows_core::PCWSTR, breplaceifexists: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLocation(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> windows_core::Result<()>;
    fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> windows_core::Result<()>;
    fn ProcessAdminPinPolicy(&self, ppinprogress: windows_core::Ref<IOfflineFilesSyncProgress>, punpinprogress: windows_core::Ref<IOfflineFilesSyncProgress>) -> windows_core::Result<()>;
    fn GetSettingObject(&self, pszsettingname: &windows_core::PCWSTR) -> windows_core::Result<IOfflineFilesSetting>;
    fn EnumSettingObjects(&self) -> windows_core::Result<IEnumOfflineFilesSettings>;
    fn IsPathCacheable(&self, pszpath: &windows_core::PCWSTR, pbcacheable: *mut windows_core::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::Result<()>;
}
impl IOfflineFilesCache_Vtbl {
    pub const fn new<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Synchronize<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, basync: windows_core::BOOL, dwsynccontrol: u32, pisyncconflicthandler: *mut core::ffi::c_void, piprogress: *mut core::ffi::c_void, psyncid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::Synchronize(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwsynccontrol), core::mem::transmute_copy(&pisyncconflicthandler), core::mem::transmute_copy(&piprogress), core::mem::transmute_copy(&psyncid)).into()
            }
        }
        unsafe extern "system" fn DeleteItems<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: windows_core::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::DeleteItems(this, core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&piprogress)).into()
            }
        }
        unsafe extern "system" fn DeleteItemsForUser<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuser: windows_core::PCWSTR, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: windows_core::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::DeleteItemsForUser(this, core::mem::transmute(&pszuser), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&piprogress)).into()
            }
        }
        unsafe extern "system" fn Pin<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: windows_core::BOOL, basync: windows_core::BOOL, dwpincontrolflags: u32, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::Pin(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&bdeep), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwpincontrolflags), core::mem::transmute_copy(&piprogress)).into()
            }
        }
        unsafe extern "system" fn Unpin<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const windows_core::PCWSTR, cpaths: u32, bdeep: windows_core::BOOL, basync: windows_core::BOOL, dwpincontrolflags: u32, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::Unpin(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&rgpszpaths), core::mem::transmute_copy(&cpaths), core::mem::transmute_copy(&bdeep), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&dwpincontrolflags), core::mem::transmute_copy(&piprogress)).into()
            }
        }
        unsafe extern "system" fn GetEncryptionStatus<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbencrypted: *mut windows_core::BOOL, pbpartial: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::GetEncryptionStatus(this, core::mem::transmute_copy(&pbencrypted), core::mem::transmute_copy(&pbpartial)).into()
            }
        }
        unsafe extern "system" fn Encrypt<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: windows_core::BOOL, dwencryptioncontrolflags: u32, basync: windows_core::BOOL, piprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::Encrypt(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bencrypt), core::mem::transmute_copy(&dwencryptioncontrolflags), core::mem::transmute_copy(&basync), core::mem::transmute_copy(&piprogress)).into()
            }
        }
        unsafe extern "system" fn FindItem<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwqueryflags: u32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesCache_Impl::FindItem(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwqueryflags)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindItemEx<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pincludefilefilter: *mut core::ffi::c_void, pincludedirfilter: *mut core::ffi::c_void, pexcludefilefilter: *mut core::ffi::c_void, pexcludedirfilter: *mut core::ffi::c_void, dwqueryflags: u32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesCache_Impl::FindItemEx(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&pincludefilefilter), core::mem::transmute_copy(&pincludedirfilter), core::mem::transmute_copy(&pexcludefilefilter), core::mem::transmute_copy(&pexcludedirfilter), core::mem::transmute_copy(&dwqueryflags)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RenameItem<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpathoriginal: windows_core::PCWSTR, pszpathnew: windows_core::PCWSTR, breplaceifexists: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::RenameItem(this, core::mem::transmute(&pszpathoriginal), core::mem::transmute(&pszpathnew), core::mem::transmute_copy(&breplaceifexists)).into()
            }
        }
        unsafe extern "system" fn GetLocation<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesCache_Impl::GetLocation(this) {
                    Ok(ok__) => {
                        ppszpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::GetDiskSpaceInformation(this, core::mem::transmute_copy(&pcbvolumetotal), core::mem::transmute_copy(&pcblimit), core::mem::transmute_copy(&pcbused), core::mem::transmute_copy(&pcbunpinnedlimit), core::mem::transmute_copy(&pcbunpinnedused)).into()
            }
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::SetDiskSpaceLimits(this, core::mem::transmute_copy(&cblimit), core::mem::transmute_copy(&cbunpinnedlimit)).into()
            }
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinprogress: *mut core::ffi::c_void, punpinprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::ProcessAdminPinPolicy(this, core::mem::transmute_copy(&ppinprogress), core::mem::transmute_copy(&punpinprogress)).into()
            }
        }
        unsafe extern "system" fn GetSettingObject<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsettingname: windows_core::PCWSTR, ppsetting: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesCache_Impl::GetSettingObject(this, core::mem::transmute(&pszsettingname)) {
                    Ok(ok__) => {
                        ppsetting.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumSettingObjects<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesCache_Impl::EnumSettingObjects(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPathCacheable<Identity: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pbcacheable: *mut windows_core::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache_Impl::IsPathCacheable(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&pbcacheable), core::mem::transmute_copy(&psharecachingmode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Synchronize: Synchronize::<Identity, OFFSET>,
            DeleteItems: DeleteItems::<Identity, OFFSET>,
            DeleteItemsForUser: DeleteItemsForUser::<Identity, OFFSET>,
            Pin: Pin::<Identity, OFFSET>,
            Unpin: Unpin::<Identity, OFFSET>,
            GetEncryptionStatus: GetEncryptionStatus::<Identity, OFFSET>,
            Encrypt: Encrypt::<Identity, OFFSET>,
            FindItem: FindItem::<Identity, OFFSET>,
            FindItemEx: FindItemEx::<Identity, OFFSET>,
            RenameItem: RenameItem::<Identity, OFFSET>,
            GetLocation: GetLocation::<Identity, OFFSET>,
            GetDiskSpaceInformation: GetDiskSpaceInformation::<Identity, OFFSET>,
            SetDiskSpaceLimits: SetDiskSpaceLimits::<Identity, OFFSET>,
            ProcessAdminPinPolicy: ProcessAdminPinPolicy::<Identity, OFFSET>,
            GetSettingObject: GetSettingObject::<Identity, OFFSET>,
            EnumSettingObjects: EnumSettingObjects::<Identity, OFFSET>,
            IsPathCacheable: IsPathCacheable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesCache as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesCache {}
windows_core::imp::define_interface!(IOfflineFilesCache2, IOfflineFilesCache2_Vtbl, 0x8c075039_1551_4ed9_8781_56705c04d3c0);
impl core::ops::Deref for IOfflineFilesCache2 {
    type Target = IOfflineFilesCache;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesCache2, windows_core::IUnknown, IOfflineFilesCache);
impl IOfflineFilesCache2 {
    pub unsafe fn RenameItemEx<P0, P1>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenameItemEx)(windows_core::Interface::as_raw(self), pszpathoriginal.param().abi(), pszpathnew.param().abi(), breplaceifexists.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache2_Vtbl {
    pub base__: IOfflineFilesCache_Vtbl,
    pub RenameItemEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesCache2_Impl: IOfflineFilesCache_Impl {
    fn RenameItemEx(&self, pszpathoriginal: &windows_core::PCWSTR, pszpathnew: &windows_core::PCWSTR, breplaceifexists: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IOfflineFilesCache2_Vtbl {
    pub const fn new<Identity: IOfflineFilesCache2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RenameItemEx<Identity: IOfflineFilesCache2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpathoriginal: windows_core::PCWSTR, pszpathnew: windows_core::PCWSTR, breplaceifexists: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesCache2_Impl::RenameItemEx(this, core::mem::transmute(&pszpathoriginal), core::mem::transmute(&pszpathnew), core::mem::transmute_copy(&breplaceifexists)).into()
            }
        }
        Self { base__: IOfflineFilesCache_Vtbl::new::<Identity, OFFSET>(), RenameItemEx: RenameItemEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesCache2 as windows_core::Interface>::IID || iid == &<IOfflineFilesCache as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesCache2 {}
windows_core::imp::define_interface!(IOfflineFilesChangeInfo, IOfflineFilesChangeInfo_Vtbl, 0xa96e6fa4_e0d1_4c29_960b_ee508fe68c72);
windows_core::imp::interface_hierarchy!(IOfflineFilesChangeInfo, windows_core::IUnknown);
impl IOfflineFilesChangeInfo {
    pub unsafe fn IsDirty(&self, pbdirty: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self), pbdirty as _) }
    }
    pub unsafe fn IsDeletedOffline(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDeletedOffline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsCreatedOffline(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCreatedOffline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLocallyModifiedData(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocallyModifiedData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLocallyModifiedAttributes(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocallyModifiedAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLocallyModifiedTime(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLocallyModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesChangeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDeletedOffline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsCreatedOffline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsLocallyModifiedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsLocallyModifiedAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsLocallyModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesChangeInfo_Impl: windows_core::IUnknownImpl {
    fn IsDirty(&self, pbdirty: *mut windows_core::BOOL) -> windows_core::HRESULT;
    fn IsDeletedOffline(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsCreatedOffline(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsLocallyModifiedData(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsLocallyModifiedAttributes(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsLocallyModifiedTime(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesChangeInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdirty: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesChangeInfo_Impl::IsDirty(this, core::mem::transmute_copy(&pbdirty))
            }
        }
        unsafe extern "system" fn IsDeletedOffline<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeletedoffline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesChangeInfo_Impl::IsDeletedOffline(this) {
                    Ok(ok__) => {
                        pbdeletedoffline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCreatedOffline<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcreatedoffline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesChangeInfo_Impl::IsCreatedOffline(this) {
                    Ok(ok__) => {
                        pbcreatedoffline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocallyModifiedData<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifieddata: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedData(this) {
                    Ok(ok__) => {
                        pblocallymodifieddata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifiedattributes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedAttributes(this) {
                    Ok(ok__) => {
                        pblocallymodifiedattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Identity: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocallymodifiedtime: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesChangeInfo_Impl::IsLocallyModifiedTime(this) {
                    Ok(ok__) => {
                        pblocallymodifiedtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            IsDeletedOffline: IsDeletedOffline::<Identity, OFFSET>,
            IsCreatedOffline: IsCreatedOffline::<Identity, OFFSET>,
            IsLocallyModifiedData: IsLocallyModifiedData::<Identity, OFFSET>,
            IsLocallyModifiedAttributes: IsLocallyModifiedAttributes::<Identity, OFFSET>,
            IsLocallyModifiedTime: IsLocallyModifiedTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesChangeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesChangeInfo {}
windows_core::imp::define_interface!(IOfflineFilesConnectionInfo, IOfflineFilesConnectionInfo_Vtbl, 0xefb23a09_a867_4be8_83a6_86969a7d0856);
windows_core::imp::interface_hierarchy!(IOfflineFilesConnectionInfo, windows_core::IUnknown);
impl IOfflineFilesConnectionInfo {
    pub unsafe fn GetConnectState(&self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConnectState)(windows_core::Interface::as_raw(self), pconnectstate as _, pofflinereason as _).ok() }
    }
    pub unsafe fn SetConnectState(&self, hwndparent: Option<super::super::Foundation::HWND>, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectState)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, dwflags, connectstate).ok() }
    }
    pub unsafe fn TransitionOnline(&self, hwndparent: Option<super::super::Foundation::HWND>, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransitionOnline)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, dwflags).ok() }
    }
    pub unsafe fn TransitionOffline(&self, hwndparent: Option<super::super::Foundation::HWND>, dwflags: u32, bforceopenfilesclosed: bool) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransitionOffline)(windows_core::Interface::as_raw(self), hwndparent.unwrap_or(core::mem::zeroed()) as _, dwflags, bforceopenfilesclosed.into(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesConnectionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OFFLINEFILES_CONNECT_STATE, *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::HRESULT,
    pub SetConnectState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, OFFLINEFILES_CONNECT_STATE) -> windows_core::HRESULT,
    pub TransitionOnline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub TransitionOffline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesConnectionInfo_Impl: windows_core::IUnknownImpl {
    fn GetConnectState(&self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::Result<()>;
    fn SetConnectState(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> windows_core::Result<()>;
    fn TransitionOnline(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
    fn TransitionOffline(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesConnectionInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectState<Identity: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesConnectionInfo_Impl::GetConnectState(this, core::mem::transmute_copy(&pconnectstate), core::mem::transmute_copy(&pofflinereason)).into()
            }
        }
        unsafe extern "system" fn SetConnectState<Identity: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesConnectionInfo_Impl::SetConnectState(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&connectstate)).into()
            }
        }
        unsafe extern "system" fn TransitionOnline<Identity: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesConnectionInfo_Impl::TransitionOnline(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn TransitionOffline<Identity: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: windows_core::BOOL, pbopenfilespreventedtransition: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesConnectionInfo_Impl::TransitionOffline(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&bforceopenfilesclosed)) {
                    Ok(ok__) => {
                        pbopenfilespreventedtransition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectState: GetConnectState::<Identity, OFFSET>,
            SetConnectState: SetConnectState::<Identity, OFFSET>,
            TransitionOnline: TransitionOnline::<Identity, OFFSET>,
            TransitionOffline: TransitionOffline::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesConnectionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesConnectionInfo {}
windows_core::imp::define_interface!(IOfflineFilesDirectoryItem, IOfflineFilesDirectoryItem_Vtbl, 0x2273597a_a08c_4a00_a37a_c1ae4e9a1cfd);
impl core::ops::Deref for IOfflineFilesDirectoryItem {
    type Target = IOfflineFilesItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesDirectoryItem, windows_core::IUnknown, IOfflineFilesItem);
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirectoryItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
pub trait IOfflineFilesDirectoryItem_Impl: IOfflineFilesItem_Impl {}
impl IOfflineFilesDirectoryItem_Vtbl {
    pub const fn new<Identity: IOfflineFilesDirectoryItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesDirectoryItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesDirectoryItem {}
windows_core::imp::define_interface!(IOfflineFilesDirtyInfo, IOfflineFilesDirtyInfo_Vtbl, 0x0f50ce33_bac9_4eaa_a11d_da0e527d047d);
windows_core::imp::interface_hierarchy!(IOfflineFilesDirtyInfo, windows_core::IUnknown);
impl IOfflineFilesDirtyInfo {
    pub unsafe fn LocalDirtyByteCount(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalDirtyByteCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoteDirtyByteCount(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteDirtyByteCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirtyInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LocalDirtyByteCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoteDirtyByteCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IOfflineFilesDirtyInfo_Impl: windows_core::IUnknownImpl {
    fn LocalDirtyByteCount(&self) -> windows_core::Result<i64>;
    fn RemoteDirtyByteCount(&self) -> windows_core::Result<i64>;
}
impl IOfflineFilesDirtyInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalDirtyByteCount<Identity: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybytecount: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesDirtyInfo_Impl::LocalDirtyByteCount(this) {
                    Ok(ok__) => {
                        pdirtybytecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Identity: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybytecount: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesDirtyInfo_Impl::RemoteDirtyByteCount(this) {
                    Ok(ok__) => {
                        pdirtybytecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LocalDirtyByteCount: LocalDirtyByteCount::<Identity, OFFSET>,
            RemoteDirtyByteCount: RemoteDirtyByteCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesDirtyInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesDirtyInfo {}
windows_core::imp::define_interface!(IOfflineFilesErrorInfo, IOfflineFilesErrorInfo_Vtbl, 0x7112fa5f_7571_435a_8eb7_195c7c1429bc);
windows_core::imp::interface_hierarchy!(IOfflineFilesErrorInfo, windows_core::IUnknown);
impl IOfflineFilesErrorInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawData(&self) -> windows_core::Result<*mut super::super::System::Com::BYTE_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRawData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRawData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::BYTE_BLOB) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRawData: usize,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOfflineFilesErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetRawData(&self) -> windows_core::Result<*mut super::super::System::Com::BYTE_BLOB>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOfflineFilesErrorInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRawData<Identity: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesErrorInfo_Impl::GetRawData(this) {
                    Ok(ok__) => {
                        ppblob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesErrorInfo_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        ppszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRawData: GetRawData::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOfflineFilesErrorInfo {}
windows_core::imp::define_interface!(IOfflineFilesEvents, IOfflineFilesEvents_Vtbl, 0xe25585c1_0caa_4eb1_873b_1cae5b77c314);
windows_core::imp::interface_hierarchy!(IOfflineFilesEvents, windows_core::IUnknown);
impl IOfflineFilesEvents {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CacheMoved)(windows_core::Interface::as_raw(self), pszoldpath.param().abi(), psznewpath.param().abi()).ok() }
    }
    pub unsafe fn CacheIsFull(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CacheIsFull)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CacheIsCorrupted(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CacheIsCorrupted)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Enabled(&self, benabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), benabled.into()).ok() }
    }
    pub unsafe fn EncryptionChanged(&self, bwasencrypted: bool, bwaspartial: bool, bisencrypted: bool, bispartial: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EncryptionChanged)(windows_core::Interface::as_raw(self), bwasencrypted.into(), bwaspartial.into(), bisencrypted.into(), bispartial.into()).ok() }
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SyncBegin)(windows_core::Interface::as_raw(self), rsyncid).ok() }
    }
    pub unsafe fn SyncFileResult<P1>(&self, rsyncid: *const windows_core::GUID, pszfile: P1, hrresult: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SyncFileResult)(windows_core::Interface::as_raw(self), rsyncid, pszfile.param().abi(), hrresult).ok() }
    }
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SyncConflictRecAdded)(windows_core::Interface::as_raw(self), pszconflictpath.param().abi(), pftconflictdatetime, conflictsyncstate).ok() }
    }
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SyncConflictRecUpdated)(windows_core::Interface::as_raw(self), pszconflictpath.param().abi(), pftconflictdatetime, conflictsyncstate).ok() }
    }
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SyncConflictRecRemoved)(windows_core::Interface::as_raw(self), pszconflictpath.param().abi(), pftconflictdatetime, conflictsyncstate).ok() }
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SyncEnd)(windows_core::Interface::as_raw(self), rsyncid, hrresult).ok() }
    }
    pub unsafe fn NetTransportArrived(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NetTransportArrived)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NoNetTransports(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NoNetTransports)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemDisconnected)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemReconnected)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemAvailableOffline)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemNotAvailableOffline)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemPinned)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemNotPinned)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemModified<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: bool, bmodifiedattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemModified)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype, bmodifieddata.into(), bmodifiedattributes.into()).ok() }
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemAddedToCache)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemDeletedFromCache)(windows_core::Interface::as_raw(self), pszpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ItemRenamed)(windows_core::Interface::as_raw(self), pszoldpath.param().abi(), psznewpath.param().abi(), itemtype).ok() }
    }
    pub unsafe fn DataLost(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DataLost)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Ping(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Ping)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CacheMoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CacheIsFull: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CacheIsCorrupted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub EncryptionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub SyncBegin: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SyncFileResult: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::HRESULT) -> windows_core::HRESULT,
    pub SyncConflictRecAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT,
    pub SyncConflictRecUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT,
    pub SyncConflictRecRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::Foundation::FILETIME, OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT,
    pub SyncEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub NetTransportArrived: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NoNetTransports: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemReconnected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemAvailableOffline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemNotAvailableOffline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemPinned: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemNotPinned: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemModified: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub ItemAddedToCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemDeletedFromCache: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub ItemRenamed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub DataLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Ping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOfflineFilesEvents_Impl: windows_core::IUnknownImpl {
    fn CacheMoved(&self, pszoldpath: &windows_core::PCWSTR, psznewpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CacheIsFull(&self) -> windows_core::Result<()>;
    fn CacheIsCorrupted(&self) -> windows_core::Result<()>;
    fn Enabled(&self, benabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn EncryptionChanged(&self, bwasencrypted: windows_core::BOOL, bwaspartial: windows_core::BOOL, bisencrypted: windows_core::BOOL, bispartial: windows_core::BOOL) -> windows_core::Result<()>;
    fn SyncBegin(&self, rsyncid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SyncFileResult(&self, rsyncid: *const windows_core::GUID, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn SyncConflictRecAdded(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncConflictRecUpdated(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncConflictRecRemoved(&self, pszconflictpath: &windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::Result<()>;
    fn SyncEnd(&self, rsyncid: *const windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn NetTransportArrived(&self) -> windows_core::Result<()>;
    fn NoNetTransports(&self) -> windows_core::Result<()>;
    fn ItemDisconnected(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemReconnected(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemAvailableOffline(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemNotAvailableOffline(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemPinned(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemNotPinned(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemModified(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: windows_core::BOOL, bmodifiedattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn ItemAddedToCache(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemDeletedFromCache(&self, pszpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn ItemRenamed(&self, pszoldpath: &windows_core::PCWSTR, psznewpath: &windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::Result<()>;
    fn DataLost(&self) -> windows_core::Result<()>;
    fn Ping(&self) -> windows_core::Result<()>;
}
impl IOfflineFilesEvents_Vtbl {
    pub const fn new<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CacheMoved<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszoldpath: windows_core::PCWSTR, psznewpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::CacheMoved(this, core::mem::transmute(&pszoldpath), core::mem::transmute(&psznewpath)).into()
            }
        }
        unsafe extern "system" fn CacheIsFull<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::CacheIsFull(this).into()
            }
        }
        unsafe extern "system" fn CacheIsCorrupted<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::CacheIsCorrupted(this).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::Enabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn EncryptionChanged<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bwasencrypted: windows_core::BOOL, bwaspartial: windows_core::BOOL, bisencrypted: windows_core::BOOL, bispartial: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::EncryptionChanged(this, core::mem::transmute_copy(&bwasencrypted), core::mem::transmute_copy(&bwaspartial), core::mem::transmute_copy(&bisencrypted), core::mem::transmute_copy(&bispartial)).into()
            }
        }
        unsafe extern "system" fn SyncBegin<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncBegin(this, core::mem::transmute_copy(&rsyncid)).into()
            }
        }
        unsafe extern "system" fn SyncFileResult<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncFileResult(this, core::mem::transmute_copy(&rsyncid), core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult)).into()
            }
        }
        unsafe extern "system" fn SyncConflictRecAdded<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncConflictRecAdded(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
            }
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncConflictRecUpdated(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
            }
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconflictpath: windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncConflictRecRemoved(this, core::mem::transmute(&pszconflictpath), core::mem::transmute_copy(&pftconflictdatetime), core::mem::transmute_copy(&conflictsyncstate)).into()
            }
        }
        unsafe extern "system" fn SyncEnd<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rsyncid: *const windows_core::GUID, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::SyncEnd(this, core::mem::transmute_copy(&rsyncid), core::mem::transmute_copy(&hrresult)).into()
            }
        }
        unsafe extern "system" fn NetTransportArrived<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::NetTransportArrived(this).into()
            }
        }
        unsafe extern "system" fn NoNetTransports<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::NoNetTransports(this).into()
            }
        }
        unsafe extern "system" fn ItemDisconnected<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemDisconnected(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemReconnected<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemReconnected(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemAvailableOffline<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemAvailableOffline(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemNotAvailableOffline(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemPinned<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemPinned(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemNotPinned<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemNotPinned(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemModified<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: windows_core::BOOL, bmodifiedattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemModified(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype), core::mem::transmute_copy(&bmodifieddata), core::mem::transmute_copy(&bmodifiedattributes)).into()
            }
        }
        unsafe extern "system" fn ItemAddedToCache<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemAddedToCache(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemDeletedFromCache<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemDeletedFromCache(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn ItemRenamed<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszoldpath: windows_core::PCWSTR, psznewpath: windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::ItemRenamed(this, core::mem::transmute(&pszoldpath), core::mem::transmute(&psznewpath), core::mem::transmute_copy(&itemtype)).into()
            }
        }
        unsafe extern "system" fn DataLost<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::DataLost(this).into()
            }
        }
        unsafe extern "system" fn Ping<Identity: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents_Impl::Ping(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CacheMoved: CacheMoved::<Identity, OFFSET>,
            CacheIsFull: CacheIsFull::<Identity, OFFSET>,
            CacheIsCorrupted: CacheIsCorrupted::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            EncryptionChanged: EncryptionChanged::<Identity, OFFSET>,
            SyncBegin: SyncBegin::<Identity, OFFSET>,
            SyncFileResult: SyncFileResult::<Identity, OFFSET>,
            SyncConflictRecAdded: SyncConflictRecAdded::<Identity, OFFSET>,
            SyncConflictRecUpdated: SyncConflictRecUpdated::<Identity, OFFSET>,
            SyncConflictRecRemoved: SyncConflictRecRemoved::<Identity, OFFSET>,
            SyncEnd: SyncEnd::<Identity, OFFSET>,
            NetTransportArrived: NetTransportArrived::<Identity, OFFSET>,
            NoNetTransports: NoNetTransports::<Identity, OFFSET>,
            ItemDisconnected: ItemDisconnected::<Identity, OFFSET>,
            ItemReconnected: ItemReconnected::<Identity, OFFSET>,
            ItemAvailableOffline: ItemAvailableOffline::<Identity, OFFSET>,
            ItemNotAvailableOffline: ItemNotAvailableOffline::<Identity, OFFSET>,
            ItemPinned: ItemPinned::<Identity, OFFSET>,
            ItemNotPinned: ItemNotPinned::<Identity, OFFSET>,
            ItemModified: ItemModified::<Identity, OFFSET>,
            ItemAddedToCache: ItemAddedToCache::<Identity, OFFSET>,
            ItemDeletedFromCache: ItemDeletedFromCache::<Identity, OFFSET>,
            ItemRenamed: ItemRenamed::<Identity, OFFSET>,
            DataLost: DataLost::<Identity, OFFSET>,
            Ping: Ping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesEvents {}
windows_core::imp::define_interface!(IOfflineFilesEvents2, IOfflineFilesEvents2_Vtbl, 0x1ead8f56_ff76_4faa_a795_6f6ef792498b);
impl core::ops::Deref for IOfflineFilesEvents2 {
    type Target = IOfflineFilesEvents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesEvents2, windows_core::IUnknown, IOfflineFilesEvents);
impl IOfflineFilesEvents2 {
    pub unsafe fn ItemReconnectBegin(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ItemReconnectBegin)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ItemReconnectEnd(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ItemReconnectEnd)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CacheEvictBegin(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CacheEvictBegin)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CacheEvictEnd(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CacheEvictEnd)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackgroundSyncBegin)(windows_core::Interface::as_raw(self), dwsynccontrolflags).ok() }
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackgroundSyncEnd)(windows_core::Interface::as_raw(self), dwsynccontrolflags).ok() }
    }
    pub unsafe fn PolicyChangeDetected(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PolicyChangeDetected)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreferenceChangeDetected)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SettingsChangesApplied(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettingsChangesApplied)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents2_Vtbl {
    pub base__: IOfflineFilesEvents_Vtbl,
    pub ItemReconnectBegin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemReconnectEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CacheEvictBegin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CacheEvictEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackgroundSyncBegin: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BackgroundSyncEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PolicyChangeDetected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreferenceChangeDetected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettingsChangesApplied: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOfflineFilesEvents2_Impl: IOfflineFilesEvents_Impl {
    fn ItemReconnectBegin(&self) -> windows_core::Result<()>;
    fn ItemReconnectEnd(&self) -> windows_core::Result<()>;
    fn CacheEvictBegin(&self) -> windows_core::Result<()>;
    fn CacheEvictEnd(&self) -> windows_core::Result<()>;
    fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> windows_core::Result<()>;
    fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> windows_core::Result<()>;
    fn PolicyChangeDetected(&self) -> windows_core::Result<()>;
    fn PreferenceChangeDetected(&self) -> windows_core::Result<()>;
    fn SettingsChangesApplied(&self) -> windows_core::Result<()>;
}
impl IOfflineFilesEvents2_Vtbl {
    pub const fn new<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ItemReconnectBegin<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::ItemReconnectBegin(this).into()
            }
        }
        unsafe extern "system" fn ItemReconnectEnd<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::ItemReconnectEnd(this).into()
            }
        }
        unsafe extern "system" fn CacheEvictBegin<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::CacheEvictBegin(this).into()
            }
        }
        unsafe extern "system" fn CacheEvictEnd<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::CacheEvictEnd(this).into()
            }
        }
        unsafe extern "system" fn BackgroundSyncBegin<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsynccontrolflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::BackgroundSyncBegin(this, core::mem::transmute_copy(&dwsynccontrolflags)).into()
            }
        }
        unsafe extern "system" fn BackgroundSyncEnd<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsynccontrolflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::BackgroundSyncEnd(this, core::mem::transmute_copy(&dwsynccontrolflags)).into()
            }
        }
        unsafe extern "system" fn PolicyChangeDetected<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::PolicyChangeDetected(this).into()
            }
        }
        unsafe extern "system" fn PreferenceChangeDetected<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::PreferenceChangeDetected(this).into()
            }
        }
        unsafe extern "system" fn SettingsChangesApplied<Identity: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents2_Impl::SettingsChangesApplied(this).into()
            }
        }
        Self {
            base__: IOfflineFilesEvents_Vtbl::new::<Identity, OFFSET>(),
            ItemReconnectBegin: ItemReconnectBegin::<Identity, OFFSET>,
            ItemReconnectEnd: ItemReconnectEnd::<Identity, OFFSET>,
            CacheEvictBegin: CacheEvictBegin::<Identity, OFFSET>,
            CacheEvictEnd: CacheEvictEnd::<Identity, OFFSET>,
            BackgroundSyncBegin: BackgroundSyncBegin::<Identity, OFFSET>,
            BackgroundSyncEnd: BackgroundSyncEnd::<Identity, OFFSET>,
            PolicyChangeDetected: PolicyChangeDetected::<Identity, OFFSET>,
            PreferenceChangeDetected: PreferenceChangeDetected::<Identity, OFFSET>,
            SettingsChangesApplied: SettingsChangesApplied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesEvents2 {}
windows_core::imp::define_interface!(IOfflineFilesEvents3, IOfflineFilesEvents3_Vtbl, 0x9ba04a45_ee69_42f0_9ab1_7db5c8805808);
impl core::ops::Deref for IOfflineFilesEvents3 {
    type Target = IOfflineFilesEvents2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesEvents3, windows_core::IUnknown, IOfflineFilesEvents, IOfflineFilesEvents2);
impl IOfflineFilesEvents3 {
    pub unsafe fn TransparentCacheItemNotify<P0, P5>(&self, pszpath: P0, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: bool, bmodifiedattributes: bool, pzsoldpath: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransparentCacheItemNotify)(windows_core::Interface::as_raw(self), pszpath.param().abi(), eventtype, itemtype, bmodifieddata.into(), bmodifiedattributes.into(), pzsoldpath.param().abi()).ok() }
    }
    pub unsafe fn PrefetchFileBegin<P0>(&self, pszpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrefetchFileBegin)(windows_core::Interface::as_raw(self), pszpath.param().abi()).ok() }
    }
    pub unsafe fn PrefetchFileEnd<P0>(&self, pszpath: P0, hrresult: windows_core::HRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrefetchFileEnd)(windows_core::Interface::as_raw(self), pszpath.param().abi(), hrresult).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents3_Vtbl {
    pub base__: IOfflineFilesEvents2_Vtbl,
    pub TransparentCacheItemNotify: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, OFFLINEFILES_EVENTS, OFFLINEFILES_ITEM_TYPE, windows_core::BOOL, windows_core::BOOL, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub PrefetchFileBegin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub PrefetchFileEnd: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IOfflineFilesEvents3_Impl: IOfflineFilesEvents2_Impl {
    fn TransparentCacheItemNotify(&self, pszpath: &windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: windows_core::BOOL, bmodifiedattributes: windows_core::BOOL, pzsoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrefetchFileBegin(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrefetchFileEnd(&self, pszpath: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IOfflineFilesEvents3_Vtbl {
    pub const fn new<Identity: IOfflineFilesEvents3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransparentCacheItemNotify<Identity: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: windows_core::BOOL, bmodifiedattributes: windows_core::BOOL, pzsoldpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents3_Impl::TransparentCacheItemNotify(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&eventtype), core::mem::transmute_copy(&itemtype), core::mem::transmute_copy(&bmodifieddata), core::mem::transmute_copy(&bmodifiedattributes), core::mem::transmute(&pzsoldpath)).into()
            }
        }
        unsafe extern "system" fn PrefetchFileBegin<Identity: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents3_Impl::PrefetchFileBegin(this, core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn PrefetchFileEnd<Identity: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents3_Impl::PrefetchFileEnd(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&hrresult)).into()
            }
        }
        Self {
            base__: IOfflineFilesEvents2_Vtbl::new::<Identity, OFFSET>(),
            TransparentCacheItemNotify: TransparentCacheItemNotify::<Identity, OFFSET>,
            PrefetchFileBegin: PrefetchFileBegin::<Identity, OFFSET>,
            PrefetchFileEnd: PrefetchFileEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents3 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesEvents3 {}
windows_core::imp::define_interface!(IOfflineFilesEvents4, IOfflineFilesEvents4_Vtbl, 0xdbd69b1e_c7d2_473e_b35f_9d8c24c0c484);
impl core::ops::Deref for IOfflineFilesEvents4 {
    type Target = IOfflineFilesEvents3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesEvents4, windows_core::IUnknown, IOfflineFilesEvents, IOfflineFilesEvents2, IOfflineFilesEvents3);
impl IOfflineFilesEvents4 {
    pub unsafe fn PrefetchCloseHandleBegin(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrefetchCloseHandleBegin)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PrefetchCloseHandleEnd(&self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrefetchCloseHandleEnd)(windows_core::Interface::as_raw(self), dwclosedhandlecount, dwopenhandlecount, hrresult).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents4_Vtbl {
    pub base__: IOfflineFilesEvents3_Vtbl,
    pub PrefetchCloseHandleBegin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrefetchCloseHandleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IOfflineFilesEvents4_Impl: IOfflineFilesEvents3_Impl {
    fn PrefetchCloseHandleBegin(&self) -> windows_core::Result<()>;
    fn PrefetchCloseHandleEnd(&self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IOfflineFilesEvents4_Vtbl {
    pub const fn new<Identity: IOfflineFilesEvents4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Identity: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents4_Impl::PrefetchCloseHandleBegin(this).into()
            }
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Identity: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEvents4_Impl::PrefetchCloseHandleEnd(this, core::mem::transmute_copy(&dwclosedhandlecount), core::mem::transmute_copy(&dwopenhandlecount), core::mem::transmute_copy(&hrresult)).into()
            }
        }
        Self {
            base__: IOfflineFilesEvents3_Vtbl::new::<Identity, OFFSET>(),
            PrefetchCloseHandleBegin: PrefetchCloseHandleBegin::<Identity, OFFSET>,
            PrefetchCloseHandleEnd: PrefetchCloseHandleEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEvents4 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents2 as windows_core::Interface>::IID || iid == &<IOfflineFilesEvents3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesEvents4 {}
windows_core::imp::define_interface!(IOfflineFilesEventsFilter, IOfflineFilesEventsFilter_Vtbl, 0x33fc4e1b_0716_40fa_ba65_6e62a84a846f);
windows_core::imp::interface_hierarchy!(IOfflineFilesEventsFilter, windows_core::IUnknown);
impl IOfflineFilesEventsFilter {
    pub unsafe fn GetPathFilter(&self, ppszfilter: *mut windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPathFilter)(windows_core::Interface::as_raw(self), ppszfilter as _, pmatch as _).ok() }
    }
    pub unsafe fn GetIncludedEvents(&self, prgevents: &mut [OFFLINEFILES_EVENTS], pcevents: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIncludedEvents)(windows_core::Interface::as_raw(self), prgevents.len().try_into().unwrap(), core::mem::transmute(prgevents.as_ptr()), pcevents as _).ok() }
    }
    pub unsafe fn GetExcludedEvents(&self, prgevents: &mut [OFFLINEFILES_EVENTS], pcevents: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetExcludedEvents)(windows_core::Interface::as_raw(self), prgevents.len().try_into().unwrap(), core::mem::transmute(prgevents.as_ptr()), pcevents as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEventsFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPathFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::HRESULT,
    pub GetIncludedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut OFFLINEFILES_EVENTS, *mut u32) -> windows_core::HRESULT,
    pub GetExcludedEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut OFFLINEFILES_EVENTS, *mut u32) -> windows_core::HRESULT,
}
pub trait IOfflineFilesEventsFilter_Impl: windows_core::IUnknownImpl {
    fn GetPathFilter(&self, ppszfilter: *mut windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::Result<()>;
    fn GetIncludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::Result<()>;
    fn GetExcludedEvents(&self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::Result<()>;
}
impl IOfflineFilesEventsFilter_Vtbl {
    pub const fn new<Identity: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPathFilter<Identity: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszfilter: *mut windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEventsFilter_Impl::GetPathFilter(this, core::mem::transmute_copy(&ppszfilter), core::mem::transmute_copy(&pmatch)).into()
            }
        }
        unsafe extern "system" fn GetIncludedEvents<Identity: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEventsFilter_Impl::GetIncludedEvents(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&prgevents), core::mem::transmute_copy(&pcevents)).into()
            }
        }
        unsafe extern "system" fn GetExcludedEvents<Identity: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesEventsFilter_Impl::GetExcludedEvents(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&prgevents), core::mem::transmute_copy(&pcevents)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPathFilter: GetPathFilter::<Identity, OFFSET>,
            GetIncludedEvents: GetIncludedEvents::<Identity, OFFSET>,
            GetExcludedEvents: GetExcludedEvents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesEventsFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesEventsFilter {}
windows_core::imp::define_interface!(IOfflineFilesFileItem, IOfflineFilesFileItem_Vtbl, 0x8dfadead_26c2_4eff_8a72_6b50723d9a00);
impl core::ops::Deref for IOfflineFilesFileItem {
    type Target = IOfflineFilesItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesFileItem, windows_core::IUnknown, IOfflineFilesItem);
impl IOfflineFilesFileItem {
    pub unsafe fn IsSparse(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSparse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEncrypted(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEncrypted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
    pub IsSparse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesFileItem_Impl: IOfflineFilesItem_Impl {
    fn IsSparse(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsEncrypted(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesFileItem_Vtbl {
    pub const fn new<Identity: IOfflineFilesFileItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSparse<Identity: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbissparse: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesFileItem_Impl::IsSparse(this) {
                    Ok(ok__) => {
                        pbissparse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEncrypted<Identity: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisencrypted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesFileItem_Impl::IsEncrypted(this) {
                    Ok(ok__) => {
                        pbisencrypted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, OFFSET>(), IsSparse: IsSparse::<Identity, OFFSET>, IsEncrypted: IsEncrypted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesFileItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesFileItem {}
windows_core::imp::define_interface!(IOfflineFilesFileSysInfo, IOfflineFilesFileSysInfo_Vtbl, 0xbc1a163f_7bfd_4d88_9c66_96ea9a6a3d6b);
windows_core::imp::interface_hierarchy!(IOfflineFilesFileSysInfo, windows_core::IUnknown);
impl IOfflineFilesFileSysInfo {
    pub unsafe fn GetAttributes(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), copy, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTimes(&self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTimes)(windows_core::Interface::as_raw(self), copy, pftcreationtime as _, pftlastwritetime as _, pftchangetime as _, pftlastaccesstime as _).ok() }
    }
    pub unsafe fn GetFileSize(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), copy, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileSysInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, OFFLINEFILES_ITEM_COPY, *mut u32) -> windows_core::HRESULT,
    pub GetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, OFFLINEFILES_ITEM_COPY, *mut super::super::Foundation::FILETIME, *mut super::super::Foundation::FILETIME, *mut super::super::Foundation::FILETIME, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, OFFLINEFILES_ITEM_COPY, *mut i64) -> windows_core::HRESULT,
}
pub trait IOfflineFilesFileSysInfo_Impl: windows_core::IUnknownImpl {
    fn GetAttributes(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<u32>;
    fn GetTimes(&self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFileSize(&self, copy: OFFLINEFILES_ITEM_COPY) -> windows_core::Result<i64>;
}
impl IOfflineFilesFileSysInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAttributes<Identity: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesFileSysInfo_Impl::GetAttributes(this, core::mem::transmute_copy(&copy)) {
                    Ok(ok__) => {
                        pdwattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimes<Identity: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesFileSysInfo_Impl::GetTimes(this, core::mem::transmute_copy(&copy), core::mem::transmute_copy(&pftcreationtime), core::mem::transmute_copy(&pftlastwritetime), core::mem::transmute_copy(&pftchangetime), core::mem::transmute_copy(&pftlastaccesstime)).into()
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesFileSysInfo_Impl::GetFileSize(this, core::mem::transmute_copy(&copy)) {
                    Ok(ok__) => {
                        psize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetTimes: GetTimes::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesFileSysInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesFileSysInfo {}
windows_core::imp::define_interface!(IOfflineFilesGhostInfo, IOfflineFilesGhostInfo_Vtbl, 0x2b09d48c_8ab5_464f_a755_a59d92f99429);
windows_core::imp::interface_hierarchy!(IOfflineFilesGhostInfo, windows_core::IUnknown);
impl IOfflineFilesGhostInfo {
    pub unsafe fn IsGhosted(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGhosted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesGhostInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsGhosted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesGhostInfo_Impl: windows_core::IUnknownImpl {
    fn IsGhosted(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesGhostInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsGhosted<Identity: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbghosted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesGhostInfo_Impl::IsGhosted(this) {
                    Ok(ok__) => {
                        pbghosted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsGhosted: IsGhosted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesGhostInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesGhostInfo {}
windows_core::imp::define_interface!(IOfflineFilesItem, IOfflineFilesItem_Vtbl, 0x4a753da6_e044_4f12_a718_5d14d079a906);
windows_core::imp::interface_hierarchy!(IOfflineFilesItem, windows_core::IUnknown);
impl IOfflineFilesItem {
    pub unsafe fn GetItemType(&self) -> windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetParentItem(&self) -> windows_core::Result<IOfflineFilesItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), dwqueryflags).ok() }
    }
    pub unsafe fn IsMarkedForDeletion(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMarkedForDeletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsMarkedForDeletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesItem_Impl: windows_core::IUnknownImpl {
    fn GetItemType(&self) -> windows_core::Result<OFFLINEFILES_ITEM_TYPE>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentItem(&self) -> windows_core::Result<IOfflineFilesItem>;
    fn Refresh(&self, dwqueryflags: u32) -> windows_core::Result<()>;
    fn IsMarkedForDeletion(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesItem_Vtbl {
    pub const fn new<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemType<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItem_Impl::GetItemType(this) {
                    Ok(ok__) => {
                        pitemtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPath<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItem_Impl::GetPath(this) {
                    Ok(ok__) => {
                        ppszpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItem_Impl::GetParentItem(this) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqueryflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesItem_Impl::Refresh(this, core::mem::transmute_copy(&dwqueryflags)).into()
            }
        }
        unsafe extern "system" fn IsMarkedForDeletion<Identity: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmarkedfordeletion: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItem_Impl::IsMarkedForDeletion(this) {
                    Ok(ok__) => {
                        pbmarkedfordeletion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            IsMarkedForDeletion: IsMarkedForDeletion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesItem {}
windows_core::imp::define_interface!(IOfflineFilesItemContainer, IOfflineFilesItemContainer_Vtbl, 0x3836f049_9413_45dd_bf46_b5aaa82dc310);
windows_core::imp::interface_hierarchy!(IOfflineFilesItemContainer, windows_core::IUnknown);
impl IOfflineFilesItemContainer {
    pub unsafe fn EnumItems(&self, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumItems)(windows_core::Interface::as_raw(self), dwqueryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumItemsEx<P0, P1, P2, P3>(&self, pincludefilefilter: P0, pincludedirfilter: P1, pexcludefilefilter: P2, pexcludedirfilter: P3, dwenumflags: u32, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems>
    where
        P0: windows_core::Param<IOfflineFilesItemFilter>,
        P1: windows_core::Param<IOfflineFilesItemFilter>,
        P2: windows_core::Param<IOfflineFilesItemFilter>,
        P3: windows_core::Param<IOfflineFilesItemFilter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumItemsEx)(windows_core::Interface::as_raw(self), pincludefilefilter.param().abi(), pincludedirfilter.param().abi(), pexcludefilefilter.param().abi(), pexcludedirfilter.param().abi(), dwenumflags, dwqueryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumItems: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumItemsEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOfflineFilesItemContainer_Impl: windows_core::IUnknownImpl {
    fn EnumItems(&self, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems>;
    fn EnumItemsEx(&self, pincludefilefilter: windows_core::Ref<IOfflineFilesItemFilter>, pincludedirfilter: windows_core::Ref<IOfflineFilesItemFilter>, pexcludefilefilter: windows_core::Ref<IOfflineFilesItemFilter>, pexcludedirfilter: windows_core::Ref<IOfflineFilesItemFilter>, dwenumflags: u32, dwqueryflags: u32) -> windows_core::Result<IEnumOfflineFilesItems>;
}
impl IOfflineFilesItemContainer_Vtbl {
    pub const fn new<Identity: IOfflineFilesItemContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumItems<Identity: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqueryflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItemContainer_Impl::EnumItems(this, core::mem::transmute_copy(&dwqueryflags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumItemsEx<Identity: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pincludefilefilter: *mut core::ffi::c_void, pincludedirfilter: *mut core::ffi::c_void, pexcludefilefilter: *mut core::ffi::c_void, pexcludedirfilter: *mut core::ffi::c_void, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesItemContainer_Impl::EnumItemsEx(this, core::mem::transmute_copy(&pincludefilefilter), core::mem::transmute_copy(&pincludedirfilter), core::mem::transmute_copy(&pexcludefilefilter), core::mem::transmute_copy(&pexcludedirfilter), core::mem::transmute_copy(&dwenumflags), core::mem::transmute_copy(&dwqueryflags)) {
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
            EnumItems: EnumItems::<Identity, OFFSET>,
            EnumItemsEx: EnumItemsEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItemContainer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesItemContainer {}
windows_core::imp::define_interface!(IOfflineFilesItemFilter, IOfflineFilesItemFilter_Vtbl, 0xf4b5a26c_dc05_4f20_ada4_551f1077be5c);
windows_core::imp::interface_hierarchy!(IOfflineFilesItemFilter, windows_core::IUnknown);
impl IOfflineFilesItemFilter {
    pub unsafe fn GetFilterFlags(&self, pullflags: *mut u64, pullmask: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFilterFlags)(windows_core::Interface::as_raw(self), pullflags as _, pullmask as _).ok() }
    }
    pub unsafe fn GetTimeFilter(&self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut windows_core::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTimeFilter)(windows_core::Interface::as_raw(self), pfttime as _, pbevaltimeofday as _, ptimetype as _, pcompare as _).ok() }
    }
    pub unsafe fn GetPatternFilter(&self, pszpattern: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPatternFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(pszpattern.as_ptr()), pszpattern.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilterFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetTimeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME, *mut windows_core::BOOL, *mut OFFLINEFILES_ITEM_TIME, *mut OFFLINEFILES_COMPARE) -> windows_core::HRESULT,
    pub GetPatternFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
pub trait IOfflineFilesItemFilter_Impl: windows_core::IUnknownImpl {
    fn GetFilterFlags(&self, pullflags: *mut u64, pullmask: *mut u64) -> windows_core::Result<()>;
    fn GetTimeFilter(&self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut windows_core::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> windows_core::Result<()>;
    fn GetPatternFilter(&self, pszpattern: windows_core::PWSTR, cchpattern: u32) -> windows_core::Result<()>;
}
impl IOfflineFilesItemFilter_Vtbl {
    pub const fn new<Identity: IOfflineFilesItemFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFilterFlags<Identity: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesItemFilter_Impl::GetFilterFlags(this, core::mem::transmute_copy(&pullflags), core::mem::transmute_copy(&pullmask)).into()
            }
        }
        unsafe extern "system" fn GetTimeFilter<Identity: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut windows_core::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesItemFilter_Impl::GetTimeFilter(this, core::mem::transmute_copy(&pfttime), core::mem::transmute_copy(&pbevaltimeofday), core::mem::transmute_copy(&ptimetype), core::mem::transmute_copy(&pcompare)).into()
            }
        }
        unsafe extern "system" fn GetPatternFilter<Identity: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PWSTR, cchpattern: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesItemFilter_Impl::GetPatternFilter(this, core::mem::transmute_copy(&pszpattern), core::mem::transmute_copy(&cchpattern)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterFlags: GetFilterFlags::<Identity, OFFSET>,
            GetTimeFilter: GetTimeFilter::<Identity, OFFSET>,
            GetPatternFilter: GetPatternFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesItemFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesItemFilter {}
windows_core::imp::define_interface!(IOfflineFilesPinInfo, IOfflineFilesPinInfo_Vtbl, 0x5b2b0655_b3fd_497d_adeb_bd156bc8355b);
windows_core::imp::interface_hierarchy!(IOfflineFilesPinInfo, windows_core::IUnknown);
impl IOfflineFilesPinInfo {
    pub unsafe fn IsPinned(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPinned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPinnedForUser)(windows_core::Interface::as_raw(self), pbpinnedforuser as _, pbinherit as _).ok() }
    }
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPinnedForUserByPolicy)(windows_core::Interface::as_raw(self), pbpinnedforuser as _, pbinherit as _).ok() }
    }
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPinnedForComputer)(windows_core::Interface::as_raw(self), pbpinnedforcomputer as _, pbinherit as _).ok() }
    }
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsPinnedForFolderRedirection)(windows_core::Interface::as_raw(self), pbpinnedforfolderredirection as _, pbinherit as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPinned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPinnedForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPinnedForUserByPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPinnedForComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsPinnedForFolderRedirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesPinInfo_Impl: windows_core::IUnknownImpl {
    fn IsPinned(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsPinnedForUser(&self, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IOfflineFilesPinInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPinned<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinned: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesPinInfo_Impl::IsPinned(this) {
                    Ok(ok__) => {
                        pbpinned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPinnedForUser<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesPinInfo_Impl::IsPinnedForUser(this, core::mem::transmute_copy(&pbpinnedforuser), core::mem::transmute_copy(&pbinherit)).into()
            }
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforuser: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesPinInfo_Impl::IsPinnedForUserByPolicy(this, core::mem::transmute_copy(&pbpinnedforuser), core::mem::transmute_copy(&pbinherit)).into()
            }
        }
        unsafe extern "system" fn IsPinnedForComputer<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforcomputer: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesPinInfo_Impl::IsPinnedForComputer(this, core::mem::transmute_copy(&pbpinnedforcomputer), core::mem::transmute_copy(&pbinherit)).into()
            }
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Identity: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpinnedforfolderredirection: *mut windows_core::BOOL, pbinherit: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesPinInfo_Impl::IsPinnedForFolderRedirection(this, core::mem::transmute_copy(&pbpinnedforfolderredirection), core::mem::transmute_copy(&pbinherit)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPinned: IsPinned::<Identity, OFFSET>,
            IsPinnedForUser: IsPinnedForUser::<Identity, OFFSET>,
            IsPinnedForUserByPolicy: IsPinnedForUserByPolicy::<Identity, OFFSET>,
            IsPinnedForComputer: IsPinnedForComputer::<Identity, OFFSET>,
            IsPinnedForFolderRedirection: IsPinnedForFolderRedirection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesPinInfo {}
windows_core::imp::define_interface!(IOfflineFilesPinInfo2, IOfflineFilesPinInfo2_Vtbl, 0x623c58a2_42ed_4ad7_b69a_0f1b30a72d0d);
impl core::ops::Deref for IOfflineFilesPinInfo2 {
    type Target = IOfflineFilesPinInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesPinInfo2, windows_core::IUnknown, IOfflineFilesPinInfo);
impl IOfflineFilesPinInfo2 {
    pub unsafe fn IsPartlyPinned(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPartlyPinned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo2_Vtbl {
    pub base__: IOfflineFilesPinInfo_Vtbl,
    pub IsPartlyPinned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesPinInfo2_Impl: IOfflineFilesPinInfo_Impl {
    fn IsPartlyPinned(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesPinInfo2_Vtbl {
    pub const fn new<Identity: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPartlyPinned<Identity: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpartlypinned: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesPinInfo2_Impl::IsPartlyPinned(this) {
                    Ok(ok__) => {
                        pbpartlypinned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IOfflineFilesPinInfo_Vtbl::new::<Identity, OFFSET>(), IsPartlyPinned: IsPartlyPinned::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo2 as windows_core::Interface>::IID || iid == &<IOfflineFilesPinInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesPinInfo2 {}
windows_core::imp::define_interface!(IOfflineFilesProgress, IOfflineFilesProgress_Vtbl, 0xfad63237_c55b_4911_9850_bcf96d4c979e);
windows_core::imp::interface_hierarchy!(IOfflineFilesProgress, windows_core::IUnknown);
impl IOfflineFilesProgress {
    pub unsafe fn Begin(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryAbort(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryAbort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn End(&self, hrresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self), hrresult).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub QueryAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IOfflineFilesProgress_Impl: windows_core::IUnknownImpl {
    fn Begin(&self) -> windows_core::Result<windows_core::BOOL>;
    fn QueryAbort(&self) -> windows_core::Result<windows_core::BOOL>;
    fn End(&self, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IOfflineFilesProgress_Vtbl {
    pub const fn new<Identity: IOfflineFilesProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbabort: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesProgress_Impl::Begin(this) {
                    Ok(ok__) => {
                        pbabort.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryAbort<Identity: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbabort: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesProgress_Impl::QueryAbort(this) {
                    Ok(ok__) => {
                        pbabort.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn End<Identity: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesProgress_Impl::End(this, core::mem::transmute_copy(&hrresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, OFFSET>,
            QueryAbort: QueryAbort::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesProgress {}
windows_core::imp::define_interface!(IOfflineFilesServerItem, IOfflineFilesServerItem_Vtbl, 0x9b1c9576_a92b_4151_8e9e_7c7b3ec2e016);
impl core::ops::Deref for IOfflineFilesServerItem {
    type Target = IOfflineFilesItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesServerItem, windows_core::IUnknown, IOfflineFilesItem);
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesServerItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
pub trait IOfflineFilesServerItem_Impl: IOfflineFilesItem_Impl {}
impl IOfflineFilesServerItem_Vtbl {
    pub const fn new<Identity: IOfflineFilesServerItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesServerItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesServerItem {}
windows_core::imp::define_interface!(IOfflineFilesSetting, IOfflineFilesSetting_Vtbl, 0xd871d3f7_f613_48a1_827e_7a34e560fff6);
windows_core::imp::interface_hierarchy!(IOfflineFilesSetting, windows_core::IUnknown);
impl IOfflineFilesSetting {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValueType(&self) -> windows_core::Result<OFFLINEFILES_SETTING_VALUE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPreference(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPreference)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarvalue), dwscope).ok() }
    }
    pub unsafe fn GetPreferenceScope(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreferenceScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPreference(&self, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreference)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarvalue), dwscope).ok() }
    }
    pub unsafe fn DeletePreference(&self, dwscope: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePreference)(windows_core::Interface::as_raw(self), dwscope).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPolicy(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarvalue), dwscope).ok() }
    }
    pub unsafe fn GetPolicyScope(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPolicyScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarvalue), pbsetbypolicy as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSetting_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPreference: usize,
    pub GetPreferenceScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPreference: usize,
    pub DeletePreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPolicy: usize,
    pub GetPolicyScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOfflineFilesSetting_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetValueType(&self) -> windows_core::Result<OFFLINEFILES_SETTING_VALUE_TYPE>;
    fn GetPreference(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn GetPreferenceScope(&self) -> windows_core::Result<u32>;
    fn SetPreference(&self, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn DeletePreference(&self, dwscope: u32) -> windows_core::Result<()>;
    fn GetPolicy(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::Result<()>;
    fn GetPolicyScope(&self) -> windows_core::Result<u32>;
    fn GetValue(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOfflineFilesSetting_Vtbl {
    pub const fn new<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSetting_Impl::GetName(this) {
                    Ok(ok__) => {
                        ppszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValueType<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSetting_Impl::GetValueType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreference<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSetting_Impl::GetPreference(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
            }
        }
        unsafe extern "system" fn GetPreferenceScope<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscope: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSetting_Impl::GetPreferenceScope(this) {
                    Ok(ok__) => {
                        pdwscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreference<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSetting_Impl::SetPreference(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
            }
        }
        unsafe extern "system" fn DeletePreference<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwscope: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSetting_Impl::DeletePreference(this, core::mem::transmute_copy(&dwscope)).into()
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSetting_Impl::GetPolicy(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&dwscope)).into()
            }
        }
        unsafe extern "system" fn GetPolicyScope<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwscope: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSetting_Impl::GetPolicyScope(this) {
                    Ok(ok__) => {
                        pdwscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSetting_Impl::GetValue(this, core::mem::transmute_copy(&pvarvalue), core::mem::transmute_copy(&pbsetbypolicy)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetValueType: GetValueType::<Identity, OFFSET>,
            GetPreference: GetPreference::<Identity, OFFSET>,
            GetPreferenceScope: GetPreferenceScope::<Identity, OFFSET>,
            SetPreference: SetPreference::<Identity, OFFSET>,
            DeletePreference: DeletePreference::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            GetPolicyScope: GetPolicyScope::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSetting as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOfflineFilesSetting {}
windows_core::imp::define_interface!(IOfflineFilesShareInfo, IOfflineFilesShareInfo_Vtbl, 0x7bcc43e7_31ce_4ca4_8ccd_1cff2dc494da);
windows_core::imp::interface_hierarchy!(IOfflineFilesShareInfo, windows_core::IUnknown);
impl IOfflineFilesShareInfo {
    pub unsafe fn GetShareItem(&self) -> windows_core::Result<IOfflineFilesShareItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShareItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetShareCachingMode(&self) -> windows_core::Result<OFFLINEFILES_CACHING_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShareCachingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsShareDfsJunction(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsShareDfsJunction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetShareItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetShareCachingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT,
    pub IsShareDfsJunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesShareInfo_Impl: windows_core::IUnknownImpl {
    fn GetShareItem(&self) -> windows_core::Result<IOfflineFilesShareItem>;
    fn GetShareCachingMode(&self) -> windows_core::Result<OFFLINEFILES_CACHING_MODE>;
    fn IsShareDfsJunction(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesShareInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesShareInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetShareItem<Identity: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshareitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesShareInfo_Impl::GetShareItem(this) {
                    Ok(ok__) => {
                        ppshareitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetShareCachingMode<Identity: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesShareInfo_Impl::GetShareCachingMode(this) {
                    Ok(ok__) => {
                        pcachingmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsShareDfsJunction<Identity: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisdfsjunction: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesShareInfo_Impl::IsShareDfsJunction(this) {
                    Ok(ok__) => {
                        pbisdfsjunction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetShareItem: GetShareItem::<Identity, OFFSET>,
            GetShareCachingMode: GetShareCachingMode::<Identity, OFFSET>,
            IsShareDfsJunction: IsShareDfsJunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesShareInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesShareInfo {}
windows_core::imp::define_interface!(IOfflineFilesShareItem, IOfflineFilesShareItem_Vtbl, 0xbab7e48d_4804_41b5_a44d_0f199b06b145);
impl core::ops::Deref for IOfflineFilesShareItem {
    type Target = IOfflineFilesItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesShareItem, windows_core::IUnknown, IOfflineFilesItem);
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
pub trait IOfflineFilesShareItem_Impl: IOfflineFilesItem_Impl {}
impl IOfflineFilesShareItem_Vtbl {
    pub const fn new<Identity: IOfflineFilesShareItem_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IOfflineFilesItem_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesShareItem as windows_core::Interface>::IID || iid == &<IOfflineFilesItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesShareItem {}
windows_core::imp::define_interface!(IOfflineFilesSimpleProgress, IOfflineFilesSimpleProgress_Vtbl, 0xc34f7f9b_c43d_4f9d_a776_c0eb6de5d401);
impl core::ops::Deref for IOfflineFilesSimpleProgress {
    type Target = IOfflineFilesProgress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesSimpleProgress, windows_core::IUnknown, IOfflineFilesProgress);
impl IOfflineFilesSimpleProgress {
    pub unsafe fn ItemBegin<P0>(&self, pszfile: P0) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemBegin)(windows_core::Interface::as_raw(self), pszfile.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ItemResult<P0>(&self, pszfile: P0, hrresult: windows_core::HRESULT) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemResult)(windows_core::Interface::as_raw(self), pszfile.param().abi(), hrresult, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSimpleProgress_Vtbl {
    pub base__: IOfflineFilesProgress_Vtbl,
    pub ItemBegin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT,
    pub ItemResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT, *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSimpleProgress_Impl: IOfflineFilesProgress_Impl {
    fn ItemBegin(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn ItemResult(&self, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
impl IOfflineFilesSimpleProgress_Vtbl {
    pub const fn new<Identity: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ItemBegin<Identity: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSimpleProgress_Impl::ItemBegin(this, core::mem::transmute(&pszfile)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemResult<Identity: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSimpleProgress_Impl::ItemResult(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOfflineFilesProgress_Vtbl::new::<Identity, OFFSET>(),
            ItemBegin: ItemBegin::<Identity, OFFSET>,
            ItemResult: ItemResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSimpleProgress as windows_core::Interface>::IID || iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSimpleProgress {}
windows_core::imp::define_interface!(IOfflineFilesSuspend, IOfflineFilesSuspend_Vtbl, 0x62c4560f_bc0b_48ca_ad9d_34cb528d99a9);
windows_core::imp::interface_hierarchy!(IOfflineFilesSuspend, windows_core::IUnknown);
impl IOfflineFilesSuspend {
    pub unsafe fn SuspendRoot(&self, bsuspend: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspendRoot)(windows_core::Interface::as_raw(self), bsuspend.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspend_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SuspendRoot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSuspend_Impl: windows_core::IUnknownImpl {
    fn SuspendRoot(&self, bsuspend: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IOfflineFilesSuspend_Vtbl {
    pub const fn new<Identity: IOfflineFilesSuspend_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SuspendRoot<Identity: IOfflineFilesSuspend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsuspend: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSuspend_Impl::SuspendRoot(this, core::mem::transmute_copy(&bsuspend)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SuspendRoot: SuspendRoot::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSuspend as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSuspend {}
windows_core::imp::define_interface!(IOfflineFilesSuspendInfo, IOfflineFilesSuspendInfo_Vtbl, 0xa457c25b_4e9c_4b04_85af_8932ccd97889);
windows_core::imp::interface_hierarchy!(IOfflineFilesSuspendInfo, windows_core::IUnknown);
impl IOfflineFilesSuspendInfo {
    pub unsafe fn IsSuspended(&self, pbsuspended: *mut windows_core::BOOL, pbsuspendedroot: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsSuspended)(windows_core::Interface::as_raw(self), pbsuspended as _, pbsuspendedroot as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspendInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSuspendInfo_Impl: windows_core::IUnknownImpl {
    fn IsSuspended(&self, pbsuspended: *mut windows_core::BOOL, pbsuspendedroot: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IOfflineFilesSuspendInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSuspended<Identity: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsuspended: *mut windows_core::BOOL, pbsuspendedroot: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSuspendInfo_Impl::IsSuspended(this, core::mem::transmute_copy(&pbsuspended), core::mem::transmute_copy(&pbsuspendedroot)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSuspended: IsSuspended::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSuspendInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSuspendInfo {}
windows_core::imp::define_interface!(IOfflineFilesSyncConflictHandler, IOfflineFilesSyncConflictHandler_Vtbl, 0xb6dd5092_c65c_46b6_97b8_fadd08e7e1be);
windows_core::imp::interface_hierarchy!(IOfflineFilesSyncConflictHandler, windows_core::IUnknown);
impl IOfflineFilesSyncConflictHandler {
    pub unsafe fn ResolveConflict<P0>(&self, pszpath: P0, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut windows_core::PWSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveConflict)(windows_core::Interface::as_raw(self), pszpath.param().abi(), fstateknown, state, fchangedetails, pconflictresolution as _, ppsznewname as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncConflictHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResolveConflict: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, OFFLINEFILES_SYNC_STATE, u32, *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSyncConflictHandler_Impl: windows_core::IUnknownImpl {
    fn ResolveConflict(&self, pszpath: &windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IOfflineFilesSyncConflictHandler_Vtbl {
    pub const fn new<Identity: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResolveConflict<Identity: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSyncConflictHandler_Impl::ResolveConflict(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&fstateknown), core::mem::transmute_copy(&state), core::mem::transmute_copy(&fchangedetails), core::mem::transmute_copy(&pconflictresolution), core::mem::transmute_copy(&ppsznewname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResolveConflict: ResolveConflict::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncConflictHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSyncConflictHandler {}
windows_core::imp::define_interface!(IOfflineFilesSyncErrorInfo, IOfflineFilesSyncErrorInfo_Vtbl, 0x59f95e46_eb54_49d1_be76_de95458d01b0);
impl core::ops::Deref for IOfflineFilesSyncErrorInfo {
    type Target = IOfflineFilesErrorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesSyncErrorInfo, windows_core::IUnknown, IOfflineFilesErrorInfo);
impl IOfflineFilesSyncErrorInfo {
    pub unsafe fn GetSyncOperation(&self) -> windows_core::Result<OFFLINEFILES_SYNC_OPERATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSyncOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemChangeFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemChangeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InfoEnumerated(&self, pblocalenumerated: *mut windows_core::BOOL, pbremoteenumerated: *mut windows_core::BOOL, pboriginalenumerated: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InfoEnumerated)(windows_core::Interface::as_raw(self), pblocalenumerated as _, pbremoteenumerated as _, pboriginalenumerated as _).ok() }
    }
    pub unsafe fn InfoAvailable(&self, pblocalinfo: *mut windows_core::BOOL, pbremoteinfo: *mut windows_core::BOOL, pboriginalinfo: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InfoAvailable)(windows_core::Interface::as_raw(self), pblocalinfo as _, pbremoteinfo as _, pboriginalinfo as _).ok() }
    }
    pub unsafe fn GetLocalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRemoteInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOriginalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginalInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorInfo_Vtbl {
    pub base__: IOfflineFilesErrorInfo_Vtbl,
    pub GetSyncOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OFFLINEFILES_SYNC_OPERATION) -> windows_core::HRESULT,
    pub GetItemChangeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InfoEnumerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub InfoAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLocalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOriginalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOfflineFilesSyncErrorInfo_Impl: IOfflineFilesErrorInfo_Impl {
    fn GetSyncOperation(&self) -> windows_core::Result<OFFLINEFILES_SYNC_OPERATION>;
    fn GetItemChangeFlags(&self) -> windows_core::Result<u32>;
    fn InfoEnumerated(&self, pblocalenumerated: *mut windows_core::BOOL, pbremoteenumerated: *mut windows_core::BOOL, pboriginalenumerated: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn InfoAvailable(&self, pblocalinfo: *mut windows_core::BOOL, pbremoteinfo: *mut windows_core::BOOL, pboriginalinfo: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLocalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetRemoteInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetOriginalInfo(&self) -> windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOfflineFilesSyncErrorInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSyncOperation<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorInfo_Impl::GetSyncOperation(this) {
                    Ok(ok__) => {
                        psyncop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemChangeFlags<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitemchangeflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorInfo_Impl::GetItemChangeFlags(this) {
                    Ok(ok__) => {
                        pdwitemchangeflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InfoEnumerated<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocalenumerated: *mut windows_core::BOOL, pbremoteenumerated: *mut windows_core::BOOL, pboriginalenumerated: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSyncErrorInfo_Impl::InfoEnumerated(this, core::mem::transmute_copy(&pblocalenumerated), core::mem::transmute_copy(&pbremoteenumerated), core::mem::transmute_copy(&pboriginalenumerated)).into()
            }
        }
        unsafe extern "system" fn InfoAvailable<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblocalinfo: *mut windows_core::BOOL, pbremoteinfo: *mut windows_core::BOOL, pboriginalinfo: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSyncErrorInfo_Impl::InfoAvailable(this, core::mem::transmute_copy(&pblocalinfo), core::mem::transmute_copy(&pbremoteinfo), core::mem::transmute_copy(&pboriginalinfo)).into()
            }
        }
        unsafe extern "system" fn GetLocalInfo<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorInfo_Impl::GetLocalInfo(this) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteInfo<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorInfo_Impl::GetRemoteInfo(this) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginalInfo<Identity: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorInfo_Impl::GetOriginalInfo(this) {
                    Ok(ok__) => {
                        ppinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOfflineFilesErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetSyncOperation: GetSyncOperation::<Identity, OFFSET>,
            GetItemChangeFlags: GetItemChangeFlags::<Identity, OFFSET>,
            InfoEnumerated: InfoEnumerated::<Identity, OFFSET>,
            InfoAvailable: InfoAvailable::<Identity, OFFSET>,
            GetLocalInfo: GetLocalInfo::<Identity, OFFSET>,
            GetRemoteInfo: GetRemoteInfo::<Identity, OFFSET>,
            GetOriginalInfo: GetOriginalInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorInfo as windows_core::Interface>::IID || iid == &<IOfflineFilesErrorInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOfflineFilesSyncErrorInfo {}
windows_core::imp::define_interface!(IOfflineFilesSyncErrorItemInfo, IOfflineFilesSyncErrorItemInfo_Vtbl, 0xecdbaf0d_6a18_4d55_8017_108f7660ba44);
windows_core::imp::interface_hierarchy!(IOfflineFilesSyncErrorItemInfo, windows_core::IUnknown);
impl IOfflineFilesSyncErrorItemInfo {
    pub unsafe fn GetFileAttributes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileTimes(&self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFileTimes)(windows_core::Interface::as_raw(self), pftlastwrite as _, pftchange as _).ok() }
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorItemInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFileTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSyncErrorItemInfo_Impl: windows_core::IUnknownImpl {
    fn GetFileAttributes(&self) -> windows_core::Result<u32>;
    fn GetFileTimes(&self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetFileSize(&self) -> windows_core::Result<i64>;
}
impl IOfflineFilesSyncErrorItemInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileAttributes<Identity: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwattributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorItemInfo_Impl::GetFileAttributes(this) {
                    Ok(ok__) => {
                        pdwattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileTimes<Identity: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOfflineFilesSyncErrorItemInfo_Impl::GetFileTimes(this, core::mem::transmute_copy(&pftlastwrite), core::mem::transmute_copy(&pftchange)).into()
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncErrorItemInfo_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        psize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileAttributes: GetFileAttributes::<Identity, OFFSET>,
            GetFileTimes: GetFileTimes::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorItemInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSyncErrorItemInfo {}
windows_core::imp::define_interface!(IOfflineFilesSyncProgress, IOfflineFilesSyncProgress_Vtbl, 0x6931f49a_6fc7_4c1b_b265_56793fc451b7);
impl core::ops::Deref for IOfflineFilesSyncProgress {
    type Target = IOfflineFilesProgress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOfflineFilesSyncProgress, windows_core::IUnknown, IOfflineFilesProgress);
impl IOfflineFilesSyncProgress {
    pub unsafe fn SyncItemBegin<P0>(&self, pszfile: P0) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SyncItemBegin)(windows_core::Interface::as_raw(self), pszfile.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SyncItemResult<P0, P2>(&self, pszfile: P0, hrresult: windows_core::HRESULT, perrorinfo: P2) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IOfflineFilesSyncErrorInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SyncItemResult)(windows_core::Interface::as_raw(self), pszfile.param().abi(), hrresult, perrorinfo.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncProgress_Vtbl {
    pub base__: IOfflineFilesProgress_Vtbl,
    pub SyncItemBegin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT,
    pub SyncItemResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT, *mut core::ffi::c_void, *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT,
}
pub trait IOfflineFilesSyncProgress_Impl: IOfflineFilesProgress_Impl {
    fn SyncItemBegin(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn SyncItemResult(&self, pszfile: &windows_core::PCWSTR, hrresult: windows_core::HRESULT, perrorinfo: windows_core::Ref<IOfflineFilesSyncErrorInfo>) -> windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
impl IOfflineFilesSyncProgress_Vtbl {
    pub const fn new<Identity: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SyncItemBegin<Identity: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncProgress_Impl::SyncItemBegin(this, core::mem::transmute(&pszfile)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SyncItemResult<Identity: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR, hrresult: windows_core::HRESULT, perrorinfo: *mut core::ffi::c_void, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesSyncProgress_Impl::SyncItemResult(this, core::mem::transmute(&pszfile), core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&perrorinfo)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IOfflineFilesProgress_Vtbl::new::<Identity, OFFSET>(),
            SyncItemBegin: SyncItemBegin::<Identity, OFFSET>,
            SyncItemResult: SyncItemResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesSyncProgress as windows_core::Interface>::IID || iid == &<IOfflineFilesProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesSyncProgress {}
windows_core::imp::define_interface!(IOfflineFilesTransparentCacheInfo, IOfflineFilesTransparentCacheInfo_Vtbl, 0xbcaf4a01_5b68_4b56_a6a1_8d2786ede8e3);
windows_core::imp::interface_hierarchy!(IOfflineFilesTransparentCacheInfo, windows_core::IUnknown);
impl IOfflineFilesTransparentCacheInfo {
    pub unsafe fn IsTransparentlyCached(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTransparentlyCached)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesTransparentCacheInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsTransparentlyCached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IOfflineFilesTransparentCacheInfo_Impl: windows_core::IUnknownImpl {
    fn IsTransparentlyCached(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IOfflineFilesTransparentCacheInfo_Vtbl {
    pub const fn new<Identity: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTransparentlyCached<Identity: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtransparentlycached: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOfflineFilesTransparentCacheInfo_Impl::IsTransparentlyCached(this) {
                    Ok(ok__) => {
                        pbtransparentlycached.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsTransparentlyCached: IsTransparentlyCached::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOfflineFilesTransparentCacheInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOfflineFilesTransparentCacheInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_CACHING_MODE(pub i32);
pub const OFFLINEFILES_CACHING_MODE_AUTO_DOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(3i32);
pub const OFFLINEFILES_CACHING_MODE_AUTO_PROGANDDOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(4i32);
pub const OFFLINEFILES_CACHING_MODE_MANUAL: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(2i32);
pub const OFFLINEFILES_CACHING_MODE_NOCACHING: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(1i32);
pub const OFFLINEFILES_CACHING_MODE_NONE: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(0i32);
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_COMPARE(pub i32);
pub const OFFLINEFILES_COMPARE_EQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(0i32);
pub const OFFLINEFILES_COMPARE_GT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(3i32);
pub const OFFLINEFILES_COMPARE_GTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(5i32);
pub const OFFLINEFILES_COMPARE_LT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(2i32);
pub const OFFLINEFILES_COMPARE_LTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(4i32);
pub const OFFLINEFILES_COMPARE_NEQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_CONNECT_STATE(pub i32);
pub const OFFLINEFILES_CONNECT_STATE_OFFLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(1i32);
pub const OFFLINEFILES_CONNECT_STATE_ONLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(2i32);
pub const OFFLINEFILES_CONNECT_STATE_PARTLY_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(4i32);
pub const OFFLINEFILES_CONNECT_STATE_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(3i32);
pub const OFFLINEFILES_CONNECT_STATE_UNKNOWN: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(0i32);
pub const OFFLINEFILES_DELETE_FLAG_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_DELETE_FLAG_DELMODIFIED: u32 = 4u32;
pub const OFFLINEFILES_DELETE_FLAG_NOAUTOCACHED: u32 = 1u32;
pub const OFFLINEFILES_DELETE_FLAG_NOPINNED: u32 = 2u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_ENUM_FLAT: u32 = 1u32;
pub const OFFLINEFILES_ENUM_FLAT_FILESONLY: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_EVENTS(pub i32);
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(11i32);
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(12i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(29i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(30i32);
pub const OFFLINEFILES_EVENT_CACHEISCORRUPTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(2i32);
pub const OFFLINEFILES_EVENT_CACHEISFULL: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(1i32);
pub const OFFLINEFILES_EVENT_CACHEMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(0i32);
pub const OFFLINEFILES_EVENT_DATALOST: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(25i32);
pub const OFFLINEFILES_EVENT_ENABLED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(3i32);
pub const OFFLINEFILES_EVENT_ENCRYPTIONCHANGED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(4i32);
pub const OFFLINEFILES_EVENT_ITEMADDEDTOCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(22i32);
pub const OFFLINEFILES_EVENT_ITEMAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(17i32);
pub const OFFLINEFILES_EVENT_ITEMDELETEDFROMCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(23i32);
pub const OFFLINEFILES_EVENT_ITEMDISCONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(15i32);
pub const OFFLINEFILES_EVENT_ITEMMODIFIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(21i32);
pub const OFFLINEFILES_EVENT_ITEMNOTAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(18i32);
pub const OFFLINEFILES_EVENT_ITEMNOTPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(20i32);
pub const OFFLINEFILES_EVENT_ITEMPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(19i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(27i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(16i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(28i32);
pub const OFFLINEFILES_EVENT_ITEMRENAMED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(24i32);
pub const OFFLINEFILES_EVENT_NETTRANSPORTARRIVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(13i32);
pub const OFFLINEFILES_EVENT_NONETTRANSPORTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(14i32);
pub const OFFLINEFILES_EVENT_PING: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(26i32);
pub const OFFLINEFILES_EVENT_POLICYCHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(31i32);
pub const OFFLINEFILES_EVENT_PREFERENCECHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(32i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(37i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(38i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(35i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(36i32);
pub const OFFLINEFILES_EVENT_SETTINGSCHANGESAPPLIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(33i32);
pub const OFFLINEFILES_EVENT_SYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(5i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECADDED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(7i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECREMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(9i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECUPDATED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(8i32);
pub const OFFLINEFILES_EVENT_SYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(10i32);
pub const OFFLINEFILES_EVENT_SYNCFILERESULT: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(6i32);
pub const OFFLINEFILES_EVENT_TRANSPARENTCACHEITEMNOTIFY: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(34i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_ITEM_COPY(pub i32);
pub const OFFLINEFILES_ITEM_COPY_LOCAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(0i32);
pub const OFFLINEFILES_ITEM_COPY_ORIGINAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(2i32);
pub const OFFLINEFILES_ITEM_COPY_REMOTE: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(1i32);
pub const OFFLINEFILES_ITEM_FILTER_FLAG_CREATED: u32 = 8u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DELETED: u32 = 16u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRECTORY: u32 = 256u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRTY: u32 = 32u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_FILE: u32 = 128u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GHOST: u32 = 8192u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_ANYACCESS: u32 = 33554432u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_READ: u32 = 16777216u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_WRITE: u32 = 8388608u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED: u32 = 4u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_DATA: u32 = 1u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OFFLINE: u32 = 32768u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_ONLINE: u32 = 65536u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_ANYACCESS: u32 = 4194304u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_READ: u32 = 2097152u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_WRITE: u32 = 1048576u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED: u32 = 4096u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_COMPUTER: u32 = 2048u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_OTHERS: u32 = 1024u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_USER: u32 = 512u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SPARSE: u32 = 64u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SUSPENDED: u32 = 16384u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_ANYACCESS: u32 = 524288u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_READ: u32 = 262144u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_WRITE: u32 = 131072u32;
pub const OFFLINEFILES_ITEM_QUERY_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_ITEM_QUERY_ATTEMPT_TRANSITIONONLINE: u32 = 32u32;
pub const OFFLINEFILES_ITEM_QUERY_CONNECTIONSTATE: u32 = 2u32;
pub const OFFLINEFILES_ITEM_QUERY_INCLUDETRANSPARENTCACHE: u32 = 16u32;
pub const OFFLINEFILES_ITEM_QUERY_LOCALDIRTYBYTECOUNT: u32 = 4u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEDIRTYBYTECOUNT: u32 = 8u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEINFO: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_ITEM_TIME(pub i32);
pub const OFFLINEFILES_ITEM_TIME_CREATION: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(0i32);
pub const OFFLINEFILES_ITEM_TIME_LASTACCESS: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(1i32);
pub const OFFLINEFILES_ITEM_TIME_LASTWRITE: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_ITEM_TYPE(pub i32);
pub const OFFLINEFILES_ITEM_TYPE_DIRECTORY: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(1i32);
pub const OFFLINEFILES_ITEM_TYPE_FILE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(0i32);
pub const OFFLINEFILES_ITEM_TYPE_SERVER: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(3i32);
pub const OFFLINEFILES_ITEM_TYPE_SHARE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(2i32);
pub const OFFLINEFILES_NUM_EVENTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(39i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_OFFLINE_REASON(pub i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_ERROR: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(4i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_FORCED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(2i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_SLOW: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(3i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_SUSPENDED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(6i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_VERSION_CONFLICT: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(5i32);
pub const OFFLINEFILES_OFFLINE_REASON_NOT_APPLICABLE: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(1i32);
pub const OFFLINEFILES_OFFLINE_REASON_UNKNOWN: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(0i32);
pub const OFFLINEFILES_OP_ABORT: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(2i32);
pub const OFFLINEFILES_OP_CONTINUE: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_OP_RESPONSE(pub i32);
pub const OFFLINEFILES_OP_RETRY: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(1i32);
pub const OFFLINEFILES_PATHFILTER_CHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(1i32);
pub const OFFLINEFILES_PATHFILTER_DESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_PATHFILTER_MATCH(pub i32);
pub const OFFLINEFILES_PATHFILTER_SELF: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(0i32);
pub const OFFLINEFILES_PATHFILTER_SELFORCHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(3i32);
pub const OFFLINEFILES_PATHFILTER_SELFORDESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(4i32);
pub const OFFLINEFILES_PINLINKTARGETS_ALWAYS: u32 = 2u32;
pub const OFFLINEFILES_PINLINKTARGETS_EXPLICIT: u32 = 1u32;
pub const OFFLINEFILES_PINLINKTARGETS_NEVER: u32 = 0u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FILL: u32 = 1u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORALL: u32 = 128u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORREDIR: u32 = 256u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER: u32 = 32u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SETTING_PinLinkTargets: windows_core::PCWSTR = windows_core::w!("LinkTargetCaching");
pub const OFFLINEFILES_SETTING_SCOPE_COMPUTER: u32 = 2u32;
pub const OFFLINEFILES_SETTING_SCOPE_USER: u32 = 1u32;
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(4i32);
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(3i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(1i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR_DBLNULTERM: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_SETTING_VALUE_TYPE(pub i32);
pub const OFFLINEFILES_SETTING_VALUE_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(0i32);
pub const OFFLINEFILES_SYNC_CONFLICT_ABORT: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_SYNC_CONFLICT_RESOLVE(pub i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPALLCHANGES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(3i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLATEST: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(4i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLOCAL: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(1i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPREMOTE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(2i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_LOG: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(5i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NONE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(0i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NUMCODES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(8i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_SKIP: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(6i32);
pub const OFFLINEFILES_SYNC_CONTROL_CR_DEFAULT: u32 = 0u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLATEST: u32 = 805306368u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLOCAL: u32 = 268435456u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPREMOTE: u32 = 536870912u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_MASK: u32 = 4026531840u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_FILLSPARSE: u32 = 1u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_NONEWFILESOUT: u32 = 131072u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORALL: u32 = 128u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORREDIR: u32 = 256u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER: u32 = 32u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINNEWFILES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SKIPSUSPENDEDDIRS: u32 = 8192u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCIN: u32 = 2u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCOUT: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_ATTRIBUTES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_CHANGETIME: u32 = 1u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_FILESIZE: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_NONE: u32 = 0u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_WRITETIME: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_SYNC_OPERATION(pub i32);
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(1i32);
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(0i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_CLIENT_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(5i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_SERVER_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(4i32);
pub const OFFLINEFILES_SYNC_OPERATION_PIN: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(6i32);
pub const OFFLINEFILES_SYNC_OPERATION_PREPARE: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(7i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(3i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFLINEFILES_SYNC_STATE(pub i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(37i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(35i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(36i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(34i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(42i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(28i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(29i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(27i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(26i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(47i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(25i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(24i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(21i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(23i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(22i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(20i32);
pub const OFFLINEFILES_SYNC_STATE_DirDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(49i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(4i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(3i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(5i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(43i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(48i32);
pub const OFFLINEFILES_SYNC_STATE_DirSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(41i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(39i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(12i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(15i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(14i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(13i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(44i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(11i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(8i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(10i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(7i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(9i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(6i32);
pub const OFFLINEFILES_SYNC_STATE_FileDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(46i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(1i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(2i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(40i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(45i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(53i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(52i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(51i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(50i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(38i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(16i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(17i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(19i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(18i32);
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
pub const OFFLINEFILES_SYNC_STATE_NUMSTATES: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(54i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(33i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(31i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(32i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(30i32);
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
pub const OFFLINEFILES_SYNC_STATE_Stable: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(0i32);
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub const OfflineFilesCache: windows_core::GUID = windows_core::GUID::from_u128(0x48c6be7c_3871_43cc_b46f_1449a1bb2ff3);
pub const OfflineFilesSetting: windows_core::GUID = windows_core::GUID::from_u128(0xfd3659e9_a920_4123_ad64_7fc76c7aacdf);
