// Copyright (c) 2022-2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A module which supports reading ZIP files.

pub mod mem;
pub mod seek;
pub mod stream;

pub(crate) mod io;

use crate::ZipString;
// Re-exported as part of the public API.
pub use crate::base::read::io::entry::WithEntry;
pub use crate::base::read::io::entry::WithoutEntry;
pub use crate::base::read::io::entry::ZipEntryReader;

use crate::date::ZipDateTime;
use crate::entry::{StoredZipEntry, ZipEntry};
use crate::error::{Result, ZipError};
use crate::file::ZipFile;
use crate::spec::attribute::AttributeCompatibility;
use crate::spec::consts::LFH_LENGTH;
use crate::spec::consts::{CDH_SIGNATURE, LFH_SIGNATURE, NON_ZIP64_MAX_SIZE, SIGNATURE_LENGTH, ZIP64_EOCDL_LENGTH};
use crate::spec::header::InfoZipUnicodeCommentExtraField;
use crate::spec::header::InfoZipUnicodePathExtraField;
use crate::spec::header::{
    CentralDirectoryRecord, EndOfCentralDirectoryHeader, ExtraField, LocalFileHeader,
    Zip64EndOfCentralDirectoryLocator, Zip64EndOfCentralDirectoryRecord, Zip64ExtendedInformationExtraField,
};
use crate::spec::Compression;
use crate::string::StringEncoding;

use crate::base::read::io::CombinedCentralDirectoryRecord;
use crate::spec::parse::parse_extra_fields;

use futures_lite::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, SeekFrom};

pub(crate) async fn file<R>(mut reader: R) -> Result<ZipFile>
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    // First find and parse the EOCDR.
    let eocdr_offset = crate::base::read::io::locator::eocdr(&mut reader).await?;

    reader.seek(SeekFrom::Start(eocdr_offset)).await?;
    let eocdr = EndOfCentralDirectoryHeader::from_reader(&mut reader).await?;

    let comment = io::read_string(&mut reader, eocdr.file_comm_length.into(), crate::StringEncoding::Utf8).await?;

    // Check the 20 bytes before the EOCDR for the Zip64 EOCDL, plus an extra 4 bytes because the offset
    // does not include the signature. If the ECODL exists we are dealing with a Zip64 file.
    let (eocdr, zip64) = match eocdr_offset.checked_sub(ZIP64_EOCDL_LENGTH + SIGNATURE_LENGTH as u64) {
        None => (CombinedCentralDirectoryRecord::from(&eocdr), false),
        Some(offset) => {
            reader.seek(SeekFrom::Start(offset)).await?;
            let zip64_locator = Zip64EndOfCentralDirectoryLocator::try_from_reader(&mut reader).await?;

            match zip64_locator {
                Some(locator) => {
                    reader.seek(SeekFrom::Start(locator.relative_offset + SIGNATURE_LENGTH as u64)).await?;
                    let zip64_eocdr = Zip64EndOfCentralDirectoryRecord::from_reader(&mut reader).await?;
                    (CombinedCentralDirectoryRecord::combine(eocdr, zip64_eocdr), true)
                }
                None => (CombinedCentralDirectoryRecord::from(&eocdr), false),
            }
        }
    };

    // Outdated feature so unlikely to ever make it into this crate.
    if eocdr.disk_number != eocdr.disk_number_start_of_cd
        || eocdr.num_entries_in_directory != eocdr.num_entries_in_directory_on_disk
    {
        return Err(ZipError::FeatureNotSupported("Spanned/split files"));
    }

    // Find and parse the central directory.
    reader.seek(SeekFrom::Start(eocdr.offset_of_start_of_directory)).await?;
    let entries = crate::base::read::cd(reader, eocdr.num_entries_in_directory, zip64).await?;

    Ok(ZipFile { entries, comment, zip64 })
}

pub(crate) async fn cd<R>(mut reader: R, num_of_entries: u64, zip64: bool) -> Result<Vec<StoredZipEntry>>
where
    R: AsyncRead + Unpin,
{
    let num_of_entries = num_of_entries.try_into().map_err(|_| ZipError::TargetZip64NotSupported)?;
    let mut entries = Vec::with_capacity(num_of_entries);

    for _ in 0..num_of_entries {
        let entry = cd_record(&mut reader, zip64).await?;
        entries.push(entry);
    }

    Ok(entries)
}

