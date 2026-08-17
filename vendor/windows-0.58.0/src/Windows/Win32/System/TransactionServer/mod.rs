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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<P0>(&self, bstrcollname: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), bstrcollname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Connect<P0>(&self, bstrconnectstring: P0) -> windows_core::Result<super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), bstrconnectstring.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MajorVersion(&self, retval: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), retval).ok()
    }
    pub unsafe fn MinorVersion(&self, retval: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), retval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
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
    pub unsafe fn InstallComponent<P0, P1, P2>(&self, bstrdllfile: P0, bstrtypelibfile: P1, bstrproxystubdllfile: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallComponent)(windows_core::Interface::as_raw(self), bstrdllfile.param().abi(), bstrtypelibfile.param().abi(), bstrproxystubdllfile.param().abi()).ok()
    }
    pub unsafe fn ImportComponent<P0>(&self, bstrclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportComponent)(windows_core::Interface::as_raw(self), bstrclsid.param().abi()).ok()
    }
    pub unsafe fn ImportComponentByName<P0>(&self, bstrprogid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ImportComponentByName)(windows_core::Interface::as_raw(self), bstrprogid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCLSIDs<P0, P1>(&self, bstrdllfile: P0, bstrtypelibfile: P1, aclsids: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetCLSIDs)(windows_core::Interface::as_raw(self), bstrdllfile.param().abi(), bstrtypelibfile.param().abi(), aclsids).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ImportComponentByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCLSIDs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCLSIDs: usize,
}
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
    pub unsafe fn InstallPackage<P0, P1>(&self, bstrpackagefile: P0, bstrinstallpath: P1, loptions: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallPackage)(windows_core::Interface::as_raw(self), bstrpackagefile.param().abi(), bstrinstallpath.param().abi(), loptions).ok()
    }
    pub unsafe fn ExportPackage<P0, P1>(&self, bstrpackageid: P0, bstrpackagefile: P1, loptions: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportPackage)(windows_core::Interface::as_raw(self), bstrpackageid.param().abi(), bstrpackagefile.param().abi(), loptions).ok()
    }
    pub unsafe fn ShutdownPackage<P0>(&self, bstrpackageid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ShutdownPackage)(windows_core::Interface::as_raw(self), bstrpackageid.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPackageUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallPackage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ExportPackage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ShutdownPackage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn InstallRemoteComponent<P0, P1, P2>(&self, bstrserver: P0, bstrpackageid: P1, bstrclsid: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallRemoteComponent)(windows_core::Interface::as_raw(self), bstrserver.param().abi(), bstrpackageid.param().abi(), bstrclsid.param().abi()).ok()
    }
    pub unsafe fn InstallRemoteComponentByName<P0, P1, P2>(&self, bstrserver: P0, bstrpackagename: P1, bstrprogid: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InstallRemoteComponentByName)(windows_core::Interface::as_raw(self), bstrserver.param().abi(), bstrpackagename.param().abi(), bstrprogid.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteComponentUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub InstallRemoteComponent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InstallRemoteComponentByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn AssociateRole<P0>(&self, bstrroleid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AssociateRole)(windows_core::Interface::as_raw(self), bstrroleid.param().abi()).ok()
    }
    pub unsafe fn AssociateRoleByName<P0>(&self, bstrrolename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AssociateRoleByName)(windows_core::Interface::as_raw(self), bstrrolename.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRoleAssociationUtil_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AssociateRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AssociateRoleByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MTSAdminErrorCodes(pub i32);
impl windows_core::TypeKind for MTSAdminErrorCodes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MTSAdminErrorCodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MTSAdminErrorCodes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MTSPackageExportOptions(pub i32);
impl windows_core::TypeKind for MTSPackageExportOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MTSPackageExportOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MTSPackageExportOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MTSPackageInstallOptions(pub i32);
impl windows_core::TypeKind for MTSPackageInstallOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MTSPackageInstallOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MTSPackageInstallOptions").field(&self.0).finish()
    }
}
pub const Catalog: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22881_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogCollection: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22883_8a19_11d0_81b6_00a0c9231c29);
pub const CatalogObject: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22882_8a19_11d0_81b6_00a0c9231c29);
pub const ComponentUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22884_8a19_11d0_81b6_00a0c9231c29);
pub const PackageUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22885_8a19_11d0_81b6_00a0c9231c29);
pub const RemoteComponentUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22886_8a19_11d0_81b6_00a0c9231c29);
pub const RoleAssociationUtil: windows_core::GUID = windows_core::GUID::from_u128(0x6eb22887_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "implement")]
core::include!("impl.rs");
