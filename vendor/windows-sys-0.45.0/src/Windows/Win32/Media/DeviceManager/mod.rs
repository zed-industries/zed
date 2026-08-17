pub type IComponentAuthenticate = *mut ::core::ffi::c_void;
pub type IMDSPDevice = *mut ::core::ffi::c_void;
pub type IMDSPDevice2 = *mut ::core::ffi::c_void;
pub type IMDSPDevice3 = *mut ::core::ffi::c_void;
pub type IMDSPDeviceControl = *mut ::core::ffi::c_void;
pub type IMDSPDirectTransfer = *mut ::core::ffi::c_void;
pub type IMDSPEnumDevice = *mut ::core::ffi::c_void;
pub type IMDSPEnumStorage = *mut ::core::ffi::c_void;
pub type IMDSPObject = *mut ::core::ffi::c_void;
pub type IMDSPObject2 = *mut ::core::ffi::c_void;
pub type IMDSPObjectInfo = *mut ::core::ffi::c_void;
pub type IMDSPRevoked = *mut ::core::ffi::c_void;
pub type IMDSPStorage = *mut ::core::ffi::c_void;
pub type IMDSPStorage2 = *mut ::core::ffi::c_void;
pub type IMDSPStorage3 = *mut ::core::ffi::c_void;
pub type IMDSPStorage4 = *mut ::core::ffi::c_void;
pub type IMDSPStorageGlobals = *mut ::core::ffi::c_void;
pub type IMDServiceProvider = *mut ::core::ffi::c_void;
pub type IMDServiceProvider2 = *mut ::core::ffi::c_void;
pub type IMDServiceProvider3 = *mut ::core::ffi::c_void;
pub type ISCPSecureAuthenticate = *mut ::core::ffi::c_void;
pub type ISCPSecureAuthenticate2 = *mut ::core::ffi::c_void;
pub type ISCPSecureExchange = *mut ::core::ffi::c_void;
pub type ISCPSecureExchange2 = *mut ::core::ffi::c_void;
pub type ISCPSecureExchange3 = *mut ::core::ffi::c_void;
pub type ISCPSecureQuery = *mut ::core::ffi::c_void;
pub type ISCPSecureQuery2 = *mut ::core::ffi::c_void;
pub type ISCPSecureQuery3 = *mut ::core::ffi::c_void;
pub type ISCPSession = *mut ::core::ffi::c_void;
pub type IWMDMDevice = *mut ::core::ffi::c_void;
pub type IWMDMDevice2 = *mut ::core::ffi::c_void;
pub type IWMDMDevice3 = *mut ::core::ffi::c_void;
pub type IWMDMDeviceControl = *mut ::core::ffi::c_void;
pub type IWMDMDeviceSession = *mut ::core::ffi::c_void;
pub type IWMDMEnumDevice = *mut ::core::ffi::c_void;
pub type IWMDMEnumStorage = *mut ::core::ffi::c_void;
pub type IWMDMLogger = *mut ::core::ffi::c_void;
pub type IWMDMMetaData = *mut ::core::ffi::c_void;
pub type IWMDMNotification = *mut ::core::ffi::c_void;
pub type IWMDMObjectInfo = *mut ::core::ffi::c_void;
pub type IWMDMOperation = *mut ::core::ffi::c_void;
pub type IWMDMOperation2 = *mut ::core::ffi::c_void;
pub type IWMDMOperation3 = *mut ::core::ffi::c_void;
pub type IWMDMProgress = *mut ::core::ffi::c_void;
pub type IWMDMProgress2 = *mut ::core::ffi::c_void;
pub type IWMDMProgress3 = *mut ::core::ffi::c_void;
pub type IWMDMRevoked = *mut ::core::ffi::c_void;
pub type IWMDMStorage = *mut ::core::ffi::c_void;
pub type IWMDMStorage2 = *mut ::core::ffi::c_void;
pub type IWMDMStorage3 = *mut ::core::ffi::c_void;
pub type IWMDMStorage4 = *mut ::core::ffi::c_void;
pub type IWMDMStorageControl = *mut ::core::ffi::c_void;
pub type IWMDMStorageControl2 = *mut ::core::ffi::c_void;
pub type IWMDMStorageControl3 = *mut ::core::ffi::c_void;
pub type IWMDMStorageGlobals = *mut ::core::ffi::c_void;
pub type IWMDeviceManager = *mut ::core::ffi::c_void;
pub type IWMDeviceManager2 = *mut ::core::ffi::c_void;
pub type IWMDeviceManager3 = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_BOF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_CUR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_EOF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_OK: u16 = 8193u16;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MediaDevMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MediaDevMgrClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const RSA_KEY_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_X509: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_MAC_LEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x86248cc9_4a59_43e2_9146_48a7f3f4140c);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_DRMINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x213dd287_41d2_432b_9e3f_3b4f7b3581dd);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_NEEDTOINDIV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x87a507c7_b469_4386_b976_d5d1ce538a6f);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_PARAMID_DRMVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41d0155d_7cc7_4217_ada9_005074624da4);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMDeviceEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMID_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMLogger: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorage: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorageEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorageGlobal: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x807b3ce1_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_APP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa4d2c26c_a881_44bb_bd5d_1f703c71f7a9);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x979e54e5_0afc_4604_8d93_dc798a4bcf45);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a11ed91_8c8f_41e4_82d1_8386e003561c);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUSY: i32 = -2147201024i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MAC_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_BLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_QUERY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_THREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_REVOKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_END: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7de8686d_78ee_43ea_a496_c625ac91cc5d);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SP_REVOKED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_BUSY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszAudioWAVECodec: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AudioWAVECodec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszVideoFourCCCodec: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/VideoFourCCCodec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArt: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumArt");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArtist: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumArtist");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverData: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverData");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverDuration: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverDuration");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverFormat: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverFormat");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverHeight: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverHeight");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverSize: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverWidth: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumCoverWidth");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumTitle: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AlbumTitle");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAudioBitDepth: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AudioBitDepth");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthor: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Author");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthorDate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/AuthorDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitRateType: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/BitRateType");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitrate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Bitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBlockAlignment: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/BlockAlignment");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBufferSize: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/BufferSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBuyNow: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/BuyNow");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMByteBookmark: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/ByteBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCategory: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Category");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCodec: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Codec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCollectionID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/CollectionID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMComposer: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Composer");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDRMId: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DRMId");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataLength: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DataLength");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataOffset: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DataOffset");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataUnits: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DataUnits");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDescription: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Description");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDestinationURL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DestinationURL");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFirmwareVersion: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceFirmwareVersion");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFriendlyName: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceFriendlyName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceModelName: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceModelName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDevicePlayCount: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DevicePlayCount");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceProtocol: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceProtocol");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceRevocationInfo: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceRevocationInfo");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceServiceProviderVendor: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceServiceProviderVendor");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceVendorExtension: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/DeviceVendorExtension");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDuration: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Duration");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEditor: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Editor");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEncodingProfile: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/EncodingProfile");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileAttributes: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FileAttributes");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileCreationDate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FileCreationDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileName: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FileName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileSize: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FileSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatCode: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FormatCode");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupported: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FormatsSupported");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupportedAreOrdered: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FormatsSupportedAreOrdered");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFrameRate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/FrameRate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMGenre: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Genre");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMHeight: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Height");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsProtected: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/IsProtected");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsRepeat: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/IsRepeat");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMKeyFrameDistance: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/KeyFrameDistance");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMLastModifiedDate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/LastModifiedDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaClassSecondaryID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaClassSecondaryID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaCredits: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaCredits");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaGuid: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaGuid");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalBroadcastDateTime: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaOriginalBroadcastDateTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalChannel: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaOriginalChannel");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaStationName: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaStationName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMetaGenre: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MetaGenre");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNonConsumable: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/NonConsumable");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNumChannels: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/NumChannels");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMObjectBookmark: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/ObjectBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMOwner: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Owner");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMParentalRating: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/ParentalRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPersistentUniqueID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/PersistentUniqueID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPlayCount: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/PlayCount");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMProviderCopyright: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/ProviderCopyright");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMQualitySetting: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/QualitySetting");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSampleRate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SampleRate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMScanType: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/ScanType");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSourceURL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SourceURL");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitle: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SubTitle");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitleDescription: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SubTitleDescription");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSupportedDeviceProperties: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SupportedDeviceProperties");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SyncID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncRelationshipID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SyncRelationshipID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncTime: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/SyncTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeBookmark: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/TimeBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeToLive: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/TimeToLive");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTitle: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Title");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTotalBitrate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/TotalBitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrack: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Track");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrackMood: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/TrackMood");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserEffectiveRating: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/UserEffectiveRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserLastPlayTime: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/UserLastPlayTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRating: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/UserRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRatingOnDevice: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/UserRatingOnDevice");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMVideoBitrate: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/VideoBitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWebmaster: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Webmaster");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWidth: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Width");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMYear: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/Year");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMediaClassPrimaryID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WMDM/MediaClassPrimaryID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWPDPassthroughPropertyValues: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WPD/PassthroughPropertyValues");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDMMessage = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = 3i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_ENUM_PROP_VALID_VALUES_FORM = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_FIND_SCOPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_FORMATCODE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = -1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = 12288i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = 12289i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = 12290i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = 12291i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = 12292i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = 12293i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = 12294i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = 12295i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = 12296i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = 12297i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = 12298i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = 12299i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = 12300i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = 12301i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = 14335i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = 14336i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = 14337i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = 14338i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = 14339i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = 14340i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = 14341i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = 14343i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = 14344i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = 14345i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = 14346i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = 14347i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = 14349i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = 14350i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = 14351i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = 14352i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = 14353i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = 16383i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = 47106i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = 47107i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = 47108i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = 47233i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = 47360i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = 47361i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = 47362i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = 47363i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = 47364i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = 47366i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = 47367i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = 47368i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = 47488i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = 47489i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = 47490i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = 47491i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = 47492i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = 47493i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = 47494i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = 47495i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = 47496i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = 47497i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = 47498i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = 47499i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = 47616i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = 47617i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = 47618i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = 47619i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = 47620i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = 47621i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = 47622i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = 47623i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = 47624i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = 47627i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = 47632i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = 47633i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = 47634i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = 47635i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = 47636i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = 47744i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = 47745i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = 47746i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = 47747i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = 47748i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = 47749i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = 47750i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = 47872i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = 47873i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = 48000i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = 48001i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = 48002i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = 48003i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = 48640i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = 48641i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = 48642i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = 48643i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = 48768i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = 1297101889i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = 860311617i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = 860303937i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = 48770i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_SESSION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = 4096i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_STORAGE_ENUM_MODE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub type WMDM_TAG_DATATYPE = i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MACINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_IN {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_OUT {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows_sys::core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl ::core::marker::Copy for OPAQUECOMMAND {}
impl ::core::clone::Clone for OPAQUECOMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl ::core::marker::Copy for WMDMDATETIME {}
impl ::core::clone::Clone for WMDMDATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub union WMDMDetermineMaxPropStringLen {
    pub sz001: [u16; 27],
    pub sz002: [u16; 31],
    pub sz003: [u16; 14],
    pub sz004: [u16; 16],
    pub sz005: [u16; 22],
    pub sz006: [u16; 14],
    pub sz007: [u16; 20],
    pub sz008: [u16; 20],
    pub sz009: [u16; 22],
    pub sz010: [u16; 11],
    pub sz011: [u16; 12],
    pub sz012: [u16; 17],
    pub sz013: [u16; 17],
    pub sz014: [u16; 16],
    pub sz015: [u16; 17],
    pub sz016: [u16; 11],
    pub sz017: [u16; 11],
    pub sz018: [u16; 15],
    pub sz019: [u16; 22],
    pub sz020: [u16; 20],
    pub sz021: [u16; 22],
    pub sz022: [u16; 21],
    pub sz023: [u16; 24],
    pub sz024: [u16; 20],
    pub sz025: [u16; 10],
    pub sz026: [u16; 14],
    pub sz027: [u16; 11],
    pub sz028: [u16; 11],
    pub sz029: [u16; 13],
    pub sz030: [u16; 17],
    pub sz031: [u16; 16],
    pub sz032: [u16; 17],
    pub sz033: [u16; 20],
    pub sz034: [u16; 19],
    pub sz035: [u16; 18],
    pub sz036: [u16; 18],
    pub sz037: [u16; 15],
    pub sz041: [u16; 14],
    pub sz043: [u16; 22],
    pub sz044: [u16; 16],
    pub sz045: [u16; 20],
    pub sz046: [u16; 14],
    pub sz047: [u16; 14],
    pub sz048: [u16; 12],
    pub sz049: [u16; 25],
    pub sz050: [u16; 26],
    pub sz051: [u16; 25],
    pub sz052: [u16; 16],
    pub sz053: [u16; 24],
    pub sz054: [u16; 15],
    pub sz055: [u16; 21],
    pub sz056: [u16; 16],
    pub sz057: [u16; 22],
    pub sz058: [u16; 14],
    pub sz059: [u16; 25],
    pub sz060: [u16; 18],
    pub sz061: [u16; 22],
    pub sz062: [u16; 26],
    pub sz063: [u16; 36],
    pub sz064: [u16; 23],
    pub sz065: [u16; 12],
    pub sz066: [u16; 24],
    pub sz067: [u16; 11],
    pub sz068: [u16; 12],
    pub sz069: [u16; 14],
    pub sz070: [u16; 20],
    pub sz071: [u16; 15],
    pub sz072: [u16; 14],
    pub sz073: [u16; 31],
    pub sz074: [u16; 24],
    pub sz075: [u16; 22],
    pub sz076: [u16; 24],
    pub sz077: [u16; 21],
    pub sz078: [u16; 27],
    pub sz079: [u16; 27],
    pub sz080: [u16; 20],
    pub sz081: [u16; 33],
    pub sz082: [u16; 21],
    pub sz083: [u16; 32],
    pub sz084: [u16; 26],
    pub sz085: [u16; 18],
    pub sz086: [u16; 30],
}
impl ::core::marker::Copy for WMDMDetermineMaxPropStringLen {}
impl ::core::clone::Clone for WMDMDetermineMaxPropStringLen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl ::core::marker::Copy for WMDMID {}
impl ::core::clone::Clone for WMDMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMMetadataView {
    pub pwszViewName: ::windows_sys::core::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
impl ::core::marker::Copy for WMDMMetadataView {}
impl ::core::clone::Clone for WMDMMetadataView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl ::core::marker::Copy for WMDMRIGHTS {}
impl ::core::clone::Clone for WMDMRIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_FORMAT_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: ::windows_sys::core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: WMDM_PROP_VALUES_RANGE,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: ::windows_sys::core::PWSTR,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for WMFILECAPABILITIES {}
impl ::core::clone::Clone for WMFILECAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
