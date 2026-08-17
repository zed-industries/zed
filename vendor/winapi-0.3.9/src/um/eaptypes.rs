// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, BYTE, DWORD};
use um::winnt::LPWSTR;
pub const eapPropCipherSuiteNegotiation: DWORD = 0x00000001;
pub const eapPropMutualAuth: DWORD = 0x00000002;
pub const eapPropIntegrity: DWORD = 0x00000004;
pub const eapPropReplayProtection: DWORD = 0x00000008;
pub const eapPropConfidentiality: DWORD = 0x00000010;
pub const eapPropKeyDerivation: DWORD = 0x00000020;
pub const eapPropKeyStrength64: DWORD = 0x00000040;
pub const eapPropKeyStrength128: DWORD = 0x00000080;
pub const eapPropKeyStrength256: DWORD = 0x00000100;
pub const eapPropKeyStrength512: DWORD = 0x00000200;
pub const eapPropKeyStrength1024: DWORD = 0x00000400;
pub const eapPropDictionaryAttackResistance: DWORD = 0x00000800;
pub const eapPropFastReconnect: DWORD = 0x00001000;
pub const eapPropCryptoBinding: DWORD = 0x00002000;
pub const eapPropSessionIndependence: DWORD = 0x00004000;
pub const eapPropFragmentation: DWORD = 0x00008000;
pub const eapPropChannelBinding: DWORD = 0x00010000;
pub const eapPropNap: DWORD = 0x00020000;
pub const eapPropStandalone: DWORD = 0x00040000;
pub const eapPropMppeEncryption: DWORD = 0x00080000;
pub const eapPropTunnelMethod: DWORD = 0x00100000;
pub const eapPropSupportsConfig: DWORD = 0x00200000;
pub const eapPropCertifiedMethod: DWORD = 0x00400000;
pub const eapPropHiddenMethod: DWORD = 0x00800000;
pub const eapPropMachineAuth: DWORD = 0x01000000;
pub const eapPropUserAuth: DWORD = 0x02000000;
pub const eapPropIdentityPrivacy: DWORD = 0x04000000;
pub const eapPropMethodChaining: DWORD = 0x08000000;
pub const eapPropSharedStateEquivalence: DWORD = 0x10000000;
pub const eapPropReserved: DWORD = 0x80000000;
pub const EAP_VALUENAME_PROPERTIES: &'static str = "Properties";
pub type EAP_SESSIONID = DWORD;
STRUCT!{struct EAP_TYPE {
    type_: BYTE,
    dwVendorId: DWORD,
    dwVendorType: DWORD,
}}
STRUCT!{struct EAP_METHOD_TYPE {
    eapType: EAP_TYPE,
    dwAuthorId: DWORD,
}}
STRUCT!{struct EAP_METHOD_INFO {
    eaptype: EAP_METHOD_TYPE,
    pwszAuthorName: LPWSTR,
    pwszFriendlyName: LPWSTR,
    eapProperties: DWORD,
    pInnerMethodInfo: *mut EAP_METHOD_INFO,
}}
STRUCT!{struct EAP_METHOD_INFO_EX {
    eaptype: EAP_METHOD_TYPE,
    pwszAuthorName: LPWSTR,
    pwszFriendlyName: LPWSTR,
    eapProperties: DWORD,
    pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}}
