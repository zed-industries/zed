#[cfg(feature = "Win32_System_Com")]
pub trait IADs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GUID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Parent(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Schema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInfo(&self) -> windows_core::Result<()>;
    fn SetInfo(&self) -> windows_core::Result<()>;
    fn Get(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn Put(&self, bstrname: &windows_core::BSTR, vprop: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn PutEx(&self, lncontrolcode: i32, bstrname: &windows_core::BSTR, vprop: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetInfoEx(&self, vproperties: &windows_core::VARIANT, lnreserved: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADs {}
#[cfg(feature = "Win32_System_Com")]
impl IADs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>() -> IADs_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::Class(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::GUID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::ADsPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::Parent(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Schema<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::Schema(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADs_Impl::GetInfo(this).into()
        }
        unsafe extern "system" fn SetInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADs_Impl::SetInfo(this).into()
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pvprop: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::Get(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    core::ptr::write(pvprop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, vprop: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADs_Impl::Put(this, core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
        }
        unsafe extern "system" fn GetEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pvprop: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADs_Impl::GetEx(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    core::ptr::write(pvprop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, vprop: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADs_Impl::PutEx(this, core::mem::transmute_copy(&lncontrolcode), core::mem::transmute(&bstrname), core::mem::transmute(&vprop)).into()
        }
        unsafe extern "system" fn GetInfoEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vproperties: core::mem::MaybeUninit<windows_core::VARIANT>, lnreserved: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADs_Impl::GetInfoEx(this, core::mem::transmute(&vproperties), core::mem::transmute_copy(&lnreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Schema: Schema::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
            PutEx: PutEx::<Identity, Impl, OFFSET>,
            GetInfoEx: GetInfoEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsADSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SiteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainShortName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsNativeMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAnyDCName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDCSiteName(&self, szserver: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn RefreshSchemaCache(&self) -> windows_core::Result<()>;
    fn GetTrees(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsADSystemInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IADsADSystemInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>() -> IADsADSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::UserName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::ComputerName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SiteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::SiteName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainShortName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::DomainShortName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainDNSName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::DomainDNSName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestDNSName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::ForestDNSName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDCRoleOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::PDCRoleOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchemaRoleOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::SchemaRoleOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNativeMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::IsNativeMode(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnyDCName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdcname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::GetAnyDCName(this) {
                Ok(ok__) => {
                    core::ptr::write(pszdcname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDCSiteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szserver: core::mem::MaybeUninit<windows_core::BSTR>, pszsitename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::GetDCSiteName(this, core::mem::transmute(&szserver)) {
                Ok(ok__) => {
                    core::ptr::write(pszsitename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshSchemaCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsADSystemInfo_Impl::RefreshSchemaCache(this).into()
        }
        unsafe extern "system" fn GetTrees<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtrees: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsADSystemInfo_Impl::GetTrees(this) {
                Ok(ok__) => {
                    core::ptr::write(pvtrees, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            SiteName: SiteName::<Identity, Impl, OFFSET>,
            DomainShortName: DomainShortName::<Identity, Impl, OFFSET>,
            DomainDNSName: DomainDNSName::<Identity, Impl, OFFSET>,
            ForestDNSName: ForestDNSName::<Identity, Impl, OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Identity, Impl, OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Identity, Impl, OFFSET>,
            IsNativeMode: IsNativeMode::<Identity, Impl, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, Impl, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, Impl, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, Impl, OFFSET>,
            GetTrees: GetTrees::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsADSystemInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAccessControlEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessMask(&self) -> windows_core::Result<i32>;
    fn SetAccessMask(&self, lnaccessmask: i32) -> windows_core::Result<()>;
    fn AceType(&self) -> windows_core::Result<i32>;
    fn SetAceType(&self, lnacetype: i32) -> windows_core::Result<()>;
    fn AceFlags(&self) -> windows_core::Result<i32>;
    fn SetAceFlags(&self, lnaceflags: i32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SetFlags(&self, lnflags: i32) -> windows_core::Result<()>;
    fn ObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectType(&self, bstrobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInheritedObjectType(&self, bstrinheritedobjecttype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Trustee(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsAccessControlEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>() -> IADsAccessControlEntry_Vtbl {
        unsafe extern "system" fn AccessMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::AccessMask(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaccessmask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetAccessMask(this, core::mem::transmute_copy(&lnaccessmask)).into()
        }
        unsafe extern "system" fn AceType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::AceType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacetype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetAceType(this, core::mem::transmute_copy(&lnacetype)).into()
        }
        unsafe extern "system" fn AceFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::AceFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaceflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetAceFlags(this, core::mem::transmute_copy(&lnaceflags)).into()
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::Flags(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetFlags(this, core::mem::transmute_copy(&lnflags)).into()
        }
        unsafe extern "system" fn ObjectType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::ObjectType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjecttype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetObjectType(this, core::mem::transmute(&bstrobjecttype)).into()
        }
        unsafe extern "system" fn InheritedObjectType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::InheritedObjectType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInheritedObjectType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinheritedobjecttype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetInheritedObjectType(this, core::mem::transmute(&bstrinheritedobjecttype)).into()
        }
        unsafe extern "system" fn Trustee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlEntry_Impl::Trustee(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrustee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlEntry_Impl::SetTrustee(this, core::mem::transmute(&bstrtrustee)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessMask: AccessMask::<Identity, Impl, OFFSET>,
            SetAccessMask: SetAccessMask::<Identity, Impl, OFFSET>,
            AceType: AceType::<Identity, Impl, OFFSET>,
            SetAceType: SetAceType::<Identity, Impl, OFFSET>,
            AceFlags: AceFlags::<Identity, Impl, OFFSET>,
            SetAceFlags: SetAceFlags::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            SetObjectType: SetObjectType::<Identity, Impl, OFFSET>,
            InheritedObjectType: InheritedObjectType::<Identity, Impl, OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
            SetTrustee: SetTrustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlEntry as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAccessControlList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AclRevision(&self) -> windows_core::Result<i32>;
    fn SetAclRevision(&self, lnaclrevision: i32) -> windows_core::Result<()>;
    fn AceCount(&self) -> windows_core::Result<i32>;
    fn SetAceCount(&self, lnacecount: i32) -> windows_core::Result<()>;
    fn AddAce(&self, paccesscontrolentry: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn RemoveAce(&self, paccesscontrolentry: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn CopyAccessList(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsAccessControlList {}
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>() -> IADsAccessControlList_Vtbl {
        unsafe extern "system" fn AclRevision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlList_Impl::AclRevision(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAclRevision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaclrevision: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlList_Impl::SetAclRevision(this, core::mem::transmute_copy(&lnaclrevision)).into()
        }
        unsafe extern "system" fn AceCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlList_Impl::AceCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAceCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnacecount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlList_Impl::SetAceCount(this, core::mem::transmute_copy(&lnacecount)).into()
        }
        unsafe extern "system" fn AddAce<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccesscontrolentry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlList_Impl::AddAce(this, windows_core::from_raw_borrowed(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn RemoveAce<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccesscontrolentry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAccessControlList_Impl::RemoveAce(this, windows_core::from_raw_borrowed(&paccesscontrolentry)).into()
        }
        unsafe extern "system" fn CopyAccessList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccesscontrollist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlList_Impl::CopyAccessList(this) {
                Ok(ok__) => {
                    core::ptr::write(ppaccesscontrollist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAccessControlList_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AclRevision: AclRevision::<Identity, Impl, OFFSET>,
            SetAclRevision: SetAclRevision::<Identity, Impl, OFFSET>,
            AceCount: AceCount::<Identity, Impl, OFFSET>,
            SetAceCount: SetAceCount::<Identity, Impl, OFFSET>,
            AddAce: AddAce::<Identity, Impl, OFFSET>,
            RemoveAce: RemoveAce::<Identity, Impl, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAccessControlList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAcl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProtectedAttrName(&self, bstrprotectedattrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubjectName(&self, bstrsubjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Privileges(&self) -> windows_core::Result<i32>;
    fn SetPrivileges(&self, lnprivileges: i32) -> windows_core::Result<()>;
    fn CopyAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsAcl {}
#[cfg(feature = "Win32_System_Com")]
impl IADsAcl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>() -> IADsAcl_Vtbl {
        unsafe extern "system" fn ProtectedAttrName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAcl_Impl::ProtectedAttrName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedAttrName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotectedattrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAcl_Impl::SetProtectedAttrName(this, core::mem::transmute(&bstrprotectedattrname)).into()
        }
        unsafe extern "system" fn SubjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAcl_Impl::SubjectName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubjectname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAcl_Impl::SetSubjectName(this, core::mem::transmute(&bstrsubjectname)).into()
        }
        unsafe extern "system" fn Privileges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAcl_Impl::Privileges(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivileges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnprivileges: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAcl_Impl::SetPrivileges(this, core::mem::transmute_copy(&lnprivileges)).into()
        }
        unsafe extern "system" fn CopyAcl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppacl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsAcl_Impl::CopyAcl(this) {
                Ok(ok__) => {
                    core::ptr::write(ppacl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProtectedAttrName: ProtectedAttrName::<Identity, Impl, OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Identity, Impl, OFFSET>,
            SubjectName: SubjectName::<Identity, Impl, OFFSET>,
            SetSubjectName: SetSubjectName::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
            SetPrivileges: SetPrivileges::<Identity, Impl, OFFSET>,
            CopyAcl: CopyAcl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAcl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IADsAggregatee_Impl: Sized {
    fn ConnectAsAggregatee(&self, pouterunknown: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DisconnectAsAggregatee(&self) -> windows_core::Result<()>;
    fn RelinquishInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RestoreInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IADsAggregatee {}
impl IADsAggregatee_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>() -> IADsAggregatee_Vtbl {
        unsafe extern "system" fn ConnectAsAggregatee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pouterunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregatee_Impl::ConnectAsAggregatee(this, windows_core::from_raw_borrowed(&pouterunknown)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregatee_Impl::DisconnectAsAggregatee(this).into()
        }
        unsafe extern "system" fn RelinquishInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregatee_Impl::RelinquishInterface(this, core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn RestoreInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregatee_Impl::RestoreInterface(this, core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregatee: ConnectAsAggregatee::<Identity, Impl, OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Identity, Impl, OFFSET>,
            RelinquishInterface: RelinquishInterface::<Identity, Impl, OFFSET>,
            RestoreInterface: RestoreInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAggregatee as windows_core::Interface>::IID
    }
}
pub trait IADsAggregator_Impl: Sized {
    fn ConnectAsAggregator(&self, paggregatee: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DisconnectAsAggregator(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IADsAggregator {}
impl IADsAggregator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>() -> IADsAggregator_Vtbl {
        unsafe extern "system" fn ConnectAsAggregator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregatee: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregator_Impl::ConnectAsAggregator(this, windows_core::from_raw_borrowed(&paggregatee)).into()
        }
        unsafe extern "system" fn DisconnectAsAggregator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsAggregator_Impl::DisconnectAsAggregator(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectAsAggregator: ConnectAsAggregator::<Identity, Impl, OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsAggregator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsBackLink_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RemoteID(&self) -> windows_core::Result<i32>;
    fn SetRemoteID(&self, lnremoteid: i32) -> windows_core::Result<()>;
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsBackLink {}
#[cfg(feature = "Win32_System_Com")]
impl IADsBackLink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>() -> IADsBackLink_Vtbl {
        unsafe extern "system" fn RemoteID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsBackLink_Impl::RemoteID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnremoteid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsBackLink_Impl::SetRemoteID(this, core::mem::transmute_copy(&lnremoteid)).into()
        }
        unsafe extern "system" fn ObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsBackLink_Impl::ObjectName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsBackLink_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RemoteID: RemoteID::<Identity, Impl, OFFSET>,
            SetRemoteID: SetRemoteID::<Identity, Impl, OFFSET>,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsBackLink as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsCaseIgnoreList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CaseIgnoreList(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCaseIgnoreList(&self, vcaseignorelist: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsCaseIgnoreList {}
#[cfg(feature = "Win32_System_Com")]
impl IADsCaseIgnoreList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>() -> IADsCaseIgnoreList_Vtbl {
        unsafe extern "system" fn CaseIgnoreList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsCaseIgnoreList_Impl::CaseIgnoreList(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcaseignorelist: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsCaseIgnoreList_Impl::SetCaseIgnoreList(this, core::mem::transmute(&vcaseignorelist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CaseIgnoreList: CaseIgnoreList::<Identity, Impl, OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCaseIgnoreList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsClass_Impl: Sized + IADs_Impl {
    fn PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCLSID(&self, bstrclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Abstract(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAbstract(&self, fabstract: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Auxiliary(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAuxiliary(&self, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MandatoryProperties(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetMandatoryProperties(&self, vmandatoryproperties: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn OptionalProperties(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetOptionalProperties(&self, voptionalproperties: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn NamingProperties(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetNamingProperties(&self, vnamingproperties: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn DerivedFrom(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDerivedFrom(&self, vderivedfrom: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AuxDerivedFrom(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetAuxDerivedFrom(&self, vauxderivedfrom: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PossibleSuperiors(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPossibleSuperiors(&self, vpossiblesuperiors: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Containment(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetContainment(&self, vcontainment: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Container(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetContainer(&self, fcontainer: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn HelpFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHelpFileName(&self, bstrhelpfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HelpFileContext(&self) -> windows_core::Result<i32>;
    fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsClass {}
#[cfg(feature = "Win32_System_Com")]
impl IADsClass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>() -> IADsClass_Vtbl {
        unsafe extern "system" fn PrimaryInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::PrimaryInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::CLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetCLSID(this, core::mem::transmute(&bstrclsid)).into()
        }
        unsafe extern "system" fn OID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::OID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetOID(this, core::mem::transmute(&bstroid)).into()
        }
        unsafe extern "system" fn Abstract<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::Abstract(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbstract<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabstract: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetAbstract(this, core::mem::transmute_copy(&fabstract)).into()
        }
        unsafe extern "system" fn Auxiliary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::Auxiliary(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxiliary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetAuxiliary(this, core::mem::transmute_copy(&fauxiliary)).into()
        }
        unsafe extern "system" fn MandatoryProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::MandatoryProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMandatoryProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vmandatoryproperties: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetMandatoryProperties(this, core::mem::transmute(&vmandatoryproperties)).into()
        }
        unsafe extern "system" fn OptionalProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::OptionalProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionalProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voptionalproperties: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetOptionalProperties(this, core::mem::transmute(&voptionalproperties)).into()
        }
        unsafe extern "system" fn NamingProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::NamingProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamingProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnamingproperties: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetNamingProperties(this, core::mem::transmute(&vnamingproperties)).into()
        }
        unsafe extern "system" fn DerivedFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::DerivedFrom(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDerivedFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vderivedfrom: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetDerivedFrom(this, core::mem::transmute(&vderivedfrom)).into()
        }
        unsafe extern "system" fn AuxDerivedFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::AuxDerivedFrom(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vauxderivedfrom: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetAuxDerivedFrom(this, core::mem::transmute(&vauxderivedfrom)).into()
        }
        unsafe extern "system" fn PossibleSuperiors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::PossibleSuperiors(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleSuperiors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpossiblesuperiors: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetPossibleSuperiors(this, core::mem::transmute(&vpossiblesuperiors)).into()
        }
        unsafe extern "system" fn Containment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::Containment(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcontainment: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetContainment(this, core::mem::transmute(&vcontainment)).into()
        }
        unsafe extern "system" fn Container<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::Container(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontainer: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetContainer(this, core::mem::transmute_copy(&fcontainer)).into()
        }
        unsafe extern "system" fn HelpFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::HelpFileName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhelpfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetHelpFileName(this, core::mem::transmute(&bstrhelpfilename)).into()
        }
        unsafe extern "system" fn HelpFileContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::HelpFileContext(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHelpFileContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhelpfilecontext: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsClass_Impl::SetHelpFileContext(this, core::mem::transmute_copy(&lnhelpfilecontext)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualifiers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsClass_Impl::Qualifiers(this) {
                Ok(ok__) => {
                    core::ptr::write(ppqualifiers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrimaryInterface: PrimaryInterface::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Abstract: Abstract::<Identity, Impl, OFFSET>,
            SetAbstract: SetAbstract::<Identity, Impl, OFFSET>,
            Auxiliary: Auxiliary::<Identity, Impl, OFFSET>,
            SetAuxiliary: SetAuxiliary::<Identity, Impl, OFFSET>,
            MandatoryProperties: MandatoryProperties::<Identity, Impl, OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Identity, Impl, OFFSET>,
            OptionalProperties: OptionalProperties::<Identity, Impl, OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Identity, Impl, OFFSET>,
            NamingProperties: NamingProperties::<Identity, Impl, OFFSET>,
            SetNamingProperties: SetNamingProperties::<Identity, Impl, OFFSET>,
            DerivedFrom: DerivedFrom::<Identity, Impl, OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Identity, Impl, OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Identity, Impl, OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Identity, Impl, OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Identity, Impl, OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Identity, Impl, OFFSET>,
            Containment: Containment::<Identity, Impl, OFFSET>,
            SetContainment: SetContainment::<Identity, Impl, OFFSET>,
            Container: Container::<Identity, Impl, OFFSET>,
            SetContainer: SetContainer::<Identity, Impl, OFFSET>,
            HelpFileName: HelpFileName::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            HelpFileContext: HelpFileContext::<Identity, Impl, OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsClass as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, bstrname: &windows_core::BSTR, vitem: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetObject(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IADsCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>() -> IADsCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, vitem: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsCollection_Impl::Add(this, core::mem::transmute(&bstrname), core::mem::transmute(&vitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemtoberemoved: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsCollection_Impl::Remove(this, core::mem::transmute(&bstritemtoberemoved)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pvitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsCollection_Impl::GetObject(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    core::ptr::write(pvitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsComputer_Impl: Sized + IADs_Impl {
    fn ComputerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Site(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrimaryUser(&self, bstrprimaryuser: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Role(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOperatingSystem(&self, bstroperatingsystem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOperatingSystemVersion(&self, bstroperatingsystemversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Processor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessor(&self, bstrprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessorCount(&self, bstrprocessorcount: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MemorySize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMemorySize(&self, bstrmemorysize: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStorageCapacity(&self, bstrstoragecapacity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NetAddresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsComputer {}
#[cfg(feature = "Win32_System_Com")]
impl IADsComputer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>() -> IADsComputer_Vtbl {
        unsafe extern "system" fn ComputerID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::ComputerID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Site<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Site(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Location(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetLocation(this, core::mem::transmute(&bstrlocation)).into()
        }
        unsafe extern "system" fn PrimaryUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::PrimaryUser(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrimaryUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprimaryuser: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetPrimaryUser(this, core::mem::transmute(&bstrprimaryuser)).into()
        }
        unsafe extern "system" fn Owner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Owner(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetOwner(this, core::mem::transmute(&bstrowner)).into()
        }
        unsafe extern "system" fn Division<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Division(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetDivision(this, core::mem::transmute(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Department(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Role<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Role(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetRole(this, core::mem::transmute(&bstrrole)).into()
        }
        unsafe extern "system" fn OperatingSystem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::OperatingSystem(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystem: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetOperatingSystem(this, core::mem::transmute(&bstroperatingsystem)).into()
        }
        unsafe extern "system" fn OperatingSystemVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::OperatingSystemVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroperatingsystemversion: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetOperatingSystemVersion(this, core::mem::transmute(&bstroperatingsystemversion)).into()
        }
        unsafe extern "system" fn Model<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Model(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetModel(this, core::mem::transmute(&bstrmodel)).into()
        }
        unsafe extern "system" fn Processor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::Processor(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessor: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetProcessor(this, core::mem::transmute(&bstrprocessor)).into()
        }
        unsafe extern "system" fn ProcessorCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::ProcessorCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessorCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprocessorcount: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetProcessorCount(this, core::mem::transmute(&bstrprocessorcount)).into()
        }
        unsafe extern "system" fn MemorySize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::MemorySize(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMemorySize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmemorysize: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetMemorySize(this, core::mem::transmute(&bstrmemorysize)).into()
        }
        unsafe extern "system" fn StorageCapacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::StorageCapacity(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageCapacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstoragecapacity: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetStorageCapacity(this, core::mem::transmute(&bstrstoragecapacity)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputer_Impl::NetAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputer_Impl::SetNetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            ComputerID: ComputerID::<Identity, Impl, OFFSET>,
            Site: Site::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            PrimaryUser: PrimaryUser::<Identity, Impl, OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            SetRole: SetRole::<Identity, Impl, OFFSET>,
            OperatingSystem: OperatingSystem::<Identity, Impl, OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Identity, Impl, OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Identity, Impl, OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Processor: Processor::<Identity, Impl, OFFSET>,
            SetProcessor: SetProcessor::<Identity, Impl, OFFSET>,
            ProcessorCount: ProcessorCount::<Identity, Impl, OFFSET>,
            SetProcessorCount: SetProcessorCount::<Identity, Impl, OFFSET>,
            MemorySize: MemorySize::<Identity, Impl, OFFSET>,
            SetMemorySize: SetMemorySize::<Identity, Impl, OFFSET>,
            StorageCapacity: StorageCapacity::<Identity, Impl, OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsComputerOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Shutdown(&self, breboot: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsComputerOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IADsComputerOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>() -> IADsComputerOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsComputerOperations_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, breboot: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsComputerOperations_Impl::Shutdown(this, core::mem::transmute_copy(&breboot)).into()
        }
        Self { base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET>, Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsComputerOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsContainer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Filter(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetFilter(&self, var: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Hints(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetHints(&self, vhints: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetObject(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Create(&self, classname: &windows_core::BSTR, relativename: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self, bstrclassname: &windows_core::BSTR, bstrrelativename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn MoveHere(&self, sourcename: &windows_core::BSTR, newname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IADsContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>() -> IADsContainer_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::Filter(this) {
                Ok(ok__) => {
                    core::ptr::write(pvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsContainer_Impl::SetFilter(this, core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn Hints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::Hints(this) {
                Ok(ok__) => {
                    core::ptr::write(pvfilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vhints: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsContainer_Impl::SetHints(this, core::mem::transmute(&vhints)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: core::mem::MaybeUninit<windows_core::BSTR>, relativename: core::mem::MaybeUninit<windows_core::BSTR>, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::GetObject(this, core::mem::transmute(&classname), core::mem::transmute(&relativename)) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: core::mem::MaybeUninit<windows_core::BSTR>, relativename: core::mem::MaybeUninit<windows_core::BSTR>, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::Create(this, core::mem::transmute(&classname), core::mem::transmute(&relativename)) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclassname: core::mem::MaybeUninit<windows_core::BSTR>, bstrrelativename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsContainer_Impl::Delete(this, core::mem::transmute(&bstrclassname), core::mem::transmute(&bstrrelativename)).into()
        }
        unsafe extern "system" fn CopyHere<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcename: core::mem::MaybeUninit<windows_core::BSTR>, newname: core::mem::MaybeUninit<windows_core::BSTR>, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::CopyHere(this, core::mem::transmute(&sourcename), core::mem::transmute(&newname)) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveHere<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcename: core::mem::MaybeUninit<windows_core::BSTR>, newname: core::mem::MaybeUninit<windows_core::BSTR>, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsContainer_Impl::MoveHere(this, core::mem::transmute(&sourcename), core::mem::transmute(&newname)) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
            SetHints: SetHints::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            CopyHere: CopyHere::<Identity, Impl, OFFSET>,
            MoveHere: MoveHere::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsContainer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDNWithBinary_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BinaryValue(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBinaryValue(&self, vbinaryvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsDNWithBinary {}
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithBinary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>() -> IADsDNWithBinary_Vtbl {
        unsafe extern "system" fn BinaryValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDNWithBinary_Impl::BinaryValue(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinaryValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbinaryvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDNWithBinary_Impl::SetBinaryValue(this, core::mem::transmute(&vbinaryvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDNWithBinary_Impl::DNString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDNWithBinary_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BinaryValue: BinaryValue::<Identity, Impl, OFFSET>,
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithBinary as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDNWithString_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StringValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStringValue(&self, bstrstringvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsDNWithString {}
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>() -> IADsDNWithString_Vtbl {
        unsafe extern "system" fn StringValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDNWithString_Impl::StringValue(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstringvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDNWithString_Impl::SetStringValue(this, core::mem::transmute(&bstrstringvalue)).into()
        }
        unsafe extern "system" fn DNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDNWithString_Impl::DNString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDNWithString_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StringValue: StringValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDNWithString as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDeleteOps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeleteObject(&self, lnflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsDeleteOps {}
#[cfg(feature = "Win32_System_Com")]
impl IADsDeleteOps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: isize>() -> IADsDeleteOps_Vtbl {
        unsafe extern "system" fn DeleteObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDeleteOps_Impl::DeleteObject(this, core::mem::transmute_copy(&lnflags)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DeleteObject: DeleteObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDeleteOps as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDomain_Impl: Sized + IADs_Impl {
    fn IsWorkgroup(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MinPasswordLength(&self) -> windows_core::Result<i32>;
    fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::Result<()>;
    fn MinPasswordAge(&self) -> windows_core::Result<i32>;
    fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::Result<()>;
    fn MaxPasswordAge(&self) -> windows_core::Result<i32>;
    fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::Result<()>;
    fn MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32>;
    fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::Result<()>;
    fn PasswordHistoryLength(&self) -> windows_core::Result<i32>;
    fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::Result<()>;
    fn PasswordAttributes(&self) -> windows_core::Result<i32>;
    fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::Result<()>;
    fn AutoUnlockInterval(&self) -> windows_core::Result<i32>;
    fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::Result<()>;
    fn LockoutObservationInterval(&self) -> windows_core::Result<i32>;
    fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsDomain {}
#[cfg(feature = "Win32_System_Com")]
impl IADsDomain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>() -> IADsDomain_Vtbl {
        unsafe extern "system" fn IsWorkgroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::IsWorkgroup(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPasswordLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::MinPasswordLength(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetMinPasswordLength(this, core::mem::transmute_copy(&lnminpasswordlength)).into()
        }
        unsafe extern "system" fn MinPasswordAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::MinPasswordAge(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPasswordAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminpasswordage: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetMinPasswordAge(this, core::mem::transmute_copy(&lnminpasswordage)).into()
        }
        unsafe extern "system" fn MaxPasswordAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::MaxPasswordAge(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPasswordAge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxpasswordage: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetMaxPasswordAge(this, core::mem::transmute_copy(&lnmaxpasswordage)).into()
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::MaxBadPasswordsAllowed(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetMaxBadPasswordsAllowed(this, core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into()
        }
        unsafe extern "system" fn PasswordHistoryLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::PasswordHistoryLength(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordhistorylength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetPasswordHistoryLength(this, core::mem::transmute_copy(&lnpasswordhistorylength)).into()
        }
        unsafe extern "system" fn PasswordAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::PasswordAttributes(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordattributes: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetPasswordAttributes(this, core::mem::transmute_copy(&lnpasswordattributes)).into()
        }
        unsafe extern "system" fn AutoUnlockInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::AutoUnlockInterval(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnautounlockinterval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetAutoUnlockInterval(this, core::mem::transmute_copy(&lnautounlockinterval)).into()
        }
        unsafe extern "system" fn LockoutObservationInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsDomain_Impl::LockoutObservationInterval(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlockoutobservationinterval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsDomain_Impl::SetLockoutObservationInterval(this, core::mem::transmute_copy(&lnlockoutobservationinterval)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsWorkgroup: IsWorkgroup::<Identity, Impl, OFFSET>,
            MinPasswordLength: MinPasswordLength::<Identity, Impl, OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Identity, Impl, OFFSET>,
            MinPasswordAge: MinPasswordAge::<Identity, Impl, OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Identity, Impl, OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Identity, Impl, OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Identity, Impl, OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Identity, Impl, OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Identity, Impl, OFFSET>,
            PasswordAttributes: PasswordAttributes::<Identity, Impl, OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Identity, Impl, OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Identity, Impl, OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Identity, Impl, OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Identity, Impl, OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsDomain as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsEmail_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> windows_core::Result<()>;
    fn Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAddress(&self, bstraddress: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsEmail {}
#[cfg(feature = "Win32_System_Com")]
impl IADsEmail_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>() -> IADsEmail_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsEmail_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsEmail_Impl::SetType(this, core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsEmail_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsEmail_Impl::SetAddress(this, core::mem::transmute(&bstraddress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsEmail as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsExtension_Impl: Sized {
    fn Operate(&self, dwcode: u32, vardata1: &windows_core::VARIANT, vardata2: &windows_core::VARIANT, vardata3: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PrivateGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> windows_core::Result<i32>;
    fn PrivateInvoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsExtension {}
#[cfg(feature = "Win32_System_Com")]
impl IADsExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>() -> IADsExtension_Vtbl {
        unsafe extern "system" fn Operate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcode: u32, vardata1: core::mem::MaybeUninit<windows_core::VARIANT>, vardata2: core::mem::MaybeUninit<windows_core::VARIANT>, vardata3: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsExtension_Impl::Operate(this, core::mem::transmute_copy(&dwcode), core::mem::transmute(&vardata1), core::mem::transmute(&vardata2), core::mem::transmute(&vardata3)).into()
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsExtension_Impl::PrivateGetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    core::ptr::write(rgdispid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateInvoke<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsExtension_Impl::PrivateInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Operate: Operate::<Identity, Impl, OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Identity, Impl, OFFSET>,
            PrivateInvoke: PrivateInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsExtension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFaxNumber_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetParameters(&self, vparameters: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsFaxNumber {}
#[cfg(feature = "Win32_System_Com")]
impl IADsFaxNumber_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>() -> IADsFaxNumber_Vtbl {
        unsafe extern "system" fn TelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFaxNumber_Impl::TelephoneNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFaxNumber_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn Parameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFaxNumber_Impl::Parameters(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vparameters: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFaxNumber_Impl::SetParameters(this, core::mem::transmute(&vparameters)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFaxNumber as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileService_Impl: Sized + IADsService_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxUserCount(&self) -> windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsFileService {}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>() -> IADsFileService_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileService_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileService_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileService_Impl::MaxUserCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileService_Impl::SetMaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base__: IADsService_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileService as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileServiceOperations_Impl: Sized + IADsServiceOperations_Impl {
    fn Sessions(&self) -> windows_core::Result<IADsCollection>;
    fn Resources(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsFileServiceOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileServiceOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>() -> IADsFileServiceOperations_Vtbl {
        unsafe extern "system" fn Sessions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsessions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileServiceOperations_Impl::Sessions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsessions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileServiceOperations_Impl::Resources(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresources, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IADsServiceOperations_Vtbl::new::<Identity, Impl, OFFSET>(),
            Sessions: Sessions::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileServiceOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID || iid == &<IADsServiceOperations as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileShare_Impl: Sized + IADs_Impl {
    fn CurrentUserCount(&self) -> windows_core::Result<i32>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxUserCount(&self) -> windows_core::Result<i32>;
    fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsFileShare {}
#[cfg(feature = "Win32_System_Com")]
impl IADsFileShare_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>() -> IADsFileShare_Vtbl {
        unsafe extern "system" fn CurrentUserCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileShare_Impl::CurrentUserCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileShare_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileShare_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn HostComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileShare_Impl::HostComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileShare_Impl::SetHostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileShare_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileShare_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
        }
        unsafe extern "system" fn MaxUserCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsFileShare_Impl::MaxUserCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxusercount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsFileShare_Impl::SetMaxUserCount(this, core::mem::transmute_copy(&lnmaxusercount)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUserCount: CurrentUserCount::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsFileShare as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsGroup_Impl: Sized + IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IADsMembers>;
    fn IsMember(&self, bstrmember: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, bstrnewitem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, bstritemtoberemoved: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IADsGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>() -> IADsGroup_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsGroup_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsGroup_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Members<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmembers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsGroup_Impl::Members(this) {
                Ok(ok__) => {
                    core::ptr::write(ppmembers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMember<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmember: core::mem::MaybeUninit<windows_core::BSTR>, bmember: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsGroup_Impl::IsMember(this, core::mem::transmute(&bstrmember)) {
                Ok(ok__) => {
                    core::ptr::write(bmember, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewitem: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsGroup_Impl::Add(this, core::mem::transmute(&bstrnewitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemtoberemoved: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsGroup_Impl::Remove(this, core::mem::transmute(&bstritemtoberemoved)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsHold_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Amount(&self) -> windows_core::Result<i32>;
    fn SetAmount(&self, lnamount: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsHold {}
#[cfg(feature = "Win32_System_Com")]
impl IADsHold_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>() -> IADsHold_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsHold_Impl::ObjectName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsHold_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Amount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsHold_Impl::Amount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnamount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsHold_Impl::SetAmount(this, core::mem::transmute_copy(&lnamount)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Amount: Amount::<Identity, Impl, OFFSET>,
            SetAmount: SetAmount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsHold as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsLargeInteger_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn HighPart(&self) -> windows_core::Result<i32>;
    fn SetHighPart(&self, lnhighpart: i32) -> windows_core::Result<()>;
    fn LowPart(&self) -> windows_core::Result<i32>;
    fn SetLowPart(&self, lnlowpart: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsLargeInteger {}
#[cfg(feature = "Win32_System_Com")]
impl IADsLargeInteger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>() -> IADsLargeInteger_Vtbl {
        unsafe extern "system" fn HighPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLargeInteger_Impl::HighPart(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnhighpart: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLargeInteger_Impl::SetHighPart(this, core::mem::transmute_copy(&lnhighpart)).into()
        }
        unsafe extern "system" fn LowPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLargeInteger_Impl::LowPart(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlowpart: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLargeInteger_Impl::SetLowPart(this, core::mem::transmute_copy(&lnlowpart)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HighPart: HighPart::<Identity, Impl, OFFSET>,
            SetHighPart: SetHighPart::<Identity, Impl, OFFSET>,
            LowPart: LowPart::<Identity, Impl, OFFSET>,
            SetLowPart: SetLowPart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLargeInteger as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsLocality_Impl: Sized + IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsLocality {}
#[cfg(feature = "Win32_System_Com")]
impl IADsLocality_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>() -> IADsLocality_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLocality_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLocality_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLocality_Impl::LocalityName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLocality_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLocality_Impl::PostalAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLocality_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsLocality_Impl::SeeAlso(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsLocality_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsLocality as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsMembers_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Filter(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetFilter(&self, pvfilter: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsMembers {}
#[cfg(feature = "Win32_System_Com")]
impl IADsMembers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>() -> IADsMembers_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsMembers_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsMembers_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Filter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsMembers_Impl::Filter(this) {
                Ok(ok__) => {
                    core::ptr::write(pvfilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfilter: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsMembers_Impl::SetFilter(this, core::mem::transmute(&pvfilter)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsMembers as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNameTranslate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetChaseReferral(&self, lnchasereferral: i32) -> windows_core::Result<()>;
    fn Init(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitEx(&self, lnsettype: i32, bstradspath: &windows_core::BSTR, bstruserid: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Set(&self, lnsettype: i32, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Get(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetEx(&self, lnformattype: i32, pvar: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetEx(&self, lnformattype: i32) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsNameTranslate {}
#[cfg(feature = "Win32_System_Com")]
impl IADsNameTranslate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>() -> IADsNameTranslate_Vtbl {
        unsafe extern "system" fn SetChaseReferral<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnchasereferral: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNameTranslate_Impl::SetChaseReferral(this, core::mem::transmute_copy(&lnchasereferral)).into()
        }
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNameTranslate_Impl::Init(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn InitEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: core::mem::MaybeUninit<windows_core::BSTR>, bstruserid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdomain: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNameTranslate_Impl::InitEx(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsettype: i32, bstradspath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNameTranslate_Impl::Set(this, core::mem::transmute_copy(&lnsettype), core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pbstradspath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsNameTranslate_Impl::Get(this, core::mem::transmute_copy(&lnformattype)) {
                Ok(ok__) => {
                    core::ptr::write(pbstradspath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNameTranslate_Impl::SetEx(this, core::mem::transmute_copy(&lnformattype), core::mem::transmute(&pvar)).into()
        }
        unsafe extern "system" fn GetEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsNameTranslate_Impl::GetEx(this, core::mem::transmute_copy(&lnformattype)) {
                Ok(ok__) => {
                    core::ptr::write(pvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetChaseReferral: SetChaseReferral::<Identity, Impl, OFFSET>,
            Init: Init::<Identity, Impl, OFFSET>,
            InitEx: InitEx::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            SetEx: SetEx::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNameTranslate as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNamespaces_Impl: Sized + IADs_Impl {
    fn DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDefaultContainer(&self, bstrdefaultcontainer: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsNamespaces {}
#[cfg(feature = "Win32_System_Com")]
impl IADsNamespaces_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>() -> IADsNamespaces_Vtbl {
        unsafe extern "system" fn DefaultContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsNamespaces_Impl::DefaultContainer(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdefaultcontainer: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNamespaces_Impl::SetDefaultContainer(this, core::mem::transmute(&bstrdefaultcontainer)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            DefaultContainer: DefaultContainer::<Identity, Impl, OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNamespaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNetAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressType(&self) -> windows_core::Result<i32>;
    fn SetAddressType(&self, lnaddresstype: i32) -> windows_core::Result<()>;
    fn Address(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetAddress(&self, vaddress: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsNetAddress {}
#[cfg(feature = "Win32_System_Com")]
impl IADsNetAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>() -> IADsNetAddress_Vtbl {
        unsafe extern "system" fn AddressType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsNetAddress_Impl::AddressType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddressType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnaddresstype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNetAddress_Impl::SetAddressType(this, core::mem::transmute_copy(&lnaddresstype)).into()
        }
        unsafe extern "system" fn Address<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsNetAddress_Impl::Address(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vaddress: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsNetAddress_Impl::SetAddress(this, core::mem::transmute(&vaddress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressType: AddressType::<Identity, Impl, OFFSET>,
            SetAddressType: SetAddressType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsNetAddress as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsO_Impl: Sized + IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsO {}
#[cfg(feature = "Win32_System_Com")]
impl IADsO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>() -> IADsO_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::LocalityName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::PostalAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::TelephoneNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::FaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsO_Impl::SeeAlso(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsO_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsO as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOU_Impl: Sized + IADs_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalityName(&self, bstrlocalityname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostalAddress(&self, bstrpostaladdress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTelephoneNumber(&self, bstrtelephonenumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBusinessCategory(&self, bstrbusinesscategory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsOU {}
#[cfg(feature = "Win32_System_Com")]
impl IADsOU_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>() -> IADsOU_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn LocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::LocalityName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalityName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocalityname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetLocalityName(this, core::mem::transmute(&bstrlocalityname)).into()
        }
        unsafe extern "system" fn PostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::PostalAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpostaladdress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetPostalAddress(this, core::mem::transmute(&bstrpostaladdress)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::TelephoneNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtelephonenumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetTelephoneNumber(this, core::mem::transmute(&bstrtelephonenumber)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::FaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfaxnumber: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetFaxNumber(this, core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::SeeAlso(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
        }
        unsafe extern "system" fn BusinessCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOU_Impl::BusinessCategory(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbusinesscategory: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOU_Impl::SetBusinessCategory(this, core::mem::transmute(&bstrbusinesscategory)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            BusinessCategory: BusinessCategory::<Identity, Impl, OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOU as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsObjectOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetOption(&self, lnoption: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn SetOption(&self, lnoption: i32, vvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsObjectOptions {}
#[cfg(feature = "Win32_System_Com")]
impl IADsObjectOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>() -> IADsObjectOptions_Vtbl {
        unsafe extern "system" fn GetOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, pvvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsObjectOptions_Impl::GetOption(this, core::mem::transmute_copy(&lnoption)) {
                Ok(ok__) => {
                    core::ptr::write(pvvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoption: i32, vvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsObjectOptions_Impl::SetOption(this, core::mem::transmute_copy(&lnoption), core::mem::transmute(&vvalue)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsObjectOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOctetList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OctetList(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetOctetList(&self, voctetlist: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsOctetList {}
#[cfg(feature = "Win32_System_Com")]
impl IADsOctetList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>() -> IADsOctetList_Vtbl {
        unsafe extern "system" fn OctetList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOctetList_Impl::OctetList(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetlist: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsOctetList_Impl::SetOctetList(this, core::mem::transmute(&voctetlist)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OctetList: OctetList::<Identity, Impl, OFFSET>,
            SetOctetList: SetOctetList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOctetList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOpenDSObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OpenDSObject(&self, lpszdnname: &windows_core::BSTR, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsOpenDSObject {}
#[cfg(feature = "Win32_System_Com")]
impl IADsOpenDSObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>() -> IADsOpenDSObject_Vtbl {
        unsafe extern "system" fn OpenDSObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszdnname: core::mem::MaybeUninit<windows_core::BSTR>, lpszusername: core::mem::MaybeUninit<windows_core::BSTR>, lpszpassword: core::mem::MaybeUninit<windows_core::BSTR>, lnreserved: i32, ppoledsobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsOpenDSObject_Impl::OpenDSObject(this, core::mem::transmute(&lpszdnname), core::mem::transmute(&lpszusername), core::mem::transmute(&lpszpassword), core::mem::transmute_copy(&lnreserved)) {
                Ok(ok__) => {
                    core::ptr::write(ppoledsobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), OpenDSObject: OpenDSObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsOpenDSObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPath_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<i32>;
    fn SetType(&self, lntype: i32) -> windows_core::Result<()>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVolumeName(&self, bstrvolumename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPath {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPath_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>() -> IADsPath_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPath_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lntype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPath_Impl::SetType(this, core::mem::transmute_copy(&lntype)).into()
        }
        unsafe extern "system" fn VolumeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPath_Impl::VolumeName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrvolumename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPath_Impl::SetVolumeName(this, core::mem::transmute(&bstrvolumename)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPath_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPath_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPath as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPathname_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Set(&self, bstradspath: &windows_core::BSTR, lnsettype: i32) -> windows_core::Result<()>;
    fn SetDisplayType(&self, lndisplaytype: i32) -> windows_core::Result<()>;
    fn Retrieve(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetNumElements(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, lnelementindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn AddLeafElement(&self, bstrleafelement: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveLeafElement(&self) -> windows_core::Result<()>;
    fn CopyPath(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetEscapedElement(&self, lnreserved: i32, bstrinstr: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn EscapedMode(&self) -> windows_core::Result<i32>;
    fn SetEscapedMode(&self, lnescapedmode: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPathname {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPathname_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>() -> IADsPathname_Vtbl {
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradspath: core::mem::MaybeUninit<windows_core::BSTR>, lnsettype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPathname_Impl::Set(this, core::mem::transmute(&bstradspath), core::mem::transmute_copy(&lnsettype)).into()
        }
        unsafe extern "system" fn SetDisplayType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lndisplaytype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPathname_Impl::SetDisplayType(this, core::mem::transmute_copy(&lndisplaytype)).into()
        }
        unsafe extern "system" fn Retrieve<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnformattype: i32, pbstradspath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::Retrieve(this, core::mem::transmute_copy(&lnformattype)) {
                Ok(ok__) => {
                    core::ptr::write(pbstradspath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumElements<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnnumpathelements: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::GetNumElements(this) {
                Ok(ok__) => {
                    core::ptr::write(plnnumpathelements, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::GetElement(this, core::mem::transmute_copy(&lnelementindex)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrelement, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLeafElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrleafelement: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPathname_Impl::AddLeafElement(this, core::mem::transmute(&bstrleafelement)).into()
        }
        unsafe extern "system" fn RemoveLeafElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPathname_Impl::RemoveLeafElement(this).into()
        }
        unsafe extern "system" fn CopyPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadspath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::CopyPath(this) {
                Ok(ok__) => {
                    core::ptr::write(ppadspath, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEscapedElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreserved: i32, bstrinstr: core::mem::MaybeUninit<windows_core::BSTR>, pbstroutstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::GetEscapedElement(this, core::mem::transmute_copy(&lnreserved), core::mem::transmute(&bstrinstr)) {
                Ok(ok__) => {
                    core::ptr::write(pbstroutstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapedMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPathname_Impl::EscapedMode(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEscapedMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnescapedmode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPathname_Impl::SetEscapedMode(this, core::mem::transmute_copy(&lnescapedmode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            Retrieve: Retrieve::<Identity, Impl, OFFSET>,
            GetNumElements: GetNumElements::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, Impl, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, Impl, OFFSET>,
            CopyPath: CopyPath::<Identity, Impl, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, Impl, OFFSET>,
            EscapedMode: EscapedMode::<Identity, Impl, OFFSET>,
            SetEscapedMode: SetEscapedMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPathname as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPostalAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PostalAddress(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPostalAddress(&self, vpostaladdress: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPostalAddress {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPostalAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>() -> IADsPostalAddress_Vtbl {
        unsafe extern "system" fn PostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPostalAddress_Impl::PostalAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdress: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPostalAddress_Impl::SetPostalAddress(this, core::mem::transmute(&vpostaladdress)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPostalAddress as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintJob_Impl: Sized + IADs_Impl {
    fn HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TimeSubmitted(&self) -> windows_core::Result<f64>;
    fn TotalPages(&self) -> windows_core::Result<i32>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()>;
    fn UntilTime(&self) -> windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()>;
    fn Notify(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNotify(&self, bstrnotify: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNotifyPath(&self, bstrnotifypath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPrintJob {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>() -> IADsPrintJob_Vtbl {
        unsafe extern "system" fn HostPrintQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::HostPrintQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::UserPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeSubmitted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::TimeSubmitted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::TotalPages(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetPriority(this, core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::StartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetStartTime(this, core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::UntilTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetUntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::Notify(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotify: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetNotify(this, core::mem::transmute(&bstrnotify)).into()
        }
        unsafe extern "system" fn NotifyPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJob_Impl::NotifyPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotifyPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnotifypath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJob_Impl::SetNotifyPath(this, core::mem::transmute(&bstrnotifypath)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostPrintQueue: HostPrintQueue::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            TimeSubmitted: TimeSubmitted::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            SetNotify: SetNotify::<Identity, Impl, OFFSET>,
            NotifyPath: NotifyPath::<Identity, Impl, OFFSET>,
            SetNotifyPath: SetNotifyPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintJobOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn TimeElapsed(&self) -> windows_core::Result<i32>;
    fn PagesPrinted(&self) -> windows_core::Result<i32>;
    fn Position(&self) -> windows_core::Result<i32>;
    fn SetPosition(&self, lnposition: i32) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPrintJobOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJobOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>() -> IADsPrintJobOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJobOperations_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeElapsed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJobOperations_Impl::TimeElapsed(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PagesPrinted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJobOperations_Impl::PagesPrinted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintJobOperations_Impl::Position(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnposition: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJobOperations_Impl::SetPosition(this, core::mem::transmute_copy(&lnposition)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJobOperations_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintJobOperations_Impl::Resume(this).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            TimeElapsed: TimeElapsed::<Identity, Impl, OFFSET>,
            PagesPrinted: PagesPrinted::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintJobOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintQueue_Impl: Sized + IADs_Impl {
    fn PrinterPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrinterPath(&self, bstrprinterpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Model(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModel(&self, bstrmodel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Datatype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDatatype(&self, bstrdatatype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrintProcessor(&self, bstrprintprocessor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Location(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocation(&self, bstrlocation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()>;
    fn UntilTime(&self) -> windows_core::Result<f64>;
    fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()>;
    fn DefaultJobPriority(&self) -> windows_core::Result<i32>;
    fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()>;
    fn BannerPage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBannerPage(&self, bstrbannerpage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintDevices(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPrintDevices(&self, vprintdevices: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn NetAddresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetNetAddresses(&self, vnetaddresses: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPrintQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>() -> IADsPrintQueue_Vtbl {
        unsafe extern "system" fn PrinterPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::PrinterPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrinterPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprinterpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetPrinterPath(this, core::mem::transmute(&bstrprinterpath)).into()
        }
        unsafe extern "system" fn Model<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::Model(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmodel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetModel(this, core::mem::transmute(&bstrmodel)).into()
        }
        unsafe extern "system" fn Datatype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::Datatype(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatatype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdatatype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetDatatype(this, core::mem::transmute(&bstrdatatype)).into()
        }
        unsafe extern "system" fn PrintProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::PrintProcessor(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintprocessor: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetPrintProcessor(this, core::mem::transmute(&bstrprintprocessor)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Location<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::Location(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlocation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetLocation(this, core::mem::transmute(&bstrlocation)).into()
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::StartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dastarttime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetStartTime(this, core::mem::transmute_copy(&dastarttime)).into()
        }
        unsafe extern "system" fn UntilTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::UntilTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntilTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dauntiltime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetUntilTime(this, core::mem::transmute_copy(&dauntiltime)).into()
        }
        unsafe extern "system" fn DefaultJobPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::DefaultJobPriority(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultJobPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lndefaultjobpriority: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetDefaultJobPriority(this, core::mem::transmute_copy(&lndefaultjobpriority)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::Priority(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpriority: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetPriority(this, core::mem::transmute_copy(&lnpriority)).into()
        }
        unsafe extern "system" fn BannerPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::BannerPage(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBannerPage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbannerpage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetBannerPage(this, core::mem::transmute(&bstrbannerpage)).into()
        }
        unsafe extern "system" fn PrintDevices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::PrintDevices(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintDevices<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprintdevices: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetPrintDevices(this, core::mem::transmute(&vprintdevices)).into()
        }
        unsafe extern "system" fn NetAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueue_Impl::NetAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vnetaddresses: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueue_Impl::SetNetAddresses(this, core::mem::transmute(&vnetaddresses)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrinterPath: PrinterPath::<Identity, Impl, OFFSET>,
            SetPrinterPath: SetPrinterPath::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Datatype: Datatype::<Identity, Impl, OFFSET>,
            SetDatatype: SetDatatype::<Identity, Impl, OFFSET>,
            PrintProcessor: PrintProcessor::<Identity, Impl, OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Identity, Impl, OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            BannerPage: BannerPage::<Identity, Impl, OFFSET>,
            SetBannerPage: SetBannerPage::<Identity, Impl, OFFSET>,
            PrintDevices: PrintDevices::<Identity, Impl, OFFSET>,
            SetPrintDevices: SetPrintDevices::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintQueueOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn PrintJobs(&self) -> windows_core::Result<IADsCollection>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Purge(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPrintQueueOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueueOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>() -> IADsPrintQueueOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueueOperations_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrintJobs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPrintQueueOperations_Impl::PrintJobs(this) {
                Ok(ok__) => {
                    core::ptr::write(pobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueueOperations_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueueOperations_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Purge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPrintQueueOperations_Impl::Purge(this).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            PrintJobs: PrintJobs::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPrintQueueOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsProperty_Impl: Sized + IADs_Impl {
    fn OID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOID(&self, bstroid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Syntax(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSyntax(&self, bstrsyntax: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxRange(&self) -> windows_core::Result<i32>;
    fn SetMaxRange(&self, lnmaxrange: i32) -> windows_core::Result<()>;
    fn MinRange(&self) -> windows_core::Result<i32>;
    fn SetMinRange(&self, lnminrange: i32) -> windows_core::Result<()>;
    fn MultiValued(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMultiValued(&self, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Qualifiers(&self) -> windows_core::Result<IADsCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsProperty {}
#[cfg(feature = "Win32_System_Com")]
impl IADsProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>() -> IADsProperty_Vtbl {
        unsafe extern "system" fn OID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::OID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsProperty_Impl::SetOID(this, core::mem::transmute(&bstroid)).into()
        }
        unsafe extern "system" fn Syntax<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::Syntax(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyntax<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsyntax: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsProperty_Impl::SetSyntax(this, core::mem::transmute(&bstrsyntax)).into()
        }
        unsafe extern "system" fn MaxRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::MaxRange(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxrange: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsProperty_Impl::SetMaxRange(this, core::mem::transmute_copy(&lnmaxrange)).into()
        }
        unsafe extern "system" fn MinRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::MinRange(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnminrange: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsProperty_Impl::SetMinRange(this, core::mem::transmute_copy(&lnminrange)).into()
        }
        unsafe extern "system" fn MultiValued<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::MultiValued(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiValued<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsProperty_Impl::SetMultiValued(this, core::mem::transmute_copy(&fmultivalued)).into()
        }
        unsafe extern "system" fn Qualifiers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqualifiers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsProperty_Impl::Qualifiers(this) {
                Ok(ok__) => {
                    core::ptr::write(ppqualifiers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Syntax: Syntax::<Identity, Impl, OFFSET>,
            SetSyntax: SetSyntax::<Identity, Impl, OFFSET>,
            MaxRange: MaxRange::<Identity, Impl, OFFSET>,
            SetMaxRange: SetMaxRange::<Identity, Impl, OFFSET>,
            MinRange: MinRange::<Identity, Impl, OFFSET>,
            SetMinRange: SetMinRange::<Identity, Impl, OFFSET>,
            MultiValued: MultiValued::<Identity, Impl, OFFSET>,
            SetMultiValued: SetMultiValued::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyEntry_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsType(&self) -> windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn ControlCode(&self) -> windows_core::Result<i32>;
    fn SetControlCode(&self, lncontrolcode: i32) -> windows_core::Result<()>;
    fn Values(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValues(&self, vvalues: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPropertyEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>() -> IADsPropertyEntry_Vtbl {
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyEntry_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyEntry_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyEntry_Impl::SetName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn ADsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyEntry_Impl::ADsType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyEntry_Impl::SetADsType(this, core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn ControlCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyEntry_Impl::ControlCode(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrolcode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyEntry_Impl::SetControlCode(this, core::mem::transmute_copy(&lncontrolcode)).into()
        }
        unsafe extern "system" fn Values<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyEntry_Impl::Values(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vvalues: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyEntry_Impl::SetValues(this, core::mem::transmute(&vvalues)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
            SetControlCode: SetControlCode::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyEntry as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PropertyCount(&self) -> windows_core::Result<i32>;
    fn Next(&self, pvariant: *mut windows_core::VARIANT) -> windows_core::HRESULT;
    fn Skip(&self, celements: i32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn GetPropertyItem(&self, bstrname: &windows_core::BSTR, lnadstype: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn PutPropertyItem(&self, vardata: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ResetPropertyItem(&self, varentry: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PurgePropertyList(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPropertyList {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>() -> IADsPropertyList_Vtbl {
        unsafe extern "system" fn PropertyCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyList_Impl::PropertyCount(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::Next(this, core::mem::transmute_copy(&pvariant))
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::Skip(this, core::mem::transmute_copy(&celements))
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyList_Impl::Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, lnadstype: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyList_Impl::GetPropertyItem(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&lnadstype)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPropertyItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardata: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::PutPropertyItem(this, core::mem::transmute(&vardata)).into()
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varentry: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::ResetPropertyItem(this, core::mem::transmute(&varentry)).into()
        }
        unsafe extern "system" fn PurgePropertyList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyList_Impl::PurgePropertyList(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyCount: PropertyCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetPropertyItem: GetPropertyItem::<Identity, Impl, OFFSET>,
            PutPropertyItem: PutPropertyItem::<Identity, Impl, OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Identity, Impl, OFFSET>,
            PurgePropertyList: PurgePropertyList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> windows_core::Result<()>;
    fn ADsType(&self) -> windows_core::Result<i32>;
    fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()>;
    fn DNString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDNString(&self, bstrdnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CaseExactString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCaseExactString(&self, bstrcaseexactstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCaseIgnoreString(&self, bstrcaseignorestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrintableString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPrintableString(&self, bstrprintablestring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NumericString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNumericString(&self, bstrnumericstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Boolean(&self) -> windows_core::Result<i32>;
    fn SetBoolean(&self, lnboolean: i32) -> windows_core::Result<()>;
    fn Integer(&self) -> windows_core::Result<i32>;
    fn SetInteger(&self, lninteger: i32) -> windows_core::Result<()>;
    fn OctetString(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetOctetString(&self, voctetstring: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&self, psecuritydescriptor: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn LargeInteger(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetLargeInteger(&self, plargeinteger: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn UTCTime(&self) -> windows_core::Result<f64>;
    fn SetUTCTime(&self, dautctime: f64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPropertyValue {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>() -> IADsPropertyValue_Vtbl {
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::Clear(this).into()
        }
        unsafe extern "system" fn ADsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::ADsType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetADsType(this, core::mem::transmute_copy(&lnadstype)).into()
        }
        unsafe extern "system" fn DNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::DNString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDNString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdnstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetDNString(this, core::mem::transmute(&bstrdnstring)).into()
        }
        unsafe extern "system" fn CaseExactString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::CaseExactString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseExactString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseexactstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetCaseExactString(this, core::mem::transmute(&bstrcaseexactstring)).into()
        }
        unsafe extern "system" fn CaseIgnoreString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::CaseIgnoreString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaseIgnoreString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaseignorestring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetCaseIgnoreString(this, core::mem::transmute(&bstrcaseignorestring)).into()
        }
        unsafe extern "system" fn PrintableString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::PrintableString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrintableString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprintablestring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetPrintableString(this, core::mem::transmute(&bstrprintablestring)).into()
        }
        unsafe extern "system" fn NumericString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::NumericString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumericString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnumericstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetNumericString(this, core::mem::transmute(&bstrnumericstring)).into()
        }
        unsafe extern "system" fn Boolean<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::Boolean(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolean<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnboolean: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetBoolean(this, core::mem::transmute_copy(&lnboolean)).into()
        }
        unsafe extern "system" fn Integer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::Integer(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInteger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninteger: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetInteger(this, core::mem::transmute_copy(&lninteger)).into()
        }
        unsafe extern "system" fn OctetString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::OctetString(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOctetString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voctetstring: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetOctetString(this, core::mem::transmute(&voctetstring)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::SecurityDescriptor(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecuritydescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetSecurityDescriptor(this, windows_core::from_raw_borrowed(&psecuritydescriptor)).into()
        }
        unsafe extern "system" fn LargeInteger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::LargeInteger(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeInteger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plargeinteger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetLargeInteger(this, windows_core::from_raw_borrowed(&plargeinteger)).into()
        }
        unsafe extern "system" fn UTCTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsPropertyValue_Impl::UTCTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dautctime: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue_Impl::SetUTCTime(this, core::mem::transmute_copy(&dautctime)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
            CaseExactString: CaseExactString::<Identity, Impl, OFFSET>,
            SetCaseExactString: SetCaseExactString::<Identity, Impl, OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Identity, Impl, OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Identity, Impl, OFFSET>,
            PrintableString: PrintableString::<Identity, Impl, OFFSET>,
            SetPrintableString: SetPrintableString::<Identity, Impl, OFFSET>,
            NumericString: NumericString::<Identity, Impl, OFFSET>,
            SetNumericString: SetNumericString::<Identity, Impl, OFFSET>,
            Boolean: Boolean::<Identity, Impl, OFFSET>,
            SetBoolean: SetBoolean::<Identity, Impl, OFFSET>,
            Integer: Integer::<Identity, Impl, OFFSET>,
            SetInteger: SetInteger::<Identity, Impl, OFFSET>,
            OctetString: OctetString::<Identity, Impl, OFFSET>,
            SetOctetString: SetOctetString::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            LargeInteger: LargeInteger::<Identity, Impl, OFFSET>,
            SetLargeInteger: SetLargeInteger::<Identity, Impl, OFFSET>,
            UTCTime: UTCTime::<Identity, Impl, OFFSET>,
            SetUTCTime: SetUTCTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyValue2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetObjectProperty(&self, lnadstype: *mut i32, pvprop: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn PutObjectProperty(&self, lnadstype: i32, vprop: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsPropertyValue2 {}
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>() -> IADsPropertyValue2_Vtbl {
        unsafe extern "system" fn GetObjectProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue2_Impl::GetObjectProperty(this, core::mem::transmute_copy(&lnadstype), core::mem::transmute_copy(&pvprop)).into()
        }
        unsafe extern "system" fn PutObjectProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnadstype: i32, vprop: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsPropertyValue2_Impl::PutObjectProperty(this, core::mem::transmute_copy(&lnadstype), core::mem::transmute(&vprop)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObjectProperty: GetObjectProperty::<Identity, Impl, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsPropertyValue2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsReplicaPointer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ServerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServerName(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReplicaType(&self) -> windows_core::Result<i32>;
    fn SetReplicaType(&self, lnreplicatype: i32) -> windows_core::Result<()>;
    fn ReplicaNumber(&self) -> windows_core::Result<i32>;
    fn SetReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn SetCount(&self, lncount: i32) -> windows_core::Result<()>;
    fn ReplicaAddressHints(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetReplicaAddressHints(&self, vreplicaaddresshints: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsReplicaPointer {}
#[cfg(feature = "Win32_System_Com")]
impl IADsReplicaPointer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>() -> IADsReplicaPointer_Vtbl {
        unsafe extern "system" fn ServerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsReplicaPointer_Impl::ServerName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsReplicaPointer_Impl::SetServerName(this, core::mem::transmute(&bstrservername)).into()
        }
        unsafe extern "system" fn ReplicaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsReplicaPointer_Impl::ReplicaType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicatype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsReplicaPointer_Impl::SetReplicaType(this, core::mem::transmute_copy(&lnreplicatype)).into()
        }
        unsafe extern "system" fn ReplicaNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsReplicaPointer_Impl::ReplicaNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnreplicanumber: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsReplicaPointer_Impl::SetReplicaNumber(this, core::mem::transmute_copy(&lnreplicanumber)).into()
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsReplicaPointer_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsReplicaPointer_Impl::SetCount(this, core::mem::transmute_copy(&lncount)).into()
        }
        unsafe extern "system" fn ReplicaAddressHints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsReplicaPointer_Impl::ReplicaAddressHints(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReplicaAddressHints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vreplicaaddresshints: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsReplicaPointer_Impl::SetReplicaAddressHints(this, core::mem::transmute(&vreplicaaddresshints)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            SetServerName: SetServerName::<Identity, Impl, OFFSET>,
            ReplicaType: ReplicaType::<Identity, Impl, OFFSET>,
            SetReplicaType: SetReplicaType::<Identity, Impl, OFFSET>,
            ReplicaNumber: ReplicaNumber::<Identity, Impl, OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            SetCount: SetCount::<Identity, Impl, OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Identity, Impl, OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsReplicaPointer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsResource_Impl: Sized + IADs_Impl {
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LockCount(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsResource {}
#[cfg(feature = "Win32_System_Com")]
impl IADsResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>() -> IADsResource_Vtbl {
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsResource_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsResource_Impl::UserPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsResource_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsResource_Impl::LockCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LockCount: LockCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsResource as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSecurityDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Revision(&self) -> windows_core::Result<i32>;
    fn SetRevision(&self, lnrevision: i32) -> windows_core::Result<()>;
    fn Control(&self) -> windows_core::Result<i32>;
    fn SetControl(&self, lncontrol: i32) -> windows_core::Result<()>;
    fn Owner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwner(&self, bstrowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOwnerDefaulted(&self, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Group(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGroup(&self, bstrgroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GroupDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupDefaulted(&self, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DiscretionaryAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetDiscretionaryAcl(&self, pdiscretionaryacl: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn DaclDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaclDefaulted(&self, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SystemAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSystemAcl(&self, psystemacl: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn SaclDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSaclDefaulted(&self, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CopySecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsSecurityDescriptor {}
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityDescriptor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>() -> IADsSecurityDescriptor_Vtbl {
        unsafe extern "system" fn Revision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::Revision(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnrevision: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetRevision(this, core::mem::transmute_copy(&lnrevision)).into()
        }
        unsafe extern "system" fn Control<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::Control(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lncontrol: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetControl(this, core::mem::transmute_copy(&lncontrol)).into()
        }
        unsafe extern "system" fn Owner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::Owner(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrowner: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetOwner(this, core::mem::transmute(&bstrowner)).into()
        }
        unsafe extern "system" fn OwnerDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::OwnerDefaulted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetOwnerDefaulted(this, core::mem::transmute_copy(&fownerdefaulted)).into()
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::Group(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroup: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetGroup(this, core::mem::transmute(&bstrgroup)).into()
        }
        unsafe extern "system" fn GroupDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::GroupDefaulted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetGroupDefaulted(this, core::mem::transmute_copy(&fgroupdefaulted)).into()
        }
        unsafe extern "system" fn DiscretionaryAcl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::DiscretionaryAcl(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiscretionaryacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetDiscretionaryAcl(this, windows_core::from_raw_borrowed(&pdiscretionaryacl)).into()
        }
        unsafe extern "system" fn DaclDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::DaclDefaulted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaclDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetDaclDefaulted(this, core::mem::transmute_copy(&fdacldefaulted)).into()
        }
        unsafe extern "system" fn SystemAcl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::SystemAcl(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemAcl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psystemacl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetSystemAcl(this, windows_core::from_raw_borrowed(&psystemacl)).into()
        }
        unsafe extern "system" fn SaclDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::SaclDefaulted(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaclDefaulted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityDescriptor_Impl::SetSaclDefaulted(this, core::mem::transmute_copy(&fsacldefaulted)).into()
        }
        unsafe extern "system" fn CopySecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecuritydescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityDescriptor_Impl::CopySecurityDescriptor(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecuritydescriptor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Revision: Revision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            Control: Control::<Identity, Impl, OFFSET>,
            SetControl: SetControl::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Identity, Impl, OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            GroupDefaulted: GroupDefaulted::<Identity, Impl, OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Identity, Impl, OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Identity, Impl, OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Identity, Impl, OFFSET>,
            DaclDefaulted: DaclDefaulted::<Identity, Impl, OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Identity, Impl, OFFSET>,
            SystemAcl: SystemAcl::<Identity, Impl, OFFSET>,
            SetSystemAcl: SetSystemAcl::<Identity, Impl, OFFSET>,
            SaclDefaulted: SaclDefaulted::<Identity, Impl, OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Identity, Impl, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityDescriptor as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSecurityUtility_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetSecurityDescriptor(&self, varpath: &windows_core::VARIANT, lpathformat: i32, lformat: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSecurityDescriptor(&self, varpath: &windows_core::VARIANT, lpathformat: i32, vardata: &windows_core::VARIANT, ldataformat: i32) -> windows_core::Result<()>;
    fn ConvertSecurityDescriptor(&self, varsd: &windows_core::VARIANT, ldataformat: i32, loutformat: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn SecurityMask(&self) -> windows_core::Result<i32>;
    fn SetSecurityMask(&self, lnsecuritymask: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsSecurityUtility {}
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityUtility_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>() -> IADsSecurityUtility_Vtbl {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: core::mem::MaybeUninit<windows_core::VARIANT>, lpathformat: i32, lformat: i32, pvariant: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityUtility_Impl::GetSecurityDescriptor(this, core::mem::transmute(&varpath), core::mem::transmute_copy(&lpathformat), core::mem::transmute_copy(&lformat)) {
                Ok(ok__) => {
                    core::ptr::write(pvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varpath: core::mem::MaybeUninit<windows_core::VARIANT>, lpathformat: i32, vardata: core::mem::MaybeUninit<windows_core::VARIANT>, ldataformat: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityUtility_Impl::SetSecurityDescriptor(this, core::mem::transmute(&varpath), core::mem::transmute_copy(&lpathformat), core::mem::transmute(&vardata), core::mem::transmute_copy(&ldataformat)).into()
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsd: core::mem::MaybeUninit<windows_core::VARIANT>, ldataformat: i32, loutformat: i32, presult: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityUtility_Impl::ConvertSecurityDescriptor(this, core::mem::transmute(&varsd), core::mem::transmute_copy(&ldataformat), core::mem::transmute_copy(&loutformat)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSecurityUtility_Impl::SecurityMask(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnsecuritymask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSecurityUtility_Impl::SetSecurityMask(this, core::mem::transmute_copy(&lnsecuritymask)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, Impl, OFFSET>,
            SecurityMask: SecurityMask::<Identity, Impl, OFFSET>,
            SetSecurityMask: SetSecurityMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSecurityUtility as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsService_Impl: Sized + IADs_Impl {
    fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHostComputer(&self, bstrhostcomputer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, bstrdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVersion(&self, bstrversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceType(&self) -> windows_core::Result<i32>;
    fn SetServiceType(&self, lnservicetype: i32) -> windows_core::Result<()>;
    fn StartType(&self) -> windows_core::Result<i32>;
    fn SetStartType(&self, lnstarttype: i32) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartupParameters(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStartupParameters(&self, bstrstartupparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ErrorControl(&self) -> windows_core::Result<i32>;
    fn SetErrorControl(&self, lnerrorcontrol: i32) -> windows_core::Result<()>;
    fn LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoadOrderGroup(&self, bstrloadordergroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceAccountName(&self, bstrserviceaccountname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceAccountPath(&self, bstrserviceaccountpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Dependencies(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDependencies(&self, vdependencies: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsService {}
#[cfg(feature = "Win32_System_Com")]
impl IADsService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>() -> IADsService_Vtbl {
        unsafe extern "system" fn HostComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::HostComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhostcomputer: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetHostComputer(this, core::mem::transmute(&bstrhostcomputer)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdisplayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetDisplayName(this, core::mem::transmute(&bstrdisplayname)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::Version(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrversion: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetVersion(this, core::mem::transmute(&bstrversion)).into()
        }
        unsafe extern "system" fn ServiceType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::ServiceType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnservicetype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetServiceType(this, core::mem::transmute_copy(&lnservicetype)).into()
        }
        unsafe extern "system" fn StartType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::StartType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnstarttype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetStartType(this, core::mem::transmute_copy(&lnstarttype)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetPath(this, core::mem::transmute(&bstrpath)).into()
        }
        unsafe extern "system" fn StartupParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::StartupParameters(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartupParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstartupparameters: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetStartupParameters(this, core::mem::transmute(&bstrstartupparameters)).into()
        }
        unsafe extern "system" fn ErrorControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::ErrorControl(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnerrorcontrol: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetErrorControl(this, core::mem::transmute_copy(&lnerrorcontrol)).into()
        }
        unsafe extern "system" fn LoadOrderGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::LoadOrderGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoadOrderGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloadordergroup: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetLoadOrderGroup(this, core::mem::transmute(&bstrloadordergroup)).into()
        }
        unsafe extern "system" fn ServiceAccountName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::ServiceAccountName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetServiceAccountName(this, core::mem::transmute(&bstrserviceaccountname)).into()
        }
        unsafe extern "system" fn ServiceAccountPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::ServiceAccountPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceAccountPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrserviceaccountpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetServiceAccountPath(this, core::mem::transmute(&bstrserviceaccountpath)).into()
        }
        unsafe extern "system" fn Dependencies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsService_Impl::Dependencies(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDependencies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdependencies: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsService_Impl::SetDependencies(this, core::mem::transmute(&vdependencies)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ServiceType: ServiceType::<Identity, Impl, OFFSET>,
            SetServiceType: SetServiceType::<Identity, Impl, OFFSET>,
            StartType: StartType::<Identity, Impl, OFFSET>,
            SetStartType: SetStartType::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            StartupParameters: StartupParameters::<Identity, Impl, OFFSET>,
            SetStartupParameters: SetStartupParameters::<Identity, Impl, OFFSET>,
            ErrorControl: ErrorControl::<Identity, Impl, OFFSET>,
            SetErrorControl: SetErrorControl::<Identity, Impl, OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Identity, Impl, OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Identity, Impl, OFFSET>,
            ServiceAccountName: ServiceAccountName::<Identity, Impl, OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Identity, Impl, OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Identity, Impl, OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            SetDependencies: SetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsService as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsServiceOperations_Impl: Sized + IADs_Impl {
    fn Status(&self) -> windows_core::Result<i32>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Continue(&self) -> windows_core::Result<()>;
    fn SetPassword(&self, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsServiceOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IADsServiceOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>() -> IADsServiceOperations_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsServiceOperations_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsServiceOperations_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsServiceOperations_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsServiceOperations_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Continue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsServiceOperations_Impl::Continue(this).into()
        }
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsServiceOperations_Impl::SetPassword(this, core::mem::transmute(&bstrnewpassword)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsServiceOperations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSession_Impl: Sized + IADs_Impl {
    fn User(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Computer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectTime(&self) -> windows_core::Result<i32>;
    fn IdleTime(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsSession {}
#[cfg(feature = "Win32_System_Com")]
impl IADsSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>() -> IADsSession_Vtbl {
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::UserPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::Computer(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::ComputerPath(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::ConnectTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdleTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSession_Impl::IdleTime(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Computer: Computer::<Identity, Impl, OFFSET>,
            ComputerPath: ComputerPath::<Identity, Impl, OFFSET>,
            ConnectTime: ConnectTime::<Identity, Impl, OFFSET>,
            IdleTime: IdleTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSession as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSyntax_Impl: Sized + IADs_Impl {
    fn OleAutoDataType(&self) -> windows_core::Result<i32>;
    fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsSyntax {}
#[cfg(feature = "Win32_System_Com")]
impl IADsSyntax_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>() -> IADsSyntax_Vtbl {
        unsafe extern "system" fn OleAutoDataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsSyntax_Impl::OleAutoDataType(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOleAutoDataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnoleautodatatype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsSyntax_Impl::SetOleAutoDataType(this, core::mem::transmute_copy(&lnoleautodatatype)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            OleAutoDataType: OleAutoDataType::<Identity, Impl, OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsSyntax as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsTimestamp_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WholeSeconds(&self) -> windows_core::Result<i32>;
    fn SetWholeSeconds(&self, lnwholeseconds: i32) -> windows_core::Result<()>;
    fn EventID(&self) -> windows_core::Result<i32>;
    fn SetEventID(&self, lneventid: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsTimestamp {}
#[cfg(feature = "Win32_System_Com")]
impl IADsTimestamp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>() -> IADsTimestamp_Vtbl {
        unsafe extern "system" fn WholeSeconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsTimestamp_Impl::WholeSeconds(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWholeSeconds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnwholeseconds: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsTimestamp_Impl::SetWholeSeconds(this, core::mem::transmute_copy(&lnwholeseconds)).into()
        }
        unsafe extern "system" fn EventID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsTimestamp_Impl::EventID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lneventid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsTimestamp_Impl::SetEventID(this, core::mem::transmute_copy(&lneventid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WholeSeconds: WholeSeconds::<Identity, Impl, OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            SetEventID: SetEventID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTimestamp as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsTypedName_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetObjectName(&self, bstrobjectname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Level(&self) -> windows_core::Result<i32>;
    fn SetLevel(&self, lnlevel: i32) -> windows_core::Result<()>;
    fn Interval(&self) -> windows_core::Result<i32>;
    fn SetInterval(&self, lninterval: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsTypedName {}
#[cfg(feature = "Win32_System_Com")]
impl IADsTypedName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>() -> IADsTypedName_Vtbl {
        unsafe extern "system" fn ObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsTypedName_Impl::ObjectName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrobjectname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsTypedName_Impl::SetObjectName(this, core::mem::transmute(&bstrobjectname)).into()
        }
        unsafe extern "system" fn Level<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsTypedName_Impl::Level(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnlevel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsTypedName_Impl::SetLevel(this, core::mem::transmute_copy(&lnlevel)).into()
        }
        unsafe extern "system" fn Interval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsTypedName_Impl::Interval(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lninterval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsTypedName_Impl::SetInterval(this, core::mem::transmute_copy(&lninterval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTypedName as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsUser_Impl: Sized + IADs_Impl {
    fn BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BadLoginCount(&self) -> windows_core::Result<i32>;
    fn LastLogin(&self) -> windows_core::Result<f64>;
    fn LastLogoff(&self) -> windows_core::Result<f64>;
    fn LastFailedLogin(&self) -> windows_core::Result<f64>;
    fn PasswordLastChanged(&self) -> windows_core::Result<f64>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Division(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDivision(&self, bstrdivision: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Department(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EmployeeID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmployeeID(&self, bstremployeeid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FullName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFullName(&self, bstrfullname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FirstName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFirstName(&self, bstrfirstname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LastName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLastName(&self, bstrlastname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OtherName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOtherName(&self, bstrothername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNamePrefix(&self, bstrnameprefix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NameSuffix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetNameSuffix(&self, bstrnamesuffix: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTitle(&self, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Manager(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetManager(&self, bstrmanager: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TelephoneHome(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetTelephoneHome(&self, vtelephonehome: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn TelephoneMobile(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetTelephoneMobile(&self, vtelephonemobile: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn TelephoneNumber(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetTelephoneNumber(&self, vtelephonenumber: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn TelephonePager(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetTelephonePager(&self, vtelephonepager: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn FaxNumber(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetFaxNumber(&self, vfaxnumber: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn OfficeLocations(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetOfficeLocations(&self, vofficelocations: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PostalAddresses(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPostalAddresses(&self, vpostaladdresses: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PostalCodes(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPostalCodes(&self, vpostalcodes: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSeeAlso(&self, vseealso: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AccountDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAccountDisabled(&self, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AccountExpirationDate(&self) -> windows_core::Result<f64>;
    fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> windows_core::Result<()>;
    fn GraceLoginsAllowed(&self) -> windows_core::Result<i32>;
    fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::Result<()>;
    fn GraceLoginsRemaining(&self) -> windows_core::Result<i32>;
    fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::Result<()>;
    fn IsAccountLocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsAccountLocked(&self, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LoginHours(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetLoginHours(&self, vloginhours: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn LoginWorkstations(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetLoginWorkstations(&self, vloginworkstations: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MaxLogins(&self) -> windows_core::Result<i32>;
    fn SetMaxLogins(&self, lnmaxlogins: i32) -> windows_core::Result<()>;
    fn MaxStorage(&self) -> windows_core::Result<i32>;
    fn SetMaxStorage(&self, lnmaxstorage: i32) -> windows_core::Result<()>;
    fn PasswordExpirationDate(&self) -> windows_core::Result<f64>;
    fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> windows_core::Result<()>;
    fn PasswordMinimumLength(&self) -> windows_core::Result<i32>;
    fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::Result<()>;
    fn PasswordRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPasswordRequired(&self, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RequireUniquePassword(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRequireUniquePassword(&self, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EmailAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEmailAddress(&self, bstremailaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomeDirectory(&self, bstrhomedirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Languages(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetLanguages(&self, vlanguages: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Profile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProfile(&self, bstrprofile: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoginScript(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoginScript(&self, bstrloginscript: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Picture(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPicture(&self, vpicture: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn HomePage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHomePage(&self, bstrhomepage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Groups(&self) -> windows_core::Result<IADsMembers>;
    fn SetPassword(&self, newpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePassword(&self, bstroldpassword: &windows_core::BSTR, bstrnewpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsUser {}
#[cfg(feature = "Win32_System_Com")]
impl IADsUser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>() -> IADsUser_Vtbl {
        unsafe extern "system" fn BadLoginAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::BadLoginAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BadLoginCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::BadLoginCount(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LastLogin(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastLogoff<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LastLogoff(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastFailedLogin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LastFailedLogin(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PasswordLastChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PasswordLastChanged(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn Division<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Division(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDivision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdivision: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetDivision(this, core::mem::transmute(&bstrdivision)).into()
        }
        unsafe extern "system" fn Department<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Department(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdepartment: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetDepartment(this, core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn EmployeeID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::EmployeeID(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmployeeID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremployeeid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetEmployeeID(this, core::mem::transmute(&bstremployeeid)).into()
        }
        unsafe extern "system" fn FullName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::FullName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfullname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetFullName(this, core::mem::transmute(&bstrfullname)).into()
        }
        unsafe extern "system" fn FirstName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::FirstName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirstName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfirstname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetFirstName(this, core::mem::transmute(&bstrfirstname)).into()
        }
        unsafe extern "system" fn LastName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LastName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlastname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetLastName(this, core::mem::transmute(&bstrlastname)).into()
        }
        unsafe extern "system" fn OtherName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::OtherName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrothername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetOtherName(this, core::mem::transmute(&bstrothername)).into()
        }
        unsafe extern "system" fn NamePrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::NamePrefix(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamePrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnameprefix: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetNamePrefix(this, core::mem::transmute(&bstrnameprefix)).into()
        }
        unsafe extern "system" fn NameSuffix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::NameSuffix(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameSuffix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnamesuffix: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetNameSuffix(this, core::mem::transmute(&bstrnamesuffix)).into()
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Title(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetTitle(this, core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn Manager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Manager(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmanager: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetManager(this, core::mem::transmute(&bstrmanager)).into()
        }
        unsafe extern "system" fn TelephoneHome<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::TelephoneHome(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneHome<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonehome: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetTelephoneHome(this, core::mem::transmute(&vtelephonehome)).into()
        }
        unsafe extern "system" fn TelephoneMobile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::TelephoneMobile(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneMobile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonemobile: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetTelephoneMobile(this, core::mem::transmute(&vtelephonemobile)).into()
        }
        unsafe extern "system" fn TelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::TelephoneNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonenumber: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetTelephoneNumber(this, core::mem::transmute(&vtelephonenumber)).into()
        }
        unsafe extern "system" fn TelephonePager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::TelephonePager(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTelephonePager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtelephonepager: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetTelephonePager(this, core::mem::transmute(&vtelephonepager)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::FaxNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vfaxnumber: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetFaxNumber(this, core::mem::transmute(&vfaxnumber)).into()
        }
        unsafe extern "system" fn OfficeLocations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::OfficeLocations(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vofficelocations: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetOfficeLocations(this, core::mem::transmute(&vofficelocations)).into()
        }
        unsafe extern "system" fn PostalAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PostalAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostaladdresses: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPostalAddresses(this, core::mem::transmute(&vpostaladdresses)).into()
        }
        unsafe extern "system" fn PostalCodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PostalCodes(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostalCodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpostalcodes: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPostalCodes(this, core::mem::transmute(&vpostalcodes)).into()
        }
        unsafe extern "system" fn SeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::SeeAlso(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeeAlso<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vseealso: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetSeeAlso(this, core::mem::transmute(&vseealso)).into()
        }
        unsafe extern "system" fn AccountDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::AccountDisabled(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetAccountDisabled(this, core::mem::transmute_copy(&faccountdisabled)).into()
        }
        unsafe extern "system" fn AccountExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::AccountExpirationDate(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, daaccountexpirationdate: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetAccountExpirationDate(this, core::mem::transmute_copy(&daaccountexpirationdate)).into()
        }
        unsafe extern "system" fn GraceLoginsAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::GraceLoginsAllowed(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsallowed: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetGraceLoginsAllowed(this, core::mem::transmute_copy(&lngraceloginsallowed)).into()
        }
        unsafe extern "system" fn GraceLoginsRemaining<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::GraceLoginsRemaining(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lngraceloginsremaining: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetGraceLoginsRemaining(this, core::mem::transmute_copy(&lngraceloginsremaining)).into()
        }
        unsafe extern "system" fn IsAccountLocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::IsAccountLocked(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccountLocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetIsAccountLocked(this, core::mem::transmute_copy(&fisaccountlocked)).into()
        }
        unsafe extern "system" fn LoginHours<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LoginHours(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginHours<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginhours: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetLoginHours(this, core::mem::transmute(&vloginhours)).into()
        }
        unsafe extern "system" fn LoginWorkstations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LoginWorkstations(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginWorkstations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vloginworkstations: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetLoginWorkstations(this, core::mem::transmute(&vloginworkstations)).into()
        }
        unsafe extern "system" fn MaxLogins<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::MaxLogins(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxLogins<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxlogins: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetMaxLogins(this, core::mem::transmute_copy(&lnmaxlogins)).into()
        }
        unsafe extern "system" fn MaxStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::MaxStorage(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxStorage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnmaxstorage: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetMaxStorage(this, core::mem::transmute_copy(&lnmaxstorage)).into()
        }
        unsafe extern "system" fn PasswordExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PasswordExpirationDate(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dapasswordexpirationdate: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPasswordExpirationDate(this, core::mem::transmute_copy(&dapasswordexpirationdate)).into()
        }
        unsafe extern "system" fn PasswordMinimumLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PasswordMinimumLength(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnpasswordminimumlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPasswordMinimumLength(this, core::mem::transmute_copy(&lnpasswordminimumlength)).into()
        }
        unsafe extern "system" fn PasswordRequired<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::PasswordRequired(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPasswordRequired<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPasswordRequired(this, core::mem::transmute_copy(&fpasswordrequired)).into()
        }
        unsafe extern "system" fn RequireUniquePassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::RequireUniquePassword(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireUniquePassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetRequireUniquePassword(this, core::mem::transmute_copy(&frequireuniquepassword)).into()
        }
        unsafe extern "system" fn EmailAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::EmailAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstremailaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetEmailAddress(this, core::mem::transmute(&bstremailaddress)).into()
        }
        unsafe extern "system" fn HomeDirectory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::HomeDirectory(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomeDirectory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomedirectory: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetHomeDirectory(this, core::mem::transmute(&bstrhomedirectory)).into()
        }
        unsafe extern "system" fn Languages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Languages(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vlanguages: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetLanguages(this, core::mem::transmute(&vlanguages)).into()
        }
        unsafe extern "system" fn Profile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Profile(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprofile: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetProfile(this, core::mem::transmute(&bstrprofile)).into()
        }
        unsafe extern "system" fn LoginScript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::LoginScript(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoginScript<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloginscript: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetLoginScript(this, core::mem::transmute(&bstrloginscript)).into()
        }
        unsafe extern "system" fn Picture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Picture(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPicture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vpicture: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPicture(this, core::mem::transmute(&vpicture)).into()
        }
        unsafe extern "system" fn HomePage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::HomePage(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrhomepage: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetHomePage(this, core::mem::transmute(&bstrhomepage)).into()
        }
        unsafe extern "system" fn Groups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsUser_Impl::Groups(this) {
                Ok(ok__) => {
                    core::ptr::write(ppgroups, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::SetPassword(this, core::mem::transmute(&newpassword)).into()
        }
        unsafe extern "system" fn ChangePassword<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroldpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IADsUser_Impl::ChangePassword(this, core::mem::transmute(&bstroldpassword), core::mem::transmute(&bstrnewpassword)).into()
        }
        Self {
            base__: IADs_Vtbl::new::<Identity, Impl, OFFSET>(),
            BadLoginAddress: BadLoginAddress::<Identity, Impl, OFFSET>,
            BadLoginCount: BadLoginCount::<Identity, Impl, OFFSET>,
            LastLogin: LastLogin::<Identity, Impl, OFFSET>,
            LastLogoff: LastLogoff::<Identity, Impl, OFFSET>,
            LastFailedLogin: LastFailedLogin::<Identity, Impl, OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            EmployeeID: EmployeeID::<Identity, Impl, OFFSET>,
            SetEmployeeID: SetEmployeeID::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            SetFullName: SetFullName::<Identity, Impl, OFFSET>,
            FirstName: FirstName::<Identity, Impl, OFFSET>,
            SetFirstName: SetFirstName::<Identity, Impl, OFFSET>,
            LastName: LastName::<Identity, Impl, OFFSET>,
            SetLastName: SetLastName::<Identity, Impl, OFFSET>,
            OtherName: OtherName::<Identity, Impl, OFFSET>,
            SetOtherName: SetOtherName::<Identity, Impl, OFFSET>,
            NamePrefix: NamePrefix::<Identity, Impl, OFFSET>,
            SetNamePrefix: SetNamePrefix::<Identity, Impl, OFFSET>,
            NameSuffix: NameSuffix::<Identity, Impl, OFFSET>,
            SetNameSuffix: SetNameSuffix::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Manager: Manager::<Identity, Impl, OFFSET>,
            SetManager: SetManager::<Identity, Impl, OFFSET>,
            TelephoneHome: TelephoneHome::<Identity, Impl, OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Identity, Impl, OFFSET>,
            TelephoneMobile: TelephoneMobile::<Identity, Impl, OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            TelephonePager: TelephonePager::<Identity, Impl, OFFSET>,
            SetTelephonePager: SetTelephonePager::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            OfficeLocations: OfficeLocations::<Identity, Impl, OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Identity, Impl, OFFSET>,
            PostalAddresses: PostalAddresses::<Identity, Impl, OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Identity, Impl, OFFSET>,
            PostalCodes: PostalCodes::<Identity, Impl, OFFSET>,
            SetPostalCodes: SetPostalCodes::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            AccountDisabled: AccountDisabled::<Identity, Impl, OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Identity, Impl, OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Identity, Impl, OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Identity, Impl, OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Identity, Impl, OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Identity, Impl, OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Identity, Impl, OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Identity, Impl, OFFSET>,
            IsAccountLocked: IsAccountLocked::<Identity, Impl, OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Identity, Impl, OFFSET>,
            LoginHours: LoginHours::<Identity, Impl, OFFSET>,
            SetLoginHours: SetLoginHours::<Identity, Impl, OFFSET>,
            LoginWorkstations: LoginWorkstations::<Identity, Impl, OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Identity, Impl, OFFSET>,
            MaxLogins: MaxLogins::<Identity, Impl, OFFSET>,
            SetMaxLogins: SetMaxLogins::<Identity, Impl, OFFSET>,
            MaxStorage: MaxStorage::<Identity, Impl, OFFSET>,
            SetMaxStorage: SetMaxStorage::<Identity, Impl, OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Identity, Impl, OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Identity, Impl, OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Identity, Impl, OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Identity, Impl, OFFSET>,
            PasswordRequired: PasswordRequired::<Identity, Impl, OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Identity, Impl, OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Identity, Impl, OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Identity, Impl, OFFSET>,
            EmailAddress: EmailAddress::<Identity, Impl, OFFSET>,
            SetEmailAddress: SetEmailAddress::<Identity, Impl, OFFSET>,
            HomeDirectory: HomeDirectory::<Identity, Impl, OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            SetLanguages: SetLanguages::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            LoginScript: LoginScript::<Identity, Impl, OFFSET>,
            SetLoginScript: SetLoginScript::<Identity, Impl, OFFSET>,
            Picture: Picture::<Identity, Impl, OFFSET>,
            SetPicture: SetPicture::<Identity, Impl, OFFSET>,
            HomePage: HomePage::<Identity, Impl, OFFSET>,
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsUser as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IADs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsWinNTSystemInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PDC(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IADsWinNTSystemInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IADsWinNTSystemInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>() -> IADsWinNTSystemInfo_Vtbl {
        unsafe extern "system" fn UserName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsWinNTSystemInfo_Impl::UserName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsWinNTSystemInfo_Impl::ComputerName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsWinNTSystemInfo_Impl::DomainName(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IADsWinNTSystemInfo_Impl::PDC(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            PDC: PDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsWinNTSystemInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ICommonQuery_Impl: Sized {
    fn OpenQueryWindow(&self, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut Option<super::super::System::Com::IDataObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ICommonQuery {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ICommonQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: isize>() -> ICommonQuery_Vtbl {
        unsafe extern "system" fn OpenQueryWindow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICommonQuery_Impl::OpenQueryWindow(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pquerywnd), core::mem::transmute_copy(&ppdataobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenQueryWindow: OpenQueryWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommonQuery as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDirectoryObject_Impl: Sized {
    fn GetObjectInformation(&self) -> windows_core::Result<*mut ADS_OBJECT_INFO>;
    fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::Result<()>;
    fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<u32>;
    fn CreateDSObject(&self, pszrdnname: &windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn DeleteDSObject(&self, pszrdnname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDirectoryObject {}
#[cfg(feature = "Win32_System_Com")]
impl IDirectoryObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>() -> IDirectoryObject_Vtbl {
        unsafe extern "system" fn GetObjectInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDirectoryObject_Impl::GetObjectInformation(this) {
                Ok(ok__) => {
                    core::ptr::write(ppobjinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectoryObject_Impl::GetObjectAttributes(this, core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes), core::mem::transmute_copy(&ppattributeentries), core::mem::transmute_copy(&pdwnumattributesreturned)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDirectoryObject_Impl::SetObjectAttributes(this, core::mem::transmute_copy(&pattributeentries), core::mem::transmute_copy(&dwnumattributes)) {
                Ok(ok__) => {
                    core::ptr::write(pdwnumattributesmodified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDSObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrdnname: windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDirectoryObject_Impl::CreateDSObject(this, core::mem::transmute(&pszrdnname), core::mem::transmute_copy(&pattributeentries), core::mem::transmute_copy(&dwnumattributes)) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDSObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrdnname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectoryObject_Impl::DeleteDSObject(this, core::mem::transmute(&pszrdnname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            CreateDSObject: CreateDSObject::<Identity, Impl, OFFSET>,
            DeleteDSObject: DeleteDSObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectoryObject as windows_core::Interface>::IID
    }
}
pub trait IDirectorySchemaMgmt_Impl: Sized {
    fn EnumAttributes(&self, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> windows_core::Result<()>;
    fn CreateAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::Result<()>;
    fn WriteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::Result<()>;
    fn DeleteAttributeDefinition(&self, pszattributename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumClasses(&self, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> windows_core::Result<()>;
    fn WriteClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::Result<()>;
    fn CreateClassDefinition(&self, pszclassname: &windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::Result<()>;
    fn DeleteClassDefinition(&self, pszclassname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectorySchemaMgmt {}
impl IDirectorySchemaMgmt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>() -> IDirectorySchemaMgmt_Vtbl {
        unsafe extern "system" fn EnumAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::EnumAttributes(this, core::mem::transmute_copy(&ppszattrnames), core::mem::transmute_copy(&dwnumattributes), core::mem::transmute_copy(&ppattrdefinition), core::mem::transmute_copy(&pdwnumattributes)).into()
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::CreateAttributeDefinition(this, core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::WriteAttributeDefinition(this, core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pattributedefinition)).into()
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::DeleteAttributeDefinition(this, core::mem::transmute(&pszattributename)).into()
        }
        unsafe extern "system" fn EnumClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::EnumClasses(this, core::mem::transmute_copy(&ppszclassnames), core::mem::transmute_copy(&dwnumclasses), core::mem::transmute_copy(&ppclassdefinition), core::mem::transmute_copy(&pdwnumclasses)).into()
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::WriteClassDefinition(this, core::mem::transmute(&pszclassname), core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::CreateClassDefinition(this, core::mem::transmute(&pszclassname), core::mem::transmute_copy(&pclassdefinition)).into()
        }
        unsafe extern "system" fn DeleteClassDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclassname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySchemaMgmt_Impl::DeleteClassDefinition(this, core::mem::transmute(&pszclassname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Identity, Impl, OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Identity, Impl, OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Identity, Impl, OFFSET>,
            EnumClasses: EnumClasses::<Identity, Impl, OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Identity, Impl, OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Identity, Impl, OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectorySchemaMgmt as windows_core::Interface>::IID
    }
}
pub trait IDirectorySearch_Impl: Sized {
    fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::Result<()>;
    fn ExecuteSearch(&self, pszsearchfilter: &windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32) -> windows_core::Result<ADS_SEARCH_HANDLE>;
    fn AbandonSearch(&self, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
    fn GetFirstRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT;
    fn GetNextRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT;
    fn GetPreviousRow(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT;
    fn GetNextColumnName(&self, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut windows_core::PWSTR) -> windows_core::HRESULT;
    fn GetColumn(&self, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: &windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::Result<()>;
    fn CloseSearchHandle(&self, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectorySearch {}
impl IDirectorySearch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>() -> IDirectorySearch_Vtbl {
        unsafe extern "system" fn SetSearchPreference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::SetSearchPreference(this, core::mem::transmute_copy(&psearchprefs), core::mem::transmute_copy(&dwnumprefs)).into()
        }
        unsafe extern "system" fn ExecuteSearch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsearchfilter: windows_core::PCWSTR, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDirectorySearch_Impl::ExecuteSearch(this, core::mem::transmute(&pszsearchfilter), core::mem::transmute_copy(&pattributenames), core::mem::transmute_copy(&dwnumberattributes)) {
                Ok(ok__) => {
                    core::ptr::write(phsearchresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbandonSearch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::AbandonSearch(this, core::mem::transmute_copy(&phsearchresult)).into()
        }
        unsafe extern "system" fn GetFirstRow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::GetFirstRow(this, core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextRow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::GetNextRow(this, core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetPreviousRow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::GetPreviousRow(this, core::mem::transmute_copy(&hsearchresult))
        }
        unsafe extern "system" fn GetNextColumnName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::GetNextColumnName(this, core::mem::transmute_copy(&hsearchhandle), core::mem::transmute_copy(&ppszcolumnname))
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::GetColumn(this, core::mem::transmute_copy(&hsearchresult), core::mem::transmute(&szcolumnname), core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn FreeColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::FreeColumn(this, core::mem::transmute_copy(&psearchcolumn)).into()
        }
        unsafe extern "system" fn CloseSearchHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirectorySearch_Impl::CloseSearchHandle(this, core::mem::transmute_copy(&hsearchresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSearchPreference: SetSearchPreference::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
            AbandonSearch: AbandonSearch::<Identity, Impl, OFFSET>,
            GetFirstRow: GetFirstRow::<Identity, Impl, OFFSET>,
            GetNextRow: GetNextRow::<Identity, Impl, OFFSET>,
            GetPreviousRow: GetPreviousRow::<Identity, Impl, OFFSET>,
            GetNextColumnName: GetNextColumnName::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            FreeColumn: FreeColumn::<Identity, Impl, OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectorySearch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDsAdminCreateObj_Impl: Sized {
    fn Initialize(&self, padscontainerobj: Option<&IADsContainer>, padscopysource: Option<&IADs>, lpszclassname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateModal(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<IADs>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDsAdminCreateObj {}
#[cfg(feature = "Win32_System_Com")]
impl IDsAdminCreateObj_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>() -> IDsAdminCreateObj_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padscontainerobj: *mut core::ffi::c_void, padscopysource: *mut core::ffi::c_void, lpszclassname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminCreateObj_Impl::Initialize(this, windows_core::from_raw_borrowed(&padscontainerobj), windows_core::from_raw_borrowed(&padscopysource), core::mem::transmute(&lpszclassname)).into()
        }
        unsafe extern "system" fn CreateModal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDsAdminCreateObj_Impl::CreateModal(this, core::mem::transmute_copy(&hwndparent)) {
                Ok(ok__) => {
                    core::ptr::write(ppadsobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateModal: CreateModal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsAdminCreateObj as windows_core::Interface>::IID
    }
}
pub trait IDsAdminNewObj_Impl: Sized {
    fn SetButtons(&self, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetPageCounts(&self, pntotal: *mut i32, pnstartindex: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDsAdminNewObj {}
impl IDsAdminNewObj_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>() -> IDsAdminNewObj_Vtbl {
        unsafe extern "system" fn SetButtons<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObj_Impl::SetButtons(this, core::mem::transmute_copy(&ncurrindex), core::mem::transmute_copy(&bvalid)).into()
        }
        unsafe extern "system" fn GetPageCounts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObj_Impl::GetPageCounts(this, core::mem::transmute_copy(&pntotal), core::mem::transmute_copy(&pnstartindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetButtons: SetButtons::<Identity, Impl, OFFSET>,
            GetPageCounts: GetPageCounts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObj as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExt_Impl: Sized {
    fn Initialize(&self, padscontainerobj: Option<&IADsContainer>, padscopysource: Option<&IADs>, lpszclassname: &windows_core::PCWSTR, pdsadminnewobj: Option<&IDsAdminNewObj>, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> windows_core::Result<()>;
    fn AddPages(&self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetObject(&self, padsobj: Option<&IADs>) -> windows_core::Result<()>;
    fn WriteData(&self, hwnd: super::super::Foundation::HWND, ucontext: u32) -> windows_core::Result<()>;
    fn OnError(&self, hwnd: super::super::Foundation::HWND, hr: windows_core::HRESULT, ucontext: u32) -> windows_core::Result<()>;
    fn GetSummaryInfo(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IDsAdminNewObjExt {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDsAdminNewObjExt_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>() -> IDsAdminNewObjExt_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padscontainerobj: *mut core::ffi::c_void, padscopysource: *mut core::ffi::c_void, lpszclassname: windows_core::PCWSTR, pdsadminnewobj: *mut core::ffi::c_void, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::Initialize(this, windows_core::from_raw_borrowed(&padscontainerobj), windows_core::from_raw_borrowed(&padscopysource), core::mem::transmute(&lpszclassname), windows_core::from_raw_borrowed(&pdsadminnewobj), core::mem::transmute_copy(&pdispinfo)).into()
        }
        unsafe extern "system" fn AddPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::AddPages(this, core::mem::transmute_copy(&lpfnaddpage), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padsobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::SetObject(this, windows_core::from_raw_borrowed(&padsobj)).into()
        }
        unsafe extern "system" fn WriteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::WriteData(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: windows_core::HRESULT, ucontext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::OnError(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&hr), core::mem::transmute_copy(&ucontext)).into()
        }
        unsafe extern "system" fn GetSummaryInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjExt_Impl::GetSummaryInfo(this, core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            GetSummaryInfo: GetSummaryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObjExt as windows_core::Interface>::IID
    }
}
pub trait IDsAdminNewObjPrimarySite_Impl: Sized {
    fn CreateNew(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDsAdminNewObjPrimarySite {}
impl IDsAdminNewObjPrimarySite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>() -> IDsAdminNewObjPrimarySite_Vtbl {
        unsafe extern "system" fn CreateNew<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjPrimarySite_Impl::CreateNew(this, core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNewObjPrimarySite_Impl::Commit(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateNew: CreateNew::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsAdminNewObjPrimarySite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDsAdminNotifyHandler_Impl: Sized {
    fn Initialize(&self, pextrainfo: Option<&super::super::System::Com::IDataObject>, pueventflags: *mut u32) -> windows_core::Result<()>;
    fn Begin(&self, uevent: u32, parg1: Option<&super::super::System::Com::IDataObject>, parg2: Option<&super::super::System::Com::IDataObject>, puflags: *mut u32, pbstr: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Notify(&self, nitem: u32, uflags: u32) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDsAdminNotifyHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IDsAdminNotifyHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>() -> IDsAdminNotifyHandler_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextrainfo: *mut core::ffi::c_void, pueventflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNotifyHandler_Impl::Initialize(this, windows_core::from_raw_borrowed(&pextrainfo), core::mem::transmute_copy(&pueventflags)).into()
        }
        unsafe extern "system" fn Begin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uevent: u32, parg1: *mut core::ffi::c_void, parg2: *mut core::ffi::c_void, puflags: *mut u32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNotifyHandler_Impl::Begin(this, core::mem::transmute_copy(&uevent), windows_core::from_raw_borrowed(&parg1), windows_core::from_raw_borrowed(&parg2), core::mem::transmute_copy(&puflags), core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitem: u32, uflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNotifyHandler_Impl::Notify(this, core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsAdminNotifyHandler_Impl::End(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsAdminNotifyHandler as windows_core::Interface>::IID
    }
}
pub trait IDsBrowseDomainTree_Impl: Sized {
    fn BrowseTo(&self, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut windows_core::PWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> windows_core::Result<()>;
    fn FreeDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE) -> windows_core::Result<()>;
    fn FlushCachedDomains(&self) -> windows_core::Result<()>;
    fn SetComputer(&self, pszcomputername: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDsBrowseDomainTree {}
impl IDsBrowseDomainTree_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>() -> IDsBrowseDomainTree_Vtbl {
        unsafe extern "system" fn BrowseTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut windows_core::PWSTR, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsBrowseDomainTree_Impl::BrowseTo(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ppsztargetpath), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetDomains<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsBrowseDomainTree_Impl::GetDomains(this, core::mem::transmute_copy(&ppdomaintree), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FreeDomains<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsBrowseDomainTree_Impl::FreeDomains(this, core::mem::transmute_copy(&ppdomaintree)).into()
        }
        unsafe extern "system" fn FlushCachedDomains<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsBrowseDomainTree_Impl::FlushCachedDomains(this).into()
        }
        unsafe extern "system" fn SetComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcomputername: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszpassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsBrowseDomainTree_Impl::SetComputer(this, core::mem::transmute(&pszcomputername), core::mem::transmute(&pszusername), core::mem::transmute(&pszpassword)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BrowseTo: BrowseTo::<Identity, Impl, OFFSET>,
            GetDomains: GetDomains::<Identity, Impl, OFFSET>,
            FreeDomains: FreeDomains::<Identity, Impl, OFFSET>,
            FlushCachedDomains: FlushCachedDomains::<Identity, Impl, OFFSET>,
            SetComputer: SetComputer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsBrowseDomainTree as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IDsDisplaySpecifier_Impl: Sized {
    fn SetServer(&self, pszserver: &windows_core::PCWSTR, pszusername: &windows_core::PCWSTR, pszpassword: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn SetLanguageID(&self, langid: u16) -> windows_core::Result<()>;
    fn GetDisplaySpecifier(&self, pszobjectclass: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetIconLocation(&self, pszobjectclass: &windows_core::PCWSTR, dwflags: u32, pszbuffer: windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> windows_core::Result<()>;
    fn GetIcon(&self, pszobjectclass: &windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    fn GetFriendlyClassName(&self, pszobjectclass: &windows_core::PCWSTR, pszbuffer: windows_core::PWSTR, cchbuffer: i32) -> windows_core::Result<()>;
    fn GetFriendlyAttributeName(&self, pszobjectclass: &windows_core::PCWSTR, pszattributename: &windows_core::PCWSTR, pszbuffer: windows_core::PWSTR, cchbuffer: u32) -> windows_core::Result<()>;
    fn IsClassContainer(&self, pszobjectclass: &windows_core::PCWSTR, pszadspath: &windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    fn GetClassCreationInfo(&self, pszobjectclass: &windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> windows_core::Result<()>;
    fn EnumClassAttributes(&self, pszobjectclass: &windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn GetAttributeADsType(&self, pszattributename: &windows_core::PCWSTR) -> ADSTYPE;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IDsDisplaySpecifier {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IDsDisplaySpecifier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>() -> IDsDisplaySpecifier_Vtbl {
        unsafe extern "system" fn SetServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserver: windows_core::PCWSTR, pszusername: windows_core::PCWSTR, pszpassword: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::SetServer(this, core::mem::transmute(&pszserver), core::mem::transmute(&pszusername), core::mem::transmute(&pszpassword), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetLanguageID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::SetLanguageID(this, core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn GetDisplaySpecifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetDisplaySpecifier(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetIconLocation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, dwflags: u32, pszbuffer: windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetIconLocation(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchbuffer), core::mem::transmute_copy(&presid)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetIcon(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&cxicon), core::mem::transmute_copy(&cyicon))
        }
        unsafe extern "system" fn GetFriendlyClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, pszbuffer: windows_core::PWSTR, cchbuffer: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetFriendlyClassName(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, pszattributename: windows_core::PCWSTR, pszbuffer: windows_core::PWSTR, cchbuffer: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetFriendlyAttributeName(this, core::mem::transmute(&pszobjectclass), core::mem::transmute(&pszattributename), core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn IsClassContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, pszadspath: windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::IsClassContainer(this, core::mem::transmute(&pszobjectclass), core::mem::transmute(&pszadspath), core::mem::transmute_copy(&dwflags))
        }
        unsafe extern "system" fn GetClassCreationInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetClassCreationInfo(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&ppdscci)).into()
        }
        unsafe extern "system" fn EnumClassAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszobjectclass: windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::EnumClassAttributes(this, core::mem::transmute(&pszobjectclass), core::mem::transmute_copy(&pcbenum), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn GetAttributeADsType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributename: windows_core::PCWSTR) -> ADSTYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsDisplaySpecifier_Impl::GetAttributeADsType(this, core::mem::transmute(&pszattributename))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetDisplaySpecifier: GetDisplaySpecifier::<Identity, Impl, OFFSET>,
            GetIconLocation: GetIconLocation::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetFriendlyClassName: GetFriendlyClassName::<Identity, Impl, OFFSET>,
            GetFriendlyAttributeName: GetFriendlyAttributeName::<Identity, Impl, OFFSET>,
            IsClassContainer: IsClassContainer::<Identity, Impl, OFFSET>,
            GetClassCreationInfo: GetClassCreationInfo::<Identity, Impl, OFFSET>,
            EnumClassAttributes: EnumClassAttributes::<Identity, Impl, OFFSET>,
            GetAttributeADsType: GetAttributeADsType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsDisplaySpecifier as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDsObjectPicker_Impl: Sized {
    fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> windows_core::Result<()>;
    fn InvokeDialog(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<super::super::System::Com::IDataObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDsObjectPicker {}
#[cfg(feature = "Win32_System_Com")]
impl IDsObjectPicker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>() -> IDsObjectPicker_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsObjectPicker_Impl::Initialize(this, core::mem::transmute_copy(&pinitinfo)).into()
        }
        unsafe extern "system" fn InvokeDialog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDsObjectPicker_Impl::InvokeDialog(this, core::mem::transmute_copy(&hwndparent)) {
                Ok(ok__) => {
                    core::ptr::write(ppdoselections, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            InvokeDialog: InvokeDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsObjectPicker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDsObjectPickerCredentials_Impl: Sized + IDsObjectPicker_Impl {
    fn SetCredentials(&self, szusername: &windows_core::PCWSTR, szpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDsObjectPickerCredentials {}
#[cfg(feature = "Win32_System_Com")]
impl IDsObjectPickerCredentials_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>() -> IDsObjectPickerCredentials_Vtbl {
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szusername: windows_core::PCWSTR, szpassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDsObjectPickerCredentials_Impl::SetCredentials(this, core::mem::transmute(&szusername), core::mem::transmute(&szpassword)).into()
        }
        Self { base__: IDsObjectPicker_Vtbl::new::<Identity, Impl, OFFSET>(), SetCredentials: SetCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDsObjectPickerCredentials as windows_core::Interface>::IID || iid == &<IDsObjectPicker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistQuery_Impl: Sized + super::super::System::Com::IPersist_Impl {
    fn WriteString(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, pvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReadString(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, pbuffer: windows_core::PWSTR, cchbuffer: i32) -> windows_core::Result<()>;
    fn WriteInt(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, value: i32) -> windows_core::Result<()>;
    fn ReadInt(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, pvalue: *mut i32) -> windows_core::Result<()>;
    fn WriteStruct(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::Result<()>;
    fn ReadStruct(&self, psection: &windows_core::PCWSTR, pvaluename: &windows_core::PCWSTR, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPersistQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>() -> IPersistQuery_Vtbl {
        unsafe extern "system" fn WriteString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, pvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::WriteString(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn ReadString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, pbuffer: windows_core::PWSTR, cchbuffer: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::ReadString(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cchbuffer)).into()
        }
        unsafe extern "system" fn WriteInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::WriteInt(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::ReadInt(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn WriteStruct<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::WriteStruct(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&pstruct), core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn ReadStruct<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psection: windows_core::PCWSTR, pvaluename: windows_core::PCWSTR, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::ReadStruct(this, core::mem::transmute(&psection), core::mem::transmute(&pvaluename), core::mem::transmute_copy(&pstruct), core::mem::transmute_copy(&cbstruct)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPersistQuery_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::System::Com::IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            WriteInt: WriteInt::<Identity, Impl, OFFSET>,
            ReadInt: ReadInt::<Identity, Impl, OFFSET>,
            WriteStruct: WriteStruct::<Identity, Impl, OFFSET>,
            ReadStruct: ReadStruct::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistQuery as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPrivateDispatch_Impl: Sized {
    fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::Result<()>;
    fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<super::super::System::Com::ITypeInfo>;
    fn ADSIGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> windows_core::Result<i32>;
    fn ADSIInvoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPrivateDispatch {}
#[cfg(feature = "Win32_System_Com")]
impl IPrivateDispatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>() -> IPrivateDispatch_Vtbl {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwextensionid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrivateDispatch_Impl::ADSIInitializeDispatchManager(this, core::mem::transmute_copy(&dwextensionid)).into()
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctinfo: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrivateDispatch_Impl::ADSIGetTypeInfoCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pctinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrivateDispatch_Impl::ADSIGetTypeInfo(this, core::mem::transmute_copy(&itinfo), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    core::ptr::write(pptinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrivateDispatch_Impl::ADSIGetIDsOfNames(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rgsznames), core::mem::transmute_copy(&cnames), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    core::ptr::write(rgdispid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADSIInvoke<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrivateDispatch_Impl::ADSIInvoke(this, core::mem::transmute_copy(&dispidmember), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Identity, Impl, OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Identity, Impl, OFFSET>,
            ADSIInvoke: ADSIInvoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrivateDispatch as windows_core::Interface>::IID
    }
}
pub trait IPrivateUnknown_Impl: Sized {
    fn ADSIInitializeObject(&self, lpszusername: &windows_core::BSTR, lpszpassword: &windows_core::BSTR, lnreserved: i32) -> windows_core::Result<()>;
    fn ADSIReleaseObject(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrivateUnknown {}
impl IPrivateUnknown_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>() -> IPrivateUnknown_Vtbl {
        unsafe extern "system" fn ADSIInitializeObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszusername: core::mem::MaybeUninit<windows_core::BSTR>, lpszpassword: core::mem::MaybeUninit<windows_core::BSTR>, lnreserved: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrivateUnknown_Impl::ADSIInitializeObject(this, core::mem::transmute(&lpszusername), core::mem::transmute(&lpszpassword), core::mem::transmute_copy(&lnreserved)).into()
        }
        unsafe extern "system" fn ADSIReleaseObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrivateUnknown_Impl::ADSIReleaseObject(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ADSIInitializeObject: ADSIInitializeObject::<Identity, Impl, OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrivateUnknown as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryForm_Impl: Sized {
    fn Initialize(&self, hkform: super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn AddForms(&self, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn AddPages(&self, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IQueryForm {}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl IQueryForm_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>() -> IQueryForm_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IQueryForm_Impl::Initialize(this, core::mem::transmute_copy(&hkform)).into()
        }
        unsafe extern "system" fn AddForms<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IQueryForm_Impl::AddForms(this, core::mem::transmute_copy(&paddformsproc), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn AddPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IQueryForm_Impl::AddPages(this, core::mem::transmute_copy(&paddpagesproc), core::mem::transmute_copy(&lparam)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddForms: AddForms::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryForm as windows_core::Interface>::IID
    }
}
