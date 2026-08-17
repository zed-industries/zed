// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD};
use um::eaptypes::EAP_METHOD_TYPE;
use um::l2cmn::L2_REASON_CODE_ONEX_BASE;
use um::winnt::HANDLE;
ENUM!{enum ONEX_AUTH_IDENTITY {
    OneXAuthIdentityNone = 0,
    OneXAuthIdentityMachine = 1,
    OneXAuthIdentityUser = 2,
    OneXAuthIdentityExplicitUser = 3,
    OneXAuthIdentityGuest = 4,
    OneXAuthIdentityInvalid = 5,
}}
pub type PONEX_AUTH_IDENTITY = *mut ONEX_AUTH_IDENTITY;
ENUM!{enum ONEX_AUTH_STATUS {
    OneXAuthNotStarted = 0,
    OneXAuthInProgress = 1,
    OneXAuthNoAuthenticatorFound = 2,
    OneXAuthSuccess = 3,
    OneXAuthFailure = 4,
    OneXAuthInvalid = 5,
}}
pub type PONEX_AUTH_STATUS = *mut ONEX_AUTH_STATUS;
ENUM!{enum ONEX_REASON_CODE {
    ONEX_REASON_CODE_SUCCESS = 0,
    ONEX_REASON_START = L2_REASON_CODE_ONEX_BASE,
    ONEX_UNABLE_TO_IDENTIFY_USER = 327681,
    ONEX_IDENTITY_NOT_FOUND = 327682,
    ONEX_UI_DISABLED = 327683,
    ONEX_UI_FAILURE = 327684,
    ONEX_EAP_FAILURE_RECEIVED = 327685,
    ONEX_AUTHENTICATOR_NO_LONGER_PRESENT = 327686,
    ONEX_NO_RESPONSE_TO_IDENTITY = 327687,
    ONEX_PROFILE_VERSION_NOT_SUPPORTED = 327688,
    ONEX_PROFILE_INVALID_LENGTH = 327689,
    ONEX_PROFILE_DISALLOWED_EAP_TYPE = 327690,
    ONEX_PROFILE_INVALID_EAP_TYPE_OR_FLAG = 327691,
    ONEX_PROFILE_INVALID_ONEX_FLAGS = 327692,
    ONEX_PROFILE_INVALID_TIMER_VALUE = 327693,
    ONEX_PROFILE_INVALID_SUPPLICANT_MODE = 327694,
    ONEX_PROFILE_INVALID_AUTH_MODE = 327695,
    ONEX_PROFILE_INVALID_EAP_CONNECTION_PROPERTIES = 327696,
    ONEX_UI_CANCELLED = 327697,
    ONEX_PROFILE_INVALID_EXPLICIT_CREDENTIALS = 327698,
    ONEX_PROFILE_EXPIRED_EXPLICIT_CREDENTIALS = 327699,
    ONEX_UI_NOT_PERMITTED = 327700,
}}
pub type PONEX_REASON_CODE = *mut ONEX_REASON_CODE;
ENUM!{enum ONEX_NOTIFICATION_TYPE {
    OneXPublicNotificationBase = 0,
    OneXNotificationTypeResultUpdate = 1,
    OneXNotificationTypeAuthRestarted = 2,
    OneXNotificationTypeEventInvalid = 3,
    OneXNumNotifications = OneXNotificationTypeEventInvalid,
}}
pub type PONEX_NOTIFICATION_TYPE = *mut ONEX_NOTIFICATION_TYPE;
ENUM!{enum ONEX_AUTH_RESTART_REASON {
    OneXRestartReasonPeerInitiated = 0,
    OneXRestartReasonMsmInitiated = 1,
    OneXRestartReasonOneXHeldStateTimeout = 2,
    OneXRestartReasonOneXAuthTimeout = 3,
    OneXRestartReasonOneXConfigurationChanged = 4,
    OneXRestartReasonOneXUserChanged = 5,
    OneXRestartReasonQuarantineStateChanged = 6,
    OneXRestartReasonAltCredsTrial = 7,
    OneXRestartReasonInvalid = 8,
}}
pub type PONEX_AUTH_RESTART_REASON = *mut ONEX_AUTH_RESTART_REASON;
STRUCT!{struct ONEX_VARIABLE_BLOB {
    dwSize: DWORD,
    dwOffset: DWORD,
}}
pub type PONEX_VARIABLE_BLOB = *mut ONEX_VARIABLE_BLOB;
STRUCT!{struct ONEX_AUTH_PARAMS {
    fUpdatePending: BOOL,
    oneXConnProfile: ONEX_VARIABLE_BLOB,
    authIdentity: ONEX_AUTH_IDENTITY,
    dwQuarantineState: DWORD,
    Bitfields: DWORD,
    dwSessionId: DWORD,
    hUserToken: HANDLE,
    OneXUserProfile: ONEX_VARIABLE_BLOB,
    Identity: ONEX_VARIABLE_BLOB,
    UserName: ONEX_VARIABLE_BLOB,
    Domain: ONEX_VARIABLE_BLOB,
}}
BITFIELD!{ONEX_AUTH_PARAMS Bitfields: DWORD [
    fSessionId set_fSessionId[0..1],
    fhUserToken set_fhUserToken[1..2],
    fOnexUserProfile set_fOnexUserProfile[2..3],
    fIdentity set_fIdentity[3..4],
    fUserName set_fUserName[4..5],
    fDomain set_fDomain[5..6],
]}
pub type PONEX_AUTH_PARAMS = *mut ONEX_AUTH_PARAMS;
STRUCT!{struct ONEX_EAP_ERROR {
    dwWinError: DWORD,
    type_: EAP_METHOD_TYPE,
    dwReasonCode: DWORD,
    rootCauseGuid: GUID,
    repairGuid: GUID,
    helpLinkGuid: GUID,
    Bitfields: DWORD,
    RootCauseString: ONEX_VARIABLE_BLOB,
    RepairString: ONEX_VARIABLE_BLOB,
}}
BITFIELD!{ONEX_EAP_ERROR Bitfields: DWORD [
    fRootCauseString set_fRootCauseString[0..1],
    fRepairString set_fRepairString[1..2],
]}
pub type PONEX_EAP_ERROR = *mut ONEX_EAP_ERROR;
STRUCT!{struct ONEX_STATUS {
    authStatus: ONEX_AUTH_STATUS,
    dwReason: DWORD,
    dwError: DWORD,
}}
pub type PONEX_STATUS = *mut ONEX_STATUS;
ENUM!{enum ONEX_EAP_METHOD_BACKEND_SUPPORT {
    OneXEapMethodBackendSupportUnknown = 0,
    OneXEapMethodBackendSupported = 1,
    OneXEapMethodBackendUnsupported = 2,
}}
STRUCT!{struct ONEX_RESULT_UPDATE_DATA {
    oneXStatus: ONEX_STATUS,
    BackendSupport: ONEX_EAP_METHOD_BACKEND_SUPPORT,
    fBackendEngaged: BOOL,
    Bitfields: DWORD,
    authParams: ONEX_VARIABLE_BLOB,
    eapError: ONEX_VARIABLE_BLOB,
}}
BITFIELD!{ONEX_RESULT_UPDATE_DATA Bitfields: DWORD [
    fOneXAuthParams set_fOneXAuthParams[0..1],
    fEapError set_fEapError[1..2],
]}
pub type PONEX_RESULT_UPDATE_DATA = *mut ONEX_RESULT_UPDATE_DATA;
STRUCT!{struct ONEX_USER_INFO {
    authIdentity: ONEX_AUTH_IDENTITY,
    Bitfields: DWORD,
    UserName: ONEX_VARIABLE_BLOB,
    DomainName: ONEX_VARIABLE_BLOB,
}}
BITFIELD!{ONEX_USER_INFO Bitfields: DWORD [
    fUserName set_fUserName[0..1],
    fDomainName set_fDomainName[1..2],
]}
pub type PONEX_USER_INFO = *mut ONEX_USER_INFO;
