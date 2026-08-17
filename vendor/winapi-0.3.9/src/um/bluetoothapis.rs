// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::bthdef::{AUTHENTICATION_REQUIREMENTS, BTH_ADDR, BTH_MAX_PIN_SIZE};
use shared::bthsdpdef::{SDP_LARGE_INTEGER_16, SDP_SPECIFICTYPE, SDP_TYPE, SDP_ULARGE_INTEGER_16};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, LPBYTE, LPVOID, PULONG, UCHAR, ULONG, USHORT};
use shared::windef::HWND;
use um::minwinbase::SYSTEMTIME;
use um::winnt::{
    CHAR, HANDLE, LONG, LONGLONG, LPCWSTR, LPWSTR, PVOID, PWSTR, SHORT, ULONGLONG, WCHAR,
};
pub const BLUETOOTH_MAX_NAME_SIZE: usize = 248;
pub const BLUETOOTH_MAX_PASSKEY_SIZE: usize = 16;
pub const BLUETOOTH_MAX_PASSKEY_BUFFER_SIZE: usize = BLUETOOTH_MAX_PASSKEY_SIZE + 1;
pub const BLUETOOTH_MAX_SERVICE_NAME_SIZE: usize = 256;
pub const BLUETOOTH_DEVICE_NAME_SIZE: usize = 256;
pub type BLUETOOTH_ADDRESS = BTH_ADDR;
pub const BLUETOOTH_NULL_ADDRESS: BLUETOOTH_ADDRESS = 0x0;
STRUCT!{struct BLUETOOTH_LOCAL_SERVICE_INFO {
    Enabled: BOOL,
    btAddr: BLUETOOTH_ADDRESS,
    szName: [WCHAR; BLUETOOTH_MAX_SERVICE_NAME_SIZE],
    szDeviceString: [WCHAR; BLUETOOTH_DEVICE_NAME_SIZE],
}}
pub type PBLUETOOTH_LOCAL_SERVICE_INFO = *mut BLUETOOTH_LOCAL_SERVICE_INFO;
STRUCT!{struct BLUETOOTH_FIND_RADIO_PARAMS {
    dwSize: DWORD,
}}
pub type HBLUETOOTH_RADIO_FIND = HANDLE;
extern "system" {
    pub fn BluetoothFindFirstRadio(
        pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS,
        phRadio: *mut HANDLE,
    ) -> HBLUETOOTH_RADIO_FIND;
    pub fn BluetoothFindNextRadio(
        hFind: HBLUETOOTH_RADIO_FIND,
        phRadio: *mut HANDLE,
    ) -> BOOL;
    pub fn BluetoothFindRadioClose(
        hFind: HBLUETOOTH_RADIO_FIND,
    ) -> BOOL;
}
STRUCT!{struct BLUETOOTH_RADIO_INFO {
    dwSize: DWORD,
    address: BLUETOOTH_ADDRESS,
    szName: [WCHAR; BLUETOOTH_MAX_NAME_SIZE],
    ulClassofDevice: ULONG,
    lmpSubversion: USHORT,
    manufacturer: USHORT,
}}
pub type PBLUETOOTH_RADIO_INFO = *mut BLUETOOTH_RADIO_INFO;
extern "system" {
    pub fn BluetoothGetRadioInfo(
        hRadio: HANDLE,
        pRadioInfo: PBLUETOOTH_RADIO_INFO,
    ) -> DWORD;
}
STRUCT!{struct BLUETOOTH_DEVICE_INFO {
    dwSize: DWORD,
    Address: BLUETOOTH_ADDRESS,
    ulClassofDevice: ULONG,
    fConnected: BOOL,
    fRemembered: BOOL,
    fAuthenticated: BOOL,
    stLastSeen: SYSTEMTIME,
    stLastUsed: SYSTEMTIME,
    szName: [WCHAR; BLUETOOTH_MAX_NAME_SIZE],
}}
pub type PBLUETOOTH_DEVICE_INFO = *mut BLUETOOTH_DEVICE_INFO;
ENUM!{enum BLUETOOTH_AUTHENTICATION_METHOD {
    BLUETOOTH_AUTHENTICATION_METHOD_LEGACY = 0x1,
    BLUETOOTH_AUTHENTICATION_METHOD_OOB,
    BLUETOOTH_AUTHENTICATION_METHOD_NUMERIC_COMPARISON,
    BLUETOOTH_AUTHENTICATION_METHOD_PASSKEY_NOTIFICATION,
    BLUETOOTH_AUTHENTICATION_METHOD_PASSKEY,
}}
pub type PBLUETOOTH_AUTHENTICATION_METHOD = *mut BLUETOOTH_AUTHENTICATION_METHOD;
ENUM!{enum BLUETOOTH_IO_CAPABILITY {
    BLUETOOTH_IO_CAPABILITY_DISPLAYONLY = 0x00,
    BLUETOOTH_IO_CAPABILITY_DISPLAYYESNO = 0x01,
    BLUETOOTH_IO_CAPABILITY_KEYBOARDONLY = 0x02,
    BLUETOOTH_IO_CAPABILITY_NOINPUTNOOUTPUT = 0x03,
    BLUETOOTH_IO_CAPABILITY_UNDEFINED = 0xff,
}}
ENUM!{enum BLUETOOTH_AUTHENTICATION_REQUIREMENTS {
    BLUETOOTH_MITM_ProtectionNotRequired = 0,
    BLUETOOTH_MITM_ProtectionRequired = 0x1,
    BLUETOOTH_MITM_ProtectionNotRequiredBonding = 0x2,
    BLUETOOTH_MITM_ProtectionRequiredBonding = 0x3,
    BLUETOOTH_MITM_ProtectionNotRequiredGeneralBonding = 0x4,
    BLUETOOTH_MITM_ProtectionRequiredGeneralBonding = 0x5,
    BLUETOOTH_MITM_ProtectionNotDefined = 0xff,
}}
UNION!{union BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_u {
    [u32; 1],
    Numeric_Value Numeric_Value_mut: ULONG,
    Passkey Passkey_mut: ULONG,
}}
STRUCT!{struct BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    deviceInfo: BLUETOOTH_DEVICE_INFO,
    authenticationMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    ioCapability: BLUETOOTH_IO_CAPABILITY,
    authenticationRequirements: BLUETOOTH_AUTHENTICATION_REQUIREMENTS,
    u: BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS_u,
}}
pub type PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS = *mut BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS;
STRUCT!{struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
    dwSize: DWORD,
    fReturnAuthenticated: BOOL,
    fReturnRemembered: BOOL,
    fReturnUnknown: BOOL,
    fReturnConnected: BOOL,
    fIssueInquiry: BOOL,
    cTimeoutMultiplier: UCHAR,
    hRadio: HANDLE,
}}
pub type HBLUETOOTH_DEVICE_FIND = HANDLE;
extern "system" {
    pub fn BluetoothFindFirstDevice(
        pbtsp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> HBLUETOOTH_DEVICE_FIND;
    pub fn BluetoothFindNextDevice(
        hFind: HBLUETOOTH_DEVICE_FIND,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> BOOL;
    pub fn BluetoothFindDeviceClose(
        hFind: HBLUETOOTH_DEVICE_FIND,
    ) -> BOOL;
    pub fn BluetoothGetDeviceInfo(
        hRadio: HANDLE,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> DWORD;
    pub fn BluetoothUpdateDeviceRecord(
        pbtdi: *const BLUETOOTH_DEVICE_INFO,
    ) -> DWORD;
    pub fn BluetoothRemoveDevice(
        pAddress: *const BLUETOOTH_ADDRESS,
    ) -> DWORD;
}
STRUCT!{struct BLUETOOTH_COD_PAIRS {
    ulCODMask: ULONG,
    pcszDescription: LPCWSTR,
}}
FN!{stdcall PFN_DEVICE_CALLBACK(
    pvParam: LPVOID,
    pDevice: *const BLUETOOTH_DEVICE_INFO,
) -> BOOL}
STRUCT!{struct BLUETOOTH_SELECT_DEVICE_PARAMS {
    dwSize: DWORD,
    cNumOfClasses: ULONG,
    prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
    pszInfo: LPWSTR,
    hwndParent: HWND,
    fForceAuthentication: BOOL,
    fShowAuthenticated: BOOL,
    fShowRemembered: BOOL,
    fShowUnknown: BOOL,
    fAddNewDeviceWizard: BOOL,
    fSkipServicesPage: BOOL,
    pfnDeviceCallback: PFN_DEVICE_CALLBACK,
    pvParam: LPVOID,
    cNumDevices: DWORD,
    pDevices: PBLUETOOTH_DEVICE_INFO,
}}
extern "system" {
    pub fn BluetoothSelectDevices(
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS,
    ) -> BOOL;
    pub fn BluetoothSelectDevicesFree(
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS,
    ) -> BOOL;
    pub fn BluetoothDisplayDeviceProperties(
        hwndParent: HWND,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> BOOL;
    // #[deprecated]
    pub fn BluetoothAuthenticateDevice(
        hwndParent: HWND,
        hRadio: HANDLE,
        pbtbi: *mut BLUETOOTH_DEVICE_INFO,
        pszPasskey: PWSTR,
        ulPasskeyLength: ULONG,
    ) -> DWORD;
}
STRUCT!{struct BLUETOOTH_PIN_INFO {
    pin: [UCHAR; BTH_MAX_PIN_SIZE],
    pinLength: UCHAR,
}}
pub type PBLUETOOTH_PIN_INFO = *mut BLUETOOTH_PIN_INFO;
STRUCT!{struct BLUETOOTH_OOB_DATA_INFO {
    C: [UCHAR; 16],
    R: [UCHAR; 16],
}}
pub type PBLUETOOTH_OOB_DATA_INFO = *mut BLUETOOTH_OOB_DATA_INFO;
STRUCT!{struct BLUETOOTH_NUMERIC_COMPARISON_INFO {
    NumericValue: ULONG,
}}
pub type PBLUETOOTH_NUMERIC_COMPARISON_INFO = *mut BLUETOOTH_NUMERIC_COMPARISON_INFO;
STRUCT!{struct BLUETOOTH_PASSKEY_INFO {
    passkey: ULONG,
}}
pub type PBLUETOOTH_PASSKEY_INFO = *mut BLUETOOTH_PASSKEY_INFO;
extern "system" {
    pub fn BluetoothAuthenticateDeviceEx(
        hwndParentIn: HWND,
        hRadioIn: HANDLE,
        pbtdiInout: *mut BLUETOOTH_DEVICE_INFO,
        pbtOobData: PBLUETOOTH_OOB_DATA_INFO,
        authenticationRequirement: AUTHENTICATION_REQUIREMENTS,
    ) -> DWORD;
    // #[deprecated]
    pub fn BluetoothAuthenticateMultipleDevices(
        hwndParent: HWND,
        hRadio: HANDLE,
        cDevices: DWORD,
        rgbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> DWORD;
}
pub const BLUETOOTH_SERVICE_DISABLE: DWORD = 0x00;
pub const BLUETOOTH_SERVICE_ENABLE: DWORD = 0x01;
pub const BLUETOOTH_SERVICE_MASK: DWORD = BLUETOOTH_SERVICE_DISABLE | BLUETOOTH_SERVICE_ENABLE;
extern "system" {
    pub fn BluetoothSetServiceState(
        hRadio: HANDLE,
        pbtdi: *const BLUETOOTH_DEVICE_INFO,
        pGuidService: *const GUID,
        dwServiceFlags: DWORD,
    ) -> DWORD;
    pub fn BluetoothEnumerateInstalledServices(
        hRadio: HANDLE,
        pbtdi: *const BLUETOOTH_DEVICE_INFO,
        pcServiceInout: *mut DWORD,
        pGuidServices: *mut GUID,
    ) -> DWORD;
    pub fn BluetoothEnableDiscovery(
        hRadio: HANDLE,
        fEnabled: BOOL,
    ) -> BOOL;
    pub fn BluetoothIsDiscoverable(
        hRadio: HANDLE,
    ) -> BOOL;
    pub fn BluetoothEnableIncomingConnections(
        hRadio: HANDLE,
        fEnabled: BOOL,
    ) -> BOOL;
    pub fn BluetoothIsConnectable(
        hRadio: HANDLE,
    ) -> BOOL;
}
pub type HBLUETOOTH_AUTHENTICATION_REGISTRATION = HANDLE;
FN!{stdcall PFN_AUTHENTICATION_CALLBACK(
    pvParam: LPVOID,
    pDevice: PBLUETOOTH_DEVICE_INFO,
) -> BOOL}
extern "system" {
    // #[deprecated]
    pub fn BluetoothRegisterForAuthentication(
        pbtdi: *const BLUETOOTH_DEVICE_INFO,
        phRegHandle: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION,
        pfnCallback: PFN_AUTHENTICATION_CALLBACK,
        pvParam: PVOID,
    ) -> DWORD;
}
FN!{stdcall PFN_AUTHENTICATION_CALLBACK_EX(
    pvParam: LPVOID,
    pAuthCallbackParams: PBLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS,
) -> BOOL}
extern "system" {
    pub fn BluetoothRegisterForAuthenticationEx(
        pbtdiIn: *const BLUETOOTH_DEVICE_INFO,
        phRegHandleOut: *mut HBLUETOOTH_AUTHENTICATION_REGISTRATION,
        pfnCallbackIn: PFN_AUTHENTICATION_CALLBACK_EX,
        pvParam: PVOID,
    ) -> DWORD;
    pub fn BluetoothUnregisterAuthentication(
        hRegHandle: HBLUETOOTH_AUTHENTICATION_REGISTRATION,
    ) -> BOOL;
    // #[deprecated]
    pub fn BluetoothSendAuthenticationResponse(
        hRadio: HANDLE,
        pbtdi: *const BLUETOOTH_DEVICE_INFO,
        pszPasskey: LPCWSTR,
    ) -> DWORD;
}
UNION!{union BLUETOOTH_AUTHENTICATE_RESPONSE_u {
    [u32; 8],
    pinInfo pinInfo_mut: BLUETOOTH_PIN_INFO,
    oobInfo oobInfo_mut: BLUETOOTH_OOB_DATA_INFO,
    numericCompInfo numericCompInfo_mut: BLUETOOTH_NUMERIC_COMPARISON_INFO,
    passkeyInfo passkeyInfo_mut: BLUETOOTH_PASSKEY_INFO,
}}
STRUCT!{struct BLUETOOTH_AUTHENTICATE_RESPONSE {
    bthAddressRemote: BLUETOOTH_ADDRESS,
    authMethod: BLUETOOTH_AUTHENTICATION_METHOD,
    u: BLUETOOTH_AUTHENTICATE_RESPONSE_u,
    negativeResponse: UCHAR,
}}
pub type PBLUETOOTH_AUTHENTICATE_RESPONSE = *mut BLUETOOTH_AUTHENTICATE_RESPONSE;
extern "system" {
    pub fn BluetoothSendAuthenticationResponseEx(
        hRadioIn: HANDLE,
        pauthResponse: PBLUETOOTH_AUTHENTICATE_RESPONSE,
    ) -> DWORD;
}
STRUCT!{struct SDP_ELEMENT_DATA_data_string {
    value: LPBYTE,
    length: ULONG,
}}
STRUCT!{struct SDP_ELEMENT_DATA_data_url {
    value: LPBYTE,
    length: ULONG,
}}
STRUCT!{struct SDP_ELEMENT_DATA_data_sequence {
    value: LPBYTE,
    length: ULONG,
}}
STRUCT!{struct SDP_ELEMENT_DATA_data_alternative {
    value: LPBYTE,
    length: ULONG,
}}
UNION!{union SDP_ELEMENT_DATA_data {
    [u64; 2],
    int128 int128_mut: SDP_LARGE_INTEGER_16,
    int64 int64_mut: LONGLONG,
    int32 int32_mut: LONG,
    int16 int16_mut: SHORT,
    int8 int8_mut: CHAR,
    uint128 uint128_mut: SDP_ULARGE_INTEGER_16,
    uint64 uint64_mut: ULONGLONG,
    uint32 uint32_mut: ULONG,
    uint16 uint16_mut: USHORT,
    uint8 uint8_mut: UCHAR,
    booleanVal booleanVal_mut: UCHAR,
    uuid128 uuid128_mut: GUID,
    uuid32 uuid32_mut: ULONG,
    uuid16 uuid16_mut: USHORT,
    string string_mut: SDP_ELEMENT_DATA_data_string,
    url url_mut: SDP_ELEMENT_DATA_data_url,
    sequence sequence_mut: SDP_ELEMENT_DATA_data_sequence,
    alternative alternative_mut: SDP_ELEMENT_DATA_data_alternative,
}}
STRUCT!{struct SDP_ELEMENT_DATA {
    type_: SDP_TYPE,
    specificType: SDP_SPECIFICTYPE,
    data: SDP_ELEMENT_DATA_data,
}}
pub type PSDP_ELEMENT_DATA = *mut SDP_ELEMENT_DATA;
extern "system" {
    pub fn BluetoothSdpGetElementData(
        pSdpStream: LPBYTE,
        cbSdpStreamLength: ULONG,
        pData: PSDP_ELEMENT_DATA,
    ) -> DWORD;
}
pub type HBLUETOOTH_CONTAINER_ELEMENT = HANDLE;
extern "system" {
    pub fn BluetoothSdpGetContainerElementData(
        pContainerStream: LPBYTE,
        cbContainerLength: ULONG,
        pElement: *mut HBLUETOOTH_CONTAINER_ELEMENT,
        pData: PSDP_ELEMENT_DATA,
    ) -> DWORD;
    pub fn BluetoothSdpGetAttributeValue(
        pRecordStream: LPBYTE,
        cbRecordLength: ULONG,
        usAttributeId: USHORT,
        pAttributeData: PSDP_ELEMENT_DATA,
    ) -> DWORD;
}
STRUCT!{struct SDP_STRING_TYPE_DATA {
    encoding: USHORT,
    mibeNum: USHORT,
    attributeId: USHORT,
}}
pub type PSDP_STRING_TYPE_DATA = *mut SDP_STRING_TYPE_DATA;
extern "system" {
    pub fn BluetoothSdpGetString(
        pRecordStream: LPBYTE,
        cbRecordLength: ULONG,
        pStringData: PSDP_STRING_TYPE_DATA,
        usStringOffset: USHORT,
        pszString: PWSTR,
        pcchStringLength: PULONG,
    ) -> DWORD;
}
FN!{stdcall PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK(
    uAttribId: ULONG,
    pValueStream: LPBYTE,
    cbStreamSize: ULONG,
    pvParam: LPVOID,
) -> BOOL}
pub use self::BluetoothSdpEnumAttributes as BluetoothEnumAttributes;
extern "system" {
    pub fn BluetoothSdpEnumAttributes(
        pSDPStream: LPBYTE,
        cbStreamSize: ULONG,
        pfnCallback: PFN_BLUETOOTH_ENUM_ATTRIBUTES_CALLBACK,
        pvParam: LPVOID,
    ) -> BOOL;
    pub fn BluetoothSetLocalServiceInfo(
        hRadioIn: HANDLE,
        pClassGuid: *const GUID,
        ulInstance: ULONG,
        pServiceInfoIn: *const BLUETOOTH_LOCAL_SERVICE_INFO,
    ) -> DWORD;
    pub fn BluetoothIsVersionAvailable(
        MajorVersion: UCHAR,
        MinorVersion: UCHAR,
    ) -> BOOL;
}
