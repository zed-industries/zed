pub type AsyncIBackgroundCopyCallback = *mut ::core::ffi::c_void;
pub type IBITSExtensionSetup = *mut ::core::ffi::c_void;
pub type IBITSExtensionSetupFactory = *mut ::core::ffi::c_void;
pub type IBackgroundCopyCallback = *mut ::core::ffi::c_void;
pub type IBackgroundCopyCallback1 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyCallback2 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyCallback3 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyError = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile2 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile3 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile4 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile5 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyFile6 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyGroup = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob1 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob2 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob3 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob4 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJob5 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJobHttpOptions = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJobHttpOptions2 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyJobHttpOptions3 = *mut ::core::ffi::c_void;
pub type IBackgroundCopyManager = *mut ::core::ffi::c_void;
pub type IBackgroundCopyQMgr = *mut ::core::ffi::c_void;
pub type IBackgroundCopyServerCertificateValidationCallback = *mut ::core::ffi::c_void;
pub type IBitsPeer = *mut ::core::ffi::c_void;
pub type IBitsPeerCacheAdministration = *mut ::core::ffi::c_void;
pub type IBitsPeerCacheRecord = *mut ::core::ffi::c_void;
pub type IBitsTokenOptions = *mut ::core::ffi::c_void;
pub type IEnumBackgroundCopyFiles = *mut ::core::ffi::c_void;
pub type IEnumBackgroundCopyGroups = *mut ::core::ffi::c_void;
pub type IEnumBackgroundCopyJobs = *mut ::core::ffi::c_void;
pub type IEnumBackgroundCopyJobs1 = *mut ::core::ffi::c_void;
pub type IEnumBitsPeerCacheRecords = *mut ::core::ffi::c_void;
pub type IEnumBitsPeers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_DACL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_GROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_SACL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_APP_PACKAGE_NOT_FOUND: i32 = -2145386390i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: i32 = -2145386389i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: i32 = -2145386386i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: i32 = -2145386393i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: i32 = -2145386392i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: i32 = -2145386407i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_GAME_MODE: i32 = -2145386385i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_POLICY: i32 = -2145386434i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: i32 = -2145386384i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BUSYCACHERECORD: i32 = -2145386424i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: i32 = -2145386462i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_COMMIT_IN_PROGRESS: i32 = -2145386429i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CONNECTION_CLOSED: i32 = -2145386450i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CONNECT_FAILURE: i32 = -2145386451i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DATABASE_CORRUPT: i32 = -2145386388i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DESTINATION_LOCKED: i32 = -2145386483i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DISCOVERY_IN_PROGRESS: i32 = -2145386428i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_EMPTY: i32 = -2145386493i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: i32 = -2145386488i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: i32 = -2145386485i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: i32 = -2145386487i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: i32 = -2145386484i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: i32 = -2145386466i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: i32 = -2145386486i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: i32 = -2145386378i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_UNKNOWN: i32 = -2145386489i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: i32 = -2145386481i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_FILE_NOT_AVAILABLE: i32 = -2145386492i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_FILE_NOT_FOUND: i32 = -2145386455i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_100: i32 = -2145845148i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_101: i32 = -2145845147i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_200: i32 = -2145845048i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_201: i32 = -2145845047i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_202: i32 = -2145845046i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_203: i32 = -2145845045i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_204: i32 = -2145845044i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_205: i32 = -2145845043i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_206: i32 = -2145845042i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_300: i32 = -2145844948i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_301: i32 = -2145844947i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_302: i32 = -2145844946i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_303: i32 = -2145844945i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_304: i32 = -2145844944i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_305: i32 = -2145844943i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_307: i32 = -2145844941i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_400: i32 = -2145844848i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_401: i32 = -2145844847i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_402: i32 = -2145844846i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_403: i32 = -2145844845i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_404: i32 = -2145844844i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_405: i32 = -2145844843i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_406: i32 = -2145844842i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_407: i32 = -2145844841i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_408: i32 = -2145844840i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_409: i32 = -2145844839i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_410: i32 = -2145844838i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_411: i32 = -2145844837i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_412: i32 = -2145844836i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_413: i32 = -2145844835i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_414: i32 = -2145844834i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_415: i32 = -2145844833i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_416: i32 = -2145844832i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_417: i32 = -2145844831i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_449: i32 = -2145844799i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_500: i32 = -2145844748i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_501: i32 = -2145844747i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_502: i32 = -2145844746i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_503: i32 = -2145844745i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_504: i32 = -2145844744i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_505: i32 = -2145844743i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: i32 = -2145386478i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: i32 = -2145386477i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_AUTH_SCHEME: i32 = -2145386456i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_AUTH_TARGET: i32 = -2145386457i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_CREDENTIALS: i32 = -2145386432i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_HASH_ALGORITHM: i32 = -2145386431i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_PROXY_INFO: i32 = -2145386433i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_RANGE: i32 = -2145386453i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_SERVER_RESPONSE: i32 = -2145386469i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_STATE: i32 = -2145386494i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_LOCAL_FILE_CHANGED: i32 = -2145386467i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAXDOWNLOAD_TIMEOUT: i32 = -2145386412i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: i32 = -2145386397i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: i32 = -2145386396i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MISSING_FILE_SIZE: i32 = -2145386479i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NETWORK_DISCONNECTED: i32 = -2145386480i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NEW_OWNER_DIFF_MAPPING: i32 = -2145386475i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: i32 = -2145386474i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NOT_FOUND: i32 = -2145386495i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: i32 = -2145386383i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NO_PROGRESS: i32 = -2145386460i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_OVERLAPPING_RANGES: i32 = -2145386452i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PASSWORD_TOO_LARGE: i32 = -2145386458i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PEERCACHING_DISABLED: i32 = -2145386425i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: i32 = -2145386400i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROTOCOL_NOT_AVAILABLE: i32 = -2145386491i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: i32 = -2145386471i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROXY_LIST_TOO_LARGE: i32 = -2145386472i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: i32 = -2145386387i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY: i32 = -2145386408i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: i32 = -2145386399i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: i32 = -2145386398i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: i32 = -2145386379i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_RECORD_DELETED: i32 = -2145386430i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_REMOTE_FILE_CHANGED: i32 = -2145386381i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_REMOTE_NOT_SUPPORTED: i32 = -2145386476i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: i32 = -2145386380i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SERVER_EXECUTE_ENABLE: i32 = -2145386461i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SESSION_NOT_FOUND: i32 = -2145386465i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_STANDBY_MODE: i32 = -2145386395i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_STRING_TOO_LONG: i32 = -2145386463i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: i32 = -2145386426i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOKEN_REQUIRED: i32 = -2145386410i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_LARGE: i32 = -2145386464i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_FILES: i32 = -2145386468i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_FILES_IN_JOB: i32 = -2145386415i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: i32 = -2145386416i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_JOBS_PER_USER: i32 = -2145386423i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_RANGES_IN_FILE: i32 = -2145386414i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UNKNOWN_PROPERTY_ID: i32 = -2145386409i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: i32 = -2145386382i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UPNP_ERROR: i32 = -2145386427i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_USERNAME_TOO_LARGE: i32 = -2145386459i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: i32 = -2145386394i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_VALIDATION_FAILED: i32 = -2145386413i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_VOLUME_CHANGED: i32 = -2145386482i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_WATCHDOG_TIMEOUT: i32 = -2145386391i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_PROXY_CHANGED: i32 = 2097194i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITSExtensionSetupFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xefbbab68_7286_4783_94bf_9461d8b7e7e9);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_ROAMING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FAILED_TO_START: i32 = -2145828856i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FATAL_IGD_ERROR: i32 = -2145828855i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FILE_DELETION_FAILED: i32 = -2145828863i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FILE_DELETION_FAILED_MORE: i32 = -2145828862i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_CANCELLED: i32 = -2145828864i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: i32 = -2145828858i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_PROPERTY_CHANGE: i32 = -2145828861i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_SCAVENGED: i32 = -2145828859i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_TAKE_OWNERSHIP: i32 = -2145828860i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_PEERCACHING_PORT: i32 = -2145828854i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_STATE_FILE_CORRUPT: i32 = -2145828857i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_WSD_PORT: i32 = -2145828853i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager10_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4bd3e4e1_7bd4_4a2b_9964_496400de5193);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager10_2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4575438f_a6c8_4976_b0fe_2f26b80d959e);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager10_3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5fd42ad5_c04e_4d36_adc7_e08ff15737ad);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager1_5: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager2_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager2_5: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x03ca98d6_ff5d_49b8_abc6_03dd84127020);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager3_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x659cdea7_489e_11d9_a9cd_000d56965251);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager4_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb6df56b_cace_11dc_9992_0019b93a3a84);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyManager5_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1ecca34c_e88a_44e3_8d6a_8921bde9e452);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BackgroundCopyQMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x69ad4aee_51be_439b_a92c_86ae490e8b30);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_DOWNLOADER_UNAVAILABLE: u32 = 2164264963u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_INVALID_STATE: u32 = 2164264961u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_ITEM_NOT_FOUND: u32 = 2164264964u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_SERVICE_UNAVAILABLE: u32 = 2164264962u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_DISABLE_NOTIFY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_FILE_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_GROUP_DONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_JOB_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_USE_PROGRESSEX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_PERCENT_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_SIZE_DONE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_TIME_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_FTP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_HTTP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_SMB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_FILE_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_FILE_INCOMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_COMPLETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_ERROR: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_FOREGROUND: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_INCOMPLETE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_SUSPENDED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_FOREGROUND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_INCOMPLETE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_AUTH_SCHEME = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_AUTH_TARGET = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_CERT_STORE_LOCATION = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = 6i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = 7i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_ERROR_CONTEXT = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = 6i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = 7i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = 8i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_JOB_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_JOB_PROXY_USAGE = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_JOB_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_JOB_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BG_TOKEN = u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_TOKEN_LOCAL_FILE: BG_TOKEN = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_TOKEN_NETWORK: BG_TOKEN = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BITS_FILE_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BITS_JOB_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type BITS_JOB_TRANSFER_POLICY = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = -2147483393i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = -2147483521i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = -2147483537i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = -2147483545i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = -2147483615i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub type GROUPPROP = i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PRIORITY: GROUPPROP = 0i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_REMOTEUSERID: GROUPPROP = 1i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_REMOTEUSERPWD: GROUPPROP = 2i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_LOCALUSERID: GROUPPROP = 3i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_LOCALUSERPWD: GROUPPROP = 4i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROTOCOLFLAGS: GROUPPROP = 5i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_NOTIFYFLAGS: GROUPPROP = 6i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_NOTIFYCLSID: GROUPPROP = 7i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSSIZE: GROUPPROP = 8i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSPERCENT: GROUPPROP = 9i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSTIME: GROUPPROP = 10i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_DISPLAYNAME: GROUPPROP = 11i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_DESCRIPTION: GROUPPROP = 12i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl ::core::marker::Copy for BG_AUTH_CREDENTIALS {}
impl ::core::clone::Clone for BG_AUTH_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl ::core::marker::Copy for BG_AUTH_CREDENTIALS_UNION {}
impl ::core::clone::Clone for BG_AUTH_CREDENTIALS_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: ::windows_sys::core::PWSTR,
    pub Password: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BG_BASIC_CREDENTIALS {}