pub(crate) fn get_zip64_extra_field(extra_fields: &[ExtraField]) -> Option<&Zip64ExtendedInformationExtraField> {
    for field in extra_fields {
        if let ExtraField::Zip64ExtendedInformation(zip64field) = field {
            return Some(zip64field);
        }
    }
    None
}

pub(crate) fn get_zip64_extra_field_mut(
    extra_fields: &mut [ExtraField],
) -> Option<&mut Zip64ExtendedInformationExtraField> {
    for field in extra_fields {
        if let ExtraField::Zip64ExtendedInformation(zip64field) = field {
            return Some(zip64field);
        }
    }
    None
}

fn get_combined_sizes(
    uncompressed_size: u32,
    compressed_size: u32,
    extra_field: &Option<&Zip64ExtendedInformationExtraField>,
) -> Result<(u64, u64)> {
    let mut uncompressed_size = uncompressed_size as u64;
    let mut compressed_size = compressed_size as u64;

    if let Some(extra_field) = extra_field {
        if let Some(s) = extra_field.uncompressed_size {
            uncompressed_size = s;
        }
        if let Some(s) = extra_field.compressed_size {
            compressed_size = s;
        }
    }

    Ok((uncompressed_size, compressed_size))
}

pub(crate) async fn cd_record<R>(mut reader: R, _zip64: bool) -> Result<StoredZipEntry>
where
    R: AsyncRead + Unpin,
{
    crate::utils::assert_signature(&mut reader, CDH_SIGNATURE).await?;

    let header = CentralDirectoryRecord::from_reader(&mut reader).await?;
    let header_size = (SIGNATURE_LENGTH + LFH_LENGTH) as u64;
    let trailing_size = header.file_name_length as u64 + header.extra_field_length as u64;
    let filename_basic = io::read_bytes(&mut reader, header.file_name_length.into()).await?;
    let compression = Compression::try_from(header.compression)?;
    let extra_field = io::read_bytes(&mut reader, header.extra_field_length.into()).await?;
    let extra_fields = parse_extra_fields(extra_field, header.uncompressed_size, header.compressed_size)?;
    let comment_basic = io::read_bytes(reader, header.file_comment_length.into()).await?;

    let zip64_extra_field = get_zip64_extra_field(&extra_fields);
    let (uncompressed_size, compressed_size) =
        get_combined_sizes(header.uncompressed_size, header.compressed_size, &zip64_extra_field)?;

    let mut file_offset = header.lh_offset as u64;
    if let Some(zip64_extra_field) = zip64_extra_field {
        if file_offset == NON_ZIP64_MAX_SIZE as u64 {
            if let Some(offset) = zip64_extra_field.relative_header_offset {
                file_offset = offset;
            }
        }
    }

    let filename = detect_filename(filename_basic, header.flags.filename_unicode, extra_fields.as_ref());
    let comment = detect_comment(comment_basic, header.flags.filename_unicode, extra_fields.as_ref());

    let entry = ZipEntry {
        filename,
        compression,
        #[cfg(any(
            feature = "deflate",
            feature = "bzip2",
            feature = "zstd",
            feature = "lzma",
            feature = "xz",
            feature = "deflate64"
        ))]
        compression_level: async_compression::Level::Default,
        attribute_compatibility: AttributeCompatibility::Unix,
        // FIXME: Default to Unix for the moment
        crc32: header.crc,
        uncompressed_size,
        compressed_size,
        last_modification_date: ZipDateTime { date: header.mod_date, time: header.mod_time },
        internal_file_attribute: header.inter_attr,
        external_file_attribute: header.exter_attr,
        extra_fields,
        comment,
        data_descriptor: header.flags.data_descriptor,
    };

    Ok(StoredZipEntry { entry, file_offset, header_size: header_size + trailing_size })
}