STRUCT!{struct EAP_METHOD_INFO_ARRAY {
    dwNumberOfMethods: DWORD,
    pEapMethods: *mut EAP_METHOD_INFO,
}}
STRUCT!{struct EAP_METHOD_INFO_ARRAY_EX {
    dwNumberOfMethods: DWORD,
    pEapMethods: *mut EAP_METHOD_INFO_EX,
}}
STRUCT!{struct EAP_ERROR {
    dwWinError: DWORD,
    type_: EAP_METHOD_TYPE,
    dwReasonCode: DWORD,
    rootCauseGuid: GUID,
    repairGuid: GUID,
    helpLinkGuid: GUID,
    pRootCauseString: LPWSTR,
    pRepairString: LPWSTR,
}}
ENUM!{enum EAP_ATTRIBUTE_TYPE {
    eatMinimum = 0,
    eatUserName = 1,
    eatUserPassword = 2,
    eatMD5CHAPPassword = 3,
    eatNASIPAddress = 4,
    eatNASPort = 5,
    eatServiceType = 6,
    eatFramedProtocol = 7,
    eatFramedIPAddress = 8,
    eatFramedIPNetmask = 9,
    eatFramedRouting = 10,
    eatFilterId = 11,
    eatFramedMTU = 12,
    eatFramedCompression = 13,
    eatLoginIPHost = 14,
    eatLoginService = 15,
    eatLoginTCPPort = 16,
    eatUnassigned17 = 17,
    eatReplyMessage = 18,
    eatCallbackNumber = 19,
    eatCallbackId = 20,
    eatUnassigned21 = 21,
    eatFramedRoute = 22,
    eatFramedIPXNetwork = 23,
    eatState = 24,
    eatClass = 25,
    eatVendorSpecific = 26,
    eatSessionTimeout = 27,
    eatIdleTimeout = 28,
    eatTerminationAction = 29,
    eatCalledStationId = 30,
    eatCallingStationId = 31,
    eatNASIdentifier = 32,
    eatProxyState = 33,
    eatLoginLATService = 34,
    eatLoginLATNode = 35,
    eatLoginLATGroup = 36,
    eatFramedAppleTalkLink = 37,
    eatFramedAppleTalkNetwork = 38,
    eatFramedAppleTalkZone = 39,
    eatAcctStatusType = 40,
    eatAcctDelayTime = 41,
    eatAcctInputOctets = 42,
    eatAcctOutputOctets = 43,
    eatAcctSessionId = 44,
    eatAcctAuthentic = 45,
    eatAcctSessionTime = 46,
    eatAcctInputPackets = 47,
    eatAcctOutputPackets = 48,
    eatAcctTerminateCause = 49,
    eatAcctMultiSessionId = 50,
    eatAcctLinkCount = 51,
    eatAcctEventTimeStamp = 55,
    eatMD5CHAPChallenge = 60,
    eatNASPortType = 61,
    eatPortLimit = 62,
    eatLoginLATPort = 63,
    eatTunnelType = 64,
    eatTunnelMediumType = 65,
    eatTunnelClientEndpoint = 66,
    eatTunnelServerEndpoint = 67,
    eatARAPPassword = 70,
    eatARAPFeatures = 71,
    eatARAPZoneAccess = 72,
    eatARAPSecurity = 73,
    eatARAPSecurityData = 74,
    eatPasswordRetry = 75,
    eatPrompt = 76,
    eatConnectInfo = 77,
    eatConfigurationToken = 78,
    eatEAPMessage = 79,
    eatSignature = 80,
    eatARAPChallengeResponse = 84,
    eatAcctInterimInterval = 85,
    eatNASIPv6Address = 95,
    eatFramedInterfaceId = 96,
    eatFramedIPv6Prefix = 97,
    eatLoginIPv6Host = 98,
    eatFramedIPv6Route = 99,
    eatFramedIPv6Pool = 100,
    eatARAPGuestLogon = 8096,
    eatCertificateOID = 8097,
    eatEAPConfiguration = 8098,
    eatPEAPEmbeddedEAPTypeId = 8099,
    eatPEAPFastRoamedSession = 8100,
    eatFastRoamedSession = 8100,
    eatEAPTLV = 8102,
    eatCredentialsChanged = 8103,
    eatInnerEapMethodType = 8104,
    eatClearTextPassword = 8107,
    eatQuarantineSoH = 8150,
    eatCertificateThumbprint = 8250,
    eatPeerId = 9000,
    eatServerId = 9001,
    eatMethodId = 9002,
    eatEMSK = 9003,
    eatSessionId = 9004,
    eatReserved = 0xFFFFFFFF,
}}
pub type EapAttributeType = EAP_ATTRIBUTE_TYPE;
STRUCT!{struct EAP_ATTRIBUTE {
    eaType: EAP_ATTRIBUTE_TYPE,
    dwLength: DWORD,
    pValue: *mut BYTE,
}}
pub type EapAttribute = EAP_ATTRIBUTE;
STRUCT!{struct EAP_ATTRIBUTES {
    dwNumberOfAttributes: DWORD,
    pAttribs: *mut EAP_ATTRIBUTE,
}}
pub type EapAttributes = EAP_ATTRIBUTES;
pub const EAP_FLAG_Reserved1: DWORD = 0x00000001;
pub const EAP_FLAG_NON_INTERACTIVE: DWORD = 0x00000002;
pub const EAP_FLAG_LOGON: DWORD = 0x00000004;
pub const EAP_FLAG_PREVIEW: DWORD = 0x00000008;
pub const EAP_FLAG_Reserved2: DWORD = 0x00000010;
pub const EAP_FLAG_MACHINE_AUTH: DWORD = 0x00000020;
pub const EAP_FLAG_GUEST_ACCESS: DWORD = 0x00000040;
pub const EAP_FLAG_Reserved3: DWORD = 0x00000080;
pub const EAP_FLAG_Reserved4: DWORD = 0x00000100;
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: DWORD = 0x00000200;
pub const EAP_FLAG_Reserved5: DWORD = 0x00000400;
pub const EAP_FLAG_Reserved6: DWORD = 0x00000800;
pub const EAP_FLAG_FULL_AUTH: DWORD = 0x00001000;
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: DWORD = 0x00002000;
pub const EAP_FLAG_Reserved7: DWORD = 0x00004000;
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: DWORD = 0x00008000;
pub const EAP_FLAG_SUPRESS_UI: DWORD = 0x00010000;
pub const EAP_FLAG_PRE_LOGON: DWORD = 0x00020000;
pub const EAP_FLAG_USER_AUTH: DWORD = 0x00040000;
pub const EAP_FLAG_CONFG_READONLY: DWORD = 0x00080000;
pub const EAP_FLAG_Reserved8: DWORD = 0x00100000;
pub const EAP_FLAG_Reserved9: DWORD = 0x00400000;
pub const EAP_FLAG_VPN: DWORD = 0x00800000;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: DWORD = 0x00000000;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: DWORD = 0x00000001;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: DWORD = 0x00000002;
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: DWORD = EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: DWORD =
    EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: DWORD = 0x00000002;
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: DWORD = 0x00000004;
ENUM!{enum EAP_CONFIG_INPUT_FIELD_TYPE {
    EapConfigInputUsername = 0,
    EapConfigInputPassword = 1,
    EapConfigInputNetworkUsername = 2,
    EapConfigInputNetworkPassword = 3,
    EapConfigInputPin = 4,
    EapConfigInputPSK = 5,
    EapConfigInputEdit = 6,
    EapConfigSmartCardUsername = 7,
    EapConfigSmartCardError = 8,
}}
pub type PEAP_CONFIG_INPUT_FIELD_TYPE = *mut EAP_CONFIG_INPUT_FIELD_TYPE;
pub const EAP_CREDENTIAL_VERSION: i32 = 1;
pub const EAP_INTERACTIVE_UI_DATA_VERSION: i32 = 1;
pub const EAPHOST_PEER_API_VERSION: i32 = 1;
pub const EAPHOST_METHOD_API_VERSION: i32 = 1;
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: i32 = 256;
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: i32 = 1024;
STRUCT!{struct EAP_CONFIG_INPUT_FIELD_DATA {
    dwSize: DWORD,
    Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    dwFlagProps: DWORD,
    pwszLabel: LPWSTR,
    pwszData: LPWSTR,
    dwMinDataLength: DWORD,
    dwMaxDataLength: DWORD,
}}
pub type PEAP_CONFIG_INPUT_FIELD_DATA = *mut EAP_CONFIG_INPUT_FIELD_DATA;
STRUCT!{struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    dwVersion: DWORD,
    dwNumberOfFields: DWORD,
    pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}}
