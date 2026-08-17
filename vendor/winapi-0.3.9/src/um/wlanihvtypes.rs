// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Structures used to hold information for IHV.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, BYTE, DWORD, ULONG};
use shared::wlantypes::{DOT11_AUTH_ALGORITHM, DOT11_BSS_TYPE, DOT11_CIPHER_ALGORITHM, DOT11_SSID};
use um::eaptypes::EAP_METHOD_TYPE;
use um::winnt::WCHAR;
STRUCT!{struct DOT11_MSSECURITY_SETTINGS {
    dot11AuthAlgorithm: DOT11_AUTH_ALGORITHM,
    dot11CipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    fOneXEnabled: BOOL,
    eapMethodType: EAP_METHOD_TYPE,
    dwEapConnectionDataLen: DWORD,
    pEapConnectionData: *mut BYTE,
}}
pub type PDOT11_MSSECURITY_SETTINGS = *mut DOT11_MSSECURITY_SETTINGS;
STRUCT!{struct DOT11EXT_IHV_SSID_LIST {
    ulCount: ULONG,
    SSIDs: [DOT11_SSID; 1],
}}
pub type PDOT11EXT_IHV_SSID_LIST = *mut DOT11EXT_IHV_SSID_LIST;
STRUCT!{struct DOT11EXT_IHV_PROFILE_PARAMS {
    pSsidList: PDOT11EXT_IHV_SSID_LIST,
    BssType: DOT11_BSS_TYPE,
    pMSSecuritySettings: PDOT11_MSSECURITY_SETTINGS,
}}
pub type PDOT11EXT_IHV_PROFILE_PARAMS = *mut DOT11EXT_IHV_PROFILE_PARAMS;
pub const MS_MAX_PROFILE_NAME_LENGTH: usize = 256;
pub const MS_PROFILE_GROUP_POLICY: DWORD = 0x00000001;
pub const MS_PROFILE_USER: DWORD = 0x00000002;
STRUCT!{struct DOT11EXT_IHV_PARAMS {
    dot11ExtIhvProfileParams: DOT11EXT_IHV_PROFILE_PARAMS,
    wstrProfileName: [WCHAR; MS_MAX_PROFILE_NAME_LENGTH],
    dwProfileTypeFlags: DWORD,
    interfaceGuid: GUID,
}}
pub type PDOT11EXT_IHV_PARAMS = *mut DOT11EXT_IHV_PARAMS;
