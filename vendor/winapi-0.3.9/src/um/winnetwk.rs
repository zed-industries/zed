// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Standard WINNET Header File for WIN32
use shared::basetsd::ULONG_PTR;
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPHANDLE, LPVOID, WORD};
use shared::windef::HWND;
use shared::winerror::{
    ERROR_ACCESS_DENIED, ERROR_ALREADY_ASSIGNED, ERROR_ALREADY_INITIALIZED, ERROR_BAD_DEVICE,
    ERROR_BAD_DEV_TYPE, ERROR_BAD_NET_NAME, ERROR_BAD_PROFILE, ERROR_BAD_PROVIDER,
    ERROR_BAD_USERNAME, ERROR_BUSY, ERROR_CANCELLED, ERROR_CANNOT_OPEN_PROFILE,
    ERROR_CONNECTED_OTHER_PASSWORD, ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT,
    ERROR_CONNECTION_UNAVAIL, ERROR_DEVICE_ALREADY_REMEMBERED, ERROR_DEVICE_IN_USE,
    ERROR_EXTENDED_ERROR, ERROR_GEN_FAILURE, ERROR_INVALID_ADDRESS, ERROR_INVALID_HANDLE,
    ERROR_INVALID_LEVEL, ERROR_INVALID_PARAMETER, ERROR_INVALID_PASSWORD, ERROR_MORE_DATA,
    ERROR_NOT_AUTHENTICATED, ERROR_NOT_CONNECTED, ERROR_NOT_CONTAINER, ERROR_NOT_ENOUGH_MEMORY,
    ERROR_NOT_LOGGED_ON, ERROR_NOT_SUPPORTED, ERROR_NO_LOGON_SERVERS, ERROR_NO_MORE_DEVICES,
    ERROR_NO_MORE_ITEMS, ERROR_NO_NETWORK, ERROR_NO_NET_OR_BAD_PATH, ERROR_OPEN_FILES, ERROR_RETRY,
    ERROR_UNEXP_NET_ERR, NO_ERROR
};
use um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
pub const RESOURCE_CONNECTED: DWORD = 0x00000001;
pub const RESOURCE_GLOBALNET: DWORD = 0x00000002;
pub const RESOURCE_REMEMBERED: DWORD = 0x00000003;
pub const RESOURCE_RECENT: DWORD = 0x00000004;
pub const RESOURCE_CONTEXT: DWORD = 0x00000005;
pub const RESOURCETYPE_ANY: DWORD = 0x00000000;
pub const RESOURCETYPE_DISK: DWORD = 0x00000001;
pub const RESOURCETYPE_PRINT: DWORD = 0x00000002;
pub const RESOURCETYPE_RESERVED: DWORD = 0x00000008;
pub const RESOURCETYPE_UNKNOWN: DWORD = 0xFFFFFFFF;
pub const RESOURCEUSAGE_CONNECTABLE: DWORD = 0x00000001;
pub const RESOURCEUSAGE_CONTAINER: DWORD = 0x00000002;
pub const RESOURCEUSAGE_NOLOCALDEVICE: DWORD = 0x00000004;
pub const RESOURCEUSAGE_SIBLING: DWORD = 0x00000008;
pub const RESOURCEUSAGE_ATTACHED: DWORD = 0x00000010;
pub const RESOURCEUSAGE_ALL: DWORD = RESOURCEUSAGE_CONNECTABLE | RESOURCEUSAGE_CONTAINER
    | RESOURCEUSAGE_ATTACHED;