impl ::core::clone::Clone for BG_BASIC_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_FILE_INFO {
    pub RemoteName: ::windows_sys::core::PWSTR,
    pub LocalName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BG_FILE_INFO {}
impl ::core::clone::Clone for BG_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BG_FILE_PROGRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BG_FILE_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BG_FILE_RANGE {}
impl ::core::clone::Clone for BG_FILE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
impl ::core::marker::Copy for BG_JOB_PROGRESS {}
impl ::core::clone::Clone for BG_JOB_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
impl ::core::marker::Copy for BG_JOB_REPLY_PROGRESS {}
impl ::core::clone::Clone for BG_JOB_REPLY_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::super::Foundation::FILETIME,
    pub ModificationTime: super::super::Foundation::FILETIME,
    pub TransferCompletionTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BG_JOB_TIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BG_JOB_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for BITS_FILE_PROPERTY_VALUE {}
impl ::core::clone::Clone for BITS_FILE_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: u32,
    pub ClsID: ::windows_sys::core::GUID,
    pub Enable: super::super::Foundation::BOOL,
    pub Uint64: u64,
    pub Target: BG_AUTH_TARGET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BITS_JOB_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BITS_JOB_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct FILESETINFO {
    pub bstrRemoteFile: ::windows_sys::core::BSTR,
    pub bstrLocalFile: ::windows_sys::core::BSTR,
    pub dwSizeHint: u32,
}
impl ::core::marker::Copy for FILESETINFO {}
impl ::core::clone::Clone for FILESETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
