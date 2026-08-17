pub type IDedupBackupSupport = *mut ::core::ffi::c_void;
pub type IDedupChunkLibrary = *mut ::core::ffi::c_void;
pub type IDedupDataPort = *mut ::core::ffi::c_void;
pub type IDedupDataPortManager = *mut ::core::ffi::c_void;
pub type IDedupIterateChunksHash32 = *mut ::core::ffi::c_void;
pub type IDedupReadFileCallback = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupBackupSupport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPort: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DEDUP_BACKUP_SUPPORT_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DEDUP_SET_PARAM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupChunkFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_None: DedupChunkFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_Compressed: DedupChunkFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupChunkingAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupCompressionAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortManagerOption = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortRequestStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = 5i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupDataPortVolumeStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = 3i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = 4i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = 5i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = 6i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub type DedupHashingAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl ::core::marker::Copy for DDP_FILE_EXTENT {}
impl ::core::clone::Clone for DDP_FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl ::core::marker::Copy for DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::clone::Clone for DEDUP_CHUNK_INFO_HASH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for DEDUP_CONTAINER_EXTENT {}
impl ::core::clone::Clone for DEDUP_CONTAINER_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl ::core::marker::Copy for DedupChunk {}
impl ::core::clone::Clone for DedupChunk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl ::core::marker::Copy for DedupHash {}
impl ::core::clone::Clone for DedupHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupStream {
    pub Path: ::windows_sys::core::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
impl ::core::marker::Copy for DedupStream {}
impl ::core::clone::Clone for DedupStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl ::core::marker::Copy for DedupStreamEntry {}
impl ::core::clone::Clone for DedupStreamEntry {
    fn clone(&self) -> Self {
        *self
    }
}
