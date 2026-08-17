pub const Catalog: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogCollection: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogObject: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
pub const ComponentUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICatalog, ICatalog_Vtbl, 0x6eb22870_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICatalog {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICatalog, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICatalog {
    pub unsafe fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Connect(&self, bstrconnectstring: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrconnectstring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), retval as _).ok() }
    }
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), retval as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalog_Impl: super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrconnectstring: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self, retval: *mut i32) -> windows_core::Result<()>;
    fn MinorVersion(&self, retval: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalog_Vtbl {
    pub const fn new<Identity: ICatalog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCollection<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrconnectstring: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalog_Impl::Connect(this, core::mem::transmute(&bstrconnectstring)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalog_Impl::MajorVersion(this, core::mem::transmute_copy(&retval)).into()
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ICatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalog_Impl::MinorVersion(this, core::mem::transmute_copy(&retval)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetCollection: GetCollection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalog as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICatalog {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IComponentUtil, IComponentUtil_Vtbl, 0x6eb22873_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IComponentUtil {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IComponentUtil, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IComponentUtil {
    pub unsafe fn InstallComponent(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, bstrproxystubdllfile: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdllfile), core::mem::transmute_copy(bstrtypelibfile), core::mem::transmute_copy(bstrproxystubdllfile)).ok() }
    }
    pub unsafe fn ImportComponent(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsid)).ok() }
    }
    pub unsafe fn ImportComponentByName(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportComponentByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprogid)).ok() }
    }
    pub unsafe fn GetCLSIDs(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCLSIDs)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdllfile), core::mem::transmute_copy(bstrtypelibfile), aclsids as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportComponentByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCLSIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComponentUtil_Impl: super::Com::IDispatch_Impl {
    fn InstallComponent(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, bstrproxystubdllfile: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportComponent(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportComponentByName(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCLSIDs(&self, bstrdllfile: &windows_core::BSTR, bstrtypelibfile: &windows_core::BSTR, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComponentUtil_Vtbl {
    pub const fn new<Identity: IComponentUtil_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallComponent<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllfile: *mut core::ffi::c_void, bstrtypelibfile: *mut core::ffi::c_void, bstrproxystubdllfile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentUtil_Impl::InstallComponent(this, core::mem::transmute(&bstrdllfile), core::mem::transmute(&bstrtypelibfile), core::mem::transmute(&bstrproxystubdllfile)).into()
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentUtil_Impl::ImportComponent(this, core::mem::transmute(&bstrclsid)).into()
            }
        }
        unsafe extern "system" fn ImportComponentByName<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprogid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentUtil_Impl::ImportComponentByName(this, core::mem::transmute(&bstrprogid)).into()
            }
        }
        unsafe extern "system" fn GetCLSIDs<Identity: IComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllfile: *mut core::ffi::c_void, bstrtypelibfile: *mut core::ffi::c_void, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComponentUtil_Impl::GetCLSIDs(this, core::mem::transmute(&bstrdllfile), core::mem::transmute(&bstrtypelibfile), core::mem::transmute_copy(&aclsids)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallComponent: InstallComponent::<Identity, OFFSET>,
            ImportComponent: ImportComponent::<Identity, OFFSET>,
            ImportComponentByName: ImportComponentByName::<Identity, OFFSET>,
            GetCLSIDs: GetCLSIDs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComponentUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IComponentUtil {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPackageUtil, IPackageUtil_Vtbl, 0x6eb22874_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPackageUtil {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPackageUtil, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPackageUtil {
    pub unsafe fn InstallPackage(&self, bstrpackagefile: &windows_core::BSTR, bstrinstallpath: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallPackage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpackagefile), core::mem::transmute_copy(bstrinstallpath), loptions).ok() }
    }
    pub unsafe fn ExportPackage(&self, bstrpackageid: &windows_core::BSTR, bstrpackagefile: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportPackage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpackageid), core::mem::transmute_copy(bstrpackagefile), loptions).ok() }
    }
    pub unsafe fn ShutdownPackage(&self, bstrpackageid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShutdownPackage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpackageid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ExportPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ShutdownPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPackageUtil_Impl: super::Com::IDispatch_Impl {
    fn InstallPackage(&self, bstrpackagefile: &windows_core::BSTR, bstrinstallpath: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn ExportPackage(&self, bstrpackageid: &windows_core::BSTR, bstrpackagefile: &windows_core::BSTR, loptions: i32) -> windows_core::Result<()>;
    fn ShutdownPackage(&self, bstrpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPackageUtil_Vtbl {
    pub const fn new<Identity: IPackageUtil_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackagefile: *mut core::ffi::c_void, bstrinstallpath: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPackageUtil_Impl::InstallPackage(this, core::mem::transmute(&bstrpackagefile), core::mem::transmute(&bstrinstallpath), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn ExportPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackageid: *mut core::ffi::c_void, bstrpackagefile: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPackageUtil_Impl::ExportPackage(this, core::mem::transmute(&bstrpackageid), core::mem::transmute(&bstrpackagefile), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn ShutdownPackage<Identity: IPackageUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpackageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPackageUtil_Impl::ShutdownPackage(this, core::mem::transmute(&bstrpackageid)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallPackage: InstallPackage::<Identity, OFFSET>,
            ExportPackage: ExportPackage::<Identity, OFFSET>,
            ShutdownPackage: ShutdownPackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPackageUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPackageUtil {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRemoteComponentUtil, IRemoteComponentUtil_Vtbl, 0x6eb22875_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRemoteComponentUtil {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRemoteComponentUtil, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteComponentUtil {
    pub unsafe fn InstallRemoteComponent(&self, bstrserver: &windows_core::BSTR, bstrpackageid: &windows_core::BSTR, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallRemoteComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserver), core::mem::transmute_copy(bstrpackageid), core::mem::transmute_copy(bstrclsid)).ok() }
    }
    pub unsafe fn InstallRemoteComponentByName(&self, bstrserver: &windows_core::BSTR, bstrpackagename: &windows_core::BSTR, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallRemoteComponentByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrserver), core::mem::transmute_copy(bstrpackagename), core::mem::transmute_copy(bstrprogid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallRemoteComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallRemoteComponentByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteComponentUtil_Impl: super::Com::IDispatch_Impl {
    fn InstallRemoteComponent(&self, bstrserver: &windows_core::BSTR, bstrpackageid: &windows_core::BSTR, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallRemoteComponentByName(&self, bstrserver: &windows_core::BSTR, bstrpackagename: &windows_core::BSTR, bstrprogid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteComponentUtil_Vtbl {
    pub const fn new<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallRemoteComponent<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserver: *mut core::ffi::c_void, bstrpackageid: *mut core::ffi::c_void, bstrclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteComponentUtil_Impl::InstallRemoteComponent(this, core::mem::transmute(&bstrserver), core::mem::transmute(&bstrpackageid), core::mem::transmute(&bstrclsid)).into()
            }
        }
        unsafe extern "system" fn InstallRemoteComponentByName<Identity: IRemoteComponentUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserver: *mut core::ffi::c_void, bstrpackagename: *mut core::ffi::c_void, bstrprogid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteComponentUtil_Impl::InstallRemoteComponentByName(this, core::mem::transmute(&bstrserver), core::mem::transmute(&bstrpackagename), core::mem::transmute(&bstrprogid)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InstallRemoteComponent: InstallRemoteComponent::<Identity, OFFSET>,
            InstallRemoteComponentByName: InstallRemoteComponentByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteComponentUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteComponentUtil {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRoleAssociationUtil, IRoleAssociationUtil_Vtbl, 0x6eb22876_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRoleAssociationUtil {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRoleAssociationUtil, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRoleAssociationUtil {
    pub unsafe fn AssociateRole(&self, bstrroleid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssociateRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrroleid)).ok() }
    }
    pub unsafe fn AssociateRoleByName(&self, bstrrolename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssociateRoleByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrolename)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRoleAssociationUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AssociateRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociateRoleByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRoleAssociationUtil_Impl: super::Com::IDispatch_Impl {
    fn AssociateRole(&self, bstrroleid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AssociateRoleByName(&self, bstrrolename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRoleAssociationUtil_Vtbl {
    pub const fn new<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssociateRole<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrroleid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRoleAssociationUtil_Impl::AssociateRole(this, core::mem::transmute(&bstrroleid)).into()
            }
        }
        unsafe extern "system" fn AssociateRoleByName<Identity: IRoleAssociationUtil_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrolename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRoleAssociationUtil_Impl::AssociateRoleByName(this, core::mem::transmute(&bstrrolename)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AssociateRole: AssociateRole::<Identity, OFFSET>,
            AssociateRoleByName: AssociateRoleByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoleAssociationUtil as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRoleAssociationUtil {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MTSAdminErrorCodes(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MTSPackageExportOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MTSPackageInstallOptions(pub i32);
pub const PackageUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
pub const mtsErrAlreadyInstalled: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368508i32);
pub const mtsErrAuthenticationLevel: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368493i32);
pub const mtsErrBadForward: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368479i32);
pub const mtsErrBadIID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368478i32);
pub const mtsErrBadPath: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368502i32);
pub const mtsErrBadRegistryLibID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368482i32);
pub const mtsErrBadRegistryProgID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368494i32);
pub const mtsErrCLSIDOrIIDMismatch: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368488i32);
pub const mtsErrCantCopyFile: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368499i32);
pub const mtsErrCoReqCompInstalled: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368496i32);
pub const mtsErrCompFileBadTLB: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368472i32);
pub const mtsErrCompFileClassNotAvail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368473i32);
pub const mtsErrCompFileDoesNotExist: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368476i32);
pub const mtsErrCompFileGetClassObj: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368474i32);
pub const mtsErrCompFileLoadDLLFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368475i32);
pub const mtsErrCompFileNoRegistrar: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368460i32);
pub const mtsErrCompFileNotInstallable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368471i32);
pub const mtsErrDllLoadFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368483i32);
pub const mtsErrDllRegisterServer: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368486i32);
pub const mtsErrDownloadFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368507i32);
pub const mtsErrInvalidUserids: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368496i32);
pub const mtsErrKeyMissing: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368509i32);
pub const mtsErrNoAccessToUNC: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368484i32);
pub const mtsErrNoRegistryCLSID: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368495i32);
pub const mtsErrNoRegistryRead: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368491i32);
pub const mtsErrNoRegistryRepair: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368489i32);
pub const mtsErrNoRegistryWrite: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368490i32);
pub const mtsErrNoServerShare: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368485i32);
pub const mtsErrNoTypeLib: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368498i32);
pub const mtsErrNoUser: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368497i32);
pub const mtsErrNotChangeable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368470i32);
pub const mtsErrNotDeletable: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368469i32);
pub const mtsErrObjectErrors: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368511i32);
pub const mtsErrObjectInvalid: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368510i32);
pub const mtsErrPDFReadFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368504i32);
pub const mtsErrPDFVersion: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368503i32);
pub const mtsErrPDFWriteFail: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368505i32);
pub const mtsErrPackDirNotFound: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368481i32);
pub const mtsErrPackageExists: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368501i32);
pub const mtsErrRegistrarFailed: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368477i32);
pub const mtsErrRemoteInterface: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368487i32);
pub const mtsErrRoleExists: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368500i32);
pub const mtsErrSession: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368468i32);
pub const mtsErrTreatAs: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368480i32);
pub const mtsErrUserPasswdNotValid: MTSAdminErrorCodes = MTSAdminErrorCodes(-2146368492i32);
pub const mtsExportUsers: MTSPackageExportOptions = MTSPackageExportOptions(1i32);
pub const mtsInstallUsers: MTSPackageInstallOptions = MTSPackageInstallOptions(1i32);
