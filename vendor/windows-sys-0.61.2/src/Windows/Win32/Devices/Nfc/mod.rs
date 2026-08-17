pub const ApplicationSelected: SECURE_ELEMENT_EVENT_TYPE = 2i32;
pub const ConnectionOriented: NFC_LLCP_SOCKET_TYPE = 0i32;
pub const Connectionless: NFC_LLCP_SOCKET_TYPE = 1i32;
pub const DefaultSnepServer: NFC_SNEP_SERVER_TYPE = 0i32;
pub const DeviceHost: SECURE_ELEMENT_TYPE = 2i32;
pub const Discovery: NFC_RELEASE_TYPE = 2i32;
pub const EVT_TRANSACTION_PARAMETER_MAX_LEN: u32 = 255u32;
pub const EVT_TRANSACTION_TAG_AID: u32 = 129u32;
pub const EVT_TRANSACTION_TAG_PARAMETERS: u32 = 130u32;
pub const EmulationDisabled: NFC_SE_EMULATION_MODE = 0i32;
pub const EmulationEnabled: NFC_SE_EMULATION_MODE = 1i32;
pub const EmulationOff: SECURE_ELEMENT_CARD_EMULATION_MODE = 0i32;
pub const EmulationOnPowerDependent: SECURE_ELEMENT_CARD_EMULATION_MODE = 2i32;
pub const EmulationOnPowerIndependent: SECURE_ELEMENT_CARD_EMULATION_MODE = 1i32;
pub const EmulationStealthListen: SECURE_ELEMENT_CARD_EMULATION_MODE = 3i32;
pub const ExtendedSnepServer: NFC_SNEP_SERVER_TYPE = 1i32;
pub const External: SECURE_ELEMENT_TYPE = 1i32;
pub const ExternalFieldEnter: SECURE_ELEMENT_EVENT_TYPE = 6i32;
pub const ExternalFieldExit: SECURE_ELEMENT_EVENT_TYPE = 7i32;
pub const ExternalReaderArrival: SECURE_ELEMENT_EVENT_TYPE = 0i32;
pub const ExternalReaderDeparture: SECURE_ELEMENT_EVENT_TYPE = 1i32;
pub const GUID_DEVINTERFACE_NFCDTA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7fd3f30b_5e49_4be1_b3aa_af06260d236a);
pub const GUID_DEVINTERFACE_NFCSE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8dc7c854_f5e5_4bed_815d_0c85ad047725);
pub const GUID_NFCSE_RADIO_MEDIA_DEVICE_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef8ba08f_148d_4116_83ef_a2679dfc3fa5);
pub const GUID_NFC_RADIO_MEDIA_DEVICE_INTERFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d51e930_750d_4a36_a9f7_91dc540fcd30);
pub const HceActivated: SECURE_ELEMENT_EVENT_TYPE = 4i32;
pub const HceDeactivated: SECURE_ELEMENT_EVENT_TYPE = 5i32;
pub const IOCTL_NFCDTA_CONFIG_P2P_PARAM: u32 = 2233376u32;
pub const IOCTL_NFCDTA_CONFIG_RF_DISCOVERY: u32 = 2233344u32;
pub const IOCTL_NFCDTA_LLCP_ACTIVATE: u32 = 2233476u32;
pub const IOCTL_NFCDTA_LLCP_CONFIG: u32 = 2233472u32;
pub const IOCTL_NFCDTA_LLCP_DEACTIVATE: u32 = 2233480u32;
pub const IOCTL_NFCDTA_LLCP_DISCOVER_SERVICES: u32 = 2233484u32;
pub const IOCTL_NFCDTA_LLCP_GET_NEXT_LINK_STATUS: u32 = 2233492u32;
pub const IOCTL_NFCDTA_LLCP_LINK_STATUS_CHECK: u32 = 2233488u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_ACCEPT: u32 = 2233512u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_BIND: u32 = 2233504u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_CLOSE: u32 = 2233500u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_CONNECT: u32 = 2233516u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_CREATE: u32 = 2233496u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_DISCONNECT: u32 = 2233520u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_GET_NEXT_ERROR: u32 = 2233540u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_LISTEN: u32 = 2233508u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_RECV: u32 = 2233524u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_RECV_FROM: u32 = 2233528u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_SEND: u32 = 2233532u32;
pub const IOCTL_NFCDTA_LLCP_SOCKET_SNED_TO: u32 = 2233536u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_CHECK_PRESENCE: u32 = 2233372u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_CONNECT: u32 = 2233352u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_DISCONNECT: u32 = 2233356u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_GET_NEXT: u32 = 2233348u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_NDEF_CHECK: u32 = 2233420u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_NDEF_CONVERT_READ_ONLY: u32 = 2233416u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_NDEF_READ: u32 = 2233412u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_NDEF_WRITE: u32 = 2233408u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_RECV: u32 = 2233364u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_SEND: u32 = 2233368u32;
pub const IOCTL_NFCDTA_REMOTE_DEV_TRANSCEIVE: u32 = 2233360u32;
pub const IOCTL_NFCDTA_SET_RF_CONFIG: u32 = 2233380u32;
pub const IOCTL_NFCDTA_SE_ENUMERATE: u32 = 2233728u32;
pub const IOCTL_NFCDTA_SE_GET_NEXT_EVENT: u32 = 2233740u32;
pub const IOCTL_NFCDTA_SE_SET_EMULATION_MODE: u32 = 2233732u32;
pub const IOCTL_NFCDTA_SE_SET_ROUTING_TABLE: u32 = 2233736u32;
pub const IOCTL_NFCDTA_SNEP_CLIENT_GET: u32 = 2233676u32;
pub const IOCTL_NFCDTA_SNEP_CLIENT_PUT: u32 = 2233672u32;
pub const IOCTL_NFCDTA_SNEP_DEINIT_CLIENT: u32 = 2233668u32;
pub const IOCTL_NFCDTA_SNEP_DEINIT_SERVER: u32 = 2233604u32;
pub const IOCTL_NFCDTA_SNEP_INIT_CLIENT: u32 = 2233664u32;
pub const IOCTL_NFCDTA_SNEP_INIT_SERVER: u32 = 2233600u32;
pub const IOCTL_NFCDTA_SNEP_SERVER_ACCEPT: u32 = 2233612u32;
pub const IOCTL_NFCDTA_SNEP_SERVER_GET_NEXT_CONNECTION: u32 = 2233608u32;
pub const IOCTL_NFCDTA_SNEP_SERVER_GET_NEXT_REQUEST: u32 = 2233616u32;
pub const IOCTL_NFCDTA_SNEP_SERVER_SEND_RESPONSE: u32 = 2233620u32;
pub const IOCTL_NFCRM_QUERY_RADIO_STATE: u32 = 5308808u32;
pub const IOCTL_NFCRM_SET_RADIO_STATE: u32 = 5308804u32;
pub const IOCTL_NFCSERM_QUERY_RADIO_STATE: u32 = 5308816u32;
pub const IOCTL_NFCSERM_SET_RADIO_STATE: u32 = 5308812u32;
pub const IOCTL_NFCSE_ENUM_ENDPOINTS: u32 = 2230272u32;
pub const IOCTL_NFCSE_GET_NEXT_EVENT: u32 = 2230280u32;
pub const IOCTL_NFCSE_GET_NFCC_CAPABILITIES: u32 = 2230288u32;
pub const IOCTL_NFCSE_GET_ROUTING_TABLE: u32 = 2230292u32;
pub const IOCTL_NFCSE_HCE_REMOTE_RECV: u32 = 2230592u32;
pub const IOCTL_NFCSE_HCE_REMOTE_SEND: u32 = 2230596u32;
pub const IOCTL_NFCSE_SET_CARD_EMULATION_MODE: u32 = 2230284u32;
pub const IOCTL_NFCSE_SET_POWER_MODE: u32 = 2230600u32;
pub const IOCTL_NFCSE_SET_ROUTING_TABLE: u32 = 2230296u32;
pub const IOCTL_NFCSE_SUBSCRIBE_FOR_EVENT: u32 = 2230276u32;
pub const ISO_7816_MAXIMUM_AID_LENGTH: u32 = 16u32;
pub const ISO_7816_MINIMUM_AID_LENGTH: u32 = 5u32;
pub const IdleMode: NFC_RELEASE_TYPE = 0i32;
pub const Integrated: SECURE_ELEMENT_TYPE = 0i32;
pub const LinkActivated: NFC_LLCP_LINK_STATUS = 0i32;
pub const LinkDeactivated: NFC_LLCP_LINK_STATUS = 1i32;
pub const MAX_ATR_LENGTH: u32 = 48u32;
pub const MAX_LLCP_SERVICE_NAME_SIZE: u32 = 256u32;
pub const MAX_SNEP_SERVER_NAME_SIZE: u32 = 256u32;
pub const MAX_UID_SIZE: u32 = 16u32;
pub const NFCRMDDI_IOCTL_BASE: u32 = 80u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFCRM_RADIO_STATE {
    pub MediaRadioOn: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFCRM_SET_RADIO_STATE {
    pub SystemStateUpdate: bool,
    pub MediaRadioOn: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_DATA_BUFFER {
    pub cbBuffer: u16,
    pub pbBuffer: [u8; 1],
}
impl Default for NFC_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NFC_DEVICE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_CONFIG {
    pub uMIU: u16,
    pub uWKS: u16,
    pub bLTO: u8,
    pub bOptions: u8,
    pub fAutoActivate: bool,
}
pub type NFC_LLCP_LINK_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_LLCP_SERVICE_DISCOVER_REQUEST {
    pub hRemoteDev: isize,
    pub NumberOfEntries: u32,
    pub ServiceNameEntries: [NFC_LLCP_SERVICE_NAME_ENTRY; 1],
}
impl Default for NFC_LLCP_SERVICE_DISCOVER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_LLCP_SERVICE_DISCOVER_SAP {
    pub NumberOfEntries: u32,
    pub SAPEntries: [u8; 1],
}
impl Default for NFC_LLCP_SERVICE_DISCOVER_SAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_LLCP_SERVICE_NAME_ENTRY {
    pub cbServiceName: u32,
    pub pbServiceName: [u8; 1],
}
impl Default for NFC_LLCP_SERVICE_NAME_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_ACCEPT_INFO {
    pub hSocket: isize,
    pub sSocketOption: NFC_LLCP_SOCKET_OPTION,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_CL_PAYLOAD {
    pub hSocket: isize,
    pub bSAP: u8,
    pub sPayload: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_LLCP_SOCKET_CONNECT_INFO {
    pub hRemoteDev: isize,
    pub hSocket: isize,
    pub eConnectType: NFC_LLCP_SOCKET_CONNECT_TYPE,
    pub Anonymous: NFC_LLCP_SOCKET_CONNECT_INFO_0,
}
impl Default for NFC_LLCP_SOCKET_CONNECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NFC_LLCP_SOCKET_CONNECT_INFO_0 {
    pub bSAP: u8,
    pub sServiceName: NFC_LLCP_SERVICE_NAME_ENTRY,
}
impl Default for NFC_LLCP_SOCKET_CONNECT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NFC_LLCP_SOCKET_CONNECT_TYPE = i32;
pub type NFC_LLCP_SOCKET_ERROR = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_ERROR_INFO {
    pub hSocket: isize,
    pub eSocketError: NFC_LLCP_SOCKET_ERROR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_INFO {
    pub eSocketType: NFC_LLCP_SOCKET_TYPE,
    pub sSocketOption: NFC_LLCP_SOCKET_OPTION,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_OPTION {
    pub uMIUX: u16,
    pub bRW: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_PAYLOAD {
    pub hSocket: isize,
    pub bSAP: u8,
    pub sPayload: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_LLCP_SOCKET_SERVICE_INFO {
    pub hSocket: isize,
    pub bSAP: u8,
    pub sServiceName: NFC_LLCP_SERVICE_NAME_ENTRY,
}
pub type NFC_LLCP_SOCKET_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_NDEF_INFO {
    pub fIsNdefFormatted: bool,
    pub fIsReadOnly: bool,
    pub dwActualMessageLength: u32,
    pub dwMaxMessageLength: u32,
}
pub type NFC_P2P_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_P2P_PARAM_CONFIG {
    pub eP2pMode: NFC_P2P_MODE,
    pub cbGeneralBytes: u8,
    pub pbGeneralBytes: [u8; 48],
}
impl Default for NFC_P2P_PARAM_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NFC_RELEASE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_REMOTE_DEVICE_DISCONNET {
    pub hRemoteDev: isize,
    pub eReleaseType: NFC_RELEASE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_REMOTE_DEV_INFO {
    pub hRemoteDev: isize,
    pub eType: NFC_DEVICE_TYPE,
    pub eRFTech: u8,
    pub eProtocol: u8,
    pub cbUid: u8,
    pub pbUid: [u8; 16],
}
impl Default for NFC_REMOTE_DEV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_REMOTE_DEV_RECV_INFO {
    pub hRemoteDev: isize,
    pub sRecvBuffer: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_REMOTE_DEV_SEND_INFO {
    pub hRemoteDev: isize,
    pub usTimeOut: u16,
    pub sSendBuffer: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_RF_DISCOVERY_CONFIG {
    pub usTotalDuration: u16,
    pub ulPollConfig: u32,
    pub fDisableCardEmulation: bool,
    pub ucNfcIPMode: u8,
    pub fNfcIPTgtModeDisable: bool,
    pub ucNfcIPTgtMode: u8,
    pub ucNfcCEMode: u8,
    pub ucBailoutConfig: u8,
    pub ucSystemCode: [u8; 2],
    pub ucRequestCode: u8,
    pub ucTimeSlotNumber: u8,
    pub eRfDiscoveryMode: NFC_RF_DISCOVERY_MODE,
}
impl Default for NFC_RF_DISCOVERY_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NFC_RF_DISCOVERY_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_SE_AID_ROUTING_INFO {
    pub hSecureElement: isize,
    pub bPowerState: u8,
    pub cbAid: u32,
    pub pbAid: [u8; 16],
}
impl Default for NFC_SE_AID_ROUTING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NFC_SE_EMULATION_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SE_EMULATION_MODE_INFO {
    pub hSecureElement: isize,
    pub eMode: NFC_SE_EMULATION_MODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_SE_EVENT_INFO {
    pub hSecureElement: isize,
    pub eEventType: SECURE_ELEMENT_EVENT_TYPE,
    pub cbEventData: u32,
    pub pbEventData: [u8; 1],
}
impl Default for NFC_SE_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SE_INFO {
    pub hSecureElement: isize,
    pub eSecureElementType: SECURE_ELEMENT_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_SE_LIST {
    pub NumberOfEndpoints: u32,
    pub EndpointList: [NFC_SE_INFO; 1],
}
impl Default for NFC_SE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SE_PROTO_ROUTING_INFO {
    pub hSecureElement: isize,
    pub bPowerState: u8,
    pub eRfProtocolType: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_SE_ROUTING_TABLE {
    pub NumberOfEntries: u32,
    pub TableEntries: [NFC_SE_ROUTING_TABLE_ENTRY; 1],
}
impl Default for NFC_SE_ROUTING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NFC_SE_ROUTING_TABLE_ENTRY {
    pub eRoutingType: SECURE_ELEMENT_ROUTING_TYPE,
    pub Anonymous: NFC_SE_ROUTING_TABLE_ENTRY_0,
}
impl Default for NFC_SE_ROUTING_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NFC_SE_ROUTING_TABLE_ENTRY_0 {
    pub TechRoutingInfo: NFC_SE_TECH_ROUTING_INFO,
    pub ProtoRoutingInfo: NFC_SE_PROTO_ROUTING_INFO,
    pub AidRoutingInfo: NFC_SE_AID_ROUTING_INFO,
}
impl Default for NFC_SE_ROUTING_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SE_TECH_ROUTING_INFO {
    pub hSecureElement: isize,
    pub bPowerState: u8,
    pub eRfTechType: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_CLIENT_GET_INFO {
    pub hSnepClient: isize,
    pub sGetPayload: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_CLIENT_INFO {
    pub hRemoteDev: isize,
    pub eServerType: NFC_SNEP_SERVER_TYPE,
    pub sSocketOption: NFC_LLCP_SOCKET_OPTION,
    pub sService: NFC_LLCP_SERVICE_NAME_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_CLIENT_PUT_INFO {
    pub hSnepClient: isize,
    pub sPutPayload: NFC_DATA_BUFFER,
}
pub type NFC_SNEP_REQUEST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_SERVER_ACCEPT_INFO {
    pub hSnepServer: isize,
    pub hConnection: isize,
    pub sSocketOption: NFC_LLCP_SOCKET_OPTION,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_SERVER_INFO {
    pub eServerType: NFC_SNEP_SERVER_TYPE,
    pub sSocketOption: NFC_LLCP_SOCKET_OPTION,
    pub usInboxSize: u16,
    pub bSAP: u8,
    pub sService: NFC_LLCP_SERVICE_NAME_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_SERVER_REQUEST {
    pub hSnepServer: isize,
    pub hConnection: isize,
    pub eRequestType: NFC_SNEP_REQUEST_TYPE,
    pub sRequestPayload: NFC_DATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NFC_SNEP_SERVER_RESPONSE_INFO {
    pub hSnepServer: isize,
    pub hConnection: isize,
    pub dwResponseStatus: u32,
    pub sResponsePayload: NFC_DATA_BUFFER,
}
pub type NFC_SNEP_SERVER_TYPE = i32;
pub const NfcConnectBySap: NFC_LLCP_SOCKET_CONNECT_TYPE = 0i32;
pub const NfcConnectByUri: NFC_LLCP_SOCKET_CONNECT_TYPE = 1i32;
pub const NfcDepDefault: NFC_P2P_MODE = 0i32;
pub const NfcDepListen: NFC_P2P_MODE = 2i32;
pub const NfcDepPoll: NFC_P2P_MODE = 1i32;
pub const NfcIP1Initiator: NFC_DEVICE_TYPE = 5i32;
pub const NfcIP1Target: NFC_DEVICE_TYPE = 4i32;
pub const NfcLlcpErrorBusyCondition: NFC_LLCP_SOCKET_ERROR = 2i32;
pub const NfcLlcpErrorDisconnected: NFC_LLCP_SOCKET_ERROR = 0i32;
pub const NfcLlcpErrorFrameRejected: NFC_LLCP_SOCKET_ERROR = 1i32;
pub const NfcLlcpErrorNotBusyCondition: NFC_LLCP_SOCKET_ERROR = 3i32;
pub const NfcReader: NFC_DEVICE_TYPE = 6i32;
pub const NfcType1Tag: NFC_DEVICE_TYPE = 0i32;
pub const NfcType2Tag: NFC_DEVICE_TYPE = 1i32;
pub const NfcType3Tag: NFC_DEVICE_TYPE = 2i32;
pub const NfcType4Tag: NFC_DEVICE_TYPE = 3i32;
pub const RFDiscoveryResume: NFC_RF_DISCOVERY_MODE = 2i32;
pub const RfDiscoveryConfig: NFC_RF_DISCOVERY_MODE = 0i32;
pub const RfDiscoveryStart: NFC_RF_DISCOVERY_MODE = 1i32;
pub const RoutingTypeAid: SECURE_ELEMENT_ROUTING_TYPE = 2i32;
pub const RoutingTypeProtocol: SECURE_ELEMENT_ROUTING_TYPE = 1i32;
pub const RoutingTypeTech: SECURE_ELEMENT_ROUTING_TYPE = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_AID_ROUTING_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub cbAid: u32,
    pub pbAid: [u8; 16],
}
impl Default for SECURE_ELEMENT_AID_ROUTING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECURE_ELEMENT_CARD_EMULATION_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_ENDPOINT_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eSecureElementType: SECURE_ELEMENT_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_ENDPOINT_LIST {
    pub NumberOfEndpoints: u32,
    pub EndpointList: [SECURE_ELEMENT_ENDPOINT_INFO; 1],
}
impl Default for SECURE_ELEMENT_ENDPOINT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_EVENT_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eEventType: SECURE_ELEMENT_EVENT_TYPE,
    pub cbEventData: u32,
    pub pbEventData: [u8; 1],
}
impl Default for SECURE_ELEMENT_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_EVENT_SUBSCRIPTION_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eEventType: SECURE_ELEMENT_EVENT_TYPE,
}
pub type SECURE_ELEMENT_EVENT_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_HCE_ACTIVATION_PAYLOAD {
    pub bConnectionId: u16,
    pub eRfTechType: u8,
    pub eRfProtocolType: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_HCE_DATA_PACKET {
    pub bConnectionId: u16,
    pub cbPayload: u16,
    pub pbPayload: [u8; 1],
}
impl Default for SECURE_ELEMENT_HCE_DATA_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_NFCC_CAPABILITIES {
    pub cbMaxRoutingTableSize: u16,
    pub IsAidRoutingSupported: bool,
    pub IsProtocolRoutingSupported: bool,
    pub IsTechRoutingSupported: bool,
}
pub type SECURE_ELEMENT_POWER_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_PROTO_ROUTING_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eRfProtocolType: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_ROUTING_TABLE {
    pub NumberOfEntries: u32,
    pub TableEntries: [SECURE_ELEMENT_ROUTING_TABLE_ENTRY; 1],
}
impl Default for SECURE_ELEMENT_ROUTING_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURE_ELEMENT_ROUTING_TABLE_ENTRY {
    pub eRoutingType: SECURE_ELEMENT_ROUTING_TYPE,
    pub Anonymous: SECURE_ELEMENT_ROUTING_TABLE_ENTRY_0,
}
impl Default for SECURE_ELEMENT_ROUTING_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SECURE_ELEMENT_ROUTING_TABLE_ENTRY_0 {
    pub TechRoutingInfo: SECURE_ELEMENT_TECH_ROUTING_INFO,
    pub ProtoRoutingInfo: SECURE_ELEMENT_PROTO_ROUTING_INFO,
    pub AidRoutingInfo: SECURE_ELEMENT_AID_ROUTING_INFO,
}
impl Default for SECURE_ELEMENT_ROUTING_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SECURE_ELEMENT_ROUTING_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_SET_CARD_EMULATION_MODE_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eMode: SECURE_ELEMENT_CARD_EMULATION_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_SET_POWER_MODE_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub powerMode: SECURE_ELEMENT_POWER_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SECURE_ELEMENT_TECH_ROUTING_INFO {
    pub guidSecureElementId: windows_sys::core::GUID,
    pub eRfTechType: u8,
}
pub type SECURE_ELEMENT_TYPE = i32;
pub const SEPowerMode_AllowOff: SECURE_ELEMENT_POWER_MODE = 1i32;
pub const SEPowerMode_ForceOn: SECURE_ELEMENT_POWER_MODE = 0i32;
pub const SleepMode: NFC_RELEASE_TYPE = 1i32;
pub const SnepRequestGet: NFC_SNEP_REQUEST_TYPE = 0i32;
pub const SnepRequestPut: NFC_SNEP_REQUEST_TYPE = 1i32;
pub const Transaction: SECURE_ELEMENT_EVENT_TYPE = 3i32;
