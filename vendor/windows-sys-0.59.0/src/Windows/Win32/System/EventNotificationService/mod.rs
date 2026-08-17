windows_targets::link!("sensapi.dll" "system" fn IsDestinationReachableA(lpszdestination : windows_sys::core::PCSTR, lpqocinfo : *mut QOCINFO) -> super::super::Foundation:: BOOL);
windows_targets::link!("sensapi.dll" "system" fn IsDestinationReachableW(lpszdestination : windows_sys::core::PCWSTR, lpqocinfo : *mut QOCINFO) -> super::super::Foundation:: BOOL);
windows_targets::link!("sensapi.dll" "system" fn IsNetworkAlive(lpdwflags : *mut u32) -> super::super::Foundation:: BOOL);
pub const CONNECTION_AOL: u32 = 4u32;
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = 0u32;
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = 1u32;
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
pub const SENSGUID_EVENTCLASS_LOGON: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5978630_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_LOGON2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5978650_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_NETWORK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5978620_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_EVENTCLASS_ONNOW: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5978640_5b9f_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_PUBLISHER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fee1bd6_5b9b_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_LCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd3938ab0_5b9d_11d1_8dd2_00aa004abd5e);
pub const SENSGUID_SUBSCRIBER_WININET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd3938ab5_5b9d_11d1_8dd2_00aa004abd5e);
pub type SENS_CONNECTION_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwInSpeed: u32,
    pub dwOutSpeed: u32,
}
pub const SENS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd597cafe_5b9f_11d1_8dd2_00aa004abd5e);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SENS_QOCINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwOutSpeed: u32,
    pub dwInSpeed: u32,
}