pub type PEAP_CONFIG_INPUT_FIELD_ARRAY = *mut EAP_CONFIG_INPUT_FIELD_ARRAY;
ENUM!{enum EAP_INTERACTIVE_UI_DATA_TYPE {
    EapCredReq = 0,
    EapCredResp = 1,
    EapCredExpiryReq = 2,
    EapCredExpiryResp = 3,
    EapCredLogonReq = 4,
    EapCredLogonResp = 5,
}}
pub type EAP_CRED_REQ = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_RESP = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_LOGON_REQ = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_LOGON_RESP = EAP_CONFIG_INPUT_FIELD_ARRAY;
STRUCT!{struct EAP_CRED_EXPIRY_REQ {
    curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}}
pub type EAP_CRED_EXPIRY_RESP = EAP_CRED_EXPIRY_REQ;
UNION!{union EAP_UI_DATA_FORMAT {
    [usize; 1],
    credData credData_mut: *mut EAP_CRED_REQ,
    credExpiryData credExpiryData_mut: *mut EAP_CRED_EXPIRY_REQ,
    credLogonData credLogonData_mut: *mut EAP_CRED_LOGON_REQ,
}}
STRUCT!{struct EAP_INTERACTIVE_UI_DATA {
    dwVersion: DWORD,
    dwSize: DWORD,
    dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    cbUiData: DWORD,
    pbUiData: EAP_UI_DATA_FORMAT,
}}
ENUM!{enum EAP_METHOD_PROPERTY_TYPE {
    emptPropCipherSuiteNegotiation = 0,
    emptPropMutualAuth = 1,
    emptPropIntegrity = 2,
    emptPropReplayProtection = 3,
    emptPropConfidentiality = 4,
    emptPropKeyDerivation = 5,
    emptPropKeyStrength64 = 6,
    emptPropKeyStrength128 = 7,
    emptPropKeyStrength256 = 8,
    emptPropKeyStrength512 = 9,
    emptPropKeyStrength1024 = 10,
    emptPropDictionaryAttackResistance = 11,
    emptPropFastReconnect = 12,
    emptPropCryptoBinding = 13,
    emptPropSessionIndependence = 14,
    emptPropFragmentation = 15,
    emptPropChannelBinding = 16,
    emptPropNap = 17,
    emptPropStandalone = 18,
    emptPropMppeEncryption = 19,
    emptPropTunnelMethod = 20,
    emptPropSupportsConfig = 21,
    emptPropCertifiedMethod = 22,
    emptPropHiddenMethod = 23,
    emptPropMachineAuth = 24,
    emptPropUserAuth = 25,
    emptPropIdentityPrivacy = 26,
    emptPropMethodChaining = 27,
    emptPropSharedStateEquivalence = 28,
    emptLegacyMethodPropertyFlag = 31,
    emptPropVendorSpecific = 255,
}}
ENUM!{enum EAP_METHOD_PROPERTY_VALUE_TYPE {
    empvtBool = 0,
    empvtDword = 1,
    empvtString = 2,
}}
STRUCT!{struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    length: DWORD,
    value: BOOL,
}}
STRUCT!{struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    length: DWORD,
    value: DWORD,
}}
STRUCT!{struct EAP_METHOD_PROPERTY_VALUE_STRING {
    length: DWORD,
    value: *mut BYTE,
}}
UNION!{union EAP_METHOD_PROPERTY_VALUE {
    [usize; 2],
    empvBool empvBool_mut: EAP_METHOD_PROPERTY_VALUE_BOOL,
    empvDword empvDword_mut: EAP_METHOD_PROPERTY_VALUE_DWORD,
    empvString empvString_mut: EAP_METHOD_PROPERTY_VALUE_STRING,
}}
STRUCT!{struct EAP_METHOD_PROPERTY {
    eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}}
