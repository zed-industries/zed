pub type IFindSimilarResults = *mut ::core::ffi::c_void;
pub type IRdcComparator = *mut ::core::ffi::c_void;
pub type IRdcFileReader = *mut ::core::ffi::c_void;
pub type IRdcFileWriter = *mut ::core::ffi::c_void;
pub type IRdcGenerator = *mut ::core::ffi::c_void;
pub type IRdcGeneratorFilterMaxParameters = *mut ::core::ffi::c_void;
pub type IRdcGeneratorParameters = *mut ::core::ffi::c_void;
pub type IRdcLibrary = *mut ::core::ffi::c_void;
pub type IRdcSignatureReader = *mut ::core::ffi::c_void;
pub type IRdcSimilarityGenerator = *mut ::core::ffi::c_void;
pub type ISimilarity = *mut ::core::ffi::c_void;
pub type ISimilarityFileIdTable = *mut ::core::ffi::c_void;
pub type ISimilarityReportProgress = *mut ::core::ffi::c_void;
pub type ISimilarityTableDumpState = *mut ::core::ffi::c_void;
pub type ISimilarityTraitsMappedView = *mut ::core::ffi::c_void;
pub type ISimilarityTraitsMapping = *mut ::core::ffi::c_void;
pub type ISimilarityTraitsTable = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const FindSimilarResults: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcComparator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcFileReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGeneratorFilterMaxParameters: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGeneratorParameters: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcSignatureReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcSimilarityGenerator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const Similarity: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMaxSize: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMinSize: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityReportProgress: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTableDumpState: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsMappedView: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsMapping: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type GeneratorParametersType = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_Unused: GeneratorParametersType = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RDC_ErrorCode = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_NoError: RDC_ErrorCode = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = 3i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderWrongType: RDC_ErrorCode = 4i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = 5i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataTooManyRecords: RDC_ErrorCode = 6i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = 7i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_ApplicationError: RDC_ErrorCode = 8i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Aborted: RDC_ErrorCode = 9i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Win32Error: RDC_ErrorCode = 10i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcCreatedTables = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_Existing: RdcCreatedTables = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_New: RdcCreatedTables = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcMappingAccessMode = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub type RdcNeedType = i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SOURCE: RdcNeedType = 0i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_TARGET: RdcNeedType = 1i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED: RdcNeedType = 2i32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED_MAX: RdcNeedType = 255i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl ::core::marker::Copy for FindSimilarFileIndexResults {}
impl ::core::clone::Clone for FindSimilarFileIndexResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl ::core::marker::Copy for RdcBufferPointer {}
impl ::core::clone::Clone for RdcBufferPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl ::core::marker::Copy for RdcNeed {}
impl ::core::clone::Clone for RdcNeed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl ::core::marker::Copy for RdcNeedPointer {}
impl ::core::clone::Clone for RdcNeedPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl ::core::marker::Copy for RdcSignature {}
impl ::core::clone::Clone for RdcSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl ::core::marker::Copy for RdcSignaturePointer {}
impl ::core::clone::Clone for RdcSignaturePointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl ::core::marker::Copy for SimilarityData {}
impl ::core::clone::Clone for SimilarityData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl ::core::marker::Copy for SimilarityDumpData {}
impl ::core::clone::Clone for SimilarityDumpData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl ::core::marker::Copy for SimilarityFileId {}
impl ::core::clone::Clone for SimilarityFileId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl ::core::marker::Copy for SimilarityMappedViewInfo {}
impl ::core::clone::Clone for SimilarityMappedViewInfo {
    fn clone(&self) -> Self {
        *self
    }
}
