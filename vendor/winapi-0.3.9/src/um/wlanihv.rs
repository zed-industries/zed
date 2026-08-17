// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definition of public APIs for WLAN Extensibility Framework.
use shared::basetsd::UINT32;
use shared::guiddef::{CLSID, GUID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPVOID, PBOOL, PBYTE, PDWORD, UCHAR, ULONG, USHORT};
use shared::windot11::{
    DOT11_ASSOC_STATUS, DOT11_DIRECTION, PDOT11_ASSOCIATION_COMPLETION_PARAMETERS,
    PDOT11_CIPHER_DEFAULT_KEY_VALUE, PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE, PDOT11_MAC_ADDRESS,
    PDOT11_PRIVACY_EXEMPTION,
};
use shared::wlantypes::{DOT11_AUTH_ALGORITHM, DOT11_BSS_TYPE, DOT11_CIPHER_ALGORITHM, DOT11_SSID};
use um::dot1x::{ONEX_AUTH_STATUS, ONEX_REASON_CODE};
use um::eaptypes::EAP_ATTRIBUTES;
use um::l2cmn::PL2_NOTIFICATION_DATA;
use um::winnt::{HANDLE, LPWSTR, PHANDLE, WCHAR};
use um::winuser::PWTSSESSION_NOTIFICATION;
use um::wlanihvtypes::{MS_MAX_PROFILE_NAME_LENGTH, PDOT11EXT_IHV_PROFILE_PARAMS};
use um::wlclient::{PDOT11_ADAPTER, PDOT11_BSS_LIST, PDOT11_PORT_STATE};
STRUCT!{struct DOT11EXT_APIS {
    Dot11ExtAllocateBuffer: DOT11EXT_ALLOCATE_BUFFER,
    Dot11ExtFreeBuffer: DOT11EXT_FREE_BUFFER,
    Dot11ExtSetProfileCustomUserData: DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA,
    Dot11ExtGetProfileCustomUserData: DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA,
    Dot11ExtSetCurrentProfile: DOT11EXT_SET_CURRENT_PROFILE,
    Dot11ExtSendUIRequest: DOT11EXT_SEND_UI_REQUEST,
    Dot11ExtPreAssociateCompletion: DOT11EXT_PRE_ASSOCIATE_COMPLETION,
    Dot11ExtPostAssociateCompletion: DOT11EXT_POST_ASSOCIATE_COMPLETION,
    Dot11ExtSendNotification: DOT11EXT_SEND_NOTIFICATION,
    Dot11ExtSendPacket: DOT11EXT_SEND_PACKET,
    Dot11ExtSetEtherTypeHandling: DOT11EXT_SET_ETHERTYPE_HANDLING,
    Dot11ExtSetAuthAlgorithm: DOT11EXT_SET_AUTH_ALGORITHM,
    Dot11ExtSetUnicastCipherAlgorithm: DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM,
    Dot11ExtSetMulticastCipherAlgorithm: DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM,
    Dot11ExtSetDefaultKey: DOT11EXT_SET_DEFAULT_KEY,
    Dot11ExtSetKeyMappingKey: DOT11EXT_SET_KEY_MAPPING_KEY,
    Dot11ExtSetDefaultKeyId: DOT11EXT_SET_DEFAULT_KEY_ID,
    Dot11ExtNicSpecificExtension: DOT11EXT_NIC_SPECIFIC_EXTENSION,
    Dot11ExtSetExcludeUnencrypted: DOT11EXT_SET_EXCLUDE_UNENCRYPTED,
    Dot11ExtStartOneX: DOT11EXT_ONEX_START,
    Dot11ExtStopOneX: DOT11EXT_ONEX_STOP,
    Dot11ExtProcessSecurityPacket: DOT11EXT_PROCESS_ONEX_PACKET,
}}
pub type PDOT11EXT_APIS = *mut DOT11EXT_APIS;
STRUCT!{struct DOT11EXT_IHV_HANDLERS {
    Dot11ExtIhvDeinitService: DOT11EXTIHV_DEINIT_SERVICE,
    Dot11ExtIhvInitAdapter: DOT11EXTIHV_INIT_ADAPTER,
    Dot11ExtIhvDeinitAdapter: DOT11EXTIHV_DEINIT_ADAPTER,
    Dot11ExtIhvPerformPreAssociate: DOT11EXTIHV_PERFORM_PRE_ASSOCIATE,
    Dot11ExtIhvAdapterReset: DOT11EXTIHV_ADAPTER_RESET,
    Dot11ExtIhvPerformPostAssociate: DOT11EXTIHV_PERFORM_POST_ASSOCIATE,
    Dot11ExtIhvStopPostAssociate: DOT11EXTIHV_STOP_POST_ASSOCIATE,
    Dot11ExtIhvValidateProfile: DOT11EXTIHV_VALIDATE_PROFILE,
    Dot11ExtIhvPerformCapabilityMatch: DOT11EXTIHV_PERFORM_CAPABILITY_MATCH,
    Dot11ExtIhvCreateDiscoveryProfiles: DOT11EXTIHV_CREATE_DISCOVERY_PROFILES,
    Dot11ExtIhvProcessSessionChange: DOT11EXTIHV_PROCESS_SESSION_CHANGE,
    Dot11ExtIhvReceiveIndication: DOT11EXTIHV_RECEIVE_INDICATION,
    Dot11ExtIhvReceivePacket: DOT11EXTIHV_RECEIVE_PACKET,
    Dot11ExtIhvSendPacketCompletion: DOT11EXTIHV_SEND_PACKET_COMPLETION,
    Dot11ExtIhvIsUIRequestPending: DOT11EXTIHV_IS_UI_REQUEST_PENDING,
    Dot11ExtIhvProcessUIResponse: DOT11EXTIHV_PROCESS_UI_RESPONSE,
    Dot11ExtIhvQueryUIRequest: DOT11EXTIHV_QUERY_UI_REQUEST,
    Dot11ExtIhvOnexIndicateResult: DOT11EXTIHV_ONEX_INDICATE_RESULT,
    Dot11ExtIhvControl: DOT11EXTIHV_CONTROL,
}}
pub type PDOT11EXT_IHV_HANDLERS = *mut DOT11EXT_IHV_HANDLERS;
STRUCT!{struct DOT11EXT_VIRTUAL_STATION_APIS {
    Dot11ExtRequestVirtualStation: DOT11EXT_REQUEST_VIRTUAL_STATION,
    Dot11ExtReleaseVirtualStation: DOT11EXT_RELEASE_VIRTUAL_STATION,
    Dot11ExtQueryVirtualStationProperties: DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES,
    Dot11ExtSetVirtualStationAPProperties: DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES,
}}
pub type PDOT11EXT_VIRTUAL_STATION_APIS = *mut DOT11EXT_VIRTUAL_STATION_APIS;
STRUCT!{struct DOT11_IHV_VERSION_INFO {
    dwVerMin: DWORD,
    dwVerMax: DWORD,
}}
pub type PDOT11_IHV_VERSION_INFO = *mut DOT11_IHV_VERSION_INFO;
ENUM!{enum DOT11EXT_IHV_CONNECTION_PHASE {
    connection_phase_any = 0,
    connection_phase_initial_connection = 1,
    connection_phase_post_l3_connection = 2,
}}
pub type PDOT11EXT_IHV_CONNECTION_PHASE = *mut DOT11EXT_IHV_CONNECTION_PHASE;
STRUCT!{struct DOT11EXT_IHV_UI_REQUEST {
    dwSessionId: DWORD,
    guidUIRequest: GUID,
    UIPageClsid: CLSID,
    dwByteCount: DWORD,
    pvUIRequest: *mut BYTE,
}}
pub type PDOT11EXT_IHV_UI_REQUEST = *mut DOT11EXT_IHV_UI_REQUEST;
ENUM!{enum DOT11_MSONEX_RESULT {
    DOT11_MSONEX_SUCCESS = 0,
    DOT11_MSONEX_FAILURE = 1,
    DOT11_MSONEX_IN_PROGRESS = 2,
}}
pub type PDOT11_MSONEX_RESULT = *mut DOT11_MSONEX_RESULT;
STRUCT!{struct DOT11_EAP_RESULT {
    dwFailureReasonCode: UINT32,
    pAttribArray: *mut EAP_ATTRIBUTES,
}}
pub type PDOT11_EAP_RESULT = *mut DOT11_EAP_RESULT;
STRUCT!{struct DOT11_MSONEX_RESULT_PARAMS {
    Dot11OnexAuthStatus: ONEX_AUTH_STATUS,
    Dot11OneXReasonCode: ONEX_REASON_CODE,
    pbMPPESendKey: PBYTE,
    dwMPPESendKeyLen: DWORD,
    pbMPPERecvKey: PBYTE,
    dwMPPERecvKeyLen: DWORD,
    pDot11EapResult: PDOT11_EAP_RESULT,
}}
pub type PDOT11_MSONEX_RESULT_PARAMS = *mut DOT11_MSONEX_RESULT_PARAMS;
STRUCT!{struct DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    pszXmlFragmentIhvConnectivity: LPWSTR,
}}
pub type PDOT11EXT_IHV_CONNECTIVITY_PROFILE = *mut DOT11EXT_IHV_CONNECTIVITY_PROFILE;
STRUCT!{struct DOT11EXT_IHV_SECURITY_PROFILE {
    pszXmlFragmentIhvSecurity: LPWSTR,
    bUseMSOnex: BOOL,
}}
pub type PDOT11EXT_IHV_SECURITY_PROFILE = *mut DOT11EXT_IHV_SECURITY_PROFILE;
STRUCT!{struct DOT11EXT_IHV_DISCOVERY_PROFILE {
    IhvConnectivityProfile: DOT11EXT_IHV_CONNECTIVITY_PROFILE,
    IhvSecurityProfile: DOT11EXT_IHV_SECURITY_PROFILE,
}}
pub type PDOT11EXT_IHV_DISCOVERY_PROFILE = *mut DOT11EXT_IHV_DISCOVERY_PROFILE;
STRUCT!{struct DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    dwCount: DWORD,
    pIhvDiscoveryProfiles: PDOT11EXT_IHV_DISCOVERY_PROFILE,
}}
pub type PDOT11EXT_IHV_DISCOVERY_PROFILE_LIST = *mut DOT11EXT_IHV_DISCOVERY_PROFILE_LIST;
ENUM!{enum DOT11EXT_IHV_INDICATION_TYPE {
    IndicationTypeNicSpecificNotification = 0,
    IndicationTypePmkidCandidateList = 1,
    IndicationTypeTkipMicFailure = 2,
    IndicationTypePhyStateChange = 3,
    IndicationTypeLinkQuality = 4,
}}
pub type PDOT11EXT_IHV_INDICATION_TYPE = *mut DOT11EXT_IHV_INDICATION_TYPE;
pub const DOT11EXT_PSK_MAX_LENGTH: usize = 64;
STRUCT!{struct DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    dot11SSID: DOT11_SSID,
    dot11AuthAlgo: DOT11_AUTH_ALGORITHM,
    dot11CipherAlgo: DOT11_CIPHER_ALGORITHM,
    bIsPassPhrase: BOOL,
    dwKeyLength: DWORD,
    ucKeyData: [UCHAR; DOT11EXT_PSK_MAX_LENGTH],
}}
pub type PDOT11EXT_VIRTUAL_STATION_AP_PROPERTY = *mut DOT11EXT_VIRTUAL_STATION_AP_PROPERTY;
pub const WDIAG_IHV_WLAN_ID_FLAG_SECURITY_ENABLED: DWORD = 0x00000001;
STRUCT!{struct WDIAG_IHV_WLAN_ID {
    strProfileName: [WCHAR; MS_MAX_PROFILE_NAME_LENGTH],
    Ssid: DOT11_SSID,
    BssType: DOT11_BSS_TYPE,
    dwFlags: DWORD,
    dwReasonCode: DWORD,
}}
pub type PWDIAG_IHV_WLAN_ID = *mut WDIAG_IHV_WLAN_ID;
FN!{stdcall DOT11EXT_ALLOCATE_BUFFER(
    dwByteCount: DWORD,
    ppvBuffer: *mut LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_FREE_BUFFER(
    pvMemory: LPVOID,
) -> ()}
FN!{stdcall DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA(
    hDot11SvcHandle: HANDLE,
    hConnectSession: HANDLE,
    dwSessionID: DWORD,
    dwDataSize: DWORD,
    pvData: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA(
    hDot11SvcHandle: HANDLE,
    hConnectSession: HANDLE,
    dwSessionID: DWORD,
    pdwDataSize: *mut DWORD,
    ppvData: *mut LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_CURRENT_PROFILE(
    hDot11SvcHandle: HANDLE,
    hConnectSession: HANDLE,
    pIhvConnProfile: PDOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pIhvSecProfile: PDOT11EXT_IHV_SECURITY_PROFILE,
) -> DWORD}
FN!{stdcall DOT11EXT_SEND_UI_REQUEST(
    hDot11SvcHandle: HANDLE,
    pIhvUIRequest: PDOT11EXT_IHV_UI_REQUEST,
) -> DWORD}
FN!{stdcall DOT11EXT_PRE_ASSOCIATE_COMPLETION(
    hDot11SvcHandle: HANDLE,
    hConnectSession: HANDLE,
    dwReasonCode: DWORD,
    dwWin32Error: DWORD,
) -> DWORD}
FN!{stdcall DOT11EXT_POST_ASSOCIATE_COMPLETION(
    hDot11SvcHandle: HANDLE,
    hSecuritySessionID: HANDLE,
    pPeer: PDOT11_MAC_ADDRESS,
    dwReasonCode: DWORD,
    dwWin32Error: DWORD,
) -> DWORD}
FN!{stdcall DOT11EXT_SEND_NOTIFICATION(
    hDot11SvcHandle: HANDLE,
    pNotificationData: PL2_NOTIFICATION_DATA,
) -> DWORD}
FN!{stdcall DOT11EXT_SEND_PACKET(
    hDot11SvcHandle: HANDLE,
    uPacketLen: ULONG,
    pvPacket: LPVOID,
    hSendCompletion: HANDLE,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_ETHERTYPE_HANDLING(
    hDot11SvcHandle: HANDLE,
    uMaxBackLog: ULONG,
    uNumOfExemption: ULONG,
    pExemption: PDOT11_PRIVACY_EXEMPTION,
    uNumOfRegistration: ULONG,
    pusRegistration: *mut USHORT,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_AUTH_ALGORITHM(
    hDot11SvcHandle: HANDLE,
    dwAuthAlgo: DWORD,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM(
    hDot11SvcHandle: HANDLE,
    dwUnicastCipherAlgo: DWORD,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM(
    hDot11SvcHandle: HANDLE,
    dwMulticastCipherAlgo: DWORD,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_DEFAULT_KEY(
    hDot11SvcHandle: HANDLE,
    pKey: PDOT11_CIPHER_DEFAULT_KEY_VALUE,
    dot11Direction: DOT11_DIRECTION,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_KEY_MAPPING_KEY(
    hDot11SvcHandle: HANDLE,
    pKey: PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_DEFAULT_KEY_ID(
    hDot11SvcHandle: HANDLE,
    uDefaultKeyId: ULONG,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_EXCLUDE_UNENCRYPTED(
    hDot11SvcHandle: HANDLE,
    bExcludeUnencrypted: BOOL,
) -> DWORD}
FN!{stdcall DOT11EXT_NIC_SPECIFIC_EXTENSION(
    hDot11SvcHandle: HANDLE,
    dwInBufferSize: DWORD,
    pvInBuffer: LPVOID,
    pdwOutBufferSize: *mut DWORD,
    pvOutBuffer: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_ONEX_START(
    hDot11SvcHandle: HANDLE,
    pEapAttributes: *mut EAP_ATTRIBUTES,
) -> DWORD}
FN!{stdcall DOT11EXT_ONEX_STOP(
    hDot11SvcHandle: HANDLE,
) -> DWORD}
FN!{stdcall DOT11EXT_PROCESS_ONEX_PACKET(
    hDot11SvcHandle: HANDLE,
    dwInPacketSize: DWORD,
    pvInPacket: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_REQUEST_VIRTUAL_STATION(
    hDot11PrimaryHandle: HANDLE,
    pvReserved: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_RELEASE_VIRTUAL_STATION(
    hDot11PrimaryHandle: HANDLE,
    pvReserved: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES(
    hDot11SvcHandle: HANDLE,
    pbIsVirtualStation: *mut BOOL,
    pgPrimary: *mut GUID,
    pvReserved: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES(
    hDot11SvcHandle: HANDLE,
    hConnectSession: HANDLE,
    dwNumProperties: DWORD,
    pProperties: PDOT11EXT_VIRTUAL_STATION_AP_PROPERTY,
    pvReserved: LPVOID,
) -> DWORD}
pub const IHV_VERSION_FUNCTION_NAME: &'static str = "Dot11ExtIhvGetVersionInfo";
pub const IHV_INIT_FUNCTION_NAME: &'static str = "Dot11ExtIhvInitService";
pub const IHV_INIT_VS_FUNCTION_NAME: &'static str = "Dot11ExtIhvInitVirtualStation";
FN!{stdcall DOT11EXTIHV_GET_VERSION_INFO(
    pDot11IHVVersionInfo: PDOT11_IHV_VERSION_INFO,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_INIT_SERVICE(
    dwVerNumUsed: DWORD,
    pDot11ExtAPI: PDOT11EXT_APIS,
    pvReserved: LPVOID,
    pDot11IHVHandlers: PDOT11EXT_IHV_HANDLERS,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_INIT_VIRTUAL_STATION(
    pDot11ExtVSAPI: PDOT11EXT_VIRTUAL_STATION_APIS,
    pvReserved: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_DEINIT_SERVICE() -> ()}
FN!{stdcall DOT11EXTIHV_INIT_ADAPTER(
    pDot11Adapter: PDOT11_ADAPTER,
    hDot11SvcHandle: HANDLE,
    phIhvExtAdapter: PHANDLE,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_DEINIT_ADAPTER(
    hIhvExtAdapter: HANDLE,
) -> ()}
FN!{stdcall DOT11EXTIHV_PERFORM_PRE_ASSOCIATE(
    hIhvExtAdapter: HANDLE,
    hConnectSession: HANDLE,
    pIhvProfileParams: PDOT11EXT_IHV_PROFILE_PARAMS,
    pIhvConnProfile: PDOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pIhvSecProfile: PDOT11EXT_IHV_SECURITY_PROFILE,
    pConnectableBssid: PDOT11_BSS_LIST,
    pdwReasonCode: PDWORD,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_ADAPTER_RESET(
    hIhvExtAdapter: HANDLE,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_PERFORM_POST_ASSOCIATE(
    hIhvExtAdapter: HANDLE,
    hSecuritySessionID: HANDLE,
    pPortState: PDOT11_PORT_STATE,
    uDot11AssocParamsBytes: ULONG,
    pDot11AssocParams: PDOT11_ASSOCIATION_COMPLETION_PARAMETERS,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_STOP_POST_ASSOCIATE(
    hIhvExtAdapter: HANDLE,
    pPeer: PDOT11_MAC_ADDRESS,
    dot11AssocStatus: DOT11_ASSOC_STATUS,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_VALIDATE_PROFILE(
    hIhvExtAdapter: HANDLE,
    pIhvProfileParams: PDOT11EXT_IHV_PROFILE_PARAMS,
    pIhvConnProfile: PDOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pIhvSecProfile: PDOT11EXT_IHV_SECURITY_PROFILE,
    pdwReasonCode: PDWORD,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_PERFORM_CAPABILITY_MATCH(
    hIhvExtAdapter: HANDLE,
    pIhvProfileParams: PDOT11EXT_IHV_PROFILE_PARAMS,
    pIhvConnProfile: PDOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pIhvSecProfile: PDOT11EXT_IHV_SECURITY_PROFILE,
    pConnectableBssid: PDOT11_BSS_LIST,
    pdwReasonCode: PDWORD,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_CREATE_DISCOVERY_PROFILES(
    hIhvExtAdapter: HANDLE,
    bInsecure: BOOL,
    pIhvProfileParams: PDOT11EXT_IHV_PROFILE_PARAMS,
    pConnectableBssid: PDOT11_BSS_LIST,
    pIhvDiscoveryProfileList: PDOT11EXT_IHV_DISCOVERY_PROFILE_LIST,
    pdwReasonCode: PDWORD,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_PROCESS_SESSION_CHANGE(
    uEventType: ULONG,
    pSessionNotification: PWTSSESSION_NOTIFICATION,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_RECEIVE_INDICATION(
    hIhvExtAdapter: HANDLE,
    indicationType: DOT11EXT_IHV_INDICATION_TYPE,
    uBufferLength: ULONG,
    pvBuffer: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_RECEIVE_PACKET(
    hIhvExtAdapter: HANDLE,
    dwInBufferSize: DWORD,
    pvInBuffer: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_SEND_PACKET_COMPLETION(
    hSendCompletion: HANDLE,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_IS_UI_REQUEST_PENDING(
    guidUIRequest: GUID,
    pbIsRequestPending: PBOOL,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_PROCESS_UI_RESPONSE(
    guidUIRequest: GUID,
    dwByteCount: DWORD,
    pvResponseBuffer: LPVOID,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_QUERY_UI_REQUEST(
    hIhvExtAdapter: HANDLE,
    connectionPhase: DOT11EXT_IHV_CONNECTION_PHASE,
    ppIhvUIRequest: *mut PDOT11EXT_IHV_UI_REQUEST,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_ONEX_INDICATE_RESULT(
    hIhvExtAdapter: HANDLE,
    msOneXResult: DOT11_MSONEX_RESULT,
    pDot11MsOneXResultParams: PDOT11_MSONEX_RESULT_PARAMS,
) -> DWORD}
FN!{stdcall DOT11EXTIHV_CONTROL(
    hIhvExtAdapter: HANDLE,
    dwInBufferSize: DWORD,
    pInBuffer: PBYTE,
    dwOutBufferSize: DWORD,
    pOutBuffer: PBYTE,
    pdwBytesReturned: PDWORD,
) -> DWORD}