pub(crate) async fn lfh<R>(mut reader: R) -> Result<Option<ZipEntry>>
where
    R: AsyncRead + Unpin,
{
    let signature = {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer).await?;
        u32::from_le_bytes(buffer)
    };
    match signature {
        actual if actual == LFH_SIGNATURE => (),
        actual if actual == CDH_SIGNATURE => return Ok(None),
        actual => return Err(ZipError::UnexpectedHeaderError(actual, LFH_SIGNATURE)),
    };

    let header = LocalFileHeader::from_reader(&mut reader).await?;
    let filename_basic = io::read_bytes(&mut reader, header.file_name_length.into()).await?;
    let compression = Compression::try_from(header.compression)?;
    let extra_field = io::read_bytes(&mut reader, header.extra_field_length.into()).await?;
    let extra_fields = parse_extra_fields(extra_field, header.uncompressed_size, header.compressed_size)?;

    let zip64_extra_field = get_zip64_extra_field(&extra_fields);
    let (uncompressed_size, compressed_size) =
        get_combined_sizes(header.uncompressed_size, header.compressed_size, &zip64_extra_field)?;

    if header.flags.data_descriptor && compression == Compression::Stored {
        return Err(ZipError::FeatureNotSupported(
            "stream reading entries with data descriptors & Stored compression mode",
        ));
    }
    if header.flags.encrypted {
        return Err(ZipError::FeatureNotSupported("encryption"));
    }

    let filename = detect_filename(filename_basic, header.flags.filename_unicode, extra_fields.as_ref());

    let entry = ZipEntry {
        filename,
        compression,
        #[cfg(any(
            feature = "deflate",
            feature = "bzip2",
            feature = "zstd",
            feature = "lzma",
            feature = "xz",
            feature = "deflate64"
        ))]
        compression_level: async_compression::Level::Default,
        attribute_compatibility: AttributeCompatibility::Unix,
        // FIXME: Default to Unix for the moment
        crc32: header.crc,
        uncompressed_size,
        compressed_size,
        last_modification_date: ZipDateTime { date: header.mod_date, time: header.mod_time },
        internal_file_attribute: 0,
        external_file_attribute: 0,
        extra_fields,
        comment: String::new().into(),
        data_descriptor: header.flags.data_descriptor,
    };

    Ok(Some(entry))
}

fn detect_comment(basic: Vec<u8>, basic_is_utf8: bool, extra_fields: &[ExtraField]) -> ZipString {
    if basic_is_utf8 {
        ZipString::new(basic, StringEncoding::Utf8)
    } else {
        let unicode_extra = extra_fields.iter().find_map(|field| match field {
            ExtraField::InfoZipUnicodeComment(InfoZipUnicodeCommentExtraField::V1 { crc32, unicode }) => {
                if *crc32 == crc32fast::hash(&basic) {
                    Some(std::string::String::from_utf8(unicode.clone()))
                } else {
                    None
                }
            }
            _ => None,
        });
        if let Some(Ok(s)) = unicode_extra {
            ZipString::new_with_alternative(s, basic)
        } else {
            // Do not treat as UTF-8 if UTF-8 flags are not set,
            // some string in MBCS may be valid UTF-8 in form, but they are not in truth.
            if basic.is_ascii() {
                // SAFETY:
                // a valid ASCII string is always a valid UTF-8 string
                unsafe { std::string::String::from_utf8_unchecked(basic).into() }
            } else {
                ZipString::new(basic, StringEncoding::Raw)
            }
        }
    }
}

fn detect_filename(basic: Vec<u8>, basic_is_utf8: bool, extra_fields: &[ExtraField]) -> ZipString {
    if basic_is_utf8 {
        ZipString::new(basic, StringEncoding::Utf8)
    } else {
        let unicode_extra = extra_fields.iter().find_map(|field| match field {
            ExtraField::InfoZipUnicodePath(InfoZipUnicodePathExtraField::V1 { crc32, unicode }) => {
                if *crc32 == crc32fast::hash(&basic) {
                    Some(std::string::String::from_utf8(unicode.clone()))
                } else {
                    None
                }
            }
            _ => None,
        });
        if let Some(Ok(s)) = unicode_extra {
            ZipString::new_with_alternative(s, basic)
        } else {
            // Do not treat as UTF-8 if UTF-8 flags are not set,
            // some string in MBCS may be valid UTF-8 in form, but they are not in truth.
            if basic.is_ascii() {
                // SAFETY:
                // a valid ASCII string is always a valid UTF-8 string
                unsafe { std::string::String::from_utf8_unchecked(basic).into() }
            } else {
                ZipString::new(basic, StringEncoding::Raw)
            }
        }
    }
}
