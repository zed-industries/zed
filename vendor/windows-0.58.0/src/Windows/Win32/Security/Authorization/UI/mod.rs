#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn CreateSecurityPage<P0>(psi: P0) -> windows_core::Result<super::super::super::UI::Controls::HPROPSHEETPAGE>
where
    P0: windows_core::Param<ISecurityInformation>,
{
    windows_targets::link!("aclui.dll" "system" fn CreateSecurityPage(psi : * mut core::ffi::c_void) -> super::super::super::UI::Controls:: HPROPSHEETPAGE);
    let result__ = CreateSecurityPage(psi.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EditSecurity<P0, P1>(hwndowner: P0, psi: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<ISecurityInformation>,
{
    windows_targets::link!("aclui.dll" "system" fn EditSecurity(hwndowner : super::super::super::Foundation:: HWND, psi : * mut core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    EditSecurity(hwndowner.param().abi(), psi.param().abi()).ok()
}
#[inline]
pub unsafe fn EditSecurityAdvanced<P0, P1>(hwndowner: P0, psi: P1, usipage: SI_PAGE_TYPE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<ISecurityInformation>,
{
    windows_targets::link!("aclui.dll" "system" fn EditSecurityAdvanced(hwndowner : super::super::super::Foundation:: HWND, psi : * mut core::ffi::c_void, usipage : SI_PAGE_TYPE) -> windows_core::HRESULT);
    EditSecurityAdvanced(hwndowner.param().abi(), psi.param().abi(), usipage).ok()
}
windows_core::imp::define_interface!(IEffectivePermission, IEffectivePermission_Vtbl, 0x3853dc76_9f35_407c_88a1_d19344365fbc);
impl core::ops::Deref for IEffectivePermission {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEffectivePermission, windows_core::IUnknown);
impl IEffectivePermission {
    pub unsafe fn GetEffectivePermission<P0, P1, P2>(&self, pguidobjecttype: *const windows_core::GUID, pusersid: P0, pszservername: P1, psd: P2, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::PSID>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::PSECURITY_DESCRIPTOR>,
    {
        (windows_core::Interface::vtable(self).GetEffectivePermission)(windows_core::Interface::as_raw(self), pguidobjecttype, pusersid.param().abi(), pszservername.param().abi(), psd.param().abi(), ppobjecttypelist, pcobjecttypelistlength, ppgrantedaccesslist, pcgrantedaccesslistlength).ok()
    }
}
#[repr(C)]
pub struct IEffectivePermission_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEffectivePermission: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::PSID, windows_core::PCWSTR, super::super::PSECURITY_DESCRIPTOR, *mut *mut super::super::OBJECT_TYPE_LIST, *mut u32, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEffectivePermission2, IEffectivePermission2_Vtbl, 0x941fabca_dd47_4fca_90bb_b0e10255f20d);
impl core::ops::Deref for IEffectivePermission2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEffectivePermission2, windows_core::IUnknown);
impl IEffectivePermission2 {
    pub unsafe fn ComputeEffectivePermissionWithSecondarySecurity<P0, P1, P2>(
        &self,
        psid: P0,
        pdevicesid: P1,
        pszservername: P2,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: Option<*const super::super::TOKEN_GROUPS>,
        pauthzusergroupsoperations: Option<*const super::AUTHZ_SID_OPERATION>,
        pdevicegroups: Option<*const super::super::TOKEN_GROUPS>,
        pauthzdevicegroupsoperations: Option<*const super::AUTHZ_SID_OPERATION>,
        pauthzuserclaims: Option<*const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>,
        pauthzuserclaimsoperations: Option<*const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION>,
        pauthzdeviceclaims: Option<*const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>,
        pauthzdeviceclaimsoperations: Option<*const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION>,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::PSID>,
        P1: windows_core::Param<super::super::PSID>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ComputeEffectivePermissionWithSecondarySecurity)(
            windows_core::Interface::as_raw(self),
            psid.param().abi(),
            pdevicesid.param().abi(),
            pszservername.param().abi(),
            psecurityobjects,
            dwsecurityobjectcount,
            core::mem::transmute(pusergroups.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzusergroupsoperations.unwrap_or(std::ptr::null())),
            core::mem::transmute(pdevicegroups.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzdevicegroupsoperations.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzuserclaims.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzuserclaimsoperations.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzdeviceclaims.unwrap_or(std::ptr::null())),
            core::mem::transmute(pauthzdeviceclaimsoperations.unwrap_or(std::ptr::null())),
            peffpermresultlists,
        )
        .ok()
    }
}
#[repr(C)]
pub struct IEffectivePermission2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComputeEffectivePermissionWithSecondarySecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::PSID, super::super::PSID, windows_core::PCWSTR, *mut SECURITY_OBJECT, u32, *const super::super::TOKEN_GROUPS, *const super::AUTHZ_SID_OPERATION, *const super::super::TOKEN_GROUPS, *const super::AUTHZ_SID_OPERATION, *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION, *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION, *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION, *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION, *mut EFFPERM_RESULT_LIST) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISecurityInformation, ISecurityInformation_Vtbl, 0x965fc360_16ff_11d0_91cb_00aa00bbb723);
impl core::ops::Deref for ISecurityInformation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityInformation, windows_core::IUnknown);
impl ISecurityInformation {
    pub unsafe fn GetObjectInformation(&self, pobjectinfo: *mut SI_OBJECT_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectInformation)(windows_core::Interface::as_raw(self), pobjectinfo).ok()
    }
    pub unsafe fn GetSecurity<P0>(&self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), requestedinformation, ppsecuritydescriptor, fdefault.param().abi()).ok()
    }
    pub unsafe fn SetSecurity<P0>(&self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::PSECURITY_DESCRIPTOR>,
    {
        (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), securityinformation, psecuritydescriptor.param().abi()).ok()
    }
    pub unsafe fn GetAccessRights(&self, pguidobjecttype: *const windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAccessRights)(windows_core::Interface::as_raw(self), pguidobjecttype, dwflags, ppaccess, pcaccesses, pidefaultaccess).ok()
    }
    pub unsafe fn MapGeneric(&self, pguidobjecttype: *const windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MapGeneric)(windows_core::Interface::as_raw(self), pguidobjecttype, paceflags, pmask).ok()
    }
    pub unsafe fn GetInheritTypes(&self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInheritTypes)(windows_core::Interface::as_raw(self), ppinherittypes, pcinherittypes).ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn PropertySheetPageCallback<P0>(&self, hwnd: P0, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).PropertySheetPageCallback)(windows_core::Interface::as_raw(self), hwnd.param().abi(), umsg, upage).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SI_OBJECT_INFO) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::OBJECT_SECURITY_INFORMATION, *mut super::super::PSECURITY_DESCRIPTOR, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::OBJECT_SECURITY_INFORMATION, super::super::PSECURITY_DESCRIPTOR) -> windows_core::HRESULT,
    pub GetAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, SECURITY_INFO_PAGE_FLAGS, *mut *mut SI_ACCESS, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub MapGeneric: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetInheritTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SI_INHERIT_TYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub PropertySheetPageCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, super::super::super::UI::Controls::PSPCB_MESSAGE, SI_PAGE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    PropertySheetPageCallback: usize,
}
windows_core::imp::define_interface!(ISecurityInformation2, ISecurityInformation2_Vtbl, 0xc3ccfdb4_6f88_11d2_a3ce_00c04fb1782a);
impl core::ops::Deref for ISecurityInformation2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityInformation2, windows_core::IUnknown);
impl ISecurityInformation2 {
    pub unsafe fn IsDaclCanonical(&self, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsDaclCanonical)(windows_core::Interface::as_raw(self), pdacl)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupSids(&self, csids: u32, rgpsids: *const super::super::PSID) -> windows_core::Result<super::super::super::System::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LookupSids)(windows_core::Interface::as_raw(self), csids, rgpsids, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISecurityInformation2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDaclCanonical: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::ACL) -> super::super::super::Foundation::BOOL,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupSids: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::PSID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupSids: usize,
}
windows_core::imp::define_interface!(ISecurityInformation3, ISecurityInformation3_Vtbl, 0xe2cdc9cc_31bd_4f8f_8c8b_b641af516a1a);
impl core::ops::Deref for ISecurityInformation3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityInformation3, windows_core::IUnknown);
impl ISecurityInformation3 {
    pub unsafe fn GetFullResourceName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFullResourceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenElevatedEditor<P0>(&self, hwnd: P0, upage: SI_PAGE_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).OpenElevatedEditor)(windows_core::Interface::as_raw(self), hwnd.param().abi(), upage).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFullResourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub OpenElevatedEditor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, SI_PAGE_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISecurityInformation4, ISecurityInformation4_Vtbl, 0xea961070_cd14_4621_ace4_f63c03e583e4);
