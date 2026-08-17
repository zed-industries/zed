// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub const SIGNATURE_LENGTH: usize = 4;

// Local file header constants
//
// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#437
pub const LFH_SIGNATURE: u32 = 0x4034b50;
#[allow(dead_code)]
pub const LFH_LENGTH: usize = 26;

// Central directory header constants
//
// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4312
pub const CDH_SIGNATURE: u32 = 0x2014b50;
#[allow(dead_code)]
pub const CDH_LENGTH: usize = 42;

// End of central directory record constants
//
// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316
pub const EOCDR_SIGNATURE: u32 = 0x6054b50;
/// The minimum length of the EOCDR, excluding the signature.
pub const EOCDR_LENGTH: usize = 18;

/// The signature for the zip64 end of central directory record.
/// Ref: https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4314
pub const ZIP64_EOCDR_SIGNATURE: u32 = 0x06064b50;
/// The signature for the zip64 end of central directory locator.
/// Ref: https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4315
pub const ZIP64_EOCDL_SIGNATURE: u32 = 0x07064b50;
/// The length of the ZIP64 EOCDL, including the signature.
/// The EOCDL has a fixed size, thankfully.
pub const ZIP64_EOCDL_LENGTH: u64 = 20;

/// The contents of a header field when one must reference the zip64 version instead.
pub const NON_ZIP64_MAX_SIZE: u32 = 0xFFFFFFFF;
/// The maximum number of files or disks in a ZIP file before it requires ZIP64.
pub const NON_ZIP64_MAX_NUM_FILES: u16 = 0xFFFF;

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#439
pub const DATA_DESCRIPTOR_SIGNATURE: u32 = 0x8074b50;
pub const DATA_DESCRIPTOR_LENGTH: usize = 12;
