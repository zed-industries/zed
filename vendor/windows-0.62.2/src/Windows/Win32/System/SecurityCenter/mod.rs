#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wscapi.dll" "system" fn WscGetAntiMalwareUri(ppszuri : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WscGetAntiMalwareUri(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_core::Result<()> {
    windows_core::link!("wscapi.dll" "system" fn WscGetSecurityProviderHealth(providers : u32, phealth : *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_core::HRESULT);
    unsafe { WscGetSecurityProviderHealth(providers, phealth as _).ok() }
}
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> windows_core::Result<()> {
    windows_core::link!("wscapi.dll" "system" fn WscQueryAntiMalwareUri() -> windows_core::HRESULT);
    unsafe { WscQueryAntiMalwareUri().ok() }
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wscapi.dll" "system" fn WscRegisterForChanges(reserved : *mut core::ffi::c_void, phcallbackregistration : *mut super::super::Foundation:: HANDLE, lpcallbackaddress : super::Threading:: LPTHREAD_START_ROUTINE, pcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WscRegisterForChanges(reserved as _, phcallbackregistration as _, lpcallbackaddress, pcontext as _).ok() }
}
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> windows_core::Result<()> {
    windows_core::link!("wscapi.dll" "system" fn WscRegisterForUserNotifications() -> windows_core::HRESULT);
    unsafe { WscRegisterForUserNotifications().ok() }
}
#[inline]
pub unsafe fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wscapi.dll" "system" fn WscUnRegisterChanges(hregistrationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WscUnRegisterChanges(hregistrationhandle).ok() }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSCDefaultProduct, IWSCDefaultProduct_Vtbl, 0x0476d69c_f21a_11e5_9ce9_5e5517507c66);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSCDefaultProduct {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSCDefaultProduct, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSCDefaultProduct {
    pub unsafe fn SetDefaultProduct(&self, etype: SECURITY_PRODUCT_TYPE, pguid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultProduct)(windows_core::Interface::as_raw(self), etype, core::mem::transmute_copy(pguid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCDefaultProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetDefaultProduct: unsafe extern "system" fn(*mut core::ffi::c_void, SECURITY_PRODUCT_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSCDefaultProduct_Impl: super::Com::IDispatch_Impl {
    fn SetDefaultProduct(&self, etype: SECURITY_PRODUCT_TYPE, pguid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSCDefaultProduct_Vtbl {
    pub const fn new<Identity: IWSCDefaultProduct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDefaultProduct<Identity: IWSCDefaultProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSCDefaultProduct_Impl::SetDefaultProduct(this, core::mem::transmute_copy(&etype), core::mem::transmute(&pguid)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), SetDefaultProduct: SetDefaultProduct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSCDefaultProduct as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSCDefaultProduct {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSCProductList, IWSCProductList_Vtbl, 0x722a338c_6e8e_4e72_ac27_1417fb0c81c2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSCProductList {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSCProductList, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSCProductList {
    pub unsafe fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), provider.0 as _).ok() }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Item(&self, index: u32) -> windows_core::Result<IWscProduct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCProductList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSCProductList_Impl: super::Com::IDispatch_Impl {
    fn Initialize(&self, provider: &WSC_SECURITY_PROVIDER) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: u32) -> windows_core::Result<IWscProduct>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSCProductList_Vtbl {
    pub const fn new<Identity: IWSCProductList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWSCProductList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSCProductList_Impl::Initialize(this, core::mem::transmute(&provider)).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IWSCProductList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSCProductList_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IWSCProductList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSCProductList_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSCProductList as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSCProductList {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWscProduct, IWscProduct_Vtbl, 0x8c38232e_3a45_4a27_92b0_1a16a975f669);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWscProduct {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWscProduct, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct {
    pub unsafe fn ProductName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductState(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SignatureStatus(&self) -> windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SignatureStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemediationPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemediationPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductStateTimestamp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductStateTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductIsDefault(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductIsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProductName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_STATE) -> windows_core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_SIGNATURE_STATUS) -> windows_core::HRESULT,
    pub RemediationPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductStateTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductIsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct_Impl: super::Com::IDispatch_Impl {
    fn ProductName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductState(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_STATE>;
    fn SignatureStatus(&self) -> windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS>;
    fn RemediationPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductStateTimestamp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductIsDefault(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWscProduct_Vtbl {
    pub const fn new<Identity: IWscProduct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProductName<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::ProductName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductState<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::ProductState(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SignatureStatus<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::SignatureStatus(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemediationPath<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::RemediationPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductStateTimestamp<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::ProductStateTimestamp(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductGuid<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::ProductGuid(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductIsDefault<Identity: IWscProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct_Impl::ProductIsDefault(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProductName: ProductName::<Identity, OFFSET>,
            ProductState: ProductState::<Identity, OFFSET>,
            SignatureStatus: SignatureStatus::<Identity, OFFSET>,
            RemediationPath: RemediationPath::<Identity, OFFSET>,
            ProductStateTimestamp: ProductStateTimestamp::<Identity, OFFSET>,
            ProductGuid: ProductGuid::<Identity, OFFSET>,
            ProductIsDefault: ProductIsDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWscProduct as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWscProduct {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWscProduct2, IWscProduct2_Vtbl, 0xf896ca54_fe09_4403_86d4_23cb488d81d8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWscProduct2 {
    type Target = IWscProduct;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWscProduct2, windows_core::IUnknown, super::Com::IDispatch, IWscProduct);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct2 {
    pub unsafe fn AntivirusScanSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntivirusScanSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntivirusSettingsSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntivirusProtectionUpdateSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallDomainProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallPrivateProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallPublicProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct2_Vtbl {
    pub base__: IWscProduct_Vtbl,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct2_Impl: IWscProduct_Impl {
    fn AntivirusScanSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusSettingsSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusProtectionUpdateSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallDomainProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPrivateProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPublicProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWscProduct2_Vtbl {
    pub const fn new<Identity: IWscProduct2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AntivirusScanSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::AntivirusScanSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::AntivirusSettingsSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::AntivirusProtectionUpdateSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::FirewallDomainProfileSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::FirewallPrivateProfileSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Identity: IWscProduct2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct2_Impl::FirewallPublicProfileSubstatus(this) {
                    Ok(ok__) => {
                        pestatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWscProduct_Vtbl::new::<Identity, OFFSET>(),
            AntivirusScanSubstatus: AntivirusScanSubstatus::<Identity, OFFSET>,
            AntivirusSettingsSubstatus: AntivirusSettingsSubstatus::<Identity, OFFSET>,
            AntivirusProtectionUpdateSubstatus: AntivirusProtectionUpdateSubstatus::<Identity, OFFSET>,
            FirewallDomainProfileSubstatus: FirewallDomainProfileSubstatus::<Identity, OFFSET>,
            FirewallPrivateProfileSubstatus: FirewallPrivateProfileSubstatus::<Identity, OFFSET>,
            FirewallPublicProfileSubstatus: FirewallPublicProfileSubstatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWscProduct2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWscProduct as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWscProduct2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWscProduct3, IWscProduct3_Vtbl, 0x55536524_d1d1_4726_8c7c_04996a1904e7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWscProduct3 {
    type Target = IWscProduct2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWscProduct3, windows_core::IUnknown, super::Com::IDispatch, IWscProduct, IWscProduct2);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct3 {
    pub unsafe fn AntivirusDaysUntilExpired(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntivirusDaysUntilExpired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct3_Vtbl {
    pub base__: IWscProduct2_Vtbl,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct3_Impl: IWscProduct2_Impl {
    fn AntivirusDaysUntilExpired(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWscProduct3_Vtbl {
    pub const fn new<Identity: IWscProduct3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Identity: IWscProduct3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdays: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWscProduct3_Impl::AntivirusDaysUntilExpired(this) {
                    Ok(ok__) => {
                        pdwdays.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWscProduct2_Vtbl::new::<Identity, OFFSET>(), AntivirusDaysUntilExpired: AntivirusDaysUntilExpired::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWscProduct3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWscProduct as windows_core::Interface>::IID || iid == &<IWscProduct2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWscProduct3 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
pub const WSCDefaultProduct: windows_core::GUID = windows_core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
pub const WSCProductList: windows_core::GUID = windows_core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
