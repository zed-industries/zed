windows_targets::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceA(lpnetresource : *const NETRESOURCEA, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
windows_targets::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceW(lpnetresource : *const NETRESOURCEW, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPAddConnection(lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPAddConnection3(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> u32);
windows_targets::link!("ntlanman.dll" "system" fn NPAddConnection4(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lpauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPCancelConnection(lpname : windows_sys::core::PCWSTR, fforce : super::super::Foundation:: BOOL) -> u32);
windows_targets::link!("ntlanman.dll" "system" fn NPCancelConnection2(lpname : windows_sys::core::PCWSTR, fforce : super::super::Foundation:: BOOL, dwflags : u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPCloseEnum(henum : super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPEnumResource(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPFormatNetworkName(lpremotename : windows_sys::core::PCWSTR, lpformattedname : windows_sys::core::PWSTR, lpnlength : *mut u32, dwflags : NETWORK_NAME_FORMAT_FLAGS, dwavecharperline : u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetCaps(ndex : u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetConnection(lplocalname : windows_sys::core::PCWSTR, lpremotename : windows_sys::core::PWSTR, lpnbufferlen : *mut u32) -> u32);
windows_targets::link!("ntlanman.dll" "system" fn NPGetConnection3(lplocalname : windows_sys::core::PCWSTR, dwlevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_targets::link!("ntlanman.dll" "system" fn NPGetConnectionPerformance(lpremotename : windows_sys::core::PCWSTR, lpnetconnectinfo : *mut NETCONNECTINFOSTRUCT) -> u32);
windows_targets::link!("ntlanman.dll" "system" fn NPGetPersistentUseOptionsForConnection(lpremotepath : windows_sys::core::PCWSTR, lpreaduseoptions : *const u8, cbreaduseoptions : u32, lpwriteuseoptions : *mut u8, lpsizewriteuseoptions : *mut u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetResourceInformation(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32, lplpsystem : *mut windows_sys::core::PWSTR) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetResourceParent(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetUniversalName(lplocalpath : windows_sys::core::PCWSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPGetUser(lpname : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PWSTR, lpnbufferlen : *mut u32) -> u32);
windows_targets::link!("davclnt.dll" "system" fn NPOpenEnum(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection2A(lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpusername : windows_sys::core::PCSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection2W(lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection3A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpusername : windows_sys::core::PCSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection3W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR, dwflags : NET_CONNECT_FLAGS) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection4A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : NET_CONNECT_FLAGS, lpuseoptions : *const u8, cbuseoptions : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnection4W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : NET_CONNECT_FLAGS, lpuseoptions : *const u8, cbuseoptions : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnectionA(lpremotename : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR, lplocalname : windows_sys::core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetAddConnectionW(lpremotename : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, lplocalname : windows_sys::core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetCancelConnection2A(lpname : windows_sys::core::PCSTR, dwflags : NET_CONNECT_FLAGS, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetCancelConnection2W(lpname : windows_sys::core::PCWSTR, dwflags : NET_CONNECT_FLAGS, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetCancelConnectionA(lpname : windows_sys::core::PCSTR, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetCancelConnectionW(lpname : windows_sys::core::PCWSTR, fforce : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetCloseEnum(henum : super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog(hwnd : super::super::Foundation:: HWND, dwtype : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog1A(lpconndlgstruct : *mut CONNECTDLGSTRUCTA) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetConnectionDialog1W(lpconndlgstruct : *mut CONNECTDLGSTRUCTW) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog(hwnd : super::super::Foundation:: HWND, dwtype : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog1A(lpconndlgstruct : *const DISCDLGSTRUCTA) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetDisconnectDialog1W(lpconndlgstruct : *const DISCDLGSTRUCTW) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetEnumResourceA(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetEnumResourceW(henum : super::super::Foundation:: HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetConnectionA(lplocalname : windows_sys::core::PCSTR, lpremotename : windows_sys::core::PSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetConnectionW(lplocalname : windows_sys::core::PCWSTR, lpremotename : windows_sys::core::PWSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetLastErrorA(lperror : *mut u32, lperrorbuf : windows_sys::core::PSTR, nerrorbufsize : u32, lpnamebuf : windows_sys::core::PSTR, nnamebufsize : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetLastErrorW(lperror : *mut u32, lperrorbuf : windows_sys::core::PWSTR, nerrorbufsize : u32, lpnamebuf : windows_sys::core::PWSTR, nnamebufsize : u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetNetworkInformationA(lpprovider : windows_sys::core::PCSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetNetworkInformationW(lpprovider : windows_sys::core::PCWSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetProviderNameA(dwnettype : u32, lpprovidername : windows_sys::core::PSTR, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetProviderNameW(dwnettype : u32, lpprovidername : windows_sys::core::PWSTR, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetResourceInformationA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_sys::core::PSTR) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetResourceInformationW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_sys::core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetResourceParentA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetResourceParentW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetUniversalNameA(lplocalpath : windows_sys::core::PCSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetUniversalNameW(lplocalpath : windows_sys::core::PCWSTR, dwinfolevel : UNC_INFO_LEVEL, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetUserA(lpname : windows_sys::core::PCSTR, lpusername : windows_sys::core::PSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetGetUserW(lpname : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PWSTR, lpnlength : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetOpenEnumA(dwscope : NET_RESOURCE_SCOPE, dwtype : NET_RESOURCE_TYPE, dwusage : WNET_OPEN_ENUM_USAGE, lpnetresource : *const NETRESOURCEA, lphenum : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetOpenEnumW(dwscope : NET_RESOURCE_SCOPE, dwtype : NET_RESOURCE_TYPE, dwusage : WNET_OPEN_ENUM_USAGE, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetSetLastErrorA(err : u32, lperror : windows_sys::core::PCSTR, lpproviders : windows_sys::core::PCSTR));
windows_targets::link!("mpr.dll" "system" fn WNetSetLastErrorW(err : u32, lperror : windows_sys::core::PCWSTR, lpproviders : windows_sys::core::PCWSTR));
windows_targets::link!("mpr.dll" "system" fn WNetUseConnection4A(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_sys::core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetUseConnection4W(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_sys::core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetUseConnectionA(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpuserid : windows_sys::core::PCSTR, dwflags : NET_CONNECT_FLAGS, lpaccessname : windows_sys::core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
windows_targets::link!("mpr.dll" "system" fn WNetUseConnectionW(hwndowner : super::super::Foundation:: HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpuserid : windows_sys::core::PCWSTR, dwflags : NET_CONNECT_FLAGS, lpaccessname : windows_sys::core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = 2u32;
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = 8u32;
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = 32u32;
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = 16u32;
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = 1u32;
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = 4u32;
pub const CONNECT_CMD_SAVECRED: NET_CONNECT_FLAGS = 4096u32;
pub const CONNECT_COMMANDLINE: NET_CONNECT_FLAGS = 2048u32;
pub const CONNECT_CRED_RESET: NET_CONNECT_FLAGS = 8192u32;
pub const CONNECT_CURRENT_MEDIA: NET_CONNECT_FLAGS = 512u32;
pub const CONNECT_DEFERRED: NET_CONNECT_FLAGS = 1024u32;
pub const CONNECT_GLOBAL_MAPPING: NET_CONNECT_FLAGS = 262144u32;
pub const CONNECT_INTERACTIVE: NET_CONNECT_FLAGS = 8u32;
pub const CONNECT_LOCALDRIVE: NET_CONNECT_FLAGS = 256u32;
pub const CONNECT_NEED_DRIVE: NET_CONNECT_FLAGS = 32u32;
pub const CONNECT_PROMPT: NET_CONNECT_FLAGS = 16u32;
pub const CONNECT_REDIRECT: NET_CONNECT_FLAGS = 128u32;
pub const CONNECT_REFCOUNT: NET_CONNECT_FLAGS = 64u32;
pub const CONNECT_REQUIRE_INTEGRITY: NET_CONNECT_FLAGS = 16384u32;
pub const CONNECT_REQUIRE_PRIVACY: NET_CONNECT_FLAGS = 32768u32;
pub const CONNECT_RESERVED: NET_CONNECT_FLAGS = 4278190080u32;
pub const CONNECT_TEMPORARY: NET_CONNECT_FLAGS = 4u32;
pub const CONNECT_UPDATE_PROFILE: NET_CONNECT_FLAGS = 1u32;
pub const CONNECT_UPDATE_RECENT: NET_CONNECT_FLAGS = 2u32;
pub const CONNECT_WRITE_THROUGH_SEMANTICS: NET_CONNECT_FLAGS = 65536u32;
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = 64u32;
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = 1u32;
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = 4u32;
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = 1u32;
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = 8u32;
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
pub const NOTIFY_POST: u32 = 2u32;
pub const NOTIFY_PRE: u32 = 1u32;
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = 2u32;
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = 0u32;
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = 1u32;
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = 2u32;
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = 19u32;
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = 16u32;
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = 1u32;
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = 2u32;
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = 0u32;
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = 1u32;
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = 5u32;
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = 2u32;
pub const RESOURCE_RECENT: u32 = 4u32;
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = 3u32;
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = 1u32;
pub const WNCON_DYNAMIC: u32 = 8u32;
pub const WNCON_FORNETCARD: u32 = 1u32;
pub const WNCON_NOTROUTED: u32 = 2u32;
pub const WNCON_SLOWLINK: u32 = 4u32;
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = 1u32;
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = 3u32;
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = 2u32;
pub const WNDT_NETWORK: u32 = 1u32;
pub const WNDT_NORMAL: u32 = 0u32;
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = 2u32;
pub const WNFMT_CONNECTION: u32 = 32u32;
pub const WNFMT_INENUM: u32 = 16u32;
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = 1u32;
pub const WNGETCON_CONNECTED: u32 = 0u32;
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
pub const WNNC_ADMIN: u32 = 9u32;
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
pub const WNNC_CONNECTION: u32 = 6u32;
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
pub const WNNC_CON_DEFER: u32 = 128u32;
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
pub const WNNC_DIALOG: u32 = 8u32;
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
pub const WNNC_ENUMERATION: u32 = 11u32;
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
pub const WNNC_NET_NONE: u32 = 0u32;
pub const WNNC_NET_TYPE: u32 = 2u32;
pub const WNNC_SPEC_VERSION: u32 = 1u32;
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
pub const WNNC_START: u32 = 12u32;
pub const WNNC_USER: u32 = 4u32;
pub const WNNC_USR_GETUSER: u32 = 1u32;
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
pub const WNPERMC_AUDIT: u32 = 2u32;
pub const WNPERMC_OWNER: u32 = 4u32;
pub const WNPERMC_PERM: u32 = 1u32;
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = 1u32;
pub const WNPERM_DLG_OWNER: WNPERM_DLG = 2u32;
pub const WNPERM_DLG_PERM: WNPERM_DLG = 0u32;
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = 1u32;
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = 0u32;
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = 2u32;
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
pub const WNTYPE_COMM: u32 = 4u32;
pub const WNTYPE_DRIVE: u32 = 1u32;
pub const WNTYPE_FILE: u32 = 2u32;
pub const WNTYPE_PRINTER: u32 = 3u32;
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
pub const WN_NETWORK_CLASS: u32 = 1u32;
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
pub const WN_SERVICE_CLASS: u32 = 8u32;
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;
pub type CONNECTDLGSTRUCT_FLAGS = u32;
pub type DISCDLGSTRUCT_FLAGS = u32;
pub type NETINFOSTRUCT_CHARACTERISTICS = u32;
pub type NETWORK_NAME_FORMAT_FLAGS = u32;
pub type NET_CONNECT_FLAGS = u32;
pub type NET_RESOURCE_SCOPE = u32;
pub type NET_RESOURCE_TYPE = u32;
pub type NPDIRECTORY_NOTIFY_OPERATION = u32;
pub type NP_PROPERTY_DIALOG_SELECTION = u32;
pub type UNC_INFO_LEVEL = u32;
pub type WNET_OPEN_ENUM_USAGE = u32;
pub type WNPERM_DLG = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: windows_sys::core::PSTR,
    pub lpRemoteName: windows_sys::core::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: windows_sys::core::PWSTR,
    pub lpRemoteName: windows_sys::core::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_sys::core::PSTR,
    pub lpRemoteName: windows_sys::core::PSTR,
    pub lpComment: windows_sys::core::PSTR,
    pub lpProvider: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_sys::core::PWSTR,
    pub lpRemoteName: windows_sys::core::PWSTR,
    pub lpComment: windows_sys::core::PWSTR,
    pub lpProvider: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_CONNECT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NOTIFYCANCEL {
    pub lpName: windows_sys::core::PWSTR,
    pub lpProvider: windows_sys::core::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: windows_sys::core::PSTR,
    pub lpConnectionName: windows_sys::core::PSTR,
    pub lpRemainingPath: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: windows_sys::core::PWSTR,
    pub lpConnectionName: windows_sys::core::PWSTR,
    pub lpRemainingPath: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: windows_sys::core::PWSTR,
}
pub type PF_AddConnectNotify = Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32>;
pub type PF_CancelConnectNotify = Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpcancelinfo: *const NOTIFYCANCEL) -> u32>;
pub type PF_NPAddConnection = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lppassword: windows_sys::core::PCWSTR, lpusername: windows_sys::core::PCWSTR) -> u32>;
pub type PF_NPAddConnection3 = Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: windows_sys::core::PCWSTR, lpusername: windows_sys::core::PCWSTR, dwflags: u32) -> u32>;
pub type PF_NPAddConnection4 = Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32>;
pub type PF_NPCancelConnection = Option<unsafe extern "system" fn(lpname: windows_sys::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32>;
pub type PF_NPCancelConnection2 = Option<unsafe extern "system" fn(lpname: windows_sys::core::PCWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32>;
pub type PF_NPCloseEnum = Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32>;
pub type PF_NPDeviceMode = Option<unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32>;
pub type PF_NPDirectoryNotify = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpdir: windows_sys::core::PCWSTR, dwoper: u32) -> u32>;
pub type PF_NPEnumResource = Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPFMXEditPerm = Option<unsafe extern "system" fn(lpdrivename: windows_sys::core::PCWSTR, hwndfmx: super::super::Foundation::HWND, ndialogtype: u32) -> u32>;
pub type PF_NPFMXGetPermCaps = Option<unsafe extern "system" fn(lpdrivename: windows_sys::core::PCWSTR) -> u32>;
pub type PF_NPFMXGetPermHelp = Option<unsafe extern "system" fn(lpdrivename: windows_sys::core::PCWSTR, ndialogtype: u32, fdirectory: super::super::Foundation::BOOL, lpfilenamebuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32, lpnhelpcontext: *mut u32) -> u32>;
pub type PF_NPFormatNetworkName = Option<unsafe extern "system" fn(lpremotename: windows_sys::core::PCWSTR, lpformattedname: windows_sys::core::PWSTR, lpnlength: *mut u32, dwflags: u32, dwavecharperline: u32) -> u32>;
pub type PF_NPGetCaps = Option<unsafe extern "system" fn(ndex: u32) -> u32>;
pub type PF_NPGetConnection = Option<unsafe extern "system" fn(lplocalname: windows_sys::core::PCWSTR, lpremotename: windows_sys::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
pub type PF_NPGetConnection3 = Option<unsafe extern "system" fn(lplocalname: windows_sys::core::PCWSTR, dwlevel: u32, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPGetConnectionPerformance = Option<unsafe extern "system" fn(lpremotename: windows_sys::core::PCWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32>;
pub type PF_NPGetDirectoryType = Option<unsafe extern "system" fn(lpname: windows_sys::core::PCWSTR, lptype: *const i32, bflushcache: super::super::Foundation::BOOL) -> u32>;
pub type PF_NPGetPersistentUseOptionsForConnection = Option<unsafe extern "system" fn(lpremotepath: windows_sys::core::PCWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32>;
pub type PF_NPGetPropertyText = Option<unsafe extern "system" fn(ibutton: u32, npropsel: u32, lpname: windows_sys::core::PCWSTR, lpbuttonname: windows_sys::core::PWSTR, nbuttonnamelen: u32, ntype: u32) -> u32>;
pub type PF_NPGetResourceInformation = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut windows_sys::core::PWSTR) -> u32>;
pub type PF_NPGetResourceParent = Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
pub type PF_NPGetUniversalName = Option<unsafe extern "system" fn(lplocalpath: windows_sys::core::PCWSTR, dwinfolevel: u32, lpbuffer: *mut core::ffi::c_void, lpnbuffersize: *mut u32) -> u32>;
pub type PF_NPGetUser = Option<unsafe extern "system" fn(lpname: windows_sys::core::PCWSTR, lpusername: windows_sys::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
pub type PF_NPLogonNotify = Option<unsafe extern "system" fn(lplogonid: *const super::super::Foundation::LUID, lpauthentinfotype: windows_sys::core::PCWSTR, lpauthentinfo: *const core::ffi::c_void, lppreviousauthentinfotype: windows_sys::core::PCWSTR, lppreviousauthentinfo: *const core::ffi::c_void, lpstationname: windows_sys::core::PCWSTR, stationhandle: *const core::ffi::c_void, lplogonscript: *mut windows_sys::core::PWSTR) -> u32>;
pub type PF_NPOpenEnum = Option<unsafe extern "system" fn(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PF_NPPasswordChangeNotify = Option<unsafe extern "system" fn(lpauthentinfotype: windows_sys::core::PCWSTR, lpauthentinfo: *const core::ffi::c_void, lppreviousauthentinfotype: windows_sys::core::PCWSTR, lppreviousauthentinfo: *const core::ffi::c_void, lpstationname: windows_sys::core::PCWSTR, stationhandle: *const core::ffi::c_void, dwchangeinfo: u32) -> u32>;
pub type PF_NPPropertyDialog = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, ibuttondlg: u32, npropsel: u32, lpfilename: windows_sys::core::PCWSTR, ntype: u32) -> u32>;
pub type PF_NPSearchDialog = Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpbuffer: *mut core::ffi::c_void, cbbuffer: u32, lpnflags: *mut u32) -> u32>;
