pub type IDummyMBNUCMExt = *mut ::core::ffi::c_void;
pub type IMbnConnection = *mut ::core::ffi::c_void;
pub type IMbnConnectionContext = *mut ::core::ffi::c_void;
pub type IMbnConnectionContextEvents = *mut ::core::ffi::c_void;
pub type IMbnConnectionEvents = *mut ::core::ffi::c_void;
pub type IMbnConnectionManager = *mut ::core::ffi::c_void;
pub type IMbnConnectionManagerEvents = *mut ::core::ffi::c_void;
pub type IMbnConnectionProfile = *mut ::core::ffi::c_void;
pub type IMbnConnectionProfileEvents = *mut ::core::ffi::c_void;
pub type IMbnConnectionProfileManager = *mut ::core::ffi::c_void;
pub type IMbnConnectionProfileManagerEvents = *mut ::core::ffi::c_void;
pub type IMbnDeviceService = *mut ::core::ffi::c_void;
pub type IMbnDeviceServiceStateEvents = *mut ::core::ffi::c_void;
pub type IMbnDeviceServicesContext = *mut ::core::ffi::c_void;
pub type IMbnDeviceServicesEvents = *mut ::core::ffi::c_void;
pub type IMbnDeviceServicesManager = *mut ::core::ffi::c_void;
pub type IMbnInterface = *mut ::core::ffi::c_void;
pub type IMbnInterfaceEvents = *mut ::core::ffi::c_void;
pub type IMbnInterfaceManager = *mut ::core::ffi::c_void;
pub type IMbnInterfaceManagerEvents = *mut ::core::ffi::c_void;
pub type IMbnMultiCarrier = *mut ::core::ffi::c_void;
pub type IMbnMultiCarrierEvents = *mut ::core::ffi::c_void;
pub type IMbnPin = *mut ::core::ffi::c_void;
pub type IMbnPinEvents = *mut ::core::ffi::c_void;
pub type IMbnPinManager = *mut ::core::ffi::c_void;
pub type IMbnPinManagerEvents = *mut ::core::ffi::c_void;
pub type IMbnRadio = *mut ::core::ffi::c_void;
pub type IMbnRadioEvents = *mut ::core::ffi::c_void;
pub type IMbnRegistration = *mut ::core::ffi::c_void;
pub type IMbnRegistrationEvents = *mut ::core::ffi::c_void;
pub type IMbnServiceActivation = *mut ::core::ffi::c_void;
pub type IMbnServiceActivationEvents = *mut ::core::ffi::c_void;
pub type IMbnSignal = *mut ::core::ffi::c_void;
pub type IMbnSignalEvents = *mut ::core::ffi::c_void;
pub type IMbnSms = *mut ::core::ffi::c_void;
pub type IMbnSmsConfiguration = *mut ::core::ffi::c_void;
pub type IMbnSmsEvents = *mut ::core::ffi::c_void;
pub type IMbnSmsReadMsgPdu = *mut ::core::ffi::c_void;
pub type IMbnSmsReadMsgTextCdma = *mut ::core::ffi::c_void;
pub type IMbnSubscriberInformation = *mut ::core::ffi::c_void;
pub type IMbnVendorSpecificEvents = *mut ::core::ffi::c_void;
pub type IMbnVendorSpecificOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnConnectionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnConnectionProfileManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnDeviceServicesManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MbnInterfaceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_ACTIVATION_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_AUTH_PROTOCOL = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_BAND_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = 256i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = 512i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = 1024i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = 2048i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = 4096i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = 8192i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = 16384i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = 32768i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = 65536i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = 131072i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = -2147483648i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CELLULAR_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_COMPRESSION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONNECTION_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONTEXT_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = 100i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = 255i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CONTEXT_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_CTRL_CAPS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DATA_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = 128i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = 65536i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = 131072i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = 262144i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = 524288i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = 1048576i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = 2097152i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = 4194304i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = -2147483648i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DEVICE_SERVICES_INTERFACE_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_DEVICE_SERVICE_SESSIONS_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_INTERFACE_CAPS_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 18i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_MSG_STATUS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PROVIDER_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = 20i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_PROVIDER_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = 32i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_RADIO = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_OFF: MBN_RADIO = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RADIO_ON: MBN_RADIO = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_READY_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTER_MODE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTER_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_REGISTRATION_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = 64i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SIGNAL_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = -1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = 99i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CAPS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CDMA_ENCODING = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_CDMA_LANG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_FLAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_SMS_STATUS_FLAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_VOICE_CALL_STATE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type MBN_VOICE_CLASS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub type WWAEXT_SMS_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = 160i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: ::windows_sys::core::BSTR,
    pub userName: ::windows_sys::core::BSTR,
    pub password: ::windows_sys::core::BSTR,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
impl ::core::marker::Copy for MBN_CONTEXT {}
impl ::core::clone::Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: ::windows_sys::core::BSTR,
    pub dataWriteSupported: super::super::Foundation::VARIANT_BOOL,
    pub dataReadSupported: super::super::Foundation::VARIANT_BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: ::windows_sys::core::BSTR,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: ::windows_sys::core::BSTR,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: ::windows_sys::core::BSTR,
    pub manufacturer: ::windows_sys::core::BSTR,
    pub model: ::windows_sys::core::BSTR,
    pub firmwareInfo: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for MBN_INTERFACE_CAPS {}
impl ::core::clone::Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl ::core::marker::Copy for MBN_PIN_INFO {}
impl ::core::clone::Clone for MBN_PIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PROVIDER {
    pub providerID: ::windows_sys::core::BSTR,
    pub providerState: u32,
    pub providerName: ::windows_sys::core::BSTR,
    pub dataClass: u32,
}
impl ::core::marker::Copy for MBN_PROVIDER {}
impl ::core::clone::Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
impl ::core::marker::Copy for MBN_PROVIDER2 {}
impl ::core::clone::Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_FILTER {}
impl ::core::clone::Clone for MBN_SMS_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_STATUS_INFO {}
impl ::core::clone::Clone for MBN_SMS_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl ::core::marker::Copy for __DummyPinType__ {}
impl ::core::clone::Clone for __DummyPinType__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`*"]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
impl ::core::marker::Copy for __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __mbnapi_ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
