#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("ualapi.dll" "system" fn UalInstrument(data : *const UAL_DATA_BLOB) -> windows_sys::core::HRESULT);
windows_targets::link!("ualapi.dll" "system" fn UalRegisterProduct(wszproductname : windows_sys::core::PCWSTR, wszrolename : windows_sys::core::PCWSTR, wszguid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("ualapi.dll" "system" fn UalStart(data : *const UAL_DATA_BLOB) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Networking_WinSock")]
windows_targets::link!("ualapi.dll" "system" fn UalStop(data : *const UAL_DATA_BLOB) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: windows_sys::core::GUID,
    pub TenantId: windows_sys::core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
