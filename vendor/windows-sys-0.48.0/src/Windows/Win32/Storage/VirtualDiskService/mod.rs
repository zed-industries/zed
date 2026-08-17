pub type IEnumVdsObject = *mut ::core::ffi::c_void;
pub type IVdsAdmin = *mut ::core::ffi::c_void;
pub type IVdsAdvancedDisk = *mut ::core::ffi::c_void;
pub type IVdsAdvancedDisk2 = *mut ::core::ffi::c_void;
pub type IVdsAdvancedDisk3 = *mut ::core::ffi::c_void;
pub type IVdsAdviseSink = *mut ::core::ffi::c_void;
pub type IVdsAsync = *mut ::core::ffi::c_void;
pub type IVdsController = *mut ::core::ffi::c_void;
pub type IVdsControllerControllerPort = *mut ::core::ffi::c_void;
pub type IVdsControllerPort = *mut ::core::ffi::c_void;
pub type IVdsCreatePartitionEx = *mut ::core::ffi::c_void;
pub type IVdsDisk = *mut ::core::ffi::c_void;
pub type IVdsDisk2 = *mut ::core::ffi::c_void;
pub type IVdsDisk3 = *mut ::core::ffi::c_void;
pub type IVdsDiskOnline = *mut ::core::ffi::c_void;
pub type IVdsDiskPartitionMF = *mut ::core::ffi::c_void;
pub type IVdsDiskPartitionMF2 = *mut ::core::ffi::c_void;
pub type IVdsDrive = *mut ::core::ffi::c_void;
pub type IVdsDrive2 = *mut ::core::ffi::c_void;
pub type IVdsHbaPort = *mut ::core::ffi::c_void;
pub type IVdsHwProvider = *mut ::core::ffi::c_void;
pub type IVdsHwProviderPrivate = *mut ::core::ffi::c_void;
pub type IVdsHwProviderPrivateMpio = *mut ::core::ffi::c_void;
pub type IVdsHwProviderStoragePools = *mut ::core::ffi::c_void;
pub type IVdsHwProviderType = *mut ::core::ffi::c_void;
pub type IVdsHwProviderType2 = *mut ::core::ffi::c_void;
pub type IVdsIscsiInitiatorAdapter = *mut ::core::ffi::c_void;
pub type IVdsIscsiInitiatorPortal = *mut ::core::ffi::c_void;
pub type IVdsIscsiPortal = *mut ::core::ffi::c_void;
pub type IVdsIscsiPortalGroup = *mut ::core::ffi::c_void;
pub type IVdsIscsiPortalLocal = *mut ::core::ffi::c_void;
pub type IVdsIscsiTarget = *mut ::core::ffi::c_void;
pub type IVdsLun = *mut ::core::ffi::c_void;
pub type IVdsLun2 = *mut ::core::ffi::c_void;
pub type IVdsLunControllerPorts = *mut ::core::ffi::c_void;
pub type IVdsLunIscsi = *mut ::core::ffi::c_void;
pub type IVdsLunMpio = *mut ::core::ffi::c_void;
pub type IVdsLunNaming = *mut ::core::ffi::c_void;
pub type IVdsLunNumber = *mut ::core::ffi::c_void;
pub type IVdsLunPlex = *mut ::core::ffi::c_void;
pub type IVdsMaintenance = *mut ::core::ffi::c_void;
pub type IVdsOpenVDisk = *mut ::core::ffi::c_void;
pub type IVdsPack = *mut ::core::ffi::c_void;
pub type IVdsPack2 = *mut ::core::ffi::c_void;
pub type IVdsProvider = *mut ::core::ffi::c_void;
pub type IVdsProviderPrivate = *mut ::core::ffi::c_void;
pub type IVdsProviderSupport = *mut ::core::ffi::c_void;
pub type IVdsRemovable = *mut ::core::ffi::c_void;
pub type IVdsService = *mut ::core::ffi::c_void;
pub type IVdsServiceHba = *mut ::core::ffi::c_void;
pub type IVdsServiceInitialization = *mut ::core::ffi::c_void;
pub type IVdsServiceIscsi = *mut ::core::ffi::c_void;
pub type IVdsServiceLoader = *mut ::core::ffi::c_void;
pub type IVdsServiceSAN = *mut ::core::ffi::c_void;
pub type IVdsServiceSw = *mut ::core::ffi::c_void;
pub type IVdsServiceUninstallDisk = *mut ::core::ffi::c_void;
pub type IVdsStoragePool = *mut ::core::ffi::c_void;
pub type IVdsSubSystem = *mut ::core::ffi::c_void;
pub type IVdsSubSystem2 = *mut ::core::ffi::c_void;
pub type IVdsSubSystemImportTarget = *mut ::core::ffi::c_void;
pub type IVdsSubSystemInterconnect = *mut ::core::ffi::c_void;
pub type IVdsSubSystemIscsi = *mut ::core::ffi::c_void;
pub type IVdsSubSystemNaming = *mut ::core::ffi::c_void;
pub type IVdsSwProvider = *mut ::core::ffi::c_void;
pub type IVdsVDisk = *mut ::core::ffi::c_void;
pub type IVdsVdProvider = *mut ::core::ffi::c_void;
pub type IVdsVolume = *mut ::core::ffi::c_void;
pub type IVdsVolume2 = *mut ::core::ffi::c_void;
pub type IVdsVolumeMF = *mut ::core::ffi::c_void;
pub type IVdsVolumeMF2 = *mut ::core::ffi::c_void;
pub type IVdsVolumeMF3 = *mut ::core::ffi::c_void;
pub type IVdsVolumeOnline = *mut ::core::ffi::c_void;
pub type IVdsVolumePlex = *mut ::core::ffi::c_void;
pub type IVdsVolumeShrink = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const CLSID_VdsLoader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9c38ed61_d565_4728_aeee_c80952f0ecde);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const CLSID_VdsService: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7d1933cb_86f6_4a98_8628_01be94c9a575);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ACCESS_DENIED: ::windows_sys::core::HRESULT = -2147212249i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ACTIVE_PARTITION: ::windows_sys::core::HRESULT = -2147212232i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = -2147211517i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: ::windows_sys::core::HRESULT = -2147211949i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_IS_ZERO: ::windows_sys::core::HRESULT = -2147211888i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: ::windows_sys::core::HRESULT = -2147211889i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: ::windows_sys::core::HRESULT = -2147211948i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_ZERO: ::windows_sys::core::HRESULT = -2147211947i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = -2147212285i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212284i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASSOCIATED_LUNS_EXIST: ::windows_sys::core::HRESULT = -2147211509i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: ::windows_sys::core::HRESULT = -2147211508i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASYNC_OBJECT_FAILURE: ::windows_sys::core::HRESULT = -2147212210i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_BOOT_DISK: ::windows_sys::core::HRESULT = -2147211898i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_COOKIE: ::windows_sys::core::HRESULT = -2147212271i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_LABEL: ::windows_sys::core::HRESULT = -2147212247i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_PNP_MESSAGE: ::windows_sys::core::HRESULT = -2147212017i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_PROVIDER_DATA: ::windows_sys::core::HRESULT = -2147212223i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_REVISION_NUMBER: ::windows_sys::core::HRESULT = -2147211880i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BLOCK_CLUSTERED: ::windows_sys::core::HRESULT = -2147210749i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_DISK: ::windows_sys::core::HRESULT = -2147211257i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: ::windows_sys::core::HRESULT = -2147210994i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: ::windows_sys::core::HRESULT = -2147212234i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CACHE_CORRUPT: ::windows_sys::core::HRESULT = -2147211946i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANCEL_TOO_LATE: ::windows_sys::core::HRESULT = -2147212276i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: ::windows_sys::core::HRESULT = -2147211945i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_EXTEND: ::windows_sys::core::HRESULT = -2147212274i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_SHRINK: ::windows_sys::core::HRESULT = -2147212002i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANT_INVALIDATE_FVE: ::windows_sys::core::HRESULT = -2147211886i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANT_QUICK_FORMAT: ::windows_sys::core::HRESULT = -2147212246i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = -2147210743i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_CRITICAL: ::windows_sys::core::HRESULT = -2147210990i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_DATA: ::windows_sys::core::HRESULT = -2147210992i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_OEM: ::windows_sys::core::HRESULT = -2147210991i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: ::windows_sys::core::HRESULT = -2147212240i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = -2147212241i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: ::windows_sys::core::HRESULT = -2147212242i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210984i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CONFIG_LIMIT: ::windows_sys::core::HRESULT = -2147211976i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_EXTENT_INFO: ::windows_sys::core::HRESULT = -2147212021i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: ::windows_sys::core::HRESULT = -2147211990i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_PARTITION_INFO: ::windows_sys::core::HRESULT = -2147212023i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_VOLUME_INFO: ::windows_sys::core::HRESULT = -2147212029i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CRASHDUMP_DISK: ::windows_sys::core::HRESULT = -2147211250i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CRITICAL_PLEX: ::windows_sys::core::HRESULT = -2147211906i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DELETE_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = -2147210745i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DELETE_WITH_CRITICAL: ::windows_sys::core::HRESULT = -2147210993i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DEVICE_IN_USE: ::windows_sys::core::HRESULT = -2147212269i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_BEING_CLEANED: ::windows_sys::core::HRESULT = -2147211944i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: ::windows_sys::core::HRESULT = -2147211975i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: ::windows_sys::core::HRESULT = -2147211974i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: ::windows_sys::core::HRESULT = -2147211973i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_DYNAMIC: ::windows_sys::core::HRESULT = -2147211972i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_HAS_BANDS: ::windows_sys::core::HRESULT = -2147210748i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IN_USE_BY_VOLUME: ::windows_sys::core::HRESULT = -2147212212i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IO_FAILING: ::windows_sys::core::HRESULT = -2147211968i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IS_OFFLINE: ::windows_sys::core::HRESULT = -2147211254i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IS_READ_ONLY: ::windows_sys::core::HRESULT = -2147211253i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: ::windows_sys::core::HRESULT = -2147211969i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE: ::windows_sys::core::HRESULT = -2147211943i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: ::windows_sys::core::HRESULT = -2147210971i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_EMPTY: ::windows_sys::core::HRESULT = -2147212268i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: ::windows_sys::core::HRESULT = -2147211987i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_IMPORTED: ::windows_sys::core::HRESULT = -2147212206i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147212265i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = -2147212217i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_MISSING: ::windows_sys::core::HRESULT = -2147212031i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_OFFLINE: ::windows_sys::core::HRESULT = -2147211883i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_ONLINE: ::windows_sys::core::HRESULT = -2147212213i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_PNP_REG_CORRUPT: ::windows_sys::core::HRESULT = -2147212203i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_REMOVEABLE: ::windows_sys::core::HRESULT = -2147211942i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: ::windows_sys::core::HRESULT = -2147211941i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISTINCT_VOLUME: ::windows_sys::core::HRESULT = -2147211909i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: ::windows_sys::core::HRESULT = -2147212252i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: ::windows_sys::core::HRESULT = -2147212256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: ::windows_sys::core::HRESULT = -2147212261i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -2147212027i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_INVALID_PARAM: ::windows_sys::core::HRESULT = -2147212004i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_NO_PACK_NAME: ::windows_sys::core::HRESULT = -2147212019i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = -2147211971i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVE_LETTER_NOT_FREE: ::windows_sys::core::HRESULT = -2147211940i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DUPLICATE_DISK: ::windows_sys::core::HRESULT = -2147211986i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DUP_EMPTY_PACK_GUID: ::windows_sys::core::HRESULT = -2147212020i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147211967i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: ::windows_sys::core::HRESULT = -2147212186i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147211939i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: ::windows_sys::core::HRESULT = -2147210968i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: ::windows_sys::core::HRESULT = -2147210967i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: ::windows_sys::core::HRESULT = -2147212011i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: ::windows_sys::core::HRESULT = -2147212237i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAILED_TO_OFFLINE_DISK: ::windows_sys::core::HRESULT = -2147211881i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAILED_TO_ONLINE_DISK: ::windows_sys::core::HRESULT = -2147211882i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210987i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210986i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147211966i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FLAG_ALREADY_SET: ::windows_sys::core::HRESULT = -2147211911i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_CRITICAL: ::windows_sys::core::HRESULT = -2147210989i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210985i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = -2147210744i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FS_NOT_DETERMINED: ::windows_sys::core::HRESULT = -2147211885i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_GET_SAN_POLICY: ::windows_sys::core::HRESULT = -2147211259i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_GPT_ATTRIBUTES_INVALID: ::windows_sys::core::HRESULT = -2147211965i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_HIBERNATION_FILE_DISK: ::windows_sys::core::HRESULT = -2147211251i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = -2147212198i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IMPORT_SET_INCOMPLETE: ::windows_sys::core::HRESULT = -2147212207i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: ::windows_sys::core::HRESULT = -2147212251i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCOMPATIBLE_MEDIA: ::windows_sys::core::HRESULT = -2147212250i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: ::windows_sys::core::HRESULT = -2147211260i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: ::windows_sys::core::HRESULT = -2147211248i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIALIZED_FAILED: ::windows_sys::core::HRESULT = -2147212287i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIALIZE_NOT_CALLED: ::windows_sys::core::HRESULT = -2147212286i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: ::windows_sys::core::HRESULT = -2147211008i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147211513i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -2147212216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_BLOCK_SIZE: ::windows_sys::core::HRESULT = -2147211982i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DISK: ::windows_sys::core::HRESULT = -2147212007i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DISK_COUNT: ::windows_sys::core::HRESULT = -2147211994i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DRIVE_LETTER: ::windows_sys::core::HRESULT = -2147211938i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: ::windows_sys::core::HRESULT = -2147211937i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ENUMERATOR: ::windows_sys::core::HRESULT = -2147212028i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_EXTENT_COUNT: ::windows_sys::core::HRESULT = -2147211993i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_FS_FLAG: ::windows_sys::core::HRESULT = -2147211936i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_FS_TYPE: ::windows_sys::core::HRESULT = -2147211935i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_IP_ADDRESS: ::windows_sys::core::HRESULT = -2147210997i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ISCSI_PATH: ::windows_sys::core::HRESULT = -2147210980i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: ::windows_sys::core::HRESULT = -2147211005i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_MEMBER_COUNT: ::windows_sys::core::HRESULT = -2147211998i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_MEMBER_ORDER: ::windows_sys::core::HRESULT = -2147211996i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_OBJECT_TYPE: ::windows_sys::core::HRESULT = -2147211934i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = -2147212267i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PACK: ::windows_sys::core::HRESULT = -2147212006i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_LAYOUT: ::windows_sys::core::HRESULT = -2147211933i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_STYLE: ::windows_sys::core::HRESULT = -2147211932i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_TYPE: ::windows_sys::core::HRESULT = -2147211931i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PATH: ::windows_sys::core::HRESULT = -2147210981i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: ::windows_sys::core::HRESULT = -2147211978i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_COUNT: ::windows_sys::core::HRESULT = -2147211999i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_GUID: ::windows_sys::core::HRESULT = -2147211988i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_ORDER: ::windows_sys::core::HRESULT = -2147211997i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_TYPE: ::windows_sys::core::HRESULT = -2147211979i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PORT_PATH: ::windows_sys::core::HRESULT = -2147211006i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_CLSID: ::windows_sys::core::HRESULT = -2147211930i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_ID: ::windows_sys::core::HRESULT = -2147211929i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_NAME: ::windows_sys::core::HRESULT = -2147211928i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_TYPE: ::windows_sys::core::HRESULT = -2147211927i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: ::windows_sys::core::HRESULT = -2147211926i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: ::windows_sys::core::HRESULT = -2147211925i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: ::windows_sys::core::HRESULT = -2147211924i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SECTOR_SIZE: ::windows_sys::core::HRESULT = -2147211984i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SERVICE_FLAG: ::windows_sys::core::HRESULT = -2147211923i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SHRINK_SIZE: ::windows_sys::core::HRESULT = -2147211241i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SPACE: ::windows_sys::core::HRESULT = -2147212282i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_STATE: ::windows_sys::core::HRESULT = -2147210747i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_STRIPE_SIZE: ::windows_sys::core::HRESULT = -2147211995i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_FLAG: ::windows_sys::core::HRESULT = -2147211922i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_LENGTH: ::windows_sys::core::HRESULT = -2147211954i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_TYPE: ::windows_sys::core::HRESULT = -2147211899i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IO_ERROR: ::windows_sys::core::HRESULT = -2147212245i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_CHAP_SECRET: ::windows_sys::core::HRESULT = -2147210998i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_GET_IKE_INFO: ::windows_sys::core::HRESULT = -2147211003i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: ::windows_sys::core::HRESULT = -2147210999i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: ::windows_sys::core::HRESULT = -2147211000i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGIN_FAILED: ::windows_sys::core::HRESULT = -2147211512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGOUT_FAILED: ::windows_sys::core::HRESULT = -2147211511i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: ::windows_sys::core::HRESULT = -2147211504i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: ::windows_sys::core::HRESULT = -2147211510i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_SET_IKE_INFO: ::windows_sys::core::HRESULT = -2147211002i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LAST_VALID_DISK: ::windows_sys::core::HRESULT = -2147211985i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: ::windows_sys::core::HRESULT = -2147212202i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LDM_TIMEOUT: ::windows_sys::core::HRESULT = -2147212191i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LEGACY_VOLUME_FORMAT: ::windows_sys::core::HRESULT = -2147212230i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LOG_UPDATE: ::windows_sys::core::HRESULT = -2147211897i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_FAILED: ::windows_sys::core::HRESULT = -2147211239i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_MISSING: ::windows_sys::core::HRESULT = -2147211240i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_NOT_READY: ::windows_sys::core::HRESULT = -2147211238i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_NO_MEDIA: ::windows_sys::core::HRESULT = -2147211237i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_READ_ONLY: ::windows_sys::core::HRESULT = -2147210978i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DYNAMIC: ::windows_sys::core::HRESULT = -2147210976i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DYNAMIC_OFFLINE: ::windows_sys::core::HRESULT = -2147210975i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_FAILED: ::windows_sys::core::HRESULT = -2147211234i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_NOT_READY: ::windows_sys::core::HRESULT = -2147211236i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_OFFLINE: ::windows_sys::core::HRESULT = -2147211235i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_SHRINK_GPT_HEADER: ::windows_sys::core::HRESULT = -2147210974i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_UPDATE_DISK: ::windows_sys::core::HRESULT = -2147210977i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MAX_USABLE_MBR: ::windows_sys::core::HRESULT = -2147212184i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEDIA_WRITE_PROTECTED: ::windows_sys::core::HRESULT = -2147212248i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_IS_HEALTHY: ::windows_sys::core::HRESULT = -2147211964i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_MISSING: ::windows_sys::core::HRESULT = -2147211958i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_REGENERATING: ::windows_sys::core::HRESULT = -2147211963i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_SIZE_INVALID: ::windows_sys::core::HRESULT = -2147212010i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MIGRATE_OPEN_VOLUME: ::windows_sys::core::HRESULT = -2147212228i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MIRROR_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210973i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MISSING_DISK: ::windows_sys::core::HRESULT = -2147212204i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: ::windows_sys::core::HRESULT = -2147211506i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MULTIPLE_PACKS: ::windows_sys::core::HRESULT = -2147212001i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NAME_NOT_UNIQUE: ::windows_sys::core::HRESULT = -2147211519i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: ::windows_sys::core::HRESULT = -2147212229i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: ::windows_sys::core::HRESULT = -2147212264i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_ENOUGH_DRIVE: ::windows_sys::core::HRESULT = -2147212272i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = -2147212273i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISCOVERY_DOMAIN: ::windows_sys::core::HRESULT = -2147211507i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISKS_FOUND: ::windows_sys::core::HRESULT = -2147212258i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISK_PATHNAME: ::windows_sys::core::HRESULT = -2147211505i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DRIVELETTER_FLAG: ::windows_sys::core::HRESULT = -2147212201i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_EXTENTS_FOR_PLEX: ::windows_sys::core::HRESULT = -2147211980i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: ::windows_sys::core::HRESULT = -2147212218i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_FREE_SPACE: ::windows_sys::core::HRESULT = -2147212233i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_HEALTHY_DISKS: ::windows_sys::core::HRESULT = -2147211977i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_IMPORT_TARGET: ::windows_sys::core::HRESULT = -2147211501i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_MAINTENANCE_MODE: ::windows_sys::core::HRESULT = -2147210750i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_MEDIA: ::windows_sys::core::HRESULT = -2147212270i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_DISK_ARRIVE: ::windows_sys::core::HRESULT = -2147212016i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_DISK_REMOVE: ::windows_sys::core::HRESULT = -2147212014i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: ::windows_sys::core::HRESULT = -2147212015i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_VOLUME_REMOVE: ::windows_sys::core::HRESULT = -2147212013i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_POOL: ::windows_sys::core::HRESULT = -2147210752i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_POOL_CREATED: ::windows_sys::core::HRESULT = -2147210751i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: ::windows_sys::core::HRESULT = -2147212032i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VALID_LOG_COPIES: ::windows_sys::core::HRESULT = -2147211894i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VOLUME_LAYOUT: ::windows_sys::core::HRESULT = -2147212030i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VOLUME_PATHNAME: ::windows_sys::core::HRESULT = -2147211503i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210988i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_DELETED: ::windows_sys::core::HRESULT = -2147212277i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_EXISTS: ::windows_sys::core::HRESULT = -2147212259i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212283i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_OUT_OF_SYNC: ::windows_sys::core::HRESULT = -2147212205i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_STATUS_FAILED: ::windows_sys::core::HRESULT = -2147212239i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OFFLINE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210970i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ONE_EXTENT_PER_DISK: ::windows_sys::core::HRESULT = -2147211983i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ONLINE_PACK_EXISTS: ::windows_sys::core::HRESULT = -2147212188i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_CANCELED: ::windows_sys::core::HRESULT = -2147212275i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_DENIED: ::windows_sys::core::HRESULT = -2147212278i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_PENDING: ::windows_sys::core::HRESULT = -2147212279i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_NAME_INVALID: ::windows_sys::core::HRESULT = -2147211962i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212208i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_OFFLINE: ::windows_sys::core::HRESULT = -2147212220i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_ONLINE: ::windows_sys::core::HRESULT = -2147212000i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PAGEFILE_DISK: ::windows_sys::core::HRESULT = -2147211252i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_LDM: ::windows_sys::core::HRESULT = -2147211891i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_LIMIT_REACHED: ::windows_sys::core::HRESULT = -2147212281i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_MSR: ::windows_sys::core::HRESULT = -2147211892i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NON_DATA: ::windows_sys::core::HRESULT = -2147211907i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: ::windows_sys::core::HRESULT = -2147211970i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_EMPTY: ::windows_sys::core::HRESULT = -2147212280i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_OEM: ::windows_sys::core::HRESULT = -2147211921i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: ::windows_sys::core::HRESULT = -2147212231i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_PROTECTED: ::windows_sys::core::HRESULT = -2147211920i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_STYLE_MISMATCH: ::windows_sys::core::HRESULT = -2147211919i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PATH_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212266i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_IS_HEALTHY: ::windows_sys::core::HRESULT = -2147211961i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_LAST_ACTIVE: ::windows_sys::core::HRESULT = -2147211960i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_MISSING: ::windows_sys::core::HRESULT = -2147211959i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = -2147211893i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_REGENERATING: ::windows_sys::core::HRESULT = -2147211957i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_SIZE_INVALID: ::windows_sys::core::HRESULT = -2147211981i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_CACHE_CORRUPT: ::windows_sys::core::HRESULT = -2147212257i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: ::windows_sys::core::HRESULT = -2147211502i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_EXITING: ::windows_sys::core::HRESULT = -2147212012i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_FAILURE: ::windows_sys::core::HRESULT = -2147212222i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: ::windows_sys::core::HRESULT = -2147212260i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = -2147211918i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212214i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212254i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_VOL_OPEN: ::windows_sys::core::HRESULT = -2147212253i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_RAID5_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210972i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_READONLY: ::windows_sys::core::HRESULT = -2147211900i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = -2147210996i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147210746i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REPAIR_VOLUMESTATE: ::windows_sys::core::HRESULT = -2147212192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: ::windows_sys::core::HRESULT = -2147212224i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_RETRY: ::windows_sys::core::HRESULT = -2147212189i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE: ::windows_sys::core::HRESULT = -2147212200i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: ::windows_sys::core::HRESULT = -2147212190i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE_SET: ::windows_sys::core::HRESULT = -2147212199i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SECTOR_SIZE_ERROR: ::windows_sys::core::HRESULT = -2147211229i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SECURITY_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = -2147211515i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SET_SAN_POLICY: ::windows_sys::core::HRESULT = -2147211258i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: ::windows_sys::core::HRESULT = -2147211004i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_DIRTY_VOLUME: ::windows_sys::core::HRESULT = -2147211878i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: ::windows_sys::core::HRESULT = -2147210496i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147211887i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: ::windows_sys::core::HRESULT = -2147210979i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_OVER_DATA: ::windows_sys::core::HRESULT = -2147211242i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: ::windows_sys::core::HRESULT = -2147211917i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = -2147211916i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: ::windows_sys::core::HRESULT = -2147210966i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_USER_CANCELLED: ::windows_sys::core::HRESULT = -2147211879i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SOURCE_IS_TARGET_PACK: ::windows_sys::core::HRESULT = -2147211992i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: ::windows_sys::core::HRESULT = -2147211001i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SYSTEM_DISK: ::windows_sys::core::HRESULT = -2147211247i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_PACK_NOT_EMPTY: ::windows_sys::core::HRESULT = -2147212003i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: ::windows_sys::core::HRESULT = -2147211007i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147211514i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TIMEOUT: ::windows_sys::core::HRESULT = -2147212193i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: ::windows_sys::core::HRESULT = -2147211261i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: ::windows_sys::core::HRESULT = -2147211249i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: ::windows_sys::core::HRESULT = -2147211955i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNRECOVERABLE_ERROR: ::windows_sys::core::HRESULT = -2147212263i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: ::windows_sys::core::HRESULT = -2147211915i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_INVALID_OP_STATE: ::windows_sys::core::HRESULT = -2147210982i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_NOT_OPEN: ::windows_sys::core::HRESULT = -2147210983i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_PATHNAME_INVALID: ::windows_sys::core::HRESULT = -2147210969i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_ATTACHED: ::windows_sys::core::HRESULT = -2147210956i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_COMPACTING: ::windows_sys::core::HRESULT = -2147210958i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_DETACHED: ::windows_sys::core::HRESULT = -2147210955i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_MERGING: ::windows_sys::core::HRESULT = -2147210957i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: ::windows_sys::core::HRESULT = -2147210959i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_ALREADY_OPEN: ::windows_sys::core::HRESULT = -2147210960i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_COMPACTING: ::windows_sys::core::HRESULT = -2147210963i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_EXPANDING: ::windows_sys::core::HRESULT = -2147210964i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_MERGING: ::windows_sys::core::HRESULT = -2147210962i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_NOT_OPEN: ::windows_sys::core::HRESULT = -2147210965i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_ATTACHED: ::windows_sys::core::HRESULT = -2147210961i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_BEING_ATTACHED: ::windows_sys::core::HRESULT = -2147210953i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_BEING_DETACHED: ::windows_sys::core::HRESULT = -2147210952i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_NOT_ATTACHED_READONLY: ::windows_sys::core::HRESULT = -2147210954i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: ::windows_sys::core::HRESULT = -2147211991i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE: ::windows_sys::core::HRESULT = -2147211230i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: ::windows_sys::core::HRESULT = -2147211232i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: ::windows_sys::core::HRESULT = -2147211233i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: ::windows_sys::core::HRESULT = -2147211231i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: ::windows_sys::core::HRESULT = -2147210995i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_HAS_PATH: ::windows_sys::core::HRESULT = -2147212194i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_HIDDEN: ::windows_sys::core::HRESULT = -2147211914i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_INCOMPLETE: ::windows_sys::core::HRESULT = -2147212238i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_INVALID_NAME: ::windows_sys::core::HRESULT = -2147212025i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: ::windows_sys::core::HRESULT = -2147211953i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_MIRRORED: ::windows_sys::core::HRESULT = -2147211896i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_A_MIRROR: ::windows_sys::core::HRESULT = -2147212219i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: ::windows_sys::core::HRESULT = -2147211908i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_HEALTHY: ::windows_sys::core::HRESULT = -2147212226i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_MOUNTED: ::windows_sys::core::HRESULT = -2147212209i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_ONLINE: ::windows_sys::core::HRESULT = -2147212227i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_RETAINED: ::windows_sys::core::HRESULT = -2147211952i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_ON_DISK: ::windows_sys::core::HRESULT = -2147212005i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: ::windows_sys::core::HRESULT = -2147212195i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_REGENERATING: ::windows_sys::core::HRESULT = -2147211904i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_RETAINED: ::windows_sys::core::HRESULT = -2147211951i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE: ::windows_sys::core::HRESULT = -2147211243i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: ::windows_sys::core::HRESULT = -2147211245i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: ::windows_sys::core::HRESULT = -2147211246i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: ::windows_sys::core::HRESULT = -2147211244i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SIMPLE_SPANNED: ::windows_sys::core::HRESULT = -2147211895i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SPANS_DISKS: ::windows_sys::core::HRESULT = -2147212225i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SYNCHRONIZING: ::windows_sys::core::HRESULT = -2147211905i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: ::windows_sys::core::HRESULT = -2147212196i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TOO_BIG: ::windows_sys::core::HRESULT = -2147212243i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TOO_SMALL: ::windows_sys::core::HRESULT = -2147212244i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ACCESS_PATH_NOT_DELETED: ::windows_sys::core::HRESULT = 279108i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ALREADY_EXISTS: ::windows_sys::core::HRESULT = 272148i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: ::windows_sys::core::HRESULT = 271414i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: ::windows_sys::core::HRESULT = 271640i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_DISMOUNT_FAILED: ::windows_sys::core::HRESULT = 272393i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_IS_MISSING: ::windows_sys::core::HRESULT = 271624i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_MOUNT_FAILED: ::windows_sys::core::HRESULT = 272392i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_PARTIALLY_CLEANED: ::windows_sys::core::HRESULT = 271386i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISMOUNT_FAILED: ::windows_sys::core::HRESULT = 271735i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: ::windows_sys::core::HRESULT = 271461i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_FS_LOCK: ::windows_sys::core::HRESULT = 271747i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = -2147212183i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = 271450i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_IN_PROGRESS: ::windows_sys::core::HRESULT = 271437i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: ::windows_sys::core::HRESULT = 272386i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: ::windows_sys::core::HRESULT = 272385i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: ::windows_sys::core::HRESULT = 272384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: ::windows_sys::core::HRESULT = 271463i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NAME_TRUNCATED: ::windows_sys::core::HRESULT = 272128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: ::windows_sys::core::HRESULT = 271626i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NO_NOTIFICATION: ::windows_sys::core::HRESULT = 271639i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = 271755i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PROPERTIES_INCOMPLETE: ::windows_sys::core::HRESULT = 272149i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: ::windows_sys::core::HRESULT = 271393i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_REMOUNT_FAILED: ::windows_sys::core::HRESULT = 271736i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: ::windows_sys::core::HRESULT = 271738i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_STATUSES_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = 272130i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_SYSTEM_PARTITION: ::windows_sys::core::HRESULT = 271630i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: ::windows_sys::core::HRESULT = 271451i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_UPDATE_BOOTFILE_FAILED: ::windows_sys::core::HRESULT = 271412i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VOLUME_COMPRESS_FAILED: ::windows_sys::core::HRESULT = 271427i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: ::windows_sys::core::HRESULT = 271745i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VSS_RELEASE_WRITES: ::windows_sys::core::HRESULT = 271746i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_WINPE_BOOTENTRY: ::windows_sys::core::HRESULT = 271758i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ASYNC_OUTPUT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = 50i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 52i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 53i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = 54i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = 55i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = 56i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = 60i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = 61i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = 62i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 63i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = 65i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = 66i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 67i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = 101i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 200i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = 201i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = 202i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 203i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = 204i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_CONTROLLER_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DISK_EXTENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_UNKNOWN: VDS_DISK_EXTENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_FREE: VDS_DISK_EXTENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_DATA: VDS_DISK_EXTENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_OEM: VDS_DISK_EXTENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_ESP: VDS_DISK_EXTENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_MSR: VDS_DISK_EXTENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_LDM: VDS_DISK_EXTENT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_CLUSTER: VDS_DISK_EXTENT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_UNUSABLE: VDS_DISK_EXTENT_TYPE = 32767i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DISK_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_AUDIO_CD: VDS_DISK_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HOTSPARE: VDS_DISK_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_RESERVE_CAPABLE: VDS_DISK_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_MASKED: VDS_DISK_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_STYLE_CONVERTIBLE: VDS_DISK_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CLUSTERED: VDS_DISK_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_READ_ONLY: VDS_DISK_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_SYSTEM_DISK: VDS_DISK_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_BOOT_DISK: VDS_DISK_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_PAGEFILE_DISK: VDS_DISK_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HIBERNATIONFILE_DISK: VDS_DISK_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CRASHDUMP_DISK: VDS_DISK_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HAS_ARC_PATH: VDS_DISK_FLAG = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_DYNAMIC: VDS_DISK_FLAG = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_BOOT_FROM_DISK: VDS_DISK_FLAG = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CURRENT_READ_ONLY: VDS_DISK_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_REFS_NOT_SUPPORTED: VDS_DISK_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DISK_OFFLINE_REASON = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonNone: VDS_DISK_OFFLINE_REASON = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonPolicy: VDS_DISK_OFFLINE_REASON = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonRedundantPath: VDS_DISK_OFFLINE_REASON = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonSnapshot: VDS_DISK_OFFLINE_REASON = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonCollision: VDS_DISK_OFFLINE_REASON = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonResourceExhaustion: VDS_DISK_OFFLINE_REASON = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonWriteFailure: VDS_DISK_OFFLINE_REASON = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonDIScan: VDS_DISK_OFFLINE_REASON = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonLostDataPersistence: VDS_DISK_OFFLINE_REASON = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DISK_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_UNKNOWN: VDS_DISK_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_ONLINE: VDS_DISK_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_NOT_READY: VDS_DISK_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_NO_MEDIA: VDS_DISK_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_FAILED: VDS_DISK_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_MISSING: VDS_DISK_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_OFFLINE: VDS_DISK_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DRIVE_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DRIVE_LETTER_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DLF_NON_PERSISTENT: VDS_DRIVE_LETTER_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_DRIVE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_FILE_SYSTEM_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_FORMAT: VDS_FILE_SYSTEM_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_QUICK_FORMAT: VDS_FILE_SYSTEM_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_COMPRESS: VDS_FILE_SYSTEM_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_SPECIFY_LABEL: VDS_FILE_SYSTEM_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_MOUNT_POINT: VDS_FILE_SYSTEM_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_REMOVABLE_MEDIA: VDS_FILE_SYSTEM_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_EXTEND: VDS_FILE_SYSTEM_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_512: VDS_FILE_SYSTEM_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_1K: VDS_FILE_SYSTEM_FLAG = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_2K: VDS_FILE_SYSTEM_FLAG = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_4K: VDS_FILE_SYSTEM_FLAG = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_8K: VDS_FILE_SYSTEM_FLAG = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_16K: VDS_FILE_SYSTEM_FLAG = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_32K: VDS_FILE_SYSTEM_FLAG = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_64K: VDS_FILE_SYSTEM_FLAG = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_128K: VDS_FILE_SYSTEM_FLAG = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_256K: VDS_FILE_SYSTEM_FLAG = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_DEFAULT: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_PREVIOUS_REVISION: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_RECOMMENDED: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_FILE_SYSTEM_PROP_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FPF_COMPRESSED: VDS_FILE_SYSTEM_PROP_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_FILE_SYSTEM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_FORMAT_OPTION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_NONE: VDS_FORMAT_OPTION_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_FORCE: VDS_FORMAT_OPTION_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_QUICK: VDS_FORMAT_OPTION_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_COMPRESSION: VDS_FORMAT_OPTION_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_DUPLICATE_METADATA: VDS_FORMAT_OPTION_FLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_HBAPORT_SPEED_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_HBAPORT_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_HBAPORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_HEALTH = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_UNKNOWN: VDS_HEALTH = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_HEALTHY: VDS_HEALTH = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_REBUILDING: VDS_HEALTH = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_STALE: VDS_HEALTH = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILING: VDS_HEALTH = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED: VDS_HEALTH = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_REPLACED: VDS_HEALTH = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_DEGRADED: VDS_HEALTH = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_HWPROVIDER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_INTERCONNECT_ADDRESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_INTERCONNECT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_IPADDRESS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ISCSI_AUTH_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ISCSI_IPSEC_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ISCSI_LOGIN_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ISCSI_LOGIN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_ISCSI_PORTAL_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LOADBALANCE_POLICY_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_PLEX_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_PLEX_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_PLEX_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_RESERVE_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_NONE: VDS_LUN_RESERVE_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_EXCLUSIVE_RW: VDS_LUN_RESERVE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_EXCLUSIVE_RO: VDS_LUN_RESERVE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_SHARED_RO: VDS_LUN_RESERVE_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_SHARED_RW: VDS_LUN_RESERVE_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_FAILED: VDS_LUN_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_LUN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_SPAN: VDS_LUN_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_PARITY: VDS_LUN_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID2: VDS_LUN_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID3: VDS_LUN_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID4: VDS_LUN_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID5: VDS_LUN_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID6: VDS_LUN_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID01: VDS_LUN_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID03: VDS_LUN_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID05: VDS_LUN_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID10: VDS_LUN_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID15: VDS_LUN_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID30: VDS_LUN_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID50: VDS_LUN_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID51: VDS_LUN_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID53: VDS_LUN_TYPE = 28i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID60: VDS_LUN_TYPE = 29i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID61: VDS_LUN_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_MAINTENANCE_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const SpinDown: VDS_MAINTENANCE_OPERATION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const SpinUp: VDS_MAINTENANCE_OPERATION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const Ping: VDS_MAINTENANCE_OPERATION = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_CONTROLLER = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_ARRIVE: VDS_NF_CONTROLLER = 103u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_DEPART: VDS_NF_CONTROLLER = 104u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_MODIFY: VDS_NF_CONTROLLER = 350u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_REMOVED: VDS_NF_CONTROLLER = 351u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_DISK = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_ARRIVE: VDS_NF_DISK = 8u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_DEPART: VDS_NF_DISK = 9u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_MODIFY: VDS_NF_DISK = 10u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_DRIVE = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_ARRIVE: VDS_NF_DRIVE = 105u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_DEPART: VDS_NF_DRIVE = 106u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_MODIFY: VDS_NF_DRIVE = 107u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_REMOVED: VDS_NF_DRIVE = 354u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_FILE_SYSTEM = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_MODIFY: VDS_NF_FILE_SYSTEM = 203u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: VDS_NF_FILE_SYSTEM = 204u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_LUN = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_ARRIVE: VDS_NF_LUN = 108u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_DEPART: VDS_NF_LUN = 109u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_MODIFY: VDS_NF_LUN = 110u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_PACK = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_ARRIVE: VDS_NF_PACK = 1u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_DEPART: VDS_NF_PACK = 2u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_MODIFY: VDS_NF_PACK = 3u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NF_PORT = u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_ARRIVE: VDS_NF_PORT = 121u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_DEPART: VDS_NF_PORT = 122u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_MODIFY: VDS_NF_PORT = 352u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_REMOVED: VDS_NF_PORT = 353u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_NOTIFICATION_TARGET_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = 60i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = 61i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 62i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = 63i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = 200i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = 30i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = 31i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = 33i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = 34i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = 35i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = 36i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = 37i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = 38i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = 39i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = 90i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = 91i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = 92i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = 101i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = 200i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = 201i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PACK_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_FOREIGN: VDS_PACK_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_NOQUORUM: VDS_PACK_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_POLICY: VDS_PACK_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_CORRUPTED: VDS_PACK_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_ONLINE_ERROR: VDS_PACK_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PACK_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_UNKNOWN: VDS_PACK_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_ONLINE: VDS_PACK_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_OFFLINE: VDS_PACK_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PARTITION_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PTF_SYSTEM: VDS_PARTITION_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PARTITION_STYLE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_UNKNOWN: VDS_PARTITION_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_MBR: VDS_PARTITION_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_GPT: VDS_PARTITION_STYLE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PATH_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PORT_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PROVIDER_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = -2147483648i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = 1073741824i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = 536870912i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PROVIDER_LBSUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_PROVIDER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_QUERY_PROVIDER_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_SOFTWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_HARDWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_VIRTUALDISK_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_RAID_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID0: VDS_RAID_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID1: VDS_RAID_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID2: VDS_RAID_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID3: VDS_RAID_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID4: VDS_RAID_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID5: VDS_RAID_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID6: VDS_RAID_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID01: VDS_RAID_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID03: VDS_RAID_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID05: VDS_RAID_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID10: VDS_RAID_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID15: VDS_RAID_TYPE = 21i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID30: VDS_RAID_TYPE = 22i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID50: VDS_RAID_TYPE = 23i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID51: VDS_RAID_TYPE = 24i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID53: VDS_RAID_TYPE = 25i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID60: VDS_RAID_TYPE = 26i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID61: VDS_RAID_TYPE = 27i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_RECOVER_ACTION = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_SAN_POLICY = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_UNKNOWN: VDS_SAN_POLICY = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_ONLINE: VDS_SAN_POLICY = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE_SHARED: VDS_SAN_POLICY = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE: VDS_SAN_POLICY = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE_INTERNAL: VDS_SAN_POLICY = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_MAX: VDS_SAN_POLICY = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_SERVICE_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_DYNAMIC: VDS_SERVICE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_FAULT_TOLERANT: VDS_SERVICE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_GPT: VDS_SERVICE_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_DYNAMIC_1394: VDS_SERVICE_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_CLUSTER_SERVICE_CONFIGURED: VDS_SERVICE_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_AUTO_MOUNT_OFF: VDS_SERVICE_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_OS_UNINSTALL_VALID: VDS_SERVICE_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_EFI: VDS_SERVICE_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_MIRROR: VDS_SERVICE_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_RAID5: VDS_SERVICE_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_REFS: VDS_SERVICE_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_STORAGE_BUS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = 127i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_STORAGE_IDENTIFIER_CODE_SET = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_STORAGE_IDENTIFIER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_STORAGE_POOL_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_STORAGE_POOL_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_SUB_SYSTEM_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_SUB_SYSTEM_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_TRANSITION_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VDISK_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_UNKNOWN: VDS_VDISK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ADDED: VDS_VDISK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_OPEN: VDS_VDISK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACH_PENDING: VDS_VDISK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACHED_NOT_OPEN: VDS_VDISK_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACHED: VDS_VDISK_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_DETACH_PENDING: VDS_VDISK_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_COMPACTING: VDS_VDISK_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_MERGING: VDS_VDISK_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_EXPANDING: VDS_VDISK_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_DELETED: VDS_VDISK_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_MAX: VDS_VDISK_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VERSION_SUPPORT_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VOLUME_FLAG = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_SYSTEM_VOLUME: VDS_VOLUME_FLAG = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BOOT_VOLUME: VDS_VOLUME_FLAG = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_ACTIVE: VDS_VOLUME_FLAG = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_READONLY: VDS_VOLUME_FLAG = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_HIDDEN: VDS_VOLUME_FLAG = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CAN_EXTEND: VDS_VOLUME_FLAG = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CAN_SHRINK: VDS_VOLUME_FLAG = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PAGEFILE: VDS_VOLUME_FLAG = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_HIBERNATION: VDS_VOLUME_FLAG = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CRASHDUMP: VDS_VOLUME_FLAG = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_INSTALLABLE: VDS_VOLUME_FLAG = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_LBN_REMAP_ENABLED: VDS_VOLUME_FLAG = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FORMATTING: VDS_VOLUME_FLAG = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NOT_FORMATTABLE: VDS_VOLUME_FLAG = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NTFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FAT32_NOT_SUPPORTED: VDS_VOLUME_FLAG = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FAT_NOT_SUPPORTED: VDS_VOLUME_FLAG = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NO_DEFAULT_DRIVE_LETTER: VDS_VOLUME_FLAG = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PERMANENTLY_DISMOUNTED: VDS_VOLUME_FLAG = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PERMANENT_DISMOUNT_SUPPORTED: VDS_VOLUME_FLAG = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_SHADOW_COPY: VDS_VOLUME_FLAG = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FVE_ENABLED: VDS_VOLUME_FLAG = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_DIRTY: VDS_VOLUME_FLAG = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_REFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BACKS_BOOT_VOLUME: VDS_VOLUME_FLAG = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BACKED_BY_WIM_IMAGE: VDS_VOLUME_FLAG = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VOLUME_PLEX_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_UNKNOWN: VDS_VOLUME_PLEX_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_ONLINE: VDS_VOLUME_PLEX_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_NO_MEDIA: VDS_VOLUME_PLEX_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_FAILED: VDS_VOLUME_PLEX_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VOLUME_PLEX_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_UNKNOWN: VDS_VOLUME_PLEX_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_SIMPLE: VDS_VOLUME_PLEX_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_SPAN: VDS_VOLUME_PLEX_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_STRIPE: VDS_VOLUME_PLEX_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_PARITY: VDS_VOLUME_PLEX_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VOLUME_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_UNKNOWN: VDS_VOLUME_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_ONLINE: VDS_VOLUME_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_NO_MEDIA: VDS_VOLUME_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_FAILED: VDS_VOLUME_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_OFFLINE: VDS_VOLUME_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type VDS_VOLUME_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_UNKNOWN: VDS_VOLUME_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_SIMPLE: VDS_VOLUME_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_SPAN: VDS_VOLUME_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_STRIPE: VDS_VOLUME_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_MIRROR: VDS_VOLUME_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_PARITY: VDS_VOLUME_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub type __VDS_PARTITION_STYLE = i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_MBR: __VDS_PARTITION_STYLE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_GPT: __VDS_PARTITION_STYLE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_RAW: __VDS_PARTITION_STYLE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_ATTRIBUTES_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CHANGE_ATTRIBUTES_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    pub attributes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_PARTITION_TYPE_PARAMETERS_0,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_0,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    pub partitionType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    pub partitionType: u8,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CREATE_PARTITION_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CREATE_PARTITION_PARAMETERS_0 {
    pub MbrPartInfo: CREATE_PARTITION_PARAMETERS_0_1,
    pub GptPartInfo: CREATE_PARTITION_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS_0_0 {
    pub partitionType: ::windows_sys::core::GUID,
    pub partitionId: ::windows_sys::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS_0_1 {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ADVANCEDDISK_PROP {
    pub pwszId: ::windows_sys::core::PWSTR,
    pub pwszPathname: ::windows_sys::core::PWSTR,
    pub pwszLocation: ::windows_sys::core::PWSTR,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pswzIdentifier: ::windows_sys::core::PWSTR,
    pub usIdentifierFormat: u16,
    pub ulNumber: u32,
    pub pwszSerialNumber: ::windows_sys::core::PWSTR,
    pub pwszFirmwareVersion: ::windows_sys::core::PWSTR,
    pub pwszManufacturer: ::windows_sys::core::PWSTR,
    pub pwszModel: ::windows_sys::core::PWSTR,
    pub ullTotalSize: u64,
    pub ullAllocatedSize: u64,
    pub ulLogicalSectorSize: u32,
    pub ulPhysicalSectorSize: u32,
    pub ulPartitionCount: u32,
    pub status: VDS_DISK_STATUS,
    pub health: VDS_HEALTH,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_ADVANCEDDISK_PROP_0,
    pub ulFlags: u32,
    pub dwDeviceType: u32,
}
impl ::core::marker::Copy for VDS_ADVANCEDDISK_PROP {}
impl ::core::clone::Clone for VDS_ADVANCEDDISK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_ADVANCEDDISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_ADVANCEDDISK_PROP_0 {}
impl ::core::clone::Clone for VDS_ADVANCEDDISK_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: VDS_ASYNC_OUTPUT_0_5,
    pub bvp: VDS_ASYNC_OUTPUT_0_0,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: VDS_ASYNC_OUTPUT_0_1,
    pub ct: VDS_ASYNC_OUTPUT_0_4,
    pub cpg: VDS_ASYNC_OUTPUT_0_3,
    pub cvd: VDS_ASYNC_OUTPUT_0_6,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_0 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_1 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_3 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_4 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_5 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_6 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_CONTROLLER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CONTROLLER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
impl ::core::marker::Copy for VDS_CONTROLLER_PROP {}
impl ::core::clone::Clone for VDS_CONTROLLER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CREATE_VDISK_PARAMETERS {
    pub UniqueId: ::windows_sys::core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub pParentPath: ::windows_sys::core::PWSTR,
    pub pSourcePath: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_CREATE_VDISK_PARAMETERS {}
impl ::core::clone::Clone for VDS_CREATE_VDISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_EXTENT {
    pub diskId: ::windows_sys::core::GUID,
    pub r#type: VDS_DISK_EXTENT_TYPE,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub volumeId: ::windows_sys::core::GUID,
    pub plexId: ::windows_sys::core::GUID,
    pub memberIdx: u32,
}
impl ::core::marker::Copy for VDS_DISK_EXTENT {}
impl ::core::clone::Clone for VDS_DISK_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_FREE_EXTENT {
    pub diskId: ::windows_sys::core::GUID,
    pub ullOffset: u64,
    pub ullSize: u64,
}
impl ::core::marker::Copy for VDS_DISK_FREE_EXTENT {}
impl ::core::clone::Clone for VDS_DISK_FREE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DISK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_PROP {
    pub id: ::windows_sys::core::GUID,
    pub status: VDS_DISK_STATUS,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP_0,
    pub pwszDiskAddress: ::windows_sys::core::PWSTR,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszAdaptorName: ::windows_sys::core::PWSTR,
    pub pwszDevicePath: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_DISK_PROP {}
impl ::core::clone::Clone for VDS_DISK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_DISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_PROP_0 {}
impl ::core::clone::Clone for VDS_DISK_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub status: VDS_DISK_STATUS,
    pub OfflineReason: VDS_DISK_OFFLINE_REASON,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP2_0,
    pub pwszDiskAddress: ::windows_sys::core::PWSTR,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszAdaptorName: ::windows_sys::core::PWSTR,
    pub pwszDevicePath: ::windows_sys::core::PWSTR,
    pub pwszLocationPath: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_DISK_PROP2 {}
impl ::core::clone::Clone for VDS_DISK_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_DISK_PROP2_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_PROP2_0 {}
impl ::core::clone::Clone for VDS_DISK_PROP2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_EXTENT {
    pub id: ::windows_sys::core::GUID,
    pub LunId: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_LETTER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_LETTER_PROP {
    pub wcLetter: u16,
    pub volumeId: ::windows_sys::core::GUID,
    pub ulFlags: u32,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_LETTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_LETTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
impl ::core::marker::Copy for VDS_DRIVE_PROP {}
impl ::core::clone::Clone for VDS_DRIVE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
impl ::core::marker::Copy for VDS_DRIVE_PROP2 {}
impl ::core::clone::Clone for VDS_DRIVE_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    pub ulFlags: u32,
    pub usRevision: u16,
    pub ulDefaultUnitAllocationSize: u32,
    pub rgulAllowedUnitAllocationSizes: [u32; 32],
    pub wszName: [u16; 32],
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: ::windows_sys::core::GUID,
    pub dwPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub volumeId: ::windows_sys::core::GUID,
    pub ulFlags: u32,
    pub ullTotalAllocationUnits: u64,
    pub ullAvailableAllocationUnits: u64,
    pub ulAllocationUnitSize: u32,
    pub pwszLabel: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_TYPE_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub wszName: [u16; 8],
    pub ulFlags: u32,
    pub ulCompressionFlags: u32,
    pub ulMaxLableLength: u32,
    pub pwszIllegalLabelCharSet: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_TYPE_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_TYPE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_HBAPORT_PROP {
    pub id: ::windows_sys::core::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl ::core::marker::Copy for VDS_HBAPORT_PROP {}
impl ::core::clone::Clone for VDS_HBAPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_INPUT_DISK {
    pub diskId: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub plexId: ::windows_sys::core::GUID,
    pub memberIdx: u32,
}
impl ::core::marker::Copy for VDS_INPUT_DISK {}
impl ::core::clone::Clone for VDS_INPUT_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl ::core::marker::Copy for VDS_INTERCONNECT {}
impl ::core::clone::Clone for VDS_INTERCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl ::core::marker::Copy for VDS_IPADDRESS {}
impl ::core::clone::Clone for VDS_IPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_IPSEC_KEY {}
impl ::core::clone::Clone for VDS_ISCSI_IPSEC_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: ::windows_sys::core::GUID,
    pub tag: u16,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTALGROUP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_SHARED_SECRET {}
impl ::core::clone::Clone for VDS_ISCSI_SHARED_SECRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszIscsiName: ::windows_sys::core::PWSTR,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_TARGET_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: ::windows_sys::core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_LUN_NOTIFICATION {}
impl ::core::clone::Clone for VDS_LUN_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_PLEX_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PLEX_PROP {}
impl ::core::clone::Clone for VDS_LUN_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub pwszUnmaskingList: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PROP {}
impl ::core::clone::Clone for VDS_LUN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_MOUNT_POINT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl ::core::marker::Copy for VDS_NOTIFICATION {}
impl ::core::clone::Clone for VDS_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl ::core::marker::Copy for VDS_NOTIFICATION_0 {}
impl ::core::clone::Clone for VDS_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PACK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PACK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PACK_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub status: VDS_PACK_STATUS,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for VDS_PACK_PROP {}
impl ::core::clone::Clone for VDS_PACK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_INFORMATION_EX {
    pub dwPartitionStyle: __VDS_PARTITION_STYLE,
    pub ullStartingOffset: u64,
    pub ullPartitionLength: u64,
    pub dwPartitionNumber: u32,
    pub bRewritePartition: super::super::Foundation::BOOLEAN,
    pub Anonymous: VDS_PARTITION_INFORMATION_EX_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VDS_PARTITION_INFORMATION_EX_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PARTITION_INFO_GPT {
    pub partitionType: ::windows_sys::core::GUID,
    pub partitionId: ::windows_sys::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl ::core::marker::Copy for VDS_PARTITION_INFO_GPT {}
impl ::core::clone::Clone for VDS_PARTITION_INFO_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_INFO_MBR {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
    pub recognizedPartition: super::super::Foundation::BOOLEAN,
    pub hiddenSectors: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFO_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFO_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: ::windows_sys::core::GUID,
    pub ullOffset: u64,
}
impl ::core::marker::Copy for VDS_PARTITION_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PARTITION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_PROP {
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub ulFlags: u32,
    pub ulPartitionNumber: u32,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub Anonymous: VDS_PARTITION_PROP_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VDS_PARTITION_PROP_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_PROP_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl ::core::marker::Copy for VDS_PATH_ID {}
impl ::core::clone::Clone for VDS_PATH_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl ::core::marker::Copy for VDS_PATH_INFO {}
impl ::core::clone::Clone for VDS_PATH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: ::windows_sys::core::GUID,
    pub targetPortalId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_0 {}
impl ::core::clone::Clone for VDS_PATH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: ::windows_sys::core::GUID,
    pub initiatorAdapterId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_1 {}
impl ::core::clone::Clone for VDS_PATH_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl ::core::marker::Copy for VDS_PATH_INFO_2 {}
impl ::core::clone::Clone for VDS_PATH_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PATH_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: ::windows_sys::core::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszValue: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_POOL_CUSTOM_ATTRIBUTES {}
impl ::core::clone::Clone for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_GROUP_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORT_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub status: VDS_PORT_STATUS,
}
impl ::core::marker::Copy for VDS_PORT_PROP {}
impl ::core::clone::Clone for VDS_PORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PROVIDER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub guidVersionId: ::windows_sys::core::GUID,
    pub pwszVersion: ::windows_sys::core::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_PROVIDER_PROP {}
impl ::core::clone::Clone for VDS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_REPARSE_POINT_PROP {
    pub SourceVolumeId: ::windows_sys::core::GUID,
    pub pwszPath: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_REPARSE_POINT_PROP {}
impl ::core::clone::Clone for VDS_REPARSE_POINT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl ::core::marker::Copy for VDS_SERVICE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SERVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SERVICE_PROP {
    pub pwszVersion: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for VDS_SERVICE_PROP {}
impl ::core::clone::Clone for VDS_SERVICE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl ::core::marker::Copy for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pwszDescription: ::windows_sys::core::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
impl ::core::marker::Copy for VDS_STORAGE_POOL_PROP {}
impl ::core::clone::Clone for VDS_STORAGE_POOL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub pwszIdentification: ::windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP2 {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_TARGET_NOTIFICATION {}
impl ::core::clone::Clone for VDS_TARGET_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
pub struct VDS_VDISK_PROPERTIES {
    pub Id: ::windows_sys::core::GUID,
    pub State: VDS_VDISK_STATE,
    pub VirtualDeviceType: super::Vhd::VIRTUAL_STORAGE_TYPE,
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub pPath: ::windows_sys::core::PWSTR,
    pub pDeviceName: ::windows_sys::core::PWSTR,
    pub DiskFlag: super::Vhd::DEPENDENT_DISK_FLAG,
    pub bIsChild: super::super::Foundation::BOOL,
    pub pParentPath: ::windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::marker::Copy for VDS_VDISK_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::clone::Clone for VDS_VDISK_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows_sys::core::GUID,
    pub plexId: ::windows_sys::core::GUID,
    pub ulPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_NOTIFICATION {}
impl ::core::clone::Clone for VDS_VOLUME_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PLEX_PROP {
    pub id: ::windows_sys::core::GUID,
    pub r#type: VDS_VOLUME_PLEX_TYPE,
    pub status: VDS_VOLUME_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulStripeSize: u32,
    pub ulNumberOfMembers: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_PLEX_PROP {}
impl ::core::clone::Clone for VDS_VOLUME_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PROP {
    pub id: ::windows_sys::core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub pwszName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VDS_VOLUME_PROP {}
impl ::core::clone::Clone for VDS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub cbUniqueId: u32,
    pub pwszName: ::windows_sys::core::PWSTR,
    pub pUniqueId: *mut u8,
}
impl ::core::marker::Copy for VDS_VOLUME_PROP2 {}
impl ::core::clone::Clone for VDS_VOLUME_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl ::core::marker::Copy for VDS_WWN {}
impl ::core::clone::Clone for VDS_WWN {
    fn clone(&self) -> Self {
        *self
    }
}
