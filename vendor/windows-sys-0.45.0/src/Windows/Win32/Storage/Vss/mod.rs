::windows_sys::core::link ! ( "vssapi.dll""system" #[doc = "*Required features: `\"Win32_Storage_Vss\"`*"] fn CreateVssExpressWriterInternal ( ppwriter : *mut IVssExpressWriter ) -> :: windows_sys::core::HRESULT );
pub type IVssAdmin = *mut ::core::ffi::c_void;
pub type IVssAdminEx = *mut ::core::ffi::c_void;
pub type IVssAsync = *mut ::core::ffi::c_void;
pub type IVssComponent = *mut ::core::ffi::c_void;
pub type IVssComponentEx = *mut ::core::ffi::c_void;
pub type IVssComponentEx2 = *mut ::core::ffi::c_void;
pub type IVssCreateExpressWriterMetadata = *mut ::core::ffi::c_void;
pub type IVssCreateWriterMetadata = *mut ::core::ffi::c_void;
pub type IVssDifferentialSoftwareSnapshotMgmt = *mut ::core::ffi::c_void;
pub type IVssDifferentialSoftwareSnapshotMgmt2 = *mut ::core::ffi::c_void;
pub type IVssDifferentialSoftwareSnapshotMgmt3 = *mut ::core::ffi::c_void;
pub type IVssEnumMgmtObject = *mut ::core::ffi::c_void;
pub type IVssEnumObject = *mut ::core::ffi::c_void;
pub type IVssExpressWriter = *mut ::core::ffi::c_void;
pub type IVssFileShareSnapshotProvider = *mut ::core::ffi::c_void;
pub type IVssHardwareSnapshotProvider = *mut ::core::ffi::c_void;
pub type IVssHardwareSnapshotProviderEx = *mut ::core::ffi::c_void;
pub type IVssProviderCreateSnapshotSet = *mut ::core::ffi::c_void;
pub type IVssProviderNotifications = *mut ::core::ffi::c_void;
pub type IVssSnapshotMgmt = *mut ::core::ffi::c_void;
pub type IVssSnapshotMgmt2 = *mut ::core::ffi::c_void;
pub type IVssSoftwareSnapshotProvider = *mut ::core::ffi::c_void;
pub type IVssWMDependency = *mut ::core::ffi::c_void;
pub type IVssWMFiledesc = *mut ::core::ffi::c_void;
pub type IVssWriterComponents = *mut ::core::ffi::c_void;
pub type IVssWriterImpl = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSSCoordinator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows_sys::core::HRESULT = -2147212280i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows_sys::core::HRESULT = -2147212267i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows_sys::core::HRESULT = -2147212282i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows_sys::core::HRESULT = -2147212287i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows_sys::core::HRESULT = -2147212286i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212278i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows_sys::core::HRESULT = -2147212268i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows_sys::core::HRESULT = -2147212284i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows_sys::core::HRESULT = -2147212285i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows_sys::core::HRESULT = -2147212269i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows_sys::core::HRESULT = -2147212281i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212270i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows_sys::core::HRESULT = -2147212283i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows_sys::core::HRESULT = -2147212266i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_AUTORECOVERY_FAILED: ::windows_sys::core::HRESULT = -2147212293i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BAD_STATE: ::windows_sys::core::HRESULT = -2147212543i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows_sys::core::HRESULT = -2147212298i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows_sys::core::HRESULT = -2147212290i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_ERROR: ::windows_sys::core::HRESULT = -2147212288i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_TIMEOUT: ::windows_sys::core::HRESULT = -2147212498i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows_sys::core::HRESULT = -2147212528i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows_sys::core::HRESULT = -2147212271i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows_sys::core::HRESULT = -2147212292i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = -2147212525i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FSS_TIMEOUT: ::windows_sys::core::HRESULT = -2147212265i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = -2147212524i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows_sys::core::HRESULT = -2147212513i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows_sys::core::HRESULT = -2147212527i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_LEGACY_PROVIDER: ::windows_sys::core::HRESULT = -2147212297i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows_sys::core::HRESULT = -2147212514i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows_sys::core::HRESULT = -2147212510i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows_sys::core::HRESULT = -2147212521i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows_sys::core::HRESULT = -2147212526i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_DISK: ::windows_sys::core::HRESULT = -2147212296i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows_sys::core::HRESULT = -2147212295i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_VOLUME: ::windows_sys::core::HRESULT = -2147212294i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows_sys::core::HRESULT = -2147212500i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows_sys::core::HRESULT = -2147212291i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212497i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows_sys::core::HRESULT = -2147212512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147212531i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = -2147212536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = -2147212541i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_IN_USE: ::windows_sys::core::HRESULT = -2147212537i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = -2147212540i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_VETO: ::windows_sys::core::HRESULT = -2147212538i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = -2147212505i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = -2147212509i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows_sys::core::HRESULT = -2147212508i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212289i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212507i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_VOLUME_LOST: ::windows_sys::core::HRESULT = -2147212506i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows_sys::core::HRESULT = -2147212501i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows_sys::core::HRESULT = -2147212522i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = -2147212511i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows_sys::core::HRESULT = -2147212504i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows_sys::core::HRESULT = -2147212503i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED: ::windows_sys::core::HRESULT = -2147212542i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows_sys::core::HRESULT = -2147212529i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows_sys::core::HRESULT = -2147212523i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSELECTED_VOLUME: ::windows_sys::core::HRESULT = -2147212502i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows_sys::core::HRESULT = -2147212517i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_IN_USE: ::windows_sys::core::HRESULT = -2147212515i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows_sys::core::HRESULT = -2147212499i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147212532i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows_sys::core::HRESULT = -2147212530i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows_sys::core::HRESULT = -2147212304i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows_sys::core::HRESULT = -2147212300i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows_sys::core::HRESULT = -2147212303i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows_sys::core::HRESULT = -2147212490i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows_sys::core::HRESULT = -2147212299i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows_sys::core::HRESULT = -2147212301i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows_sys::core::HRESULT = -2147212302i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows_sys::core::HRESULT = -2147212518i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows_sys::core::HRESULT = -2147212520i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows_sys::core::HRESULT = -2147212519i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = -2147212279i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_CANCELLED: ::windows_sys::core::HRESULT = 271115i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_FINISHED: ::windows_sys::core::HRESULT = 271114i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_PENDING: ::windows_sys::core::HRESULT = 271113i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = 271137i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VssSnapshotMgmt: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_ALTERNATE_WRITER_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_APPLICATION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_BACKUP_SCHEMA = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_BACKUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_COMPONENT_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_COMPONENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_FILE_RESTORE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_FILE_SPEC_BACKUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 3840i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_HARDWARE_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_MGMT_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROTECTION_FAULT = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROTECTION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROVIDER_CAPABILITIES = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_PROVIDER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RECOVERY_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTOREMETHOD_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTORE_TARGET = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_RESTORE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_ROLLFORWARD_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_COMPATIBILITY = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_CONTEXT = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 25i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = 29i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_PROPERTY_ID = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SNAPSHOT_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SOURCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_SUBSCRIBE_MASK = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_USAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_VOLUME_SNAPSHOT_ATTRIBUTES = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 32i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 64i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 128i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 67108864i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_WRITERRESTORE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub type VSS_WRITER_STATE = i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_STABLE: VSS_WRITER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_COUNT: VSS_WRITER_STATE = 16i32;
#[repr(C)]
pub struct IVssExamineWriterMetadata(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_AREA_PROP {}
impl ::core::clone::Clone for VSS_DIFF_AREA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_DIFF_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl ::core::marker::Copy for VSS_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows_sys::core::GUID,
    pub m_ClassId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VSS_PROVIDER_PROP {}
impl ::core::clone::Clone for VSS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows_sys::core::GUID,
    pub m_SnapshotSetId: ::windows_sys::core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl ::core::marker::Copy for VSS_SNAPSHOT_PROP {}
impl ::core::clone::Clone for VSS_SNAPSHOT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl ::core::marker::Copy for VSS_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VSS_VOLUME_PROTECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