pub const RESOURCEUSAGE_RESERVED: DWORD = 0x80000000;
pub const RESOURCEDISPLAYTYPE_GENERIC: DWORD = 0x00000000;
pub const RESOURCEDISPLAYTYPE_DOMAIN: DWORD = 0x00000001;
pub const RESOURCEDISPLAYTYPE_SERVER: DWORD = 0x00000002;
pub const RESOURCEDISPLAYTYPE_SHARE: DWORD = 0x00000003;
pub const RESOURCEDISPLAYTYPE_FILE: DWORD = 0x00000004;
pub const RESOURCEDISPLAYTYPE_GROUP: DWORD = 0x00000005;
pub const RESOURCEDISPLAYTYPE_NETWORK: DWORD = 0x00000006;
pub const RESOURCEDISPLAYTYPE_ROOT: DWORD = 0x00000007;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: DWORD = 0x00000008;
pub const RESOURCEDISPLAYTYPE_DIRECTORY: DWORD = 0x00000009;
pub const RESOURCEDISPLAYTYPE_TREE: DWORD = 0x0000000A;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: DWORD = 0x0000000B;
STRUCT!{struct NETRESOURCEA {
    dwScope: DWORD,
    dwType: DWORD,
    dwDisplayType: DWORD,
    dwUsage: DWORD,
    lpLocalName: LPSTR,
    lpRemoteName: LPSTR,
    lpComment: LPSTR,
    lpProvider: LPSTR,
}}
pub type LPNETRESOURCEA = *mut NETRESOURCEA;
STRUCT!{struct NETRESOURCEW {
    dwScope: DWORD,
    dwType: DWORD,
    dwDisplayType: DWORD,
    dwUsage: DWORD,
    lpLocalName: LPWSTR,
    lpRemoteName: LPWSTR,
    lpComment: LPWSTR,
    lpProvider: LPWSTR,
}}
pub type LPNETRESOURCEW = *mut NETRESOURCEW;
pub const NETPROPERTY_PERSISTENT: DWORD = 1;
pub const CONNECT_UPDATE_PROFILE: DWORD = 0x00000001;
pub const CONNECT_UPDATE_RECENT: DWORD = 0x00000002;
pub const CONNECT_TEMPORARY: DWORD = 0x00000004;
pub const CONNECT_INTERACTIVE: DWORD = 0x00000008;
pub const CONNECT_PROMPT: DWORD = 0x00000010;
pub const CONNECT_NEED_DRIVE: DWORD = 0x00000020;
pub const CONNECT_REFCOUNT: DWORD = 0x00000040;
pub const CONNECT_REDIRECT: DWORD = 0x00000080;
pub const CONNECT_LOCALDRIVE: DWORD = 0x00000100;
pub const CONNECT_CURRENT_MEDIA: DWORD = 0x00000200;
pub const CONNECT_DEFERRED: DWORD = 0x00000400;
pub const CONNECT_RESERVED: DWORD = 0xFF000000;
pub const CONNECT_COMMANDLINE: DWORD = 0x00000800;
pub const CONNECT_CMD_SAVECRED: DWORD = 0x00001000;
pub const CONNECT_CRED_RESET: DWORD = 0x00002000;
extern "system" {
    pub fn WNetAddConnection2A(
        lpNetResource: LPNETRESOURCEA,
        lpPassword: LPCSTR,
        lpUsername: LPCSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection2W(
        lpNetResource: LPNETRESOURCEW,
        lpPassword: LPCWSTR,
        lpUsername: LPCWSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection3A(
        hwndOwner: HWND,
        lpNetResource: LPNETRESOURCEA,
        lpPassword: LPCSTR,
        lpUsername: LPCSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection3W(
        hwndOwner: HWND,
        lpNetResource: LPNETRESOURCEW,
        lpPassword: LPCWSTR,
        lpUsername: LPCWSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetCancelConnectionA(
        lpName: LPCSTR,
        fForce: BOOL,
    ) -> DWORD;
    pub fn WNetCancelConnectionW(
        lpName: LPCWSTR,
        fForce: BOOL,
    ) -> DWORD;
    pub fn WNetCancelConnection2A(
        lpName: LPCSTR,
        dwFlags: DWORD,
        fForce: BOOL,
    ) -> DWORD;
    pub fn WNetCancelConnection2W(
        lpName: LPCWSTR,
        dwFlags: DWORD,
        fForce: BOOL,
    ) -> DWORD;
    pub fn WNetGetConnectionA(
        lpLocalName: LPCSTR,
        lpRemoteName: LPSTR,
        lpnLength: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetConnectionW(
        lpLocalName: LPCWSTR,
        lpRemoteName: LPWSTR,
        lpnLength: LPDWORD,
    ) -> DWORD;
    pub fn WNetUseConnectionA(
        hwndOwner: HWND,
        lpNetResource: LPNETRESOURCEA,
        lpPassword: LPCSTR,
        lpUserId: LPCSTR,
        dwFlags: DWORD,
        lpAccessName: LPSTR,
        lpBufferSize: LPDWORD,
        lpResult: LPDWORD,
    ) -> DWORD;
    pub fn WNetUseConnectionW(
        hwndOwner: HWND,
        lpNetResource: LPNETRESOURCEW,
        lpPassword: LPCWSTR,
        lpUserId: LPCWSTR,
        dwFlags: DWORD,
        lpAccessName: LPWSTR,
        lpBufferSize: LPDWORD,
        lpResult: LPDWORD,
    ) -> DWORD;
    pub fn WNetConnectionDialog(
        hwnd: HWND,
        dwType: DWORD,
    ) -> DWORD;
    pub fn WNetDisconnectDialog(
        hwnd: HWND,
        dwType: DWORD,
    ) -> DWORD;
}
STRUCT!{struct CONNECTDLGSTRUCTA {
    cbStructure: DWORD,
    hwndOwner: HWND,
    lpConnRes: LPNETRESOURCEA,
    dwFlags: DWORD,
    dwDevNum: DWORD,
}}
pub type LPCONNECTDLGSTRUCTA = *mut CONNECTDLGSTRUCTA;
STRUCT!{struct CONNECTDLGSTRUCTW {
    cbStructure: DWORD,
    hwndOwner: HWND,
    lpConnRes: LPNETRESOURCEW,
    dwFlags: DWORD,
    dwDevNum: DWORD,
}}
pub type LPCONNECTDLGSTRUCTW = *mut CONNECTDLGSTRUCTW;
pub const CONNDLG_RO_PATH: DWORD = 0x00000001;
pub const CONNDLG_CONN_POINT: DWORD = 0x00000002;
pub const CONNDLG_USE_MRU: DWORD = 0x00000004;
pub const CONNDLG_HIDE_BOX: DWORD = 0x00000008;
pub const CONNDLG_PERSIST: DWORD = 0x00000010;
pub const CONNDLG_NOT_PERSIST: DWORD = 0x00000020;
extern "system" {
    pub fn WNetConnectionDialog1A(
        lpConnDlgStruct: LPCONNECTDLGSTRUCTA,
    ) -> DWORD;
    pub fn WNetConnectionDialog1W(
        lpConnDlgStruct: LPCONNECTDLGSTRUCTW,
    ) -> DWORD;
}
STRUCT!{struct DISCDLGSTRUCTA {
    cbStructure: DWORD,
    hwndOwner: HWND,
    lpLocalName: LPSTR,
    lpRemoteName: LPSTR,
    dwFlags: DWORD,
}}
pub type LPDISCDLGSTRUCTA = *mut DISCDLGSTRUCTA;
STRUCT!{struct DISCDLGSTRUCTW {
    cbStructure: DWORD,
    hwndOwner: HWND,
    lpLocalName: LPWSTR,
    lpRemoteName: LPWSTR,
    dwFlags: DWORD,
}}
pub type LPDISCDLGSTRUCTW = *mut DISCDLGSTRUCTW;
pub const DISC_UPDATE_PROFILE: DWORD = 0x00000001;
pub const DISC_NO_FORCE: DWORD = 0x00000040;
extern "system" {
    pub fn WNetDisconnectDialog1A(
        lpConnDlgStruct: LPDISCDLGSTRUCTA,
    ) -> DWORD;
    pub fn WNetDisconnectDialog1W(
        lpConnDlgStruct: LPDISCDLGSTRUCTW,
    ) -> DWORD;
    pub fn WNetOpenEnumA(
        dwScope: DWORD,
        dwType: DWORD,
        dwUsage: DWORD,
        lpNetResource: LPNETRESOURCEA,
        lphEnum: LPHANDLE,
    ) -> DWORD;
    pub fn WNetOpenEnumW(
        dwScope: DWORD,
        dwType: DWORD,
        dwUsage: DWORD,
        lpNetResource: LPNETRESOURCEW,
        lphEnum: LPHANDLE,
    ) -> DWORD;
    pub fn WNetEnumResourceA(
        hEnum: HANDLE,
        lpcCount: LPDWORD,
        lpBuffer: LPVOID,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetEnumResourceW(
        hEnum: HANDLE,
        lpcCount: LPDWORD,
        lpBuffer: LPVOID,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetCloseEnum(
        hEnum: HANDLE,
    ) -> DWORD;
    pub fn WNetGetResourceParentA(
        lpNetResource: LPNETRESOURCEA,
        lpBuffer: LPVOID,
        lpcbBuffer: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetResourceParentW(
        lpNetResource: LPNETRESOURCEW,
        lpBuffer: LPVOID,
        lpcbBuffer: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetResourceInformationA(
        lpNetResource: LPNETRESOURCEA,
        lpBuffer: LPVOID,
        lpcbBuffer: LPDWORD,
        lplpSystem: *mut LPSTR,
    ) -> DWORD;
    pub fn WNetGetResourceInformationW(
        lpNetResource: LPNETRESOURCEW,
        lpBuffer: LPVOID,
        lpcbBuffer: LPDWORD,
        lplpSystem: *mut LPWSTR,
    ) -> DWORD;
}
pub const UNIVERSAL_NAME_INFO_LEVEL: DWORD = 0x00000001;
pub const REMOTE_NAME_INFO_LEVEL: DWORD = 0x00000002;
STRUCT!{struct UNIVERSAL_NAME_INFOA {
    lpUniversalName: LPSTR,
}}
pub type LPUNIVERSAL_NAME_INFOA = *mut UNIVERSAL_NAME_INFOA;
STRUCT!{struct UNIVERSAL_NAME_INFOW {
    lpUniversalName: LPWSTR,
}}
pub type LPUNIVERSAL_NAME_INFOW = *mut UNIVERSAL_NAME_INFOW;
STRUCT!{struct REMOTE_NAME_INFOA {
    lpUniversalName: LPSTR,
    lpConnectionName: LPSTR,
    lpRemainingPath: LPSTR,
}}
pub type LPREMOTE_NAME_INFOA = *mut REMOTE_NAME_INFOA;
STRUCT!{struct REMOTE_NAME_INFOW {
    lpUniversalName: LPWSTR,
    lpConnectionName: LPWSTR,
    lpRemainingPath: LPWSTR,
}}
pub type LPREMOTE_NAME_INFOW = *mut REMOTE_NAME_INFOW;
extern "system" {
    pub fn WNetGetUniversalNameA(
        lpLocalPath: LPCSTR,
        dwInfoLevel: DWORD,
        lpBuffer: LPVOID,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUniversalNameW(
        lpLocalPath: LPCWSTR,
        dwInfoLevel: DWORD,
        lpBuffer: LPVOID,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUserA(
        lpName: LPCSTR,
        lpUserName: LPSTR,
        lpnLength: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUserW(
        lpName: LPCWSTR,
        lpUserName: LPWSTR,
        lpnLength: LPDWORD,
    ) -> DWORD;
}
pub const WNFMT_MULTILINE: DWORD = 0x01;
pub const WNFMT_ABBREVIATED: DWORD = 0x02;
pub const WNFMT_INENUM: DWORD = 0x10;
pub const WNFMT_CONNECTION: DWORD = 0x20;
extern "system" {
    pub fn WNetGetProviderNameA(
        dwNetType: DWORD,
        lpProviderName: LPSTR,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetProviderNameW(
        dwNetType: DWORD,
        lpProviderName: LPWSTR,
        lpBufferSize: LPDWORD,
    ) -> DWORD;
}
STRUCT!{struct NETINFOSTRUCT {
    cbStructure: DWORD,
    dwProviderVersion: DWORD,
    dwStatus: DWORD,
    dwCharacteristics: DWORD,
    dwHandle: ULONG_PTR,
    wNetType: WORD,
    dwPrinters: DWORD,
    dwDrives: DWORD,
}}
pub type LPNETINFOSTRUCT = *mut NETINFOSTRUCT;
pub const NETINFO_DLL16: DWORD = 0x00000001;
pub const NETINFO_DISKRED: DWORD = 0x00000004;
pub const NETINFO_PRINTERRED: DWORD = 0x00000008;
extern "system" {
    pub fn WNetGetNetworkInformationA(
        lpProvider: LPCSTR,
        lpNetInfoStruct: LPNETINFOSTRUCT,
    ) -> DWORD;
    pub fn WNetGetNetworkInformationW(
        lpProvider: LPCWSTR,
        lpNetInfoStruct: LPNETINFOSTRUCT,
    ) -> DWORD;
    pub fn WNetGetLastErrorA(
        lpError: LPDWORD,
        lpErrorBuf: LPSTR,
        nErrorBufSize: DWORD,
        lpNameBuf: LPSTR,
        nNameBufSize: DWORD,
    ) -> DWORD;
    pub fn WNetGetLastErrorW(
        lpError: LPDWORD,
        lpErrorBuf: LPWSTR,
        nErrorBufSize: DWORD,
        lpNameBuf: LPWSTR,
        nNameBufSize: DWORD,
    ) -> DWORD;
}
pub const WN_SUCCESS: DWORD = NO_ERROR;
pub const WN_NO_ERROR: DWORD = NO_ERROR;
pub const WN_NOT_SUPPORTED: DWORD = ERROR_NOT_SUPPORTED;
pub const WN_CANCEL: DWORD = ERROR_CANCELLED;
pub const WN_RETRY: DWORD = ERROR_RETRY;
pub const WN_NET_ERROR: DWORD = ERROR_UNEXP_NET_ERR;
pub const WN_MORE_DATA: DWORD = ERROR_MORE_DATA;
pub const WN_BAD_POINTER: DWORD = ERROR_INVALID_ADDRESS;
pub const WN_BAD_VALUE: DWORD = ERROR_INVALID_PARAMETER;
pub const WN_BAD_USER: DWORD = ERROR_BAD_USERNAME;
pub const WN_BAD_PASSWORD: DWORD = ERROR_INVALID_PASSWORD;
pub const WN_ACCESS_DENIED: DWORD = ERROR_ACCESS_DENIED;
pub const WN_FUNCTION_BUSY: DWORD = ERROR_BUSY;
pub const WN_WINDOWS_ERROR: DWORD = ERROR_UNEXP_NET_ERR;
pub const WN_OUT_OF_MEMORY: DWORD = ERROR_NOT_ENOUGH_MEMORY;
pub const WN_NO_NETWORK: DWORD = ERROR_NO_NETWORK;
pub const WN_EXTENDED_ERROR: DWORD = ERROR_EXTENDED_ERROR;
pub const WN_BAD_LEVEL: DWORD = ERROR_INVALID_LEVEL;
pub const WN_BAD_HANDLE: DWORD = ERROR_INVALID_HANDLE;
pub const WN_NOT_INITIALIZING: DWORD = ERROR_ALREADY_INITIALIZED;
pub const WN_NO_MORE_DEVICES: DWORD = ERROR_NO_MORE_DEVICES;
pub const WN_NOT_CONNECTED: DWORD = ERROR_NOT_CONNECTED;
pub const WN_OPEN_FILES: DWORD = ERROR_OPEN_FILES;
pub const WN_DEVICE_IN_USE: DWORD = ERROR_DEVICE_IN_USE;
pub const WN_BAD_NETNAME: DWORD = ERROR_BAD_NET_NAME;
pub const WN_BAD_LOCALNAME: DWORD = ERROR_BAD_DEVICE;
pub const WN_ALREADY_CONNECTED: DWORD = ERROR_ALREADY_ASSIGNED;
pub const WN_DEVICE_ERROR: DWORD = ERROR_GEN_FAILURE;
pub const WN_CONNECTION_CLOSED: DWORD = ERROR_CONNECTION_UNAVAIL;
pub const WN_NO_NET_OR_BAD_PATH: DWORD = ERROR_NO_NET_OR_BAD_PATH;
pub const WN_BAD_PROVIDER: DWORD = ERROR_BAD_PROVIDER;
pub const WN_CANNOT_OPEN_PROFILE: DWORD = ERROR_CANNOT_OPEN_PROFILE;
pub const WN_BAD_PROFILE: DWORD = ERROR_BAD_PROFILE;
pub const WN_BAD_DEV_TYPE: DWORD = ERROR_BAD_DEV_TYPE;
pub const WN_DEVICE_ALREADY_REMEMBERED: DWORD = ERROR_DEVICE_ALREADY_REMEMBERED;
pub const WN_CONNECTED_OTHER_PASSWORD: DWORD = ERROR_CONNECTED_OTHER_PASSWORD;
pub const WN_CONNECTED_OTHER_PASSWORD_DEFAULT: DWORD = ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT;
pub const WN_NO_MORE_ENTRIES: DWORD = ERROR_NO_MORE_ITEMS;
pub const WN_NOT_CONTAINER: DWORD = ERROR_NOT_CONTAINER;
pub const WN_NOT_AUTHENTICATED: DWORD = ERROR_NOT_AUTHENTICATED;
pub const WN_NOT_LOGGED_ON: DWORD = ERROR_NOT_LOGGED_ON;
pub const WN_NOT_VALIDATED: DWORD = ERROR_NO_LOGON_SERVERS;
STRUCT!{struct NETCONNECTINFOSTRUCT {
    cbStructure: DWORD,
    dwFlags: DWORD,
    dwSpeed: DWORD,
    dwDelay: DWORD,
    dwOptDataSize: DWORD,
}}
pub type LPNETCONNECTINFOSTRUCT = *mut NETCONNECTINFOSTRUCT;
pub const WNCON_FORNETCARD: DWORD = 0x00000001;
pub const WNCON_NOTROUTED: DWORD = 0x00000002;
pub const WNCON_SLOWLINK: DWORD = 0x00000004;
pub const WNCON_DYNAMIC: DWORD = 0x00000008;
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(
        lpNetResource: LPNETRESOURCEA,
        lpNetConnectInfoStruct: LPNETCONNECTINFOSTRUCT,
    ) -> DWORD;
    pub fn MultinetGetConnectionPerformanceW(
        lpNetResource: LPNETRESOURCEW,
        lpNetConnectInfoStruct: LPNETCONNECTINFOSTRUCT,
    ) -> DWORD;
}