impl core::ops::Deref for ISecurityInformation4 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityInformation4, windows_core::IUnknown);
impl ISecurityInformation4 {
    pub unsafe fn GetSecondarySecurity(&self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSecondarySecurity)(windows_core::Interface::as_raw(self), psecurityobjects, psecurityobjectcount).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSecondarySecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SECURITY_OBJECT, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISecurityObjectTypeInfo, ISecurityObjectTypeInfo_Vtbl, 0xfc3066eb_79ef_444b_9111_d18a75ebf2fa);
impl core::ops::Deref for ISecurityObjectTypeInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISecurityObjectTypeInfo, windows_core::IUnknown);
impl ISecurityObjectTypeInfo {
    pub unsafe fn GetInheritSource(&self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInheritSource)(windows_core::Interface::as_raw(self), si, pacl, ppinheritarray).ok()
    }
}
#[repr(C)]
pub struct ISecurityObjectTypeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInheritSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::ACL, *mut *mut super::INHERITED_FROMA) -> windows_core::HRESULT,
}
pub const CFSTR_ACLUI_SID_INFO_LIST: windows_core::PCWSTR = windows_core::w!("CFSTR_ACLUI_SID_INFO_LIST");
pub const DOBJ_COND_NTACLS: i32 = 8i32;
pub const DOBJ_RES_CONT: i32 = 1i32;
pub const DOBJ_RES_ROOT: i32 = 2i32;
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(16u32);
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(33554432u32);
pub const SI_CONTAINER: i32 = 4i32;
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2147483648u32);
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(2u32);
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(131072u32);
pub const SI_EDIT_OWNER: i32 = 1i32;
pub const SI_EDIT_PERMS: i32 = 0i32;
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(128u32);
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1073741824u32);
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(536870912u32);
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(268435456u32);
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2097152u32);
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
pub const SI_OBJECT_GUID: i32 = 65536i32;
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(67108864u32);
pub const SI_OWNER_READONLY: i32 = 64i32;
pub const SI_OWNER_RECURSE: i32 = 256i32;
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = SI_PAGE_TYPE(1i32);
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = SI_PAGE_TYPE(2i32);
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = SI_PAGE_TYPE(4i32);
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = SI_PAGE_TYPE(3i32);
pub const SI_PAGE_PERM: SI_PAGE_TYPE = SI_PAGE_TYPE(0i32);
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = SI_PAGE_TYPE(6i32);
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = SI_PAGE_TYPE(5i32);
pub const SI_PAGE_TITLE: i32 = 2048i32;
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(16777216u32);
pub const SI_READONLY: i32 = 8i32;
pub const SI_RESET: i32 = 32i32;
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(262144u32);
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1048576u32);
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(524288u32);
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(134217728u32);
pub const SI_SERVER_IS_DC: i32 = 4096i32;
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(2i32);
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(6i32);
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(0i32);
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(4i32);
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(3i32);
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(1i32);
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(5i32);
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(4194304u32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_INFO_PAGE_FLAGS(pub u32);
impl windows_core::TypeKind for SECURITY_INFO_PAGE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_INFO_PAGE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_INFO_PAGE_FLAGS").field(&self.0).finish()
    }
}
impl SECURITY_INFO_PAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SI_OBJECT_INFO_FLAGS(pub u32);
impl windows_core::TypeKind for SI_OBJECT_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SI_OBJECT_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SI_OBJECT_INFO_FLAGS").field(&self.0).finish()
    }
}
impl SI_OBJECT_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SI_OBJECT_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SI_OBJECT_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SI_PAGE_ACTIVATED(pub i32);
impl windows_core::TypeKind for SI_PAGE_ACTIVATED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SI_PAGE_ACTIVATED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SI_PAGE_ACTIVATED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SI_PAGE_TYPE(pub i32);
impl windows_core::TypeKind for SI_PAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SI_PAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SI_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EFFPERM_RESULT_LIST {
    pub fEvaluated: super::super::super::Foundation::BOOLEAN,
    pub cObjectTypeListLength: u32,
    pub pObjectTypeList: *mut super::super::OBJECT_TYPE_LIST,
    pub pGrantedAccessList: *mut u32,
}
impl windows_core::TypeKind for EFFPERM_RESULT_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for EFFPERM_RESULT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_OBJECT {
    pub pwszName: windows_core::PWSTR,
    pub pData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub pData2: *mut core::ffi::c_void,
    pub cbData2: u32,
    pub Id: u32,
    pub fWellKnown: super::super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SECURITY_OBJECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID_INFO {
    pub pSid: super::super::PSID,
    pub pwzCommonName: windows_core::PWSTR,
    pub pwzClass: windows_core::PWSTR,
    pub pwzUPN: windows_core::PWSTR,
}
impl windows_core::TypeKind for SID_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SID_INFO_LIST {
    pub cItems: u32,
    pub aSidInfo: [SID_INFO; 1],
}
impl windows_core::TypeKind for SID_INFO_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SID_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SI_ACCESS {
    pub pguid: *const windows_core::GUID,
    pub mask: u32,
    pub pszName: windows_core::PCWSTR,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for SI_ACCESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SI_ACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SI_INHERIT_TYPE {
    pub pguid: *const windows_core::GUID,
    pub dwFlags: super::super::ACE_FLAGS,
    pub pszName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for SI_INHERIT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SI_INHERIT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SI_OBJECT_INFO {
    pub dwFlags: SI_OBJECT_INFO_FLAGS,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pszServerName: windows_core::PWSTR,
    pub pszObjectName: windows_core::PWSTR,
    pub pszPageTitle: windows_core::PWSTR,
    pub guidObjectType: windows_core::GUID,
}
impl windows_core::TypeKind for SI_OBJECT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SI_OBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