STRUCT!{struct EAP_METHOD_PROPERTY_ARRAY {
    dwNumberOfProperties: DWORD,
    pMethodProperty: *mut EAP_METHOD_PROPERTY,
}}
STRUCT!{struct EAPHOST_IDENTITY_UI_PARAMS {
    eapMethodType: EAP_METHOD_TYPE,
    dwFlags: DWORD,
    dwSizeofConnectionData: DWORD,
    pConnectionData: *mut BYTE,
    dwSizeofUserData: DWORD,
    pUserData: *mut BYTE,
    dwSizeofUserDataOut: DWORD,
    pUserDataOut: *mut BYTE,
    pwszIdentity: LPWSTR,
    dwError: DWORD,
    pEapError: *mut EAP_ERROR,
}}
STRUCT!{struct EAPHOST_INTERACTIVE_UI_PARAMS {
    dwSizeofContextData: DWORD,
    pContextData: *mut BYTE,
    dwSizeofInteractiveUIData: DWORD,
    pInteractiveUIData: *mut BYTE,
    dwError: DWORD,
    pEapError: *mut EAP_ERROR,
}}
ENUM!{enum EapCredentialType {
    EAP_EMPTY_CREDENTIAL = 0,
    EAP_USERNAME_PASSWORD_CREDENTIAL = 1,
    EAP_WINLOGON_CREDENTIAL = 2,
    EAP_CERTIFICATE_CREDENTIAL = 3,
    EAP_SIM_CREDENTIAL = 4,
}}
STRUCT!{struct EapUsernamePasswordCredential {
    username: LPWSTR,
    password: LPWSTR,
}}
pub const CERTIFICATE_HASH_LENGTH: usize = 20;
STRUCT!{struct EapCertificateCredential {
    certHash: [BYTE; CERTIFICATE_HASH_LENGTH],
    password: LPWSTR,
}}
STRUCT!{struct EapSimCredential {
    iccID: LPWSTR,
}}
UNION!{union EapCredentialTypeData {
    [u32; 6] [u64; 4],
    username_password username_password_mut: EapUsernamePasswordCredential,
    certificate certificate_mut: EapCertificateCredential,
    sim sim_mut: EapSimCredential,
}}
STRUCT!{struct EapCredential {
    credType: EapCredentialType,
    credData: EapCredentialTypeData,
}}
