pub const AdSyncTask: windows_core::GUID = windows_core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdrClientDisplayFlags(pub i32);
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdrClientErrorType(pub i32);
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdrClientFlags(pub i32);
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AdrEmailFlags(pub i32);
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DIFsrmClassificationEvents, DIFsrmClassificationEvents_Vtbl, 0x26942db0_dabf_41d8_bbdd_b129a9f70424);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DIFsrmClassificationEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DIFsrmClassificationEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DIFsrmClassificationEvents_Impl: super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DIFsrmClassificationEvents_Vtbl {
    pub const fn new<Identity: DIFsrmClassificationEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DIFsrmClassificationEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DIFsrmClassificationEvents {}
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: windows_core::HRESULT = windows_core::HRESULT(0x8004537E_u32 as _);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: windows_core::HRESULT = windows_core::HRESULT(0x80045392_u32 as _);
pub const FSRM_E_ADR_PATH_IS_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x80045391_u32 as _);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045390_u32 as _);
pub const FSRM_E_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045303_u32 as _);
pub const FSRM_E_AUTO_QUOTA: windows_core::HRESULT = windows_core::HRESULT(0x4531B_u32 as _);
pub const FSRM_E_CACHE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80045345_u32 as _);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045346_u32 as _);
pub const FSRM_E_CANNOT_AGGREGATE: windows_core::HRESULT = windows_core::HRESULT(0x80045337_u32 as _);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: windows_core::HRESULT = windows_core::HRESULT(0x80045356_u32 as _);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8004533B_u32 as _);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: windows_core::HRESULT = windows_core::HRESULT(0x8004537C_u32 as _);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80045379_u32 as _);
pub const FSRM_E_CANNOT_REMOVE_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80045393_u32 as _);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004533A_u32 as _);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x80045355_u32 as _);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80045371_u32 as _);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004536F_u32 as _);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533D_u32 as _);
pub const FSRM_E_CLASSIFICATION_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80045373_u32 as _);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533E_u32 as _);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x80045378_u32 as _);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8004536C_u32 as _);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045377_u32 as _);
pub const FSRM_E_CLUSTER_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004532E_u32 as _);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045396_u32 as _);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80045331_u32 as _);
pub const FSRM_E_DRIVER_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80045313_u32 as _);
pub const FSRM_E_DUPLICATE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80045310_u32 as _);
pub const FSRM_E_EMAIL_NOT_SENT: windows_core::HRESULT = windows_core::HRESULT(0x8004531C_u32 as _);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045353_u32 as _);
pub const FSRM_E_ERROR_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004537B_u32 as _);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: windows_core::HRESULT = windows_core::HRESULT(0x80045397_u32 as _);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80045398_u32 as _);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: windows_core::HRESULT = windows_core::HRESULT(0x80045399_u32 as _);
pub const FSRM_E_FAIL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x80045309_u32 as _);
pub const FSRM_E_FILE_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x80045364_u32 as _);
pub const FSRM_E_FILE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8004537A_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045368_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045367_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80045347_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045348_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533F_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: windows_core::HRESULT = windows_core::HRESULT(0x80045341_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: windows_core::HRESULT = windows_core::HRESULT(0x8004539A_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: windows_core::HRESULT = windows_core::HRESULT(0x80045340_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80045394_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004536E_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80045342_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8004536D_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: windows_core::HRESULT = windows_core::HRESULT(0x80045388_u32 as _);
pub const FSRM_E_FILE_OPEN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80045343_u32 as _);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8004531F_u32 as _);
pub const FSRM_E_INCOMPATIBLE_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80045363_u32 as _);
pub const FSRM_E_INPROC_MODULE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80045352_u32 as _);
pub const FSRM_E_INSECURE_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80045317_u32 as _);
pub const FSRM_E_INSUFFICIENT_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80045314_u32 as _);
pub const FSRM_E_INVALID_AD_CLAIM: windows_core::HRESULT = windows_core::HRESULT(0x80045372_u32 as _);
pub const FSRM_E_INVALID_COMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x8004530F_u32 as _);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: windows_core::HRESULT = windows_core::HRESULT(0x80045324_u32 as _);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004531E_u32 as _);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: windows_core::HRESULT = windows_core::HRESULT(0x80045321_u32 as _);
pub const FSRM_E_INVALID_FILENAME: windows_core::HRESULT = windows_core::HRESULT(0x8004532A_u32 as _);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: windows_core::HRESULT = windows_core::HRESULT(0x80045374_u32 as _);
pub const FSRM_E_INVALID_IMPORT_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8004530B_u32 as _);
pub const FSRM_E_INVALID_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80045307_u32 as _);
pub const FSRM_E_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80045308_u32 as _);
pub const FSRM_E_INVALID_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80045306_u32 as _);
pub const FSRM_E_INVALID_REPORT_DESC: windows_core::HRESULT = windows_core::HRESULT(0x80045329_u32 as _);
pub const FSRM_E_INVALID_REPORT_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80045328_u32 as _);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80045302_u32 as _);
pub const FSRM_E_INVALID_SMTP_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80045318_u32 as _);
pub const FSRM_E_INVALID_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x8004530A_u32 as _);
pub const FSRM_E_INVALID_USER: windows_core::HRESULT = windows_core::HRESULT(0x80045305_u32 as _);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80045350_u32 as _);
pub const FSRM_E_LEGACY_SCHEDULE: windows_core::HRESULT = windows_core::HRESULT(0x80045395_u32 as _);
pub const FSRM_E_LOADING_DISABLED_MODULE: windows_core::HRESULT = windows_core::HRESULT(0x80045336_u32 as _);
pub const FSRM_E_LONG_CMDLINE: windows_core::HRESULT = windows_core::HRESULT(0x80045320_u32 as _);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004533C_u32 as _);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80045338_u32 as _);
pub const FSRM_E_MODULE_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x8004536A_u32 as _);
pub const FSRM_E_MODULE_INVALID_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x80045369_u32 as _);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x8004536B_u32 as _);
pub const FSRM_E_MODULE_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004539B_u32 as _);
pub const FSRM_E_NOT_CLUSTER_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80045330_u32 as _);
pub const FSRM_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045301_u32 as _);
pub const FSRM_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045311_u32 as _);
pub const FSRM_E_NO_EMAIL_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004537D_u32 as _);
pub const FSRM_E_NO_PROPERTY_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80045351_u32 as _);
pub const FSRM_E_OBJECT_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80045339_u32 as _);
pub const FSRM_E_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x8004530D_u32 as _);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045357_u32 as _);
pub const FSRM_E_PATH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045304_u32 as _);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x8004537F_u32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045365_u32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x8004535A_u32 as _);
pub const FSRM_E_PROPERTY_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x80045349_u32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: windows_core::HRESULT = windows_core::HRESULT(0x80045376_u32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: windows_core::HRESULT = windows_core::HRESULT(0x80045384_u32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: windows_core::HRESULT = windows_core::HRESULT(0x80045386_u32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: windows_core::HRESULT = windows_core::HRESULT(0x80045385_u32 as _);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80045375_u32 as _);
pub const FSRM_E_REPORT_GENERATION_ERR: windows_core::HRESULT = windows_core::HRESULT(0x80045334_u32 as _);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80045333_u32 as _);
pub const FSRM_E_REPORT_TASK_TRIGGER: windows_core::HRESULT = windows_core::HRESULT(0x80045335_u32 as _);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045332_u32 as _);
pub const FSRM_E_REQD_PARAM_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x8004530E_u32 as _);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80045382_u32 as _);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80045383_u32 as _);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045380_u32 as _);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045381_u32 as _);
pub const FSRM_E_SET_PROPERTY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045354_u32 as _);
pub const FSRM_E_SHADOW_COPY: windows_core::HRESULT = windows_core::HRESULT(0x8004532C_u32 as _);
pub const FSRM_E_STORE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x8004532F_u32 as _);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: windows_core::HRESULT = windows_core::HRESULT(0x80045389_u32 as _);
pub const FSRM_E_SYNC_TASK_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045370_u32 as _);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80045362_u32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: windows_core::HRESULT = windows_core::HRESULT(0x80045360_u32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045359_u32 as _);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80045358_u32 as _);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80045361_u32 as _);
pub const FSRM_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80045316_u32 as _);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: windows_core::HRESULT = windows_core::HRESULT(0x80045344_u32 as _);
pub const FSRM_E_VOLUME_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80045366_u32 as _);
pub const FSRM_E_VOLUME_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045315_u32 as _);
pub const FSRM_E_WMI_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80045387_u32 as _);
pub const FSRM_E_XML_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x8004532D_u32 as _);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: windows_core::HRESULT = windows_core::HRESULT(0x45306_u32 as _);
pub const FSRM_S_PARTIAL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x45304_u32 as _);
pub const FSRM_S_PARTIAL_CLASSIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x45305_u32 as _);
pub const FsrmAccessDeniedRemediationClient: windows_core::GUID = windows_core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmAccountType(pub i32);
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmActionType(pub i32);
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmClassificationLoggingFlags(pub i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
pub const FsrmClassificationManager: windows_core::GUID = windows_core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmCollectionState(pub i32);
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmCommitOptions(pub i32);
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmEnumOptions(pub i32);
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmEventType(pub i32);
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmExecutionOption(pub i32);
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
pub const FsrmExportImport: windows_core::GUID = windows_core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileConditionType(pub i32);
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
pub const FsrmFileGroupManager: windows_core::GUID = windows_core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: windows_core::GUID = windows_core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileManagementType(pub i32);
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileScreenFlags(pub i32);
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
pub const FsrmFileScreenManager: windows_core::GUID = windows_core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileStreamingMode(pub i32);
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmFileSystemPropertyId(pub i32);
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmGetFilePropertyOptions(pub i32);
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: windows_core::GUID = windows_core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: windows_core::GUID = windows_core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPipelineModuleType(pub i32);
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyBagField(pub i32);
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyBagFlags(pub i32);
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyConditionType(pub i32);
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyDefinitionType(pub i32);
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyFlags(pub i32);
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmPropertyValueType(pub i32);
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmQuotaFlags(pub i32);
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
pub const FsrmQuotaManager: windows_core::GUID = windows_core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportFilter(pub i32);
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportFormat(pub i32);
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportGenerationContext(pub i32);
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportLimit(pub i32);
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
pub const FsrmReportManager: windows_core::GUID = windows_core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportRunningStatus(pub i32);
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
pub const FsrmReportScheduler: windows_core::GUID = windows_core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmReportType(pub i32);
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmRuleFlags(pub i32);
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmRuleType(pub i32);
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
pub const FsrmSetting: windows_core::GUID = windows_core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmStorageModuleCaps(pub i32);
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmStorageModuleType(pub i32);
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FsrmTemplateApplyOptions(pub i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAccessDeniedRemediationClient, IFsrmAccessDeniedRemediationClient_Vtbl, 0x40002314_590b_45a5_8e1b_8c05da527e52);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAccessDeniedRemediationClient {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAccessDeniedRemediationClient, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAccessDeniedRemediationClient {
    pub unsafe fn Show(&self, parentwnd: usize, accesspath: &windows_core::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), parentwnd, core::mem::transmute_copy(accesspath), errortype, flags, core::mem::transmute_copy(windowtitle), core::mem::transmute_copy(windowmessage), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void, AdrClientErrorType, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: super::super::System::Com::IDispatch_Impl {
    fn Show(&self, parentwnd: usize, accesspath: &windows_core::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmAccessDeniedRemediationClient_Vtbl {
    pub const fn new<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwnd: usize, accesspath: *mut core::ffi::c_void, errortype: AdrClientErrorType, flags: i32, windowtitle: *mut core::ffi::c_void, windowmessage: *mut core::ffi::c_void, result: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAccessDeniedRemediationClient_Impl::Show(this, core::mem::transmute_copy(&parentwnd), core::mem::transmute(&accesspath), core::mem::transmute_copy(&errortype), core::mem::transmute_copy(&flags), core::mem::transmute(&windowtitle), core::mem::transmute(&windowmessage)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAction, IFsrmAction_Vtbl, 0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAction {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAction, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAction {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActionType(&self) -> windows_core::Result<FsrmActionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RunLimitInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunLimitInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunLimitInterval)(windows_core::Interface::as_raw(self), minutes).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmActionType) -> windows_core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAction_Impl: super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::GUID>;
    fn ActionType(&self) -> windows_core::Result<FsrmActionType>;
    fn RunLimitInterval(&self) -> windows_core::Result<i32>;
    fn SetRunLimitInterval(&self, minutes: i32) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmAction_Vtbl {
    pub const fn new<Identity: IFsrmAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::Id(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionType<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: *mut FsrmActionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::ActionType(this) {
                    Ok(ok__) => {
                        actiontype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::RunLimitInterval(this) {
                    Ok(ok__) => {
                        minutes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::SetRunLimitInterval(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            ActionType: ActionType::<Identity, OFFSET>,
            RunLimitInterval: RunLimitInterval::<Identity, OFFSET>,
            SetRunLimitInterval: SetRunLimitInterval::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmAction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionCommand, IFsrmActionCommand_Vtbl, 0x12937789_e247_4917_9c20_f3ee9c7ee783);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionCommand {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionCommand, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionCommand {
    pub unsafe fn ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecutablePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExecutablePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(executablepath)).ok() }
    }
    pub unsafe fn Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArguments(&self, arguments: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(arguments)).ok() }
    }
    pub unsafe fn Account(&self) -> windows_core::Result<FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), account).ok() }
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(workingdirectory)).ok() }
    }
    pub unsafe fn MonitorCommand(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MonitorCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonitorCommand)(windows_core::Interface::as_raw(self), monitorcommand).ok() }
    }
    pub unsafe fn KillTimeOut(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KillTimeOut)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKillTimeOut)(windows_core::Interface::as_raw(self), minutes).ok() }
    }
    pub unsafe fn LogResult(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogResult(&self, logresults: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogResult)(windows_core::Interface::as_raw(self), logresults).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmAccountType) -> windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmAccountType) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub KillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LogResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionCommand_Impl: IFsrmAction_Impl {
    fn ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Arguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArguments(&self, arguments: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Account(&self) -> windows_core::Result<FsrmAccountType>;
    fn SetAccount(&self, account: FsrmAccountType) -> windows_core::Result<()>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MonitorCommand(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonitorCommand(&self, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn KillTimeOut(&self) -> windows_core::Result<i32>;
    fn SetKillTimeOut(&self, minutes: i32) -> windows_core::Result<()>;
    fn LogResult(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogResult(&self, logresults: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmActionCommand_Vtbl {
    pub const fn new<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::ExecutablePath(this) {
                    Ok(ok__) => {
                        executablepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetExecutablePath(this, core::mem::transmute(&executablepath)).into()
            }
        }
        unsafe extern "system" fn Arguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::Arguments(this) {
                    Ok(ok__) => {
                        arguments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetArguments(this, core::mem::transmute(&arguments)).into()
            }
        }
        unsafe extern "system" fn Account<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: *mut FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::Account(this) {
                    Ok(ok__) => {
                        account.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccount<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetAccount(this, core::mem::transmute_copy(&account)).into()
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::WorkingDirectory(this) {
                    Ok(ok__) => {
                        workingdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        unsafe extern "system" fn MonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::MonitorCommand(this) {
                    Ok(ok__) => {
                        monitorcommand.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetMonitorCommand(this, core::mem::transmute_copy(&monitorcommand)).into()
            }
        }
        unsafe extern "system" fn KillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::KillTimeOut(this) {
                    Ok(ok__) => {
                        minutes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetKillTimeOut(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn LogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::LogResult(this) {
                    Ok(ok__) => {
                        logresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetLogResult(this, core::mem::transmute_copy(&logresults)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            ExecutablePath: ExecutablePath::<Identity, OFFSET>,
            SetExecutablePath: SetExecutablePath::<Identity, OFFSET>,
            Arguments: Arguments::<Identity, OFFSET>,
            SetArguments: SetArguments::<Identity, OFFSET>,
            Account: Account::<Identity, OFFSET>,
            SetAccount: SetAccount::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            MonitorCommand: MonitorCommand::<Identity, OFFSET>,
            SetMonitorCommand: SetMonitorCommand::<Identity, OFFSET>,
            KillTimeOut: KillTimeOut::<Identity, OFFSET>,
            SetKillTimeOut: SetKillTimeOut::<Identity, OFFSET>,
            LogResult: LogResult::<Identity, OFFSET>,
            SetLogResult: SetLogResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionCommand as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmActionCommand {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEmail, IFsrmActionEmail_Vtbl, 0xd646567d_26ae_4caa_9f84_4e0aad207fca);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEmail {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail {
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)).ok() }
    }
    pub unsafe fn MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailReplyTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailreplyto)).ok() }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
    pub unsafe fn MailCc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailCc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailCc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailcc)).ok() }
    }
    pub unsafe fn MailBcc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailBcc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailBcc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailbcc)).ok() }
    }
    pub unsafe fn MailSubject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailSubject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailsubject)).ok() }
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEmail_Impl: IFsrmAction_Impl {
    fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailCc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailBcc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailSubject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmActionEmail_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn MailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailReplyTo(this) {
                    Ok(ok__) => {
                        mailreplyto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailReplyTo(this, core::mem::transmute(&mailreplyto)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn MailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailCc(this) {
                    Ok(ok__) => {
                        mailcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailCc(this, core::mem::transmute(&mailcc)).into()
            }
        }
        unsafe extern "system" fn MailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailBcc(this) {
                    Ok(ok__) => {
                        mailbcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailBcc(this, core::mem::transmute(&mailbcc)).into()
            }
        }
        unsafe extern "system" fn MailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailSubject(this) {
                    Ok(ok__) => {
                        mailsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailSubject(this, core::mem::transmute(&mailsubject)).into()
            }
        }
        unsafe extern "system" fn MessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            MailFrom: MailFrom::<Identity, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, OFFSET>,
            MailReplyTo: MailReplyTo::<Identity, OFFSET>,
            SetMailReplyTo: SetMailReplyTo::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
            MailCc: MailCc::<Identity, OFFSET>,
            SetMailCc: SetMailCc::<Identity, OFFSET>,
            MailBcc: MailBcc::<Identity, OFFSET>,
            SetMailBcc: SetMailBcc::<Identity, OFFSET>,
            MailSubject: MailSubject::<Identity, OFFSET>,
            SetMailSubject: SetMailSubject::<Identity, OFFSET>,
            MessageText: MessageText::<Identity, OFFSET>,
            SetMessageText: SetMessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmActionEmail {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEmail2, IFsrmActionEmail2_Vtbl, 0x8276702f_2532_4839_89bf_4872609a2ea4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEmail2 {
    type Target = IFsrmActionEmail;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction, IFsrmActionEmail);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail2 {
    pub unsafe fn AttachmentFileListSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachmentFileListSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttachmentFileListSize)(windows_core::Interface::as_raw(self), attachmentfilelistsize).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEmail2_Impl: IFsrmActionEmail_Impl {
    fn AttachmentFileListSize(&self) -> windows_core::Result<i32>;
    fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmActionEmail2_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail2_Impl::AttachmentFileListSize(this) {
                    Ok(ok__) => {
                        attachmentfilelistsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail2_Impl::SetAttachmentFileListSize(this, core::mem::transmute_copy(&attachmentfilelistsize)).into()
            }
        }
        Self {
            base__: IFsrmActionEmail_Vtbl::new::<Identity, OFFSET>(),
            AttachmentFileListSize: AttachmentFileListSize::<Identity, OFFSET>,
            SetAttachmentFileListSize: SetAttachmentFileListSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<IFsrmActionEmail as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmActionEmail2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEventLog, IFsrmActionEventLog_Vtbl, 0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEventLog {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEventLog, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEventLog {
    pub unsafe fn EventType(&self) -> windows_core::Result<FsrmEventType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventType)(windows_core::Interface::as_raw(self), eventtype).ok() }
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmEventType) -> windows_core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEventType) -> windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionEventLog_Impl: IFsrmAction_Impl {
    fn EventType(&self) -> windows_core::Result<FsrmEventType>;
    fn SetEventType(&self, eventtype: FsrmEventType) -> windows_core::Result<()>;
    fn MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmActionEventLog_Vtbl {
    pub const fn new<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: *mut FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::EventType(this) {
                    Ok(ok__) => {
                        eventtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::SetEventType(this, core::mem::transmute_copy(&eventtype)).into()
            }
        }
        unsafe extern "system" fn MessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::SetMessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            EventType: EventType::<Identity, OFFSET>,
            SetEventType: SetEventType::<Identity, OFFSET>,
            MessageText: MessageText::<Identity, OFFSET>,
            SetMessageText: SetMessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmActionEventLog {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionReport, IFsrmActionReport_Vtbl, 0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionReport {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionReport, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionReport {
    pub unsafe fn ReportTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportTypes)(windows_core::Interface::as_raw(self), reporttypes).ok() }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmActionReport_Impl: IFsrmAction_Impl {
    fn ReportTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmActionReport_Vtbl {
    pub const fn new<Identity: IFsrmActionReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::ReportTypes(this) {
                    Ok(ok__) => {
                        reporttypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::SetReportTypes(this, core::mem::transmute_copy(&reporttypes)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            ReportTypes: ReportTypes::<Identity, OFFSET>,
            SetReportTypes: SetReportTypes::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionReport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmActionReport {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAutoApplyQuota, IFsrmAutoApplyQuota_Vtbl, 0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAutoApplyQuota {
    type Target = IFsrmQuotaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAutoApplyQuota, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAutoApplyQuota {
    pub unsafe fn ExcludeFolders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExcludeFolders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExcludeFolders)(windows_core::Interface::as_raw(self), folders).ok() }
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    pub ExcludeFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetExcludeFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmAutoApplyQuota_Impl: IFsrmQuotaObject_Impl {
    fn ExcludeFolders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmAutoApplyQuota_Vtbl {
    pub const fn new<Identity: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExcludeFolders<Identity: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAutoApplyQuota_Impl::ExcludeFolders(this) {
                    Ok(ok__) => {
                        folders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExcludeFolders<Identity: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAutoApplyQuota_Impl::SetExcludeFolders(this, core::mem::transmute_copy(&folders)).into()
            }
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAutoApplyQuota_Impl::CommitAndUpdateDerived(this, core::mem::transmute_copy(&commitoptions), core::mem::transmute_copy(&applyoptions)) {
                    Ok(ok__) => {
                        derivedobjectsresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmQuotaObject_Vtbl::new::<Identity, OFFSET>(),
            ExcludeFolders: ExcludeFolders::<Identity, OFFSET>,
            SetExcludeFolders: SetExcludeFolders::<Identity, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAutoApplyQuota as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmQuotaBase as windows_core::Interface>::IID || iid == &<IFsrmQuotaObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmAutoApplyQuota {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationManager, IFsrmClassificationManager_Vtbl, 0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager {
    pub unsafe fn ClassificationReportFormats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportFormats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportFormats)(windows_core::Interface::as_raw(self), formats).ok() }
    }
    pub unsafe fn Logging(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogging)(windows_core::Interface::as_raw(self), logging).ok() }
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportMailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportEnabled)(windows_core::Interface::as_raw(self), reportenabled).ok() }
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationLastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationRunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPropertyDefinitions)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyDefinition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyDefinition(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRules)(windows_core::Interface::as_raw(self), ruletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRule)(windows_core::Interface::as_raw(self), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRule(&self, rulename: &windows_core::BSTR, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulename), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumModuleDefinitions)(windows_core::Interface::as_raw(self), moduletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateModuleDefinition)(windows_core::Interface::as_raw(self), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetModuleDefinition(&self, modulename: &windows_core::BSTR, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModuleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(modulename), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RunClassification(&self, context: FsrmReportGenerationContext, reserved: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunClassification)(windows_core::Interface::as_raw(self), context, core::mem::transmute_copy(reserved)).ok() }
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForClassificationCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelClassification(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelClassification)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumFileProperties(&self, filepath: &windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(propertyname), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, propertyvalue: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(propertyname), core::mem::transmute_copy(propertyvalue)).ok() }
    }
    pub unsafe fn ClearFileProperty(&self, filepath: &windows_core::BSTR, property: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(property)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassificationLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassificationRunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    pub EnumPropertyDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRules: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmRuleType, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRule: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumModuleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPipelineModuleType, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunClassification: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForClassificationCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CancelClassification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn ClassificationReportFormats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Logging(&self) -> windows_core::Result<i32>;
    fn SetLogging(&self, logging: i32) -> windows_core::Result<()>;
    fn ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClassificationReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetClassificationReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClassificationRunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus>;
    fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection>;
    fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition>;
    fn GetPropertyDefinition(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyDefinition>;
    fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection>;
    fn CreateRule(&self, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule>;
    fn GetRule(&self, rulename: &windows_core::BSTR, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule>;
    fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection>;
    fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn GetModuleDefinition(&self, modulename: &windows_core::BSTR, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn RunClassification(&self, context: FsrmReportGenerationContext, reserved: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WaitForClassificationCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CancelClassification(&self) -> windows_core::Result<()>;
    fn EnumFileProperties(&self, filepath: &windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmCollection>;
    fn GetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, propertyvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClearFileProperty(&self, filepath: &windows_core::BSTR, property: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmClassificationManager_Vtbl {
    pub const fn new<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportFormats(this) {
                    Ok(ok__) => {
                        formats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportFormats(this, core::mem::transmute_copy(&formats)).into()
            }
        }
        unsafe extern "system" fn Logging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::Logging(this) {
                    Ok(ok__) => {
                        logging.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetLogging(this, core::mem::transmute_copy(&logging)).into()
            }
        }
        unsafe extern "system" fn ClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportMailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn ClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportEnabled(this) {
                    Ok(ok__) => {
                        reportenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportEnabled(this, core::mem::transmute_copy(&reportenabled)).into()
            }
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastreportpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationLastReportPathWithoutExtension(this) {
                    Ok(ok__) => {
                        lastreportpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassificationLastError<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lasterror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationLastError(this) {
                    Ok(ok__) => {
                        lasterror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassificationRunningStatus<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationRunningStatus(this) {
                    Ok(ok__) => {
                        runningstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumPropertyDefinitions(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        propertydefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreatePropertyDefinition(this) {
                    Ok(ok__) => {
                        propertydefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, propertydefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetPropertyDefinition(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        propertydefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRules<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumRules(this, core::mem::transmute_copy(&ruletype), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRule<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreateRule(this, core::mem::transmute_copy(&ruletype)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRule<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulename: *mut core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetRule(this, core::mem::transmute(&rulename), core::mem::transmute_copy(&ruletype)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumModuleDefinitions<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumModuleDefinitions(this, core::mem::transmute_copy(&moduletype), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        moduledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateModuleDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreateModuleDefinition(this, core::mem::transmute_copy(&moduletype)) {
                    Ok(ok__) => {
                        moduledefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModuleDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulename: *mut core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetModuleDefinition(this, core::mem::transmute(&modulename), core::mem::transmute_copy(&moduletype)) {
                    Ok(ok__) => {
                        moduledefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunClassification<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: FsrmReportGenerationContext, reserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::RunClassification(this, core::mem::transmute_copy(&context), core::mem::transmute(&reserved)).into()
            }
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::WaitForClassificationCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelClassification<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::CancelClassification(this).into()
            }
        }
        unsafe extern "system" fn EnumFileProperties<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, options: FsrmGetFilePropertyOptions, fileproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumFileProperties(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        fileproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, options: FsrmGetFilePropertyOptions, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&propertyname), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, propertyvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&propertyname), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn ClearFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, property: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::ClearFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&property)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ClassificationReportFormats: ClassificationReportFormats::<Identity, OFFSET>,
            SetClassificationReportFormats: SetClassificationReportFormats::<Identity, OFFSET>,
            Logging: Logging::<Identity, OFFSET>,
            SetLogging: SetLogging::<Identity, OFFSET>,
            ClassificationReportMailTo: ClassificationReportMailTo::<Identity, OFFSET>,
            SetClassificationReportMailTo: SetClassificationReportMailTo::<Identity, OFFSET>,
            ClassificationReportEnabled: ClassificationReportEnabled::<Identity, OFFSET>,
            SetClassificationReportEnabled: SetClassificationReportEnabled::<Identity, OFFSET>,
            ClassificationLastReportPathWithoutExtension: ClassificationLastReportPathWithoutExtension::<Identity, OFFSET>,
            ClassificationLastError: ClassificationLastError::<Identity, OFFSET>,
            ClassificationRunningStatus: ClassificationRunningStatus::<Identity, OFFSET>,
            EnumPropertyDefinitions: EnumPropertyDefinitions::<Identity, OFFSET>,
            CreatePropertyDefinition: CreatePropertyDefinition::<Identity, OFFSET>,
            GetPropertyDefinition: GetPropertyDefinition::<Identity, OFFSET>,
            EnumRules: EnumRules::<Identity, OFFSET>,
            CreateRule: CreateRule::<Identity, OFFSET>,
            GetRule: GetRule::<Identity, OFFSET>,
            EnumModuleDefinitions: EnumModuleDefinitions::<Identity, OFFSET>,
            CreateModuleDefinition: CreateModuleDefinition::<Identity, OFFSET>,
            GetModuleDefinition: GetModuleDefinition::<Identity, OFFSET>,
            RunClassification: RunClassification::<Identity, OFFSET>,
            WaitForClassificationCompletion: WaitForClassificationCompletion::<Identity, OFFSET>,
            CancelClassification: CancelClassification::<Identity, OFFSET>,
            EnumFileProperties: EnumFileProperties::<Identity, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, OFFSET>,
            ClearFileProperty: ClearFileProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmClassificationManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationManager2, IFsrmClassificationManager2_Vtbl, 0x0004c1c9_127e_4765_ba07_6a3147bca112);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationManager2 {
    type Target = IFsrmClassificationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmClassificationManager);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager2 {
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassifyFiles)(windows_core::Interface::as_raw(self), filepaths, propertynames, propertyvalues, options).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    pub ClassifyFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *const super::super::System::Com::SAFEARRAY, *const super::super::System::Com::SAFEARRAY, FsrmGetFilePropertyOptions) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationManager2_Impl: IFsrmClassificationManager_Impl {
    fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmClassificationManager2_Vtbl {
    pub const fn new<Identity: IFsrmClassificationManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClassifyFiles<Identity: IFsrmClassificationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager2_Impl::ClassifyFiles(this, core::mem::transmute_copy(&filepaths), core::mem::transmute_copy(&propertynames), core::mem::transmute_copy(&propertyvalues), core::mem::transmute_copy(&options)).into()
            }
        }
        Self { base__: IFsrmClassificationManager_Vtbl::new::<Identity, OFFSET>(), ClassifyFiles: ClassifyFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationManager2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmClassificationManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmClassificationManager2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationRule, IFsrmClassificationRule_Vtbl, 0xafc052c2_5315_45ab_841b_c6db0e120148);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationRule {
    type Target = IFsrmRule;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationRule, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmRule);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationRule {
    pub unsafe fn ExecutionOption(&self) -> windows_core::Result<FsrmExecutionOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecutionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExecutionOption)(windows_core::Interface::as_raw(self), executionoption).ok() }
    }
    pub unsafe fn PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyAffected)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property)).ok() }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    pub ExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmExecutionOption) -> windows_core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmExecutionOption) -> windows_core::HRESULT,
    pub PropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassificationRule_Impl: IFsrmRule_Impl {
    fn ExecutionOption(&self) -> windows_core::Result<FsrmExecutionOption>;
    fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> windows_core::Result<()>;
    fn PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmClassificationRule_Vtbl {
    pub const fn new<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::ExecutionOption(this) {
                    Ok(ok__) => {
                        executionoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetExecutionOption(this, core::mem::transmute_copy(&executionoption)).into()
            }
        }
        unsafe extern "system" fn PropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::PropertyAffected(this) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetPropertyAffected(this, core::mem::transmute(&property)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: IFsrmRule_Vtbl::new::<Identity, OFFSET>(),
            ExecutionOption: ExecutionOption::<Identity, OFFSET>,
            SetExecutionOption: SetExecutionOption::<Identity, OFFSET>,
            PropertyAffected: PropertyAffected::<Identity, OFFSET>,
            SetPropertyAffected: SetPropertyAffected::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmClassificationRule {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassifierModuleDefinition, IFsrmClassifierModuleDefinition_Vtbl, 0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassifierModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleDefinition {
    pub unsafe fn PropertiesAffected(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertiesAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertiesAffected)(windows_core::Interface::as_raw(self), propertiesaffected).ok() }
    }
    pub unsafe fn PropertiesUsed(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertiesUsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertiesUsed)(windows_core::Interface::as_raw(self), propertiesused).ok() }
    }
    pub unsafe fn NeedsExplicitValue(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsExplicitValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNeedsExplicitValue)(windows_core::Interface::as_raw(self), needsexplicitvalue).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub PropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub NeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassifierModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn PropertiesAffected(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn PropertiesUsed(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn NeedsExplicitValue(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsExplicitValue(&self, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmClassifierModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::PropertiesAffected(this) {
                    Ok(ok__) => {
                        propertiesaffected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetPropertiesAffected(this, core::mem::transmute_copy(&propertiesaffected)).into()
            }
        }
        unsafe extern "system" fn PropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::PropertiesUsed(this) {
                    Ok(ok__) => {
                        propertiesused.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetPropertiesUsed(this, core::mem::transmute_copy(&propertiesused)).into()
            }
        }
        unsafe extern "system" fn NeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::NeedsExplicitValue(this) {
                    Ok(ok__) => {
                        needsexplicitvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetNeedsExplicitValue(this, core::mem::transmute_copy(&needsexplicitvalue)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            PropertiesAffected: PropertiesAffected::<Identity, OFFSET>,
            SetPropertiesAffected: SetPropertiesAffected::<Identity, OFFSET>,
            PropertiesUsed: PropertiesUsed::<Identity, OFFSET>,
            SetPropertiesUsed: SetPropertiesUsed::<Identity, OFFSET>,
            NeedsExplicitValue: NeedsExplicitValue::<Identity, OFFSET>,
            SetNeedsExplicitValue: SetNeedsExplicitValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmClassifierModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassifierModuleImplementation, IFsrmClassifierModuleImplementation_Vtbl, 0x4c968fc6_6edb_4051_9c18_73b7291ae106);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassifierModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleImplementation {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LastModified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UseRulesAndDefinitions<P0, P1>(&self, rules: P0, propertydefinitions: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmCollection>,
        P1: windows_core::Param<IFsrmCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).UseRulesAndDefinitions)(windows_core::Interface::as_raw(self), rules.param().abi(), propertydefinitions.param().abi()).ok() }
    }
    pub unsafe fn OnBeginFile<P0>(&self, propertybag: P0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeginFile)(windows_core::Interface::as_raw(self), propertybag.param().abi(), arrayruleids).ok() }
    }
    pub unsafe fn DoesPropertyValueApply(&self, property: &windows_core::BSTR, value: &windows_core::BSTR, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoesPropertyValueApply)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), core::mem::transmute_copy(value), applyvalue as _, core::mem::transmute(idrule), core::mem::transmute(idpropdef)).ok() }
    }
    pub unsafe fn GetPropertyValueToApply(&self, property: &windows_core::BSTR, value: *mut windows_core::BSTR, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyValueToApply)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), core::mem::transmute(value), core::mem::transmute(idrule), core::mem::transmute(idpropdef)).ok() }
    }
    pub unsafe fn OnEndFile(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEndFile)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LastModified: usize,
    pub UseRulesAndDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBeginFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub DoesPropertyValueApply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub GetPropertyValueToApply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub OnEndFile: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmClassifierModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn LastModified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn UseRulesAndDefinitions(&self, rules: windows_core::Ref<IFsrmCollection>, propertydefinitions: windows_core::Ref<IFsrmCollection>) -> windows_core::Result<()>;
    fn OnBeginFile(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn DoesPropertyValueApply(&self, property: &windows_core::BSTR, value: &windows_core::BSTR, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: &windows_core::GUID, idpropdef: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetPropertyValueToApply(&self, property: &windows_core::BSTR, value: *mut windows_core::BSTR, idrule: &windows_core::GUID, idpropdef: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnEndFile(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmClassifierModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastModified<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleImplementation_Impl::LastModified(this) {
                    Ok(ok__) => {
                        lastmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut core::ffi::c_void, propertydefinitions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::UseRulesAndDefinitions(this, core::mem::transmute_copy(&rules), core::mem::transmute_copy(&propertydefinitions)).into()
            }
        }
        unsafe extern "system" fn OnBeginFile<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::OnBeginFile(this, core::mem::transmute_copy(&propertybag), core::mem::transmute_copy(&arrayruleids)).into()
            }
        }
        unsafe extern "system" fn DoesPropertyValueApply<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, value: *mut core::ffi::c_void, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::DoesPropertyValueApply(this, core::mem::transmute(&property), core::mem::transmute(&value), core::mem::transmute_copy(&applyvalue), core::mem::transmute(&idrule), core::mem::transmute(&idpropdef)).into()
            }
        }
        unsafe extern "system" fn GetPropertyValueToApply<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::GetPropertyValueToApply(this, core::mem::transmute(&property), core::mem::transmute_copy(&value), core::mem::transmute(&idrule), core::mem::transmute(&idpropdef)).into()
            }
        }
        unsafe extern "system" fn OnEndFile<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::OnEndFile(this).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, OFFSET>(),
            LastModified: LastModified::<Identity, OFFSET>,
            UseRulesAndDefinitions: UseRulesAndDefinitions::<Identity, OFFSET>,
            OnBeginFile: OnBeginFile::<Identity, OFFSET>,
            DoesPropertyValueApply: DoesPropertyValueApply::<Identity, OFFSET>,
            GetPropertyValueToApply: GetPropertyValueToApply::<Identity, OFFSET>,
            OnEndFile: OnEndFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleImplementation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmClassifierModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmCollection, IFsrmCollection_Vtbl, 0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<FsrmCollectionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetById(&self, id: windows_core::GUID) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetById)(windows_core::Interface::as_raw(self), core::mem::transmute(id), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmCollectionState) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetById: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmCollection_Impl: super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn State(&self) -> windows_core::Result<FsrmCollectionState>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetById(&self, id: &windows_core::GUID) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmCollection_Vtbl {
    pub const fn new<Identity: IFsrmCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        unknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut FsrmCollectionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::State(this) {
                    Ok(ok__) => {
                        state.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmCollection_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::WaitForCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetById<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::GUID, entry: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::GetById(this, core::mem::transmute(&id)) {
                    Ok(ok__) => {
                        entry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            GetById: GetById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmCommittableCollection, IFsrmCommittableCollection_Vtbl, 0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmCommittableCollection {
    type Target = IFsrmMutableCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmCommittableCollection, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection, IFsrmMutableCollection);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCommittableCollection {
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmCommittableCollection_Impl: IFsrmMutableCollection_Impl {
    fn Commit(&self, options: FsrmCommitOptions) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmCommittableCollection_Vtbl {
    pub const fn new<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmCommitOptions, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCommittableCollection_Impl::Commit(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFsrmMutableCollection_Vtbl::new::<Identity, OFFSET>(), Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmCommittableCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<IFsrmMutableCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmCommittableCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmDerivedObjectsResult, IFsrmDerivedObjectsResult_Vtbl, 0x39322a2d_38ee_4d0d_8095_421a80849a82);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmDerivedObjectsResult {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmDerivedObjectsResult, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmDerivedObjectsResult {
    pub unsafe fn DerivedObjects(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DerivedObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Results(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DerivedObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmDerivedObjectsResult_Impl: super::super::System::Com::IDispatch_Impl {
    fn DerivedObjects(&self) -> windows_core::Result<IFsrmCollection>;
    fn Results(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmDerivedObjectsResult_Vtbl {
    pub const fn new<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DerivedObjects<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, derivedobjects: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::DerivedObjects(this) {
                    Ok(ok__) => {
                        derivedobjects.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Results<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::Results(this) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DerivedObjects: DerivedObjects::<Identity, OFFSET>,
            Results: Results::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmDerivedObjectsResult {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmExportImport, IFsrmExportImport_Vtbl, 0xefcb0ab1_16c4_4a79_812c_725614c3306b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmExportImport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmExportImport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmExportImport {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(filegroupnamessafearray), core::mem::transmute_copy(remotehost)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(filegroupnamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportQuotaTemplates: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmExportImport_Impl: super::super::System::Com::IDispatch_Impl {
    fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmExportImport_Vtbl {
    pub const fn new<Identity: IFsrmExportImport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileGroups(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&filegroupnamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void, filegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportFileGroups(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&filegroupnamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        filegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileScreenTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportFileScreenTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        templates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportQuotaTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::super::System::Variant::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportQuotaTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        templates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExportFileGroups: ExportFileGroups::<Identity, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Identity, OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Identity, OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Identity, OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmExportImport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmExportImport {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileCondition, IFsrmFileCondition_Vtbl, 0x70684ffc_691a_4a1a_b922_97752e138cc1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileCondition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileCondition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileCondition {
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmFileConditionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileConditionType) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileCondition_Impl: super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<FsrmFileConditionType>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileCondition_Vtbl {
    pub const fn new<Identity: IFsrmFileCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsrmFileConditionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileCondition_Impl::Type(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileCondition_Impl::Delete(this).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Type: Type::<Identity, OFFSET>, Delete: Delete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileCondition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileCondition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileConditionProperty, IFsrmFileConditionProperty_Vtbl, 0x81926775_b981_4479_988f_da171d627360);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileConditionProperty {
    type Target = IFsrmFileCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileConditionProperty, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmFileCondition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileConditionProperty {
    pub unsafe fn PropertyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPropertyName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn PropertyId(&self) -> windows_core::Result<FsrmFileSystemPropertyId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyId)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn Operator(&self) -> windows_core::Result<FsrmPropertyConditionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOperator)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ValueType(&self) -> windows_core::Result<FsrmPropertyValueType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValueType)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, newval: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionProperty_Vtbl {
    pub base__: IFsrmFileCondition_Vtbl,
    pub PropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileSystemPropertyId) -> windows_core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileSystemPropertyId) -> windows_core::HRESULT,
    pub Operator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub ValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyValueType) -> windows_core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyValueType) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileConditionProperty_Impl: IFsrmFileCondition_Impl {
    fn PropertyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPropertyName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PropertyId(&self) -> windows_core::Result<FsrmFileSystemPropertyId>;
    fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> windows_core::Result<()>;
    fn Operator(&self) -> windows_core::Result<FsrmPropertyConditionType>;
    fn SetOperator(&self, newval: FsrmPropertyConditionType) -> windows_core::Result<()>;
    fn ValueType(&self) -> windows_core::Result<FsrmPropertyValueType>;
    fn SetValueType(&self, newval: FsrmPropertyValueType) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(&self, newval: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileConditionProperty_Vtbl {
    pub const fn new<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyName<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileConditionProperty_Impl::PropertyName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertyName<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileConditionProperty_Impl::SetPropertyName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn PropertyId<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileConditionProperty_Impl::PropertyId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertyId<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileConditionProperty_Impl::SetPropertyId(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Operator<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileConditionProperty_Impl::Operator(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperator<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsrmPropertyConditionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileConditionProperty_Impl::SetOperator(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ValueType<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileConditionProperty_Impl::ValueType(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueType<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsrmPropertyValueType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileConditionProperty_Impl::SetValueType(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileConditionProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileConditionProperty_Impl::SetValue(this, core::mem::transmute(&newval)).into()
            }
        }
        Self {
            base__: IFsrmFileCondition_Vtbl::new::<Identity, OFFSET>(),
            PropertyName: PropertyName::<Identity, OFFSET>,
            SetPropertyName: SetPropertyName::<Identity, OFFSET>,
            PropertyId: PropertyId::<Identity, OFFSET>,
            SetPropertyId: SetPropertyId::<Identity, OFFSET>,
            Operator: Operator::<Identity, OFFSET>,
            SetOperator: SetOperator::<Identity, OFFSET>,
            ValueType: ValueType::<Identity, OFFSET>,
            SetValueType: SetValueType::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileConditionProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmFileCondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileConditionProperty {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroup, IFsrmFileGroup_Vtbl, 0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroup {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroup, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Members(&self) -> windows_core::Result<IFsrmMutableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMembers<P0>(&self, members: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMembers)(windows_core::Interface::as_raw(self), members.param().abi()).ok() }
    }
    pub unsafe fn NonMembers(&self) -> windows_core::Result<IFsrmMutableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NonMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetNonMembers<P0>(&self, nonmembers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNonMembers)(windows_core::Interface::as_raw(self), nonmembers.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroup_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroup_Impl: IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IFsrmMutableCollection>;
    fn SetMembers(&self, members: windows_core::Ref<IFsrmMutableCollection>) -> windows_core::Result<()>;
    fn NonMembers(&self) -> windows_core::Result<IFsrmMutableCollection>;
    fn SetNonMembers(&self, nonmembers: windows_core::Ref<IFsrmMutableCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileGroup_Vtbl {
    pub const fn new<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileGroup_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Members<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, members: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroup_Impl::Members(this) {
                    Ok(ok__) => {
                        members.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMembers<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, members: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileGroup_Impl::SetMembers(this, core::mem::transmute_copy(&members)).into()
            }
        }
        unsafe extern "system" fn NonMembers<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nonmembers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroup_Impl::NonMembers(this) {
                    Ok(ok__) => {
                        nonmembers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNonMembers<Identity: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nonmembers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileGroup_Impl::SetNonMembers(this, core::mem::transmute_copy(&nonmembers)).into()
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            SetMembers: SetMembers::<Identity, OFFSET>,
            NonMembers: NonMembers::<Identity, OFFSET>,
            SetNonMembers: SetNonMembers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileGroup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroupImported, IFsrmFileGroupImported_Vtbl, 0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroupImported {
    type Target = IFsrmFileGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroupImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileGroup);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImported_Vtbl {
    pub base__: IFsrmFileGroup_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroupImported_Impl: IFsrmFileGroup_Impl {
    fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileGroupImported_Vtbl {
    pub const fn new<Identity: IFsrmFileGroupImported_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OverwriteOnCommit<Identity: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupImported_Impl::OverwriteOnCommit(this) {
                    Ok(ok__) => {
                        overwrite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileGroupImported_Impl::SetOverwriteOnCommit(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        Self {
            base__: IFsrmFileGroup_Vtbl::new::<Identity, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileGroupImported as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmFileGroup as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileGroupImported {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroupManager, IFsrmFileGroupManager_Vtbl, 0x426677d5_018c_485c_8a51_20b86d00bdc4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroupManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroupManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupManager {
    pub unsafe fn CreateFileGroup(&self) -> windows_core::Result<IFsrmFileGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileGroup(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileGroups)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute(filegroupnamesarray), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportFileGroups(&self, serializedfilegroups: &windows_core::BSTR, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serializedfilegroups), core::mem::transmute(filegroupnamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CreateFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportFileGroups: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileGroupManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn CreateFileGroup(&self) -> windows_core::Result<IFsrmFileGroup>;
    fn GetFileGroup(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileGroup>;
    fn EnumFileGroups(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn ImportFileGroups(&self, serializedfilegroups: &windows_core::BSTR, filegroupnamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileGroupManager_Vtbl {
    pub const fn new<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFileGroup<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filegroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupManager_Impl::CreateFileGroup(this) {
                    Ok(ok__) => {
                        filegroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileGroup<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, filegroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupManager_Impl::GetFileGroup(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        filegroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFileGroups<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupManager_Impl::EnumFileGroups(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        filegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportFileGroups<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filegroupnamesarray: *const super::super::System::Variant::VARIANT, serializedfilegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupManager_Impl::ExportFileGroups(this, core::mem::transmute_copy(&filegroupnamesarray)) {
                    Ok(ok__) => {
                        serializedfilegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportFileGroups<Identity: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serializedfilegroups: *mut core::ffi::c_void, filegroupnamesarray: *const super::super::System::Variant::VARIANT, filegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileGroupManager_Impl::ImportFileGroups(this, core::mem::transmute(&serializedfilegroups), core::mem::transmute_copy(&filegroupnamesarray)) {
                    Ok(ok__) => {
                        filegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateFileGroup: CreateFileGroup::<Identity, OFFSET>,
            GetFileGroup: GetFileGroup::<Identity, OFFSET>,
            EnumFileGroups: EnumFileGroups::<Identity, OFFSET>,
            ExportFileGroups: ExportFileGroups::<Identity, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileGroupManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileGroupManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileManagementJob, IFsrmFileManagementJob_Vtbl, 0x0770687e_9f36_4d6f_8778_599d188461c9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileManagementJob {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileManagementJob, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJob {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn OperationType(&self) -> windows_core::Result<FsrmFileManagementType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OperationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOperationType)(windows_core::Interface::as_raw(self), operationtype).ok() }
    }
    pub unsafe fn ExpirationDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetExpirationDirectory(&self, expirationdirectory: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExpirationDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(expirationdirectory)).ok() }
    }
    pub unsafe fn CustomAction(&self) -> windows_core::Result<IFsrmActionCommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CustomAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Notifications(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Notifications)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Logging(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogging)(windows_core::Interface::as_raw(self), loggingflags).ok() }
    }
    pub unsafe fn ReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportEnabled)(windows_core::Interface::as_raw(self), reportenabled).ok() }
    }
    pub unsafe fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Formats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormats)(windows_core::Interface::as_raw(self), formats).ok() }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DaysSinceFileCreated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysSinceFileCreated)(windows_core::Interface::as_raw(self), dayssincecreation).ok() }
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DaysSinceFileLastAccessed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysSinceFileLastAccessed)(windows_core::Interface::as_raw(self), dayssinceaccess).ok() }
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DaysSinceFileLastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysSinceFileLastModified)(windows_core::Interface::as_raw(self), dayssincemodify).ok() }
    }
    pub unsafe fn PropertyConditions(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyConditions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FromDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FromDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFromDate)(windows_core::Interface::as_raw(self), fromdate).ok() }
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname)).ok() }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok() }
    }
    pub unsafe fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LastRun(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileNamePattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileNamePattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileNamePattern(&self, filenamepattern: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileNamePattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filenamepattern)).ok() }
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), context).ok() }
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddNotification(&self, days: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddNotification)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteNotification)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyNotification)(windows_core::Interface::as_raw(self), days, newdays).ok() }
    }
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNotificationAction)(windows_core::Interface::as_raw(self), days, actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumNotificationActions)(windows_core::Interface::as_raw(self), days, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePropertyCondition(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyCondition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomAction(&self) -> windows_core::Result<IFsrmActionCommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OperationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileManagementType) -> windows_core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileManagementType) -> windows_core::HRESULT,
    pub ExpirationDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExpirationDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CustomAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Formats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DaysSinceFileCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PropertyConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub RunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub FileNamePattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileNamePattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CreateNotificationAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumNotificationActions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePropertyCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCustomAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileManagementJob_Impl: IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn OperationType(&self) -> windows_core::Result<FsrmFileManagementType>;
    fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> windows_core::Result<()>;
    fn ExpirationDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExpirationDirectory(&self, expirationdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CustomAction(&self) -> windows_core::Result<IFsrmActionCommand>;
    fn Notifications(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Logging(&self) -> windows_core::Result<i32>;
    fn SetLogging(&self, loggingflags: i32) -> windows_core::Result<()>;
    fn ReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DaysSinceFileCreated(&self) -> windows_core::Result<i32>;
    fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> windows_core::Result<()>;
    fn DaysSinceFileLastAccessed(&self) -> windows_core::Result<i32>;
    fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> windows_core::Result<()>;
    fn DaysSinceFileLastModified(&self) -> windows_core::Result<i32>;
    fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> windows_core::Result<()>;
    fn PropertyConditions(&self) -> windows_core::Result<IFsrmCollection>;
    fn FromDate(&self) -> windows_core::Result<f64>;
    fn SetFromDate(&self, fromdate: f64) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus>;
    fn LastError(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastRun(&self) -> windows_core::Result<f64>;
    fn FileNamePattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileNamePattern(&self, filenamepattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn AddNotification(&self, days: i32) -> windows_core::Result<()>;
    fn DeleteNotification(&self, days: i32) -> windows_core::Result<()>;
    fn ModifyNotification(&self, days: i32, newdays: i32) -> windows_core::Result<()>;
    fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction>;
    fn EnumNotificationActions(&self, days: i32) -> windows_core::Result<IFsrmCollection>;
    fn CreatePropertyCondition(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyCondition>;
    fn CreateCustomAction(&self) -> windows_core::Result<IFsrmActionCommand>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileManagementJob_Vtbl {
    pub const fn new<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn NamespaceRoots<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::NamespaceRoots(this) {
                    Ok(ok__) => {
                        namespaceroots.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetNamespaceRoots(this, core::mem::transmute_copy(&namespaceroots)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn OperationType<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::OperationType(this) {
                    Ok(ok__) => {
                        operationtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOperationType<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationtype: FsrmFileManagementType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetOperationType(this, core::mem::transmute_copy(&operationtype)).into()
            }
        }
        unsafe extern "system" fn ExpirationDirectory<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expirationdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::ExpirationDirectory(this) {
                    Ok(ok__) => {
                        expirationdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExpirationDirectory<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expirationdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetExpirationDirectory(this, core::mem::transmute(&expirationdirectory)).into()
            }
        }
        unsafe extern "system" fn CustomAction<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::CustomAction(this) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notifications<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Notifications(this) {
                    Ok(ok__) => {
                        notifications.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Logging<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Logging(this) {
                    Ok(ok__) => {
                        loggingflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogging<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loggingflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetLogging(this, core::mem::transmute_copy(&loggingflags)).into()
            }
        }
        unsafe extern "system" fn ReportEnabled<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::ReportEnabled(this) {
                    Ok(ok__) => {
                        reportenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportEnabled<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetReportEnabled(this, core::mem::transmute_copy(&reportenabled)).into()
            }
        }
        unsafe extern "system" fn Formats<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Formats(this) {
                    Ok(ok__) => {
                        formats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormats<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetFormats(this, core::mem::transmute_copy(&formats)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn DaysSinceFileCreated<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssincecreation: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::DaysSinceFileCreated(this) {
                    Ok(ok__) => {
                        dayssincecreation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDaysSinceFileCreated<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssincecreation: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetDaysSinceFileCreated(this, core::mem::transmute_copy(&dayssincecreation)).into()
            }
        }
        unsafe extern "system" fn DaysSinceFileLastAccessed<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssinceaccess: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::DaysSinceFileLastAccessed(this) {
                    Ok(ok__) => {
                        dayssinceaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastAccessed<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssinceaccess: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetDaysSinceFileLastAccessed(this, core::mem::transmute_copy(&dayssinceaccess)).into()
            }
        }
        unsafe extern "system" fn DaysSinceFileLastModified<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssincemodify: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::DaysSinceFileLastModified(this) {
                    Ok(ok__) => {
                        dayssincemodify.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastModified<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dayssincemodify: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetDaysSinceFileLastModified(this, core::mem::transmute_copy(&dayssincemodify)).into()
            }
        }
        unsafe extern "system" fn PropertyConditions<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyconditions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::PropertyConditions(this) {
                    Ok(ok__) => {
                        propertyconditions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FromDate<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fromdate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::FromDate(this) {
                    Ok(ok__) => {
                        fromdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFromDate<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fromdate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetFromDate(this, core::mem::transmute_copy(&fromdate)).into()
            }
        }
        unsafe extern "system" fn Task<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Task(this) {
                    Ok(ok__) => {
                        taskname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetTask(this, core::mem::transmute(&taskname)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        unsafe extern "system" fn RunningStatus<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::RunningStatus(this) {
                    Ok(ok__) => {
                        runningstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastError<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lasterror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::LastError(this) {
                    Ok(ok__) => {
                        lasterror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastReportPathWithoutExtension<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::LastReportPathWithoutExtension(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastRun<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastrun: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::LastRun(this) {
                    Ok(ok__) => {
                        lastrun.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileNamePattern<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filenamepattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::FileNamePattern(this) {
                    Ok(ok__) => {
                        filenamepattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileNamePattern<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filenamepattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::SetFileNamePattern(this, core::mem::transmute(&filenamepattern)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: FsrmReportGenerationContext) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::Run(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::WaitForCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn AddNotification<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::AddNotification(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn DeleteNotification<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::DeleteNotification(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn ModifyNotification<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32, newdays: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileManagementJob_Impl::ModifyNotification(this, core::mem::transmute_copy(&days), core::mem::transmute_copy(&newdays)).into()
            }
        }
        unsafe extern "system" fn CreateNotificationAction<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::CreateNotificationAction(this, core::mem::transmute_copy(&days), core::mem::transmute_copy(&actiontype)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumNotificationActions<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::EnumNotificationActions(this, core::mem::transmute_copy(&days)) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, propertycondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::CreatePropertyCondition(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        propertycondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomAction<Identity: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJob_Impl::CreateCustomAction(this) {
                    Ok(ok__) => {
                        customaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            OperationType: OperationType::<Identity, OFFSET>,
            SetOperationType: SetOperationType::<Identity, OFFSET>,
            ExpirationDirectory: ExpirationDirectory::<Identity, OFFSET>,
            SetExpirationDirectory: SetExpirationDirectory::<Identity, OFFSET>,
            CustomAction: CustomAction::<Identity, OFFSET>,
            Notifications: Notifications::<Identity, OFFSET>,
            Logging: Logging::<Identity, OFFSET>,
            SetLogging: SetLogging::<Identity, OFFSET>,
            ReportEnabled: ReportEnabled::<Identity, OFFSET>,
            SetReportEnabled: SetReportEnabled::<Identity, OFFSET>,
            Formats: Formats::<Identity, OFFSET>,
            SetFormats: SetFormats::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
            DaysSinceFileCreated: DaysSinceFileCreated::<Identity, OFFSET>,
            SetDaysSinceFileCreated: SetDaysSinceFileCreated::<Identity, OFFSET>,
            DaysSinceFileLastAccessed: DaysSinceFileLastAccessed::<Identity, OFFSET>,
            SetDaysSinceFileLastAccessed: SetDaysSinceFileLastAccessed::<Identity, OFFSET>,
            DaysSinceFileLastModified: DaysSinceFileLastModified::<Identity, OFFSET>,
            SetDaysSinceFileLastModified: SetDaysSinceFileLastModified::<Identity, OFFSET>,
            PropertyConditions: PropertyConditions::<Identity, OFFSET>,
            FromDate: FromDate::<Identity, OFFSET>,
            SetFromDate: SetFromDate::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            RunningStatus: RunningStatus::<Identity, OFFSET>,
            LastError: LastError::<Identity, OFFSET>,
            LastReportPathWithoutExtension: LastReportPathWithoutExtension::<Identity, OFFSET>,
            LastRun: LastRun::<Identity, OFFSET>,
            FileNamePattern: FileNamePattern::<Identity, OFFSET>,
            SetFileNamePattern: SetFileNamePattern::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            AddNotification: AddNotification::<Identity, OFFSET>,
            DeleteNotification: DeleteNotification::<Identity, OFFSET>,
            ModifyNotification: ModifyNotification::<Identity, OFFSET>,
            CreateNotificationAction: CreateNotificationAction::<Identity, OFFSET>,
            EnumNotificationActions: EnumNotificationActions::<Identity, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, OFFSET>,
            CreateCustomAction: CreateCustomAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileManagementJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileManagementJob {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileManagementJobManager, IFsrmFileManagementJobManager_Vtbl, 0xee321ecb_d95e_48e9_907c_c7685a013235);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileManagementJobManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileManagementJobManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJobManager {
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileManagementJobs)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileManagementJob(&self) -> windows_core::Result<IFsrmFileManagementJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileManagementJob)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileManagementJob(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileManagementJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileManagementJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub EnumFileManagementJobs: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileManagementJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileManagementJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileManagementJobManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection>;
    fn CreateFileManagementJob(&self) -> windows_core::Result<IFsrmFileManagementJob>;
    fn GetFileManagementJob(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileManagementJob>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileManagementJobManager_Vtbl {
    pub const fn new<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActionVariables<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJobManager_Impl::ActionVariables(this) {
                    Ok(ok__) => {
                        variables.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJobManager_Impl::ActionVariableDescriptions(this) {
                    Ok(ok__) => {
                        descriptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFileManagementJobs<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJobManager_Impl::EnumFileManagementJobs(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        filemanagementjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileManagementJob<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filemanagementjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJobManager_Impl::CreateFileManagementJob(this) {
                    Ok(ok__) => {
                        filemanagementjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileManagementJob<Identity: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, filemanagementjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileManagementJobManager_Impl::GetFileManagementJob(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        filemanagementjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, OFFSET>,
            EnumFileManagementJobs: EnumFileManagementJobs::<Identity, OFFSET>,
            CreateFileManagementJob: CreateFileManagementJob::<Identity, OFFSET>,
            GetFileManagementJob: GetFileManagementJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileManagementJobManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileManagementJobManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreen, IFsrmFileScreen_Vtbl, 0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreen {
    type Target = IFsrmFileScreenBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreen, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreen {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceTemplateName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchesSourceTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ApplyTemplate(&self, filescreentemplatename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplyTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filescreentemplatename)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreen_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreen_Impl: IFsrmFileScreenBase_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ApplyTemplate(&self, filescreentemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreen_Vtbl {
    pub const fn new<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreen_Impl::Path(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SourceTemplateName<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreentemplatename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreen_Impl::SourceTemplateName(this) {
                    Ok(ok__) => {
                        filescreentemplatename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreen_Impl::MatchesSourceTemplate(this) {
                    Ok(ok__) => {
                        matches.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserSid<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usersid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreen_Impl::UserSid(this) {
                    Ok(ok__) => {
                        usersid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserAccount<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreen_Impl::UserAccount(this) {
                    Ok(ok__) => {
                        useraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplyTemplate<Identity: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreentemplatename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreen_Impl::ApplyTemplate(this, core::mem::transmute(&filescreentemplatename)).into()
            }
        }
        Self {
            base__: IFsrmFileScreenBase_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, OFFSET>,
            UserSid: UserSid::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreen as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmFileScreenBase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreen {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenBase, IFsrmFileScreenBase_Vtbl, 0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenBase {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenBase, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenBase {
    pub unsafe fn BlockedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockedFileGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlockedFileGroups)(windows_core::Interface::as_raw(self), blocklist.param().abi()).ok() }
    }
    pub unsafe fn FileScreenFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileScreenFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileScreenFlags)(windows_core::Interface::as_raw(self), filescreenflags).ok() }
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAction)(windows_core::Interface::as_raw(self), actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumActions(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub BlockedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBlockedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileScreenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateAction: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenBase_Impl: IFsrmObject_Impl {
    fn BlockedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection>;
    fn SetBlockedFileGroups(&self, blocklist: windows_core::Ref<IFsrmMutableCollection>) -> windows_core::Result<()>;
    fn FileScreenFlags(&self) -> windows_core::Result<i32>;
    fn SetFileScreenFlags(&self, filescreenflags: i32) -> windows_core::Result<()>;
    fn CreateAction(&self, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction>;
    fn EnumActions(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenBase_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BlockedFileGroups<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocklist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenBase_Impl::BlockedFileGroups(this) {
                    Ok(ok__) => {
                        blocklist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBlockedFileGroups<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocklist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenBase_Impl::SetBlockedFileGroups(this, core::mem::transmute_copy(&blocklist)).into()
            }
        }
        unsafe extern "system" fn FileScreenFlags<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreenflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenBase_Impl::FileScreenFlags(this) {
                    Ok(ok__) => {
                        filescreenflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileScreenFlags<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreenflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenBase_Impl::SetFileScreenFlags(this, core::mem::transmute_copy(&filescreenflags)).into()
            }
        }
        unsafe extern "system" fn CreateAction<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: FsrmActionType, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenBase_Impl::CreateAction(this, core::mem::transmute_copy(&actiontype)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumActions<Identity: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenBase_Impl::EnumActions(this) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            BlockedFileGroups: BlockedFileGroups::<Identity, OFFSET>,
            SetBlockedFileGroups: SetBlockedFileGroups::<Identity, OFFSET>,
            FileScreenFlags: FileScreenFlags::<Identity, OFFSET>,
            SetFileScreenFlags: SetFileScreenFlags::<Identity, OFFSET>,
            CreateAction: CreateAction::<Identity, OFFSET>,
            EnumActions: EnumActions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenBase as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenBase {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenException, IFsrmFileScreenException_Vtbl, 0xbee7ce02_df77_4515_9389_78f01c5afc1a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenException {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenException, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenException {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AllowedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowedFileGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetAllowedFileGroups<P0>(&self, allowlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAllowedFileGroups)(windows_core::Interface::as_raw(self), allowlist.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenException_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllowedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenException_Impl: IFsrmObject_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AllowedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection>;
    fn SetAllowedFileGroups(&self, allowlist: windows_core::Ref<IFsrmMutableCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenException_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenException_Impl::Path(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllowedFileGroups<Identity: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenException_Impl::AllowedFileGroups(this) {
                    Ok(ok__) => {
                        allowlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowedFileGroups<Identity: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenException_Impl::SetAllowedFileGroups(this, core::mem::transmute_copy(&allowlist)).into()
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            AllowedFileGroups: AllowedFileGroups::<Identity, OFFSET>,
            SetAllowedFileGroups: SetAllowedFileGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenException as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenException {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenManager, IFsrmFileScreenManager_Vtbl, 0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenManager {
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateFileScreen(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreen> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileScreen)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileScreen(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreen> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileScreen)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumFileScreens(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileScreens)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileScreenException(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenException> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileScreenException)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileScreenException(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenException> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileScreenException)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumFileScreenExceptions(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileScreenExceptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileScreenCollection(&self) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileScreenCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CreateFileScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFileScreens: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileScreenException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileScreenException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumFileScreenExceptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileScreenCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateFileScreen(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreen>;
    fn GetFileScreen(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreen>;
    fn EnumFileScreens(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenException(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenException>;
    fn GetFileScreenException(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenException>;
    fn EnumFileScreenExceptions(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenCollection(&self) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenManager_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActionVariables<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::ActionVariables(this) {
                    Ok(ok__) => {
                        variables.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::ActionVariableDescriptions(this) {
                    Ok(ok__) => {
                        descriptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileScreen<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filescreen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::CreateFileScreen(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        filescreen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileScreen<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filescreen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::GetFileScreen(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        filescreen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFileScreens<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, filescreens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::EnumFileScreens(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        filescreens.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileScreenException<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filescreenexception: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::CreateFileScreenException(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        filescreenexception.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileScreenException<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filescreenexception: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::GetFileScreenException(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        filescreenexception.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFileScreenExceptions<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, filescreenexceptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::EnumFileScreenExceptions(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        filescreenexceptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileScreenCollection<Identity: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenManager_Impl::CreateFileScreenCollection(this) {
                    Ok(ok__) => {
                        collection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, OFFSET>,
            CreateFileScreen: CreateFileScreen::<Identity, OFFSET>,
            GetFileScreen: GetFileScreen::<Identity, OFFSET>,
            EnumFileScreens: EnumFileScreens::<Identity, OFFSET>,
            CreateFileScreenException: CreateFileScreenException::<Identity, OFFSET>,
            GetFileScreenException: GetFileScreenException::<Identity, OFFSET>,
            EnumFileScreenExceptions: EnumFileScreenExceptions::<Identity, OFFSET>,
            CreateFileScreenCollection: CreateFileScreenCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplate, IFsrmFileScreenTemplate_Vtbl, 0x205bebf8_dd93_452a_95a6_32b566b35828);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplate {
    type Target = IFsrmFileScreenBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplate, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplate {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn CopyTemplate(&self, filescreentemplatename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filescreentemplatename)).ok() }
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplate_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplate_Impl: IFsrmFileScreenBase_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyTemplate(&self, filescreentemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenTemplate_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplate_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenTemplate_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn CopyTemplate<Identity: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreentemplatename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenTemplate_Impl::CopyTemplate(this, core::mem::transmute(&filescreentemplatename)).into()
            }
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplate_Impl::CommitAndUpdateDerived(this, core::mem::transmute_copy(&commitoptions), core::mem::transmute_copy(&applyoptions)) {
                    Ok(ok__) => {
                        derivedobjectsresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmFileScreenBase_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplate as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmFileScreenBase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenTemplate {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplateImported, IFsrmFileScreenTemplateImported_Vtbl, 0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplateImported {
    type Target = IFsrmFileScreenTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplateImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase, IFsrmFileScreenTemplate);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImported_Vtbl {
    pub base__: IFsrmFileScreenTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplateImported_Impl: IFsrmFileScreenTemplate_Impl {
    fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenTemplateImported_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OverwriteOnCommit<Identity: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateImported_Impl::OverwriteOnCommit(this) {
                    Ok(ok__) => {
                        overwrite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmFileScreenTemplateImported_Impl::SetOverwriteOnCommit(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        Self {
            base__: IFsrmFileScreenTemplate_Vtbl::new::<Identity, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateImported as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmFileScreenBase as windows_core::Interface>::IID || iid == &<IFsrmFileScreenTemplate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplateManager, IFsrmFileScreenTemplateManager_Vtbl, 0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplateManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplateManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateManager {
    pub unsafe fn CreateTemplate(&self) -> windows_core::Result<IFsrmFileScreenTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTemplate(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTemplates)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute(filescreentemplatenamesarray), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportTemplates(&self, serializedfilescreentemplates: &windows_core::BSTR, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serializedfilescreentemplates), core::mem::transmute(filescreentemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CreateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportTemplates: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmFileScreenTemplateManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&self) -> windows_core::Result<IFsrmFileScreenTemplate>;
    fn GetTemplate(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmFileScreenTemplate>;
    fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn ImportTemplates(&self, serializedfilescreentemplates: &windows_core::BSTR, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmFileScreenTemplateManager_Vtbl {
    pub const fn new<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTemplate<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreentemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateManager_Impl::CreateTemplate(this) {
                    Ok(ok__) => {
                        filescreentemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTemplate<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, filescreentemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateManager_Impl::GetTemplate(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        filescreentemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumTemplates<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateManager_Impl::EnumTemplates(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        filescreentemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportTemplates<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT, serializedfilescreentemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateManager_Impl::ExportTemplates(this, core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                    Ok(ok__) => {
                        serializedfilescreentemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportTemplates<Identity: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serializedfilescreentemplates: *mut core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Variant::VARIANT, filescreentemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmFileScreenTemplateManager_Impl::ImportTemplates(this, core::mem::transmute(&serializedfilescreentemplates), core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                    Ok(ok__) => {
                        filescreentemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateTemplate: CreateTemplate::<Identity, OFFSET>,
            GetTemplate: GetTemplate::<Identity, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmFileScreenTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmMutableCollection, IFsrmMutableCollection_Vtbl, 0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmMutableCollection {
    type Target = IFsrmCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmMutableCollection, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmMutableCollection {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, item: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(item)).ok() }
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn RemoveById(&self, id: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveById)(windows_core::Interface::as_raw(self), core::mem::transmute(id)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IFsrmMutableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmMutableCollection_Impl: IFsrmCollection_Impl {
    fn Add(&self, item: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn RemoveById(&self, id: &windows_core::GUID) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmMutableCollection_Vtbl {
    pub const fn new<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::Add(this, core::mem::transmute(&item)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn RemoveById<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::RemoveById(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmMutableCollection_Impl::Clone(this) {
                    Ok(ok__) => {
                        collection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmCollection_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveById: RemoveById::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmMutableCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmMutableCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmObject, IFsrmObject_Vtbl, 0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmObject {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmObject_Impl: super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::GUID>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmObject_Vtbl {
    pub const fn new<Identity: IFsrmObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::Id(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::Commit(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmObject {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPathMapper, IFsrmPathMapper_Vtbl, 0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPathMapper {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPathMapper, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPathMapper {
    pub unsafe fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharePathsForLocalPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localpath), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPathMapper_Impl: super::super::System::Com::IDispatch_Impl {
    fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPathMapper_Vtbl {
    pub const fn new<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharePathsForLocalPath<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpath: *mut core::ffi::c_void, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPathMapper_Impl::GetSharePathsForLocalPath(this, core::mem::transmute(&localpath)) {
                    Ok(ok__) => {
                        sharepaths.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPathMapper as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPathMapper {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleConnector, IFsrmPipelineModuleConnector_Vtbl, 0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleConnector {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleConnector, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleConnector {
    pub unsafe fn ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleImplementation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ModuleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostingUserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostingProcessPid(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostingProcessPid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Bind<P0, P1>(&self, moduledefinition: P0, moduleimplementation: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
        P1: windows_core::Param<IFsrmPipelineModuleImplementation>,
    {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), moduleimplementation.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ModuleImplementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostingUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostingProcessPid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleConnector_Impl: super::super::System::Com::IDispatch_Impl {
    fn ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation>;
    fn ModuleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HostingProcessPid(&self) -> windows_core::Result<i32>;
    fn Bind(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>, moduleimplementation: windows_core::Ref<IFsrmPipelineModuleImplementation>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPipelineModuleConnector_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleImplementation<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipelinemoduleimplementation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::ModuleImplementation(this) {
                    Ok(ok__) => {
                        pipelinemoduleimplementation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModuleName<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::ModuleName(this) {
                    Ok(ok__) => {
                        username.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostingUserAccount<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::HostingUserAccount(this) {
                    Ok(ok__) => {
                        useraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostingProcessPid<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::HostingProcessPid(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Bind<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinition: *mut core::ffi::c_void, moduleimplementation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleConnector_Impl::Bind(this, core::mem::transmute_copy(&moduledefinition), core::mem::transmute_copy(&moduleimplementation)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ModuleImplementation: ModuleImplementation::<Identity, OFFSET>,
            ModuleName: ModuleName::<Identity, OFFSET>,
            HostingUserAccount: HostingUserAccount::<Identity, OFFSET>,
            HostingProcessPid: HostingProcessPid::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleConnector {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleDefinition, IFsrmPipelineModuleDefinition_Vtbl, 0x515c1277_2c81_440e_8fcf_367921ed4f59);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleDefinition {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModuleClsid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduleclsid)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Company(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Company)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCompany(&self, company: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompany)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(company)).ok() }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(version)).ok() }
    }
    pub unsafe fn ModuleType(&self) -> windows_core::Result<FsrmPipelineModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn NeedsFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNeedsFileContent)(windows_core::Interface::as_raw(self), needsfilecontent).ok() }
    }
    pub unsafe fn Account(&self) -> windows_core::Result<FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), retrievalaccount).ok() }
    }
    pub unsafe fn SupportedExtensions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSupportedExtensions)(windows_core::Interface::as_raw(self), supportedextensions).ok() }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub ModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPipelineModuleType) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmAccountType) -> windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmAccountType) -> windows_core::HRESULT,
    pub SupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetSupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleDefinition_Impl: IFsrmObject_Impl {
    fn ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCompany(&self, company: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ModuleType(&self) -> windows_core::Result<FsrmPipelineModuleType>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NeedsFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsFileContent(&self, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Account(&self) -> windows_core::Result<FsrmAccountType>;
    fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> windows_core::Result<()>;
    fn SupportedExtensions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPipelineModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::ModuleClsid(this) {
                    Ok(ok__) => {
                        moduleclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetModuleClsid(this, core::mem::transmute(&moduleclsid)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Company<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Company(this) {
                    Ok(ok__) => {
                        company.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompany<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetCompany(this, core::mem::transmute(&company)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetVersion(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn ModuleType<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::ModuleType(this) {
                    Ok(ok__) => {
                        moduletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn NeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::NeedsFileContent(this) {
                    Ok(ok__) => {
                        needsfilecontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetNeedsFileContent(this, core::mem::transmute_copy(&needsfilecontent)).into()
            }
        }
        unsafe extern "system" fn Account<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Account(this) {
                    Ok(ok__) => {
                        retrievalaccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccount<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetAccount(this, core::mem::transmute_copy(&retrievalaccount)).into()
            }
        }
        unsafe extern "system" fn SupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::SupportedExtensions(this) {
                    Ok(ok__) => {
                        supportedextensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetSupportedExtensions(this, core::mem::transmute_copy(&supportedextensions)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            ModuleClsid: ModuleClsid::<Identity, OFFSET>,
            SetModuleClsid: SetModuleClsid::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Company: Company::<Identity, OFFSET>,
            SetCompany: SetCompany::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            ModuleType: ModuleType::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            NeedsFileContent: NeedsFileContent::<Identity, OFFSET>,
            SetNeedsFileContent: SetNeedsFileContent::<Identity, OFFSET>,
            Account: Account::<Identity, OFFSET>,
            SetAccount: SetAccount::<Identity, OFFSET>,
            SupportedExtensions: SupportedExtensions::<Identity, OFFSET>,
            SetSupportedExtensions: SetSupportedExtensions::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleImplementation, IFsrmPipelineModuleImplementation_Vtbl, 0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleImplementation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleImplementation {
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> windows_core::Result<IFsrmPipelineModuleConnector>
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnUnload(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPipelineModuleImplementation_Impl: super::super::System::Com::IDispatch_Impl {
    fn OnLoad(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>) -> windows_core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPipelineModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLoad<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinition: *mut core::ffi::c_void, moduleconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleImplementation_Impl::OnLoad(this, core::mem::transmute_copy(&moduledefinition)) {
                    Ok(ok__) => {
                        moduleconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnUnload<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleImplementation_Impl::OnUnload(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OnLoad: OnLoad::<Identity, OFFSET>,
            OnUnload: OnUnload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmProperty, IFsrmProperty_Vtbl, 0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Sources(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropertyFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmProperty_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Sources(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyFlags(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmProperty_Vtbl {
    pub const fn new<Identity: IFsrmProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sources<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Sources(this) {
                    Ok(ok__) => {
                        sources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyFlags<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::PropertyFlags(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            Sources: Sources::<Identity, OFFSET>,
            PropertyFlags: PropertyFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmProperty {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyBag, IFsrmPropertyBag_Vtbl, 0x774589d1_d300_4f7a_9a24_f7b766800250);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyBag {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RelativePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativeNamespaceRoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn FileId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ParentDirectoryId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentDirectoryId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Size(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SizeAllocated(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeAllocated)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreationTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LastAccessTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LastModificationTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OwnerSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FilePropertyNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilePropertyNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Messages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Messages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropertyBagFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyBagFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(message)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileStreamInterface)(windows_core::Interface::as_raw(self), accessmode, interfacetype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativeNamespaceRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub FileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    FileId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ParentDirectoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Size: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SizeAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreationTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LastModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OwnerSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Messages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertyBagFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileStreamingMode, FsrmFileStreamingInterfaceType, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFileStreamInterface: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyBag_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RelativePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeIndex(&self) -> windows_core::Result<u32>;
    fn FileId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ParentDirectoryId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Size(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SizeAllocated(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CreationTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn LastAccessTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn LastModificationTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Attributes(&self) -> windows_core::Result<u32>;
    fn OwnerSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FilePropertyNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Messages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyBagFlags(&self) -> windows_core::Result<u32>;
    fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyBag_Vtbl {
    pub const fn new<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativePath<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::RelativePath(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        volumename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativenamespaceroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::RelativeNamespaceRoot(this) {
                    Ok(ok__) => {
                        relativenamespaceroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeIndex<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumeid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::VolumeIndex(this) {
                    Ok(ok__) => {
                        volumeid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileid: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::FileId(this) {
                    Ok(ok__) => {
                        fileid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParentDirectoryId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentdirectoryid: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::ParentDirectoryId(this) {
                    Ok(ok__) => {
                        parentdirectoryid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Size(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeAllocated<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizeallocated: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::SizeAllocated(this) {
                    Ok(ok__) => {
                        sizeallocated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creationtime: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        creationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastAccessTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastaccesstime: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::LastAccessTime(this) {
                    Ok(ok__) => {
                        lastaccesstime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastModificationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodificationtime: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::LastModificationTime(this) {
                    Ok(ok__) => {
                        lastmodificationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Attributes<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OwnerSid<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ownersid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::OwnerSid(this) {
                    Ok(ok__) => {
                        ownersid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilePropertyNames<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::FilePropertyNames(this) {
                    Ok(ok__) => {
                        filepropertynames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Messages<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Messages(this) {
                    Ok(ok__) => {
                        messages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyBagFlags<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::PropertyBagFlags(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, fileproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::GetFileProperty(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        fileproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyBag_Impl::SetFileProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn AddMessage<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyBag_Impl::AddMessage(this, core::mem::transmute(&message)).into()
            }
        }
        unsafe extern "system" fn GetFileStreamInterface<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::GetFileStreamInterface(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&interfacetype)) {
                    Ok(ok__) => {
                        pstreaminterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            RelativePath: RelativePath::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            RelativeNamespaceRoot: RelativeNamespaceRoot::<Identity, OFFSET>,
            VolumeIndex: VolumeIndex::<Identity, OFFSET>,
            FileId: FileId::<Identity, OFFSET>,
            ParentDirectoryId: ParentDirectoryId::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SizeAllocated: SizeAllocated::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            LastAccessTime: LastAccessTime::<Identity, OFFSET>,
            LastModificationTime: LastModificationTime::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
            OwnerSid: OwnerSid::<Identity, OFFSET>,
            FilePropertyNames: FilePropertyNames::<Identity, OFFSET>,
            Messages: Messages::<Identity, OFFSET>,
            PropertyBagFlags: PropertyBagFlags::<Identity, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            GetFileStreamInterface: GetFileStreamInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyBag as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyBag2, IFsrmPropertyBag2_Vtbl, 0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyBag2 {
    type Target = IFsrmPropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPropertyBag);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFieldValue)(windows_core::Interface::as_raw(self), field, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUntrustedInFileProperties(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUntrustedInFileProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFieldValue: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyBagField, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFieldValue: usize,
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyBag2_Impl: IFsrmPropertyBag_Impl {
    fn GetFieldValue(&self, field: FsrmPropertyBagField) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetUntrustedInFileProperties(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyBag2_Vtbl {
    pub const fn new<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFieldValue<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag2_Impl::GetFieldValue(this, core::mem::transmute_copy(&field)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, props: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag2_Impl::GetUntrustedInFileProperties(this) {
                    Ok(ok__) => {
                        props.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmPropertyBag_Vtbl::new::<Identity, OFFSET>(),
            GetFieldValue: GetFieldValue::<Identity, OFFSET>,
            GetUntrustedInFileProperties: GetUntrustedInFileProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyBag2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyCondition, IFsrmPropertyCondition_Vtbl, 0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyCondition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyCondition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyCondition {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmPropertyConditionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyCondition_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<FsrmPropertyConditionType>;
    fn SetType(&self, r#type: FsrmPropertyConditionType) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyCondition_Vtbl {
    pub const fn new<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyCondition_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyCondition_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyCondition_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: FsrmPropertyConditionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyCondition_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyCondition_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyCondition_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyCondition_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyCondition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyCondition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinition, IFsrmPropertyDefinition_Vtbl, 0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinition {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmPropertyDefinitionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn PossibleValues(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PossibleValues)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPossibleValues)(windows_core::Interface::as_raw(self), possiblevalues).ok() }
    }
    pub unsafe fn ValueDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValueDescriptions)(windows_core::Interface::as_raw(self), valuedescriptions).ok() }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    pub PossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinition_Impl: IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<FsrmPropertyDefinitionType>;
    fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> windows_core::Result<()>;
    fn PossibleValues(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn ValueDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn PossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::PossibleValues(this) {
                    Ok(ok__) => {
                        possiblevalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetPossibleValues(this, core::mem::transmute_copy(&possiblevalues)).into()
            }
        }
        unsafe extern "system" fn ValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::ValueDescriptions(this) {
                    Ok(ok__) => {
                        valuedescriptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetValueDescriptions(this, core::mem::transmute_copy(&valuedescriptions)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            PossibleValues: PossibleValues::<Identity, OFFSET>,
            SetPossibleValues: SetPossibleValues::<Identity, OFFSET>,
            ValueDescriptions: ValueDescriptions::<Identity, OFFSET>,
            SetValueDescriptions: SetValueDescriptions::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinition2, IFsrmPropertyDefinition2_Vtbl, 0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinition2 {
    type Target = IFsrmPropertyDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPropertyDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition2 {
    pub unsafe fn PropertyDefinitionFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyDefinitionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn AppliesTo(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppliesTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ValueDefinitions(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppliesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ValueDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinition2_Impl: IFsrmPropertyDefinition_Impl {
    fn PropertyDefinitionFlags(&self) -> windows_core::Result<i32>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AppliesTo(&self) -> windows_core::Result<i32>;
    fn ValueDefinitions(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyDefinition2_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyDefinitionFlags<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinitionflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::PropertyDefinitionFlags(this) {
                    Ok(ok__) => {
                        propertydefinitionflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition2_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn AppliesTo<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appliesto: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::AppliesTo(this) {
                    Ok(ok__) => {
                        appliesto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValueDefinitions<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::ValueDefinitions(this) {
                    Ok(ok__) => {
                        valuedefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmPropertyDefinition_Vtbl::new::<Identity, OFFSET>(),
            PropertyDefinitionFlags: PropertyDefinitionFlags::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            AppliesTo: AppliesTo::<Identity, OFFSET>,
            ValueDefinitions: ValueDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinition2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinitionValue, IFsrmPropertyDefinitionValue_Vtbl, 0xe946d148_bd67_4178_8e22_1c44925ed710);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinitionValue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinitionValue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinitionValue {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UniqueID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UniqueID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmPropertyDefinitionValue_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmPropertyDefinitionValue_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UniqueID<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uniqueid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::UniqueID(this) {
                    Ok(ok__) => {
                        uniqueid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            UniqueID: UniqueID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinitionValue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuota, IFsrmQuota_Vtbl, 0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuota {
    type Target = IFsrmQuotaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuota, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuota {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn QuotaUsed(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuotaUsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn QuotaPeakUsage(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuotaPeakUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuotaPeakUsageTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResetPeakUsage(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetPeakUsage)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RefreshUsageProperties(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshUsageProperties)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub QuotaUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    QuotaUsed: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub QuotaPeakUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    QuotaPeakUsage: usize,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuota_Impl: IFsrmQuotaObject_Impl {
    fn QuotaUsed(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn QuotaPeakUsage(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn QuotaPeakUsageTime(&self) -> windows_core::Result<f64>;
    fn ResetPeakUsage(&self) -> windows_core::Result<()>;
    fn RefreshUsageProperties(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuota_Vtbl {
    pub const fn new<Identity: IFsrmQuota_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QuotaUsed<Identity: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, used: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuota_Impl::QuotaUsed(this) {
                    Ok(ok__) => {
                        used.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuotaPeakUsage<Identity: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peakusage: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuota_Impl::QuotaPeakUsage(this) {
                    Ok(ok__) => {
                        peakusage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuotaPeakUsageTime<Identity: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peakusagedatetime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuota_Impl::QuotaPeakUsageTime(this) {
                    Ok(ok__) => {
                        peakusagedatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetPeakUsage<Identity: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuota_Impl::ResetPeakUsage(this).into()
            }
        }
        unsafe extern "system" fn RefreshUsageProperties<Identity: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuota_Impl::RefreshUsageProperties(this).into()
            }
        }
        Self {
            base__: IFsrmQuotaObject_Vtbl::new::<Identity, OFFSET>(),
            QuotaUsed: QuotaUsed::<Identity, OFFSET>,
            QuotaPeakUsage: QuotaPeakUsage::<Identity, OFFSET>,
            QuotaPeakUsageTime: QuotaPeakUsageTime::<Identity, OFFSET>,
            ResetPeakUsage: ResetPeakUsage::<Identity, OFFSET>,
            RefreshUsageProperties: RefreshUsageProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuota as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmQuotaBase as windows_core::Interface>::IID || iid == &<IFsrmQuotaObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuota {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaBase, IFsrmQuotaBase_Vtbl, 0x1568a795_3924_4118_b74b_68d8f0fa5daf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaBase {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaBase, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaBase {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn QuotaLimit(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuotaLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuotaLimit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(quotalimit)).ok() }
    }
    pub unsafe fn QuotaFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuotaFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuotaFlags)(windows_core::Interface::as_raw(self), quotaflags).ok() }
    }
    pub unsafe fn Thresholds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Thresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddThreshold)(windows_core::Interface::as_raw(self), threshold).ok() }
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteThreshold)(windows_core::Interface::as_raw(self), threshold).ok() }
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyThreshold)(windows_core::Interface::as_raw(self), threshold, newthreshold).ok() }
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateThresholdAction)(windows_core::Interface::as_raw(self), threshold, actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumThresholdActions)(windows_core::Interface::as_raw(self), threshold, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub QuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    QuotaLimit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetQuotaLimit: usize,
    pub QuotaFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Thresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub AddThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub CreateThresholdAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumThresholdActions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaBase_Impl: IFsrmObject_Impl {
    fn QuotaLimit(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetQuotaLimit(&self, quotalimit: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn QuotaFlags(&self) -> windows_core::Result<i32>;
    fn SetQuotaFlags(&self, quotaflags: i32) -> windows_core::Result<()>;
    fn Thresholds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddThreshold(&self, threshold: i32) -> windows_core::Result<()>;
    fn DeleteThreshold(&self, threshold: i32) -> windows_core::Result<()>;
    fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> windows_core::Result<()>;
    fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction>;
    fn EnumThresholdActions(&self, threshold: i32) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaBase_Vtbl {
    pub const fn new<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QuotaLimit<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotalimit: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaBase_Impl::QuotaLimit(this) {
                    Ok(ok__) => {
                        quotalimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotalimit: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaBase_Impl::SetQuotaLimit(this, core::mem::transmute(&quotalimit)).into()
            }
        }
        unsafe extern "system" fn QuotaFlags<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotaflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaBase_Impl::QuotaFlags(this) {
                    Ok(ok__) => {
                        quotaflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuotaFlags<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotaflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaBase_Impl::SetQuotaFlags(this, core::mem::transmute_copy(&quotaflags)).into()
            }
        }
        unsafe extern "system" fn Thresholds<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaBase_Impl::Thresholds(this) {
                    Ok(ok__) => {
                        thresholds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddThreshold<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threshold: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaBase_Impl::AddThreshold(this, core::mem::transmute_copy(&threshold)).into()
            }
        }
        unsafe extern "system" fn DeleteThreshold<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threshold: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaBase_Impl::DeleteThreshold(this, core::mem::transmute_copy(&threshold)).into()
            }
        }
        unsafe extern "system" fn ModifyThreshold<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threshold: i32, newthreshold: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaBase_Impl::ModifyThreshold(this, core::mem::transmute_copy(&threshold), core::mem::transmute_copy(&newthreshold)).into()
            }
        }
        unsafe extern "system" fn CreateThresholdAction<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaBase_Impl::CreateThresholdAction(this, core::mem::transmute_copy(&threshold), core::mem::transmute_copy(&actiontype)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumThresholdActions<Identity: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threshold: i32, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaBase_Impl::EnumThresholdActions(this, core::mem::transmute_copy(&threshold)) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            QuotaLimit: QuotaLimit::<Identity, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, OFFSET>,
            QuotaFlags: QuotaFlags::<Identity, OFFSET>,
            SetQuotaFlags: SetQuotaFlags::<Identity, OFFSET>,
            Thresholds: Thresholds::<Identity, OFFSET>,
            AddThreshold: AddThreshold::<Identity, OFFSET>,
            DeleteThreshold: DeleteThreshold::<Identity, OFFSET>,
            ModifyThreshold: ModifyThreshold::<Identity, OFFSET>,
            CreateThresholdAction: CreateThresholdAction::<Identity, OFFSET>,
            EnumThresholdActions: EnumThresholdActions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaBase as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaBase {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaManager, IFsrmQuotaManager_Vtbl, 0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManager {
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAutoApplyQuota(&self, quotatemplatename: &windows_core::BSTR, path: &windows_core::BSTR) -> windows_core::Result<IFsrmAutoApplyQuota> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAutoApplyQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(quotatemplatename), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAutoApplyQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmAutoApplyQuota> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutoApplyQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRestrictiveQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictiveQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumQuotas)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumAutoApplyQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAutoApplyQuotas)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumEffectiveQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumEffectiveQuotas)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Scan(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpath)).ok() }
    }
    pub unsafe fn CreateQuotaCollection(&self) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuotaCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub CreateQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAutoApplyQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutoApplyQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictiveQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEffectiveQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateQuotaCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota>;
    fn CreateAutoApplyQuota(&self, quotatemplatename: &windows_core::BSTR, path: &windows_core::BSTR) -> windows_core::Result<IFsrmAutoApplyQuota>;
    fn GetQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota>;
    fn GetAutoApplyQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmAutoApplyQuota>;
    fn GetRestrictiveQuota(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsrmQuota>;
    fn EnumQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn EnumAutoApplyQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn EnumEffectiveQuotas(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn Scan(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CreateQuotaCollection(&self) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaManager_Vtbl {
    pub const fn new<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActionVariables<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::ActionVariables(this) {
                    Ok(ok__) => {
                        variables.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::ActionVariableDescriptions(this) {
                    Ok(ok__) => {
                        descriptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateQuota<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, quota: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::CreateQuota(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        quota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAutoApplyQuota<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplatename: *mut core::ffi::c_void, path: *mut core::ffi::c_void, quota: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::CreateAutoApplyQuota(this, core::mem::transmute(&quotatemplatename), core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        quota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQuota<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, quota: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::GetQuota(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        quota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAutoApplyQuota<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, quota: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::GetAutoApplyQuota(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        quota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictiveQuota<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, quota: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::GetRestrictiveQuota(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        quota.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumQuotas<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::EnumQuotas(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        quotas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumAutoApplyQuotas<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::EnumAutoApplyQuotas(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        quotas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumEffectiveQuotas<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::EnumEffectiveQuotas(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        quotas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Scan<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaManager_Impl::Scan(this, core::mem::transmute(&strpath)).into()
            }
        }
        unsafe extern "system" fn CreateQuotaCollection<Identity: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManager_Impl::CreateQuotaCollection(this) {
                    Ok(ok__) => {
                        collection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, OFFSET>,
            CreateQuota: CreateQuota::<Identity, OFFSET>,
            CreateAutoApplyQuota: CreateAutoApplyQuota::<Identity, OFFSET>,
            GetQuota: GetQuota::<Identity, OFFSET>,
            GetAutoApplyQuota: GetAutoApplyQuota::<Identity, OFFSET>,
            GetRestrictiveQuota: GetRestrictiveQuota::<Identity, OFFSET>,
            EnumQuotas: EnumQuotas::<Identity, OFFSET>,
            EnumAutoApplyQuotas: EnumAutoApplyQuotas::<Identity, OFFSET>,
            EnumEffectiveQuotas: EnumEffectiveQuotas::<Identity, OFFSET>,
            Scan: Scan::<Identity, OFFSET>,
            CreateQuotaCollection: CreateQuotaCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaManagerEx, IFsrmQuotaManagerEx_Vtbl, 0x4846cb01_d430_494f_abb4_b1054999fb09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaManagerEx {
    type Target = IFsrmQuotaManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaManagerEx, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmQuotaManager);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManagerEx {
    pub unsafe fn IsAffectedByQuota(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAffectedByQuota)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), options, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerEx_Vtbl {
    pub base__: IFsrmQuotaManager_Vtbl,
    pub IsAffectedByQuota: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsrmEnumOptions, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaManagerEx_Impl: IFsrmQuotaManager_Impl {
    fn IsAffectedByQuota(&self, path: &windows_core::BSTR, options: FsrmEnumOptions) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaManagerEx_Vtbl {
    pub const fn new<Identity: IFsrmQuotaManagerEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsAffectedByQuota<Identity: IFsrmQuotaManagerEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, options: FsrmEnumOptions, affected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaManagerEx_Impl::IsAffectedByQuota(this, core::mem::transmute(&path), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        affected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFsrmQuotaManager_Vtbl::new::<Identity, OFFSET>(), IsAffectedByQuota: IsAffectedByQuota::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaManagerEx as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmQuotaManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaManagerEx {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaObject, IFsrmQuotaObject_Vtbl, 0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaObject {
    type Target = IFsrmQuotaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaObject, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaObject {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceTemplateName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchesSourceTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ApplyTemplate(&self, quotatemplatename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplyTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(quotatemplatename)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObject_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaObject_Impl: IFsrmQuotaBase_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ApplyTemplate(&self, quotatemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaObject_Vtbl {
    pub const fn new<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaObject_Impl::Path(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserSid<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usersid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaObject_Impl::UserSid(this) {
                    Ok(ok__) => {
                        usersid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserAccount<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaObject_Impl::UserAccount(this) {
                    Ok(ok__) => {
                        useraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SourceTemplateName<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplatename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaObject_Impl::SourceTemplateName(this) {
                    Ok(ok__) => {
                        quotatemplatename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaObject_Impl::MatchesSourceTemplate(this) {
                    Ok(ok__) => {
                        matches.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplyTemplate<Identity: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplatename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaObject_Impl::ApplyTemplate(this, core::mem::transmute(&quotatemplatename)).into()
            }
        }
        Self {
            base__: IFsrmQuotaBase_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            UserSid: UserSid::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmQuotaBase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaObject {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplate, IFsrmQuotaTemplate_Vtbl, 0xa2efab31_295e_46bb_b976_e86d58b52e8b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplate {
    type Target = IFsrmQuotaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplate, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplate {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn CopyTemplate(&self, quotatemplatename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(quotatemplatename)).ok() }
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplate_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplate_Impl: IFsrmQuotaBase_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyTemplate(&self, quotatemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaTemplate_Vtbl {
    pub const fn new<Identity: IFsrmQuotaTemplate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplate_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaTemplate_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn CopyTemplate<Identity: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplatename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaTemplate_Impl::CopyTemplate(this, core::mem::transmute(&quotatemplatename)).into()
            }
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplate_Impl::CommitAndUpdateDerived(this, core::mem::transmute_copy(&commitoptions), core::mem::transmute_copy(&applyoptions)) {
                    Ok(ok__) => {
                        derivedobjectsresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmQuotaBase_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplate as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmQuotaBase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaTemplate {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplateImported, IFsrmQuotaTemplateImported_Vtbl, 0x9a2bf113_a329_44cc_809a_5c00fce8da40);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplateImported {
    type Target = IFsrmQuotaTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplateImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaTemplate);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImported_Vtbl {
    pub base__: IFsrmQuotaTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplateImported_Impl: IFsrmQuotaTemplate_Impl {
    fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaTemplateImported_Vtbl {
    pub const fn new<Identity: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OverwriteOnCommit<Identity: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateImported_Impl::OverwriteOnCommit(this) {
                    Ok(ok__) => {
                        overwrite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmQuotaTemplateImported_Impl::SetOverwriteOnCommit(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        Self {
            base__: IFsrmQuotaTemplate_Vtbl::new::<Identity, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateImported as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmQuotaBase as windows_core::Interface>::IID || iid == &<IFsrmQuotaTemplate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplateManager, IFsrmQuotaTemplateManager_Vtbl, 0x4173ac41_172d_4d52_963c_fdc7e415f717);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplateManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplateManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateManager {
    pub unsafe fn CreateTemplate(&self) -> windows_core::Result<IFsrmQuotaTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTemplate(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmQuotaTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemplate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumTemplates)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute(quotatemplatenamesarray), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportTemplates(&self, serializedquotatemplates: &windows_core::BSTR, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serializedquotatemplates), core::mem::transmute(quotatemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CreateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ExportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportTemplates: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmQuotaTemplateManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&self) -> windows_core::Result<IFsrmQuotaTemplate>;
    fn GetTemplate(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmQuotaTemplate>;
    fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn ImportTemplates(&self, serializedquotatemplates: &windows_core::BSTR, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmQuotaTemplateManager_Vtbl {
    pub const fn new<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTemplate<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateManager_Impl::CreateTemplate(this) {
                    Ok(ok__) => {
                        quotatemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTemplate<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, quotatemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateManager_Impl::GetTemplate(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        quotatemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumTemplates<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateManager_Impl::EnumTemplates(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        quotatemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportTemplates<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT, serializedquotatemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateManager_Impl::ExportTemplates(this, core::mem::transmute_copy(&quotatemplatenamesarray)) {
                    Ok(ok__) => {
                        serializedquotatemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportTemplates<Identity: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serializedquotatemplates: *mut core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Variant::VARIANT, quotatemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmQuotaTemplateManager_Impl::ImportTemplates(this, core::mem::transmute(&serializedquotatemplates), core::mem::transmute_copy(&quotatemplatenamesarray)) {
                    Ok(ok__) => {
                        quotatemplates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateTemplate: CreateTemplate::<Identity, OFFSET>,
            GetTemplate: GetTemplate::<Identity, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmQuotaTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReport, IFsrmReport_Vtbl, 0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReport {
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmReportType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastGeneratedFileNamePrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), filter, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetFilter(&self, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), filter, core::mem::transmute_copy(filtervalue)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportType) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportFilter, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFilter: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportFilter, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetFilter: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReport_Impl: super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<FsrmReportType>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LastGeneratedFileNamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFilter(&self, filter: FsrmReportFilter) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(&self, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmReport_Vtbl {
    pub const fn new<Identity: IFsrmReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *mut FsrmReportType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReport_Impl::Type(this) {
                    Ok(ok__) => {
                        reporttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReport_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReport_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReport_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReport_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn LastGeneratedFileNamePrefix<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReport_Impl::LastGeneratedFileNamePrefix(this) {
                    Ok(ok__) => {
                        prefix.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilter<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReport_Impl::GetFilter(this, core::mem::transmute_copy(&filter)) {
                    Ok(ok__) => {
                        filtervalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: FsrmReportFilter, filtervalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReport_Impl::SetFilter(this, core::mem::transmute_copy(&filter), core::mem::transmute(&filtervalue)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReport_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            LastGeneratedFileNamePrefix: LastGeneratedFileNamePrefix::<Identity, OFFSET>,
            GetFilter: GetFilter::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmReport as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmReport {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportJob, IFsrmReportJob_Vtbl, 0x38e87280_715c_4c7d_a280_ea1651a19fef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportJob {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportJob, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportJob {
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname)).ok() }
    }
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok() }
    }
    pub unsafe fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Formats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormats)(windows_core::Interface::as_raw(self), formats).ok() }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
    pub unsafe fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastRun(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LastGeneratedInDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastGeneratedInDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumReports(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumReports)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> windows_core::Result<IFsrmReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), context).ok() }
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub Formats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastGeneratedInDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateReport: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportJob_Impl: IFsrmObject_Impl {
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus>;
    fn LastRun(&self) -> windows_core::Result<f64>;
    fn LastError(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastGeneratedInDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumReports(&self) -> windows_core::Result<IFsrmCollection>;
    fn CreateReport(&self, reporttype: FsrmReportType) -> windows_core::Result<IFsrmReport>;
    fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmReportJob_Vtbl {
    pub const fn new<Identity: IFsrmReportJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Task<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::Task(this) {
                    Ok(ok__) => {
                        taskname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::SetTask(this, core::mem::transmute(&taskname)).into()
            }
        }
        unsafe extern "system" fn NamespaceRoots<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::NamespaceRoots(this) {
                    Ok(ok__) => {
                        namespaceroots.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::SetNamespaceRoots(this, core::mem::transmute_copy(&namespaceroots)).into()
            }
        }
        unsafe extern "system" fn Formats<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::Formats(this) {
                    Ok(ok__) => {
                        formats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormats<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::SetFormats(this, core::mem::transmute_copy(&formats)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn RunningStatus<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::RunningStatus(this) {
                    Ok(ok__) => {
                        runningstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastRun<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastrun: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::LastRun(this) {
                    Ok(ok__) => {
                        lastrun.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastError<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lasterror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::LastError(this) {
                    Ok(ok__) => {
                        lasterror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastGeneratedInDirectory<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::LastGeneratedInDirectory(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumReports<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::EnumReports(this) {
                    Ok(ok__) => {
                        reports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateReport<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: FsrmReportType, report: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::CreateReport(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        report.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Run<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: FsrmReportGenerationContext) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::Run(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportJob_Impl::WaitForCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportJob_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, OFFSET>,
            Formats: Formats::<Identity, OFFSET>,
            SetFormats: SetFormats::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
            RunningStatus: RunningStatus::<Identity, OFFSET>,
            LastRun: LastRun::<Identity, OFFSET>,
            LastError: LastError::<Identity, OFFSET>,
            LastGeneratedInDirectory: LastGeneratedInDirectory::<Identity, OFFSET>,
            EnumReports: EnumReports::<Identity, OFFSET>,
            CreateReport: CreateReport::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmReportJob as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmReportJob {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportManager, IFsrmReportManager_Vtbl, 0x27b899fe_6ffa_4481_a184_d3daade8a02b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportManager {
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumReportJobs)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateReportJob(&self) -> windows_core::Result<IFsrmReportJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReportJob)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReportJob(&self, taskname: &windows_core::BSTR) -> windows_core::Result<IFsrmReportJob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputDirectory)(windows_core::Interface::as_raw(self), context, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOutputDirectory(&self, context: FsrmReportGenerationContext, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputDirectory)(windows_core::Interface::as_raw(self), context, core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFilterValidForReportType)(windows_core::Interface::as_raw(self), reporttype, filter, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultFilter)(windows_core::Interface::as_raw(self), reporttype, filter, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultFilter)(windows_core::Interface::as_raw(self), reporttype, filter, core::mem::transmute_copy(filtervalue)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportSizeLimit)(windows_core::Interface::as_raw(self), limit, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetReportSizeLimit(&self, limit: FsrmReportLimit, limitvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReportSizeLimit)(windows_core::Interface::as_raw(self), limit, core::mem::transmute_copy(limitvalue)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnumReportJobs: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateReportJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReportJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFilterValidForReportType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetReportSizeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportLimit, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetReportSizeLimit: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetReportSizeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportLimit, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetReportSizeLimit: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn EnumReportJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection>;
    fn CreateReportJob(&self) -> windows_core::Result<IFsrmReportJob>;
    fn GetReportJob(&self, taskname: &windows_core::BSTR) -> windows_core::Result<IFsrmReportJob>;
    fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> windows_core::Result<windows_core::BSTR>;
    fn SetOutputDirectory(&self, context: FsrmReportGenerationContext, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetReportSizeLimit(&self, limit: FsrmReportLimit, limitvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmReportManager_Vtbl {
    pub const fn new<Identity: IFsrmReportManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumReportJobs<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::EnumReportJobs(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        reportjobs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateReportJob<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::CreateReportJob(this) {
                    Ok(ok__) => {
                        reportjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReportJob<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void, reportjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::GetReportJob(this, core::mem::transmute(&taskname)) {
                    Ok(ok__) => {
                        reportjob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputDirectory<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::GetOutputDirectory(this, core::mem::transmute_copy(&context)) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutputDirectory<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportManager_Impl::SetOutputDirectory(this, core::mem::transmute_copy(&context), core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn IsFilterValidForReportType<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::IsFilterValidForReportType(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&filter)) {
                    Ok(ok__) => {
                        valid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::GetDefaultFilter(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&filter)) {
                    Ok(ok__) => {
                        filtervalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultFilter<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportManager_Impl::SetDefaultFilter(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&filter), core::mem::transmute(&filtervalue)).into()
            }
        }
        unsafe extern "system" fn GetReportSizeLimit<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmReportManager_Impl::GetReportSizeLimit(this, core::mem::transmute_copy(&limit)) {
                    Ok(ok__) => {
                        limitvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportSizeLimit<Identity: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, limit: FsrmReportLimit, limitvalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportManager_Impl::SetReportSizeLimit(this, core::mem::transmute_copy(&limit), core::mem::transmute(&limitvalue)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumReportJobs: EnumReportJobs::<Identity, OFFSET>,
            CreateReportJob: CreateReportJob::<Identity, OFFSET>,
            GetReportJob: GetReportJob::<Identity, OFFSET>,
            GetOutputDirectory: GetOutputDirectory::<Identity, OFFSET>,
            SetOutputDirectory: SetOutputDirectory::<Identity, OFFSET>,
            IsFilterValidForReportType: IsFilterValidForReportType::<Identity, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, OFFSET>,
            SetDefaultFilter: SetDefaultFilter::<Identity, OFFSET>,
            GetReportSizeLimit: GetReportSizeLimit::<Identity, OFFSET>,
            SetReportSizeLimit: SetReportSizeLimit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmReportManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmReportManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportScheduler, IFsrmReportScheduler_Vtbl, 0x6879caf9_6617_4484_8719_71c3d8645f94);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportScheduler {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportScheduler, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportScheduler {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VerifyNamespaces)(windows_core::Interface::as_raw(self), core::mem::transmute(namespacessafearray)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateScheduleTask(&self, taskname: &windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateScheduleTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname), core::mem::transmute(namespacessafearray), core::mem::transmute_copy(serializedtask)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ModifyScheduleTask(&self, taskname: &windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyScheduleTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname), core::mem::transmute(namespacessafearray), core::mem::transmute_copy(serializedtask)).ok() }
    }
    pub unsafe fn DeleteScheduleTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteScheduleTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(taskname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportScheduler_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub VerifyNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    VerifyNamespaces: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateScheduleTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ModifyScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ModifyScheduleTask: usize,
    pub DeleteScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmReportScheduler_Impl: super::super::System::Com::IDispatch_Impl {
    fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn CreateScheduleTask(&self, taskname: &windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ModifyScheduleTask(&self, taskname: &windows_core::BSTR, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteScheduleTask(&self, taskname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmReportScheduler_Vtbl {
    pub const fn new<Identity: IFsrmReportScheduler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VerifyNamespaces<Identity: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespacessafearray: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportScheduler_Impl::VerifyNamespaces(this, core::mem::transmute_copy(&namespacessafearray)).into()
            }
        }
        unsafe extern "system" fn CreateScheduleTask<Identity: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportScheduler_Impl::CreateScheduleTask(this, core::mem::transmute(&taskname), core::mem::transmute_copy(&namespacessafearray), core::mem::transmute(&serializedtask)).into()
            }
        }
        unsafe extern "system" fn ModifyScheduleTask<Identity: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void, namespacessafearray: *const super::super::System::Variant::VARIANT, serializedtask: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportScheduler_Impl::ModifyScheduleTask(this, core::mem::transmute(&taskname), core::mem::transmute_copy(&namespacessafearray), core::mem::transmute(&serializedtask)).into()
            }
        }
        unsafe extern "system" fn DeleteScheduleTask<Identity: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmReportScheduler_Impl::DeleteScheduleTask(this, core::mem::transmute(&taskname)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            VerifyNamespaces: VerifyNamespaces::<Identity, OFFSET>,
            CreateScheduleTask: CreateScheduleTask::<Identity, OFFSET>,
            ModifyScheduleTask: ModifyScheduleTask::<Identity, OFFSET>,
            DeleteScheduleTask: DeleteScheduleTask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmReportScheduler as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmReportScheduler {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmRule, IFsrmRule_Vtbl, 0xcb0df960_16f5_4495_9079_3f9360d831df);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmRule {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmRule, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmRule {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn RuleType(&self) -> windows_core::Result<FsrmRuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleDefinitionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModuleDefinitionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduledefinitionname)).ok() }
    }
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok() }
    }
    pub unsafe fn RuleFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRuleFlags)(windows_core::Interface::as_raw(self), ruleflags).ok() }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LastModified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmRuleType) -> windows_core::HRESULT,
    pub ModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub RuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LastModified: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmRule_Impl: IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RuleType(&self) -> windows_core::Result<FsrmRuleType>;
    fn ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RuleFlags(&self) -> windows_core::Result<i32>;
    fn SetRuleFlags(&self, ruleflags: i32) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn LastModified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmRule_Vtbl {
    pub const fn new<Identity: IFsrmRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn RuleType<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: *mut FsrmRuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::RuleType(this) {
                    Ok(ok__) => {
                        ruletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::ModuleDefinitionName(this) {
                    Ok(ok__) => {
                        moduledefinitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetModuleDefinitionName(this, core::mem::transmute(&moduledefinitionname)).into()
            }
        }
        unsafe extern "system" fn NamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::NamespaceRoots(this) {
                    Ok(ok__) => {
                        namespaceroots.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetNamespaceRoots(this, core::mem::transmute_copy(&namespaceroots)).into()
            }
        }
        unsafe extern "system" fn RuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::RuleFlags(this) {
                    Ok(ok__) => {
                        ruleflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetRuleFlags(this, core::mem::transmute_copy(&ruleflags)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        unsafe extern "system" fn LastModified<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::LastModified(this) {
                    Ok(ok__) => {
                        lastmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            RuleType: RuleType::<Identity, OFFSET>,
            ModuleDefinitionName: ModuleDefinitionName::<Identity, OFFSET>,
            SetModuleDefinitionName: SetModuleDefinitionName::<Identity, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, OFFSET>,
            RuleFlags: RuleFlags::<Identity, OFFSET>,
            SetRuleFlags: SetRuleFlags::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            LastModified: LastModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmRule {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmSetting, IFsrmSetting_Vtbl, 0xf411d4fd_14be_4260_8c40_03b7c95e608a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmSetting {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmSetting, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmSetting {
    pub unsafe fn SmtpServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmtpServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSmtpServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(smtpserver)).ok() }
    }
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)).ok() }
    }
    pub unsafe fn AdminEmail(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminEmail)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAdminEmail)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(adminemail)).ok() }
    }
    pub unsafe fn DisableCommandLine(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableCommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisableCommandLine)(windows_core::Interface::as_raw(self), disablecommandline).ok() }
    }
    pub unsafe fn EnableScreeningAudit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableScreeningAudit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableScreeningAudit)(windows_core::Interface::as_raw(self), enablescreeningaudit).ok() }
    }
    pub unsafe fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EmailTest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)).ok() }
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, delaytimeminutes).ok() }
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EmailTest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, i32) -> windows_core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmSetting_Impl: super::super::System::Com::IDispatch_Impl {
    fn SmtpServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AdminEmail(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisableCommandLine(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableCommandLine(&self, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableScreeningAudit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableScreeningAudit(&self, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> windows_core::Result<()>;
    fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmSetting_Vtbl {
    pub const fn new<Identity: IFsrmSetting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::SmtpServer(this) {
                    Ok(ok__) => {
                        smtpserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetSmtpServer(this, core::mem::transmute(&smtpserver)).into()
            }
        }
        unsafe extern "system" fn MailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetMailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn AdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::AdminEmail(this) {
                    Ok(ok__) => {
                        adminemail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetAdminEmail(this, core::mem::transmute(&adminemail)).into()
            }
        }
        unsafe extern "system" fn DisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::DisableCommandLine(this) {
                    Ok(ok__) => {
                        disablecommandline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetDisableCommandLine(this, core::mem::transmute_copy(&disablecommandline)).into()
            }
        }
        unsafe extern "system" fn EnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::EnableScreeningAudit(this) {
                    Ok(ok__) => {
                        enablescreeningaudit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetEnableScreeningAudit(this, core::mem::transmute_copy(&enablescreeningaudit)).into()
            }
        }
        unsafe extern "system" fn EmailTest<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::EmailTest(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetActionRunLimitInterval(this, core::mem::transmute_copy(&actiontype), core::mem::transmute_copy(&delaytimeminutes)).into()
            }
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::GetActionRunLimitInterval(this, core::mem::transmute_copy(&actiontype)) {
                    Ok(ok__) => {
                        delaytimeminutes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SmtpServer: SmtpServer::<Identity, OFFSET>,
            SetSmtpServer: SetSmtpServer::<Identity, OFFSET>,
            MailFrom: MailFrom::<Identity, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, OFFSET>,
            AdminEmail: AdminEmail::<Identity, OFFSET>,
            SetAdminEmail: SetAdminEmail::<Identity, OFFSET>,
            DisableCommandLine: DisableCommandLine::<Identity, OFFSET>,
            SetDisableCommandLine: SetDisableCommandLine::<Identity, OFFSET>,
            EnableScreeningAudit: EnableScreeningAudit::<Identity, OFFSET>,
            SetEnableScreeningAudit: SetEnableScreeningAudit::<Identity, OFFSET>,
            EmailTest: EmailTest::<Identity, OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Identity, OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmSetting as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmSetting {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmStorageModuleDefinition, IFsrmStorageModuleDefinition_Vtbl, 0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmStorageModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleDefinition {
    pub unsafe fn Capabilities(&self) -> windows_core::Result<FsrmStorageModuleCaps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCapabilities)(windows_core::Interface::as_raw(self), capabilities).ok() }
    }
    pub unsafe fn StorageType(&self) -> windows_core::Result<FsrmStorageModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StorageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStorageType)(windows_core::Interface::as_raw(self), storagetype).ok() }
    }
    pub unsafe fn UpdatesFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdatesFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUpdatesFileContent)(windows_core::Interface::as_raw(self), updatesfilecontent).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmStorageModuleCaps) -> windows_core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmStorageModuleCaps) -> windows_core::HRESULT,
    pub StorageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmStorageModuleType) -> windows_core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmStorageModuleType) -> windows_core::HRESULT,
    pub UpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmStorageModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn Capabilities(&self) -> windows_core::Result<FsrmStorageModuleCaps>;
    fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> windows_core::Result<()>;
    fn StorageType(&self) -> windows_core::Result<FsrmStorageModuleType>;
    fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> windows_core::Result<()>;
    fn UpdatesFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUpdatesFileContent(&self, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmStorageModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        capabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCapabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetCapabilities(this, core::mem::transmute_copy(&capabilities)).into()
            }
        }
        unsafe extern "system" fn StorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::StorageType(this) {
                    Ok(ok__) => {
                        storagetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetStorageType(this, core::mem::transmute_copy(&storagetype)).into()
            }
        }
        unsafe extern "system" fn UpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::UpdatesFileContent(this) {
                    Ok(ok__) => {
                        updatesfilecontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetUpdatesFileContent(this, core::mem::transmute_copy(&updatesfilecontent)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            Capabilities: Capabilities::<Identity, OFFSET>,
            SetCapabilities: SetCapabilities::<Identity, OFFSET>,
            StorageType: StorageType::<Identity, OFFSET>,
            SetStorageType: SetStorageType::<Identity, OFFSET>,
            UpdatesFileContent: UpdatesFileContent::<Identity, OFFSET>,
            SetUpdatesFileContent: SetUpdatesFileContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmStorageModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmStorageModuleImplementation, IFsrmStorageModuleImplementation_Vtbl, 0x0af4a0da_895a_4e50_8712_a96724bcec64);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmStorageModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleImplementation {
    pub unsafe fn UseDefinitions<P0>(&self, propertydefinitions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).UseDefinitions)(windows_core::Interface::as_raw(self), propertydefinitions.param().abi()).ok() }
    }
    pub unsafe fn LoadProperties<P0>(&self, propertybag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()).ok() }
    }
    pub unsafe fn SaveProperties<P0>(&self, propertybag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    pub UseDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsrmStorageModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(&self, propertydefinitions: windows_core::Ref<IFsrmCollection>) -> windows_core::Result<()>;
    fn LoadProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
    fn SaveProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsrmStorageModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseDefinitions<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinitions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::UseDefinitions(this, core::mem::transmute_copy(&propertydefinitions)).into()
            }
        }
        unsafe extern "system" fn LoadProperties<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::LoadProperties(this, core::mem::transmute_copy(&propertybag)).into()
            }
        }
        unsafe extern "system" fn SaveProperties<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::SaveProperties(this, core::mem::transmute_copy(&propertybag)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, OFFSET>(),
            UseDefinitions: UseDefinitions::<Identity, OFFSET>,
            LoadProperties: LoadProperties::<Identity, OFFSET>,
            SaveProperties: SaveProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmStorageModuleImplementation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsrmStorageModuleImplementation {}
pub const MessageSizeLimit: u32 = 4096u32;
