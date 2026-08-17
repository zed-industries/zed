#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalInstrument(data: *const UAL_DATA_BLOB) -> windows_core::Result<()> {
    windows_targets::link!("ualapi.dll" "system" fn UalInstrument(data : *const UAL_DATA_BLOB) -> windows_core::HRESULT);
    UalInstrument(data).ok()
}
#[inline]
pub unsafe fn UalRegisterProduct<P0, P1, P2>(wszproductname: P0, wszrolename: P1, wszguid: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ualapi.dll" "system" fn UalRegisterProduct(wszproductname : windows_core::PCWSTR, wszrolename : windows_core::PCWSTR, wszguid : windows_core::PCWSTR) -> windows_core::HRESULT);
    UalRegisterProduct(wszproductname.param().abi(), wszrolename.param().abi(), wszguid.param().abi()).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalStart(data: *const UAL_DATA_BLOB) -> windows_core::Result<()> {
    windows_targets::link!("ualapi.dll" "system" fn UalStart(data : *const UAL_DATA_BLOB) -> windows_core::HRESULT);
    UalStart(data).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalStop(data: *const UAL_DATA_BLOB) -> windows_core::Result<()> {
    windows_targets::link!("ualapi.dll" "system" fn UalStop(data : *const UAL_DATA_BLOB) -> windows_core::HRESULT);
    UalStop(data).ok()
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: windows_core::GUID,
    pub TenantId: windows_core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Copy for UAL_DATA_BLOB {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Clone for UAL_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl core::fmt::Debug for UAL_DATA_BLOB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UAL_DATA_BLOB").field("Size", &self.Size).field("RoleGuid", &self.RoleGuid).field("TenantId", &self.TenantId).field("Address", &self.Address).field("UserName", &self.UserName).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for UAL_DATA_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl PartialEq for UAL_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.RoleGuid == other.RoleGuid && self.TenantId == other.TenantId && self.Address == other.Address && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Eq for UAL_DATA_BLOB {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
