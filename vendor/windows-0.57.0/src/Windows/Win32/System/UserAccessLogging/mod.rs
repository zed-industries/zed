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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: windows_core::GUID,
    pub TenantId: windows_core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for UAL_DATA_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
