// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#437
pub struct LocalFileHeader {
    pub version: u16,
    pub flags: GeneralPurposeFlag,
    pub compression: u16,
    pub mod_time: u16,
    pub mod_date: u16,
    pub crc: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#444
#[derive(Copy, Clone)]
pub struct GeneralPurposeFlag {
    pub encrypted: bool,
    pub data_descriptor: bool,
    pub filename_unicode: bool,
}

/// 2 byte header ids
/// Ref https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#452
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HeaderId(pub u16);

impl HeaderId {
    pub const ZIP64_EXTENDED_INFORMATION_EXTRA_FIELD: HeaderId = HeaderId(0x0001);
    pub const INFO_ZIP_UNICODE_COMMENT_EXTRA_FIELD: HeaderId = HeaderId(0x6375);
    pub const INFO_ZIP_UNICODE_PATH_EXTRA_FIELD: HeaderId = HeaderId(0x7075);
}

impl From<u16> for HeaderId {
    fn from(value: u16) -> Self {
        HeaderId(value)
    }
}

impl From<HeaderId> for u16 {
    fn from(value: HeaderId) -> Self {
        value.0
    }
}

/// Represents each extra field.
/// Not strictly part of the spec, but is the most useful way to represent the data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ExtraField {
    Zip64ExtendedInformation(Zip64ExtendedInformationExtraField),
    InfoZipUnicodeComment(InfoZipUnicodeCommentExtraField),
    InfoZipUnicodePath(InfoZipUnicodePathExtraField),
    Unknown(UnknownExtraField),
}

/// An extended information header for Zip64.
/// This field is used both for local file headers and central directory records.
/// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#453
#[derive(Clone, Debug)]
pub struct Zip64ExtendedInformationExtraField {
    pub header_id: HeaderId,
    pub uncompressed_size: Option<u64>,
    pub compressed_size: Option<u64>,
    // While not specified in the spec, these two fields are often left out in practice.
    pub relative_header_offset: Option<u64>,
    pub disk_start_number: Option<u32>,
}

impl Zip64ExtendedInformationExtraField {
    pub(crate) fn content_size(&self) -> usize {
        self.uncompressed_size.map(|_| 8).unwrap_or_default()
            + self.compressed_size.map(|_| 8).unwrap_or_default()
            + self.relative_header_offset.map(|_| 8).unwrap_or_default()
            + self.disk_start_number.map(|_| 8).unwrap_or_default()
    }
}

/// Stores the UTF-8 version of the file comment as stored in the central directory header.
/// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#468
#[derive(Clone, Debug)]
pub enum InfoZipUnicodeCommentExtraField {
    V1 { crc32: u32, unicode: Vec<u8> },
    Unknown { version: u8, data: Vec<u8> },
}

/// Stores the UTF-8 version of the file name field as stored in the local header and central directory header.
/// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#469
#[derive(Clone, Debug)]
pub enum InfoZipUnicodePathExtraField {
    V1 { crc32: u32, unicode: Vec<u8> },
    Unknown { version: u8, data: Vec<u8> },
}

/// Represents any unparsed extra field.
#[derive(Clone, Debug)]
pub struct UnknownExtraField {
    pub header_id: HeaderId,
    pub data_size: u16,
    pub content: Vec<u8>,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4312
pub struct CentralDirectoryRecord {
    pub v_made_by: u16,
    pub v_needed: u16,
    pub flags: GeneralPurposeFlag,
    pub compression: u16,
    pub mod_time: u16,
    pub mod_date: u16,
    pub crc: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_start: u16,
    pub inter_attr: u16,
    pub exter_attr: u32,
    pub lh_offset: u32,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316
#[derive(Debug)]
pub struct EndOfCentralDirectoryHeader {
    pub(crate) disk_num: u16,
    pub(crate) start_cent_dir_disk: u16,
    pub(crate) num_of_entries_disk: u16,
    pub(crate) num_of_entries: u16,
    pub(crate) size_cent_dir: u32,
    pub(crate) cent_dir_offset: u32,
    pub(crate) file_comm_length: u16,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4314
#[derive(Debug, PartialEq)]
pub struct Zip64EndOfCentralDirectoryRecord {
    /// The size of this Zip64EndOfCentralDirectoryRecord.
    /// This is specified because there is a variable-length extra zip64 information sector.
    /// However, we will gleefully ignore this sector because it is reserved for use by PKWare.
    pub size_of_zip64_end_of_cd_record: u64,
    pub version_made_by: u16,
    pub version_needed_to_extract: u16,
    pub disk_number: u32,
    pub disk_number_start_of_cd: u32,
    pub num_entries_in_directory_on_disk: u64,
    pub num_entries_in_directory: u64,
    pub directory_size: u64,
    pub offset_of_start_of_directory: u64,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4315
#[derive(Debug, PartialEq)]
pub struct Zip64EndOfCentralDirectoryLocator {
    pub number_of_disk_with_start_of_zip64_end_of_central_directory: u32,
    pub relative_offset: u64,
    pub total_number_of_disks: u32,
}
