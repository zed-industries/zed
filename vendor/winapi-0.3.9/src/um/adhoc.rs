// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::ULONG;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{BOOLEAN, HRESULT, LONG, LPCWSTR, LPWSTR};
ENUM!{enum DOT11_ADHOC_CIPHER_ALGORITHM {
    DOT11_ADHOC_CIPHER_ALGO_INVALID = -1i32 as u32,
    DOT11_ADHOC_CIPHER_ALGO_NONE = 0,
    DOT11_ADHOC_CIPHER_ALGO_CCMP = 0x4,
    DOT11_ADHOC_CIPHER_ALGO_WEP = 0x101,
}}
ENUM!{enum DOT11_ADHOC_AUTH_ALGORITHM {
    DOT11_ADHOC_AUTH_ALGO_INVALID = -1i32 as u32,
    DOT11_ADHOC_AUTH_ALGO_80211_OPEN = 1,
    DOT11_ADHOC_AUTH_ALGO_RSNA_PSK = 7,
}}
ENUM!{enum DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    DOT11_ADHOC_NETWORK_CONNECTION_STATUS_INVALID = 0,
    DOT11_ADHOC_NETWORK_CONNECTION_STATUS_DISCONNECTED = 11,
    DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTING = 12,
    DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTED = 13,
    DOT11_ADHOC_NETWORK_CONNECTION_STATUS_FORMED = 14,
}}
ENUM!{enum DOT11_ADHOC_CONNECT_FAIL_REASON {
    DOT11_ADHOC_CONNECT_FAIL_DOMAIN_MISMATCH = 0,
    DOT11_ADHOC_CONNECT_FAIL_PASSPHRASE_MISMATCH = 1,
    DOT11_ADHOC_CONNECT_FAIL_OTHER = 2,
}}
RIDL!{#[uuid(0x8f10cc26, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocManager(IDot11AdHocManagerVtbl): IUnknown(IUnknownVtbl) {
    fn CreateNetwork(
        Name: LPCWSTR,
        Password: LPCWSTR,
        GeographicalId: LONG,
        pInterface: *mut IDot11AdHocInterface,
        pSecurity: *mut IDot11AdHocSecuritySettings,
        pContextGuid: *mut GUID,
        pIAdHoc: *mut *mut IDot11AdHocNetwork,
    ) -> HRESULT,
    fn CommitCreatedNetwork(
        pIAdHoc: *mut IDot11AdHocNetwork,
        fSaveProfile: BOOLEAN,
        fMakeSavedProfileUserSpecific: BOOLEAN,
    ) -> HRESULT,
    fn GetIEnumDot11AdHocNetworks(
        pContextGuid: *mut GUID,
        ppEnum: *mut *mut IEnumDot11AdHocNetworks,
    ) -> HRESULT,
    fn GetIEnumDot11AdHocInterfaces(
        ppEnum: *mut *mut IEnumDot11AdHocInterfaces,
    ) -> HRESULT,
    fn GetNetwork(
        NetworkSignature: *mut GUID,
        pNetwork: *mut *mut IDot11AdHocNetwork,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc27, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocManagerNotificationSink(IDot11AdHocManagerNotificationSinkVtbl):
    IUnknown(IUnknownVtbl) {
    fn OnNetworkAdd(
        pIAdHocNetwork: *mut IDot11AdHocNetwork,
    ) -> HRESULT,
    fn OnNetworkRemove(
        Signature: *mut GUID,
    ) -> HRESULT,
    fn OnInterfaceAdd(
        pIAdHocInterface: *mut IDot11AdHocInterface,
    ) -> HRESULT,
    fn OnInterfaceRemove(
        Signature: *mut GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc28, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IEnumDot11AdHocNetworks(IEnumDot11AdHocNetworksVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cElt: ULONG,
        rgElt: *mut *mut IDot11AdHocNetwork,
        pcEltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cElt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumDot11AdHocNetworks,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc29, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocNetwork(IDot11AdHocNetworkVtbl): IUnknown(IUnknownVtbl) {
    fn GetStatus(
        eStatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS,
    ) -> HRESULT,
    fn GetSSID(
        ppszwSSID: *mut LPWSTR,
    ) -> HRESULT,
    fn HasProfile(
        pf11d: *mut BOOLEAN,
    ) -> HRESULT,
    fn GetProfileName(
        ppszwProfileName: *mut LPWSTR,
    ) -> HRESULT,
    fn DeleteProfile() -> HRESULT,
    fn GetSignalQuality(
        puStrengthValue: *mut ULONG,
        puStrengthMax: *mut ULONG,
    ) -> HRESULT,
    fn GetSecuritySetting(
        pAdHocSecuritySetting: *mut *mut IDot11AdHocSecuritySettings,
    ) -> HRESULT,
    fn GetContextGuid(
        pContextGuid: *mut GUID,
    ) -> HRESULT,
    fn GetSignature(
        pSignature: *mut GUID,
    ) -> HRESULT,
    fn GetInterface(
        pAdHocInterface: *mut *mut IDot11AdHocInterface,
    ) -> HRESULT,
    fn Connect(
        Passphrase: LPCWSTR,
        GeographicalId: LONG,
        fSaveProfile: BOOLEAN,
        fMakeSavedProfileUserSpecific: BOOLEAN,
    ) -> HRESULT,
    fn Disconnect() -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2a, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocNetworkNotificationSink(IDot11AdHocNetworkNotificationSinkVtbl):
    IUnknown(IUnknownVtbl) {
    fn OnStatusChange(
        eStatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS,
    ) -> HRESULT,
    fn OnConnectFail(
        eFailReason: DOT11_ADHOC_CONNECT_FAIL_REASON,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2b, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocInterface(IDot11AdHocInterfaceVtbl): IUnknown(IUnknownVtbl) {
    fn GetDeviceSignature(
        pSignature: *mut GUID,
    ) -> HRESULT,
    fn GetFriendlyName(
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn IsDot11d(
        pf11d: *mut BOOLEAN,
    ) -> HRESULT,
    fn IsAdHocCapable(
        pfAdHocCapable: *mut BOOLEAN,
    ) -> HRESULT,
    fn IsRadioOn(
        pfIsRadioOn: *mut BOOLEAN,
    ) -> HRESULT,
    fn GetActiveNetwork(
        ppNetwork: *mut *mut IDot11AdHocNetwork,
    ) -> HRESULT,
    fn GetIEnumSecuritySettings(
        ppEnum: *mut *mut IEnumDot11AdHocSecuritySettings,
    ) -> HRESULT,
    fn GetIEnumDot11AdHocNetworks(
        pFilterGuid: *mut GUID,
        ppEnum: *mut *mut IEnumDot11AdHocNetworks,
    ) -> HRESULT,
    fn GetStatus(
        pState: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2c, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IEnumDot11AdHocInterfaces(IEnumDot11AdHocInterfacesVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        cElt: ULONG,
        rgElt: *mut *mut IDot11AdHocInterface,
        pcEltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cElt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumDot11AdHocInterfaces,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2d, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IEnumDot11AdHocSecuritySettings(IEnumDot11AdHocSecuritySettingsVtbl):
    IUnknown(IUnknownVtbl) {
    fn Next(
        cElt: ULONG,
        rgElt: *mut *mut IDot11AdHocSecuritySettings,
        pcEltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        cElt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumDot11AdHocSecuritySettings,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2e, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocSecuritySettings(IDot11AdHocSecuritySettingsVtbl): IUnknown(IUnknownVtbl) {
    fn GetDot11AuthAlgorithm(
        pAuth: *mut DOT11_ADHOC_AUTH_ALGORITHM,
    ) -> HRESULT,
    fn GetDot11CipherAlgorithm(
        pCipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8f10cc2f, 0xcf0d, 0x42a0, 0xac, 0xbe, 0xe2, 0xde, 0x70, 0x07, 0x38, 0x4d)]
interface IDot11AdHocInterfaceNotificationSink(IDot11AdHocInterfaceNotificationSinkVtbl):
    IUnknown(IUnknownVtbl) {
    fn OnConnectionStatusChange(
        eStatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdd06a84f, 0x83bd, 0x4d01, 0x8a, 0xb9, 0x23, 0x89, 0xfe, 0xa0, 0x86, 0x9e)]
class Dot11AdHocManager;}
