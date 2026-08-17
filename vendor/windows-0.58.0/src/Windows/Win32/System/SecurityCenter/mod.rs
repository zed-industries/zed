#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("wscapi.dll" "system" fn WscGetAntiMalwareUri(ppszuri : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WscGetAntiMalwareUri(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_core::Result<()> {
    windows_targets::link!("wscapi.dll" "system" fn WscGetSecurityProviderHealth(providers : u32, phealth : *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_core::HRESULT);
    WscGetSecurityProviderHealth(providers, phealth).ok()
}
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> windows_core::Result<()> {
    windows_targets::link!("wscapi.dll" "system" fn WscQueryAntiMalwareUri() -> windows_core::HRESULT);
    WscQueryAntiMalwareUri().ok()
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("wscapi.dll" "system" fn WscRegisterForChanges(reserved : *mut core::ffi::c_void, phcallbackregistration : *mut super::super::Foundation:: HANDLE, lpcallbackaddress : super::Threading:: LPTHREAD_START_ROUTINE, pcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    WscRegisterForChanges(reserved, phcallbackregistration, lpcallbackaddress, pcontext).ok()
}
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> windows_core::Result<()> {
    windows_targets::link!("wscapi.dll" "system" fn WscRegisterForUserNotifications() -> windows_core::HRESULT);
    WscRegisterForUserNotifications().ok()
}
#[inline]
pub unsafe fn WscUnRegisterChanges<P0>(hregistrationhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wscapi.dll" "system" fn WscUnRegisterChanges(hregistrationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    WscUnRegisterChanges(hregistrationhandle.param().abi()).ok()
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
    pub unsafe fn SetDefaultProduct<P0>(&self, etype: SECURITY_PRODUCT_TYPE, pguid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultProduct)(windows_core::Interface::as_raw(self), etype, pguid.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWSCDefaultProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetDefaultProduct: unsafe extern "system" fn(*mut core::ffi::c_void, SECURITY_PRODUCT_TYPE, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), provider.0 as _).ok()
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: u32) -> windows_core::Result<IWscProduct> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWSCProductList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductState(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SignatureStatus(&self) -> windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SignatureStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemediationPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemediationPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductStateTimestamp(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductStateTimestamp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProductIsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProductIsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProductName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_STATE) -> windows_core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_SIGNATURE_STATUS) -> windows_core::HRESULT,
    pub RemediationPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductStateTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProductIsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AntivirusScanSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AntivirusSettingsSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AntivirusProtectionUpdateSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirewallDomainProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirewallPrivateProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirewallPublicProfileSubstatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct2_Vtbl {
    pub base__: IWscProduct_Vtbl,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AntivirusDaysUntilExpired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct3_Vtbl {
    pub base__: IWscProduct2_Vtbl,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
impl windows_core::TypeKind for SECURITY_PRODUCT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_PRODUCT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
impl windows_core::TypeKind for WSC_SECURITY_PRODUCT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSC_SECURITY_PRODUCT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
impl windows_core::TypeKind for WSC_SECURITY_PRODUCT_SUBSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_SUBSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
impl windows_core::TypeKind for WSC_SECURITY_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSC_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
impl windows_core::TypeKind for WSC_SECURITY_PROVIDER_HEALTH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSC_SECURITY_PROVIDER_HEALTH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER_HEALTH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
impl windows_core::TypeKind for WSC_SECURITY_SIGNATURE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSC_SECURITY_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
pub const WSCDefaultProduct: windows_core::GUID = windows_core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
pub const WSCProductList: windows_core::GUID = windows_core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
#[cfg(feature = "implement")]
core::include!("impl.rs");
