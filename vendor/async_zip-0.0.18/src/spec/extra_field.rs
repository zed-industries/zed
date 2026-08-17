// Copyright Cognite AS, 2023

use crate::error::{Result as ZipResult, ZipError};
use crate::spec::header::{
    ExtraField, HeaderId, InfoZipUnicodeCommentExtraField, InfoZipUnicodePathExtraField, UnknownExtraField,
    Zip64ExtendedInformationExtraField,
};

use super::consts::NON_ZIP64_MAX_SIZE;

pub(crate) trait ExtraFieldAsBytes {
    fn as_bytes(&self) -> Vec<u8>;

    fn count_bytes(&self) -> usize;
}

impl ExtraFieldAsBytes for &[ExtraField] {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for field in self.iter() {
            buffer.append(&mut field.as_bytes());
        }
        buffer
    }

    fn count_bytes(&self) -> usize {
        self.iter().map(|field| field.count_bytes()).sum()
    }
}

impl ExtraFieldAsBytes for ExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            ExtraField::Zip64ExtendedInformation(field) => field.as_bytes(),
            ExtraField::InfoZipUnicodeComment(field) => field.as_bytes(),
            ExtraField::InfoZipUnicodePath(field) => field.as_bytes(),
            ExtraField::Unknown(field) => field.as_bytes(),
        }
    }

    fn count_bytes(&self) -> usize {
        match self {
            ExtraField::Zip64ExtendedInformation(field) => field.count_bytes(),
            ExtraField::InfoZipUnicodeComment(field) => field.count_bytes(),
            ExtraField::InfoZipUnicodePath(field) => field.count_bytes(),
            ExtraField::Unknown(field) => field.count_bytes(),
        }
    }
}

impl ExtraFieldAsBytes for UnknownExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = self.header_id.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        bytes.append(&mut self.data_size.to_le_bytes().to_vec());
        bytes.append(&mut self.content.clone());

        bytes
    }

    fn count_bytes(&self) -> usize {
        4 + self.content.len()
    }
}

impl ExtraFieldAsBytes for Zip64ExtendedInformationExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = self.header_id.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        bytes.append(&mut (self.content_size() as u16).to_le_bytes().to_vec());
        if let Some(uncompressed_size) = &self.uncompressed_size {
            bytes.append(&mut uncompressed_size.to_le_bytes().to_vec());
        }
        if let Some(compressed_size) = &self.compressed_size {
            bytes.append(&mut compressed_size.to_le_bytes().to_vec());
        }
        if let Some(relative_header_offset) = &self.relative_header_offset {
            bytes.append(&mut relative_header_offset.to_le_bytes().to_vec());
        }
        if let Some(disk_start_number) = &self.disk_start_number {
            bytes.append(&mut disk_start_number.to_le_bytes().to_vec());
        }

        bytes
    }

    fn count_bytes(&self) -> usize {
        4 + self.content_size()
    }
}

impl ExtraFieldAsBytes for InfoZipUnicodeCommentExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = HeaderId::INFO_ZIP_UNICODE_COMMENT_EXTRA_FIELD.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        match self {
            InfoZipUnicodeCommentExtraField::V1 { crc32, unicode } => {
                let data_size: u16 = (5 + unicode.len()).try_into().unwrap();
                bytes.append(&mut data_size.to_le_bytes().to_vec());
                bytes.push(1);
                bytes.append(&mut crc32.to_le_bytes().to_vec());
                bytes.append(&mut unicode.clone());
            }
            InfoZipUnicodeCommentExtraField::Unknown { version, data } => {
                let data_size: u16 = (1 + data.len()).try_into().unwrap();
                bytes.append(&mut data_size.to_le_bytes().to_vec());
                bytes.push(*version);
                bytes.append(&mut data.clone());
            }
        }
        bytes
    }

    fn count_bytes(&self) -> usize {
        match self {
            InfoZipUnicodeCommentExtraField::V1 { unicode, .. } => 9 + unicode.len(),
            InfoZipUnicodeCommentExtraField::Unknown { data, .. } => 5 + data.len(),
        }
    }
}

impl ExtraFieldAsBytes for InfoZipUnicodePathExtraField {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let header_id: u16 = HeaderId::INFO_ZIP_UNICODE_PATH_EXTRA_FIELD.into();
        bytes.append(&mut header_id.to_le_bytes().to_vec());
        match self {
            InfoZipUnicodePathExtraField::V1 { crc32, unicode } => {
                let data_size: u16 = (5 + unicode.len()).try_into().unwrap();
                bytes.append(&mut data_size.to_le_bytes().to_vec());
                bytes.push(1);
                bytes.append(&mut crc32.to_le_bytes().to_vec());
                bytes.append(&mut unicode.clone());
            }
            InfoZipUnicodePathExtraField::Unknown { version, data } => {
                let data_size: u16 = (1 + data.len()).try_into().unwrap();
                bytes.append(&mut data_size.to_le_bytes().to_vec());
                bytes.push(*version);
                bytes.append(&mut data.clone());
            }
        }
        bytes
    }

    fn count_bytes(&self) -> usize {
        match self {
            InfoZipUnicodePathExtraField::V1 { unicode, .. } => 9 + unicode.len(),
            InfoZipUnicodePathExtraField::Unknown { data, .. } => 5 + data.len(),
        }
    }
}

/// Parse a zip64 extra field from bytes.
/// The content of "data" should exclude the header.
fn zip64_extended_information_field_from_bytes(
    header_id: HeaderId,
    data: &[u8],
    uncompressed_size: u32,
    compressed_size: u32,
) -> ZipResult<Zip64ExtendedInformationExtraField> {
    // slice.take is nightly-only so we'll just use an index to track the current position
    let mut current_idx = 0;
    let uncompressed_size = if uncompressed_size == NON_ZIP64_MAX_SIZE && data.len() >= current_idx + 8 {
        let val = Some(u64::from_le_bytes(data[current_idx..current_idx + 8].try_into().unwrap()));
        current_idx += 8;
        val
    } else {
        None
    };

    let compressed_size = if compressed_size == NON_ZIP64_MAX_SIZE && data.len() >= current_idx + 8 {
        let val = Some(u64::from_le_bytes(data[current_idx..current_idx + 8].try_into().unwrap()));
        current_idx += 8;
        val
    } else {
        None
    };

    let relative_header_offset = if data.len() >= current_idx + 8 {
        let val = Some(u64::from_le_bytes(data[current_idx..current_idx + 8].try_into().unwrap()));
        current_idx += 8;
        val
    } else {
        None
    };

    #[allow(unused_assignments)]
    let disk_start_number = if data.len() >= current_idx + 4 {
        let val = Some(u32::from_le_bytes(data[current_idx..current_idx + 4].try_into().unwrap()));
        current_idx += 4;
        val
    } else {
        None
    };

    Ok(Zip64ExtendedInformationExtraField {
        header_id,
        uncompressed_size,
        compressed_size,
        relative_header_offset,
        disk_start_number,
    })
}

fn info_zip_unicode_comment_extra_field_from_bytes(
    _header_id: HeaderId,
    data_size: u16,
    data: &[u8],
) -> ZipResult<InfoZipUnicodeCommentExtraField> {
    if data.is_empty() {
        return Err(ZipError::InfoZipUnicodeCommentFieldIncomplete);
    }
    let version = data[0];
    match version {
        1 => {
            if data.len() < 5 {
                return Err(ZipError::InfoZipUnicodeCommentFieldIncomplete);
            }
            let crc32 = u32::from_le_bytes(data[1..5].try_into().unwrap());
            let unicode = data[5..(data_size as usize)].to_vec();
            Ok(InfoZipUnicodeCommentExtraField::V1 { crc32, unicode })
        }
        _ => Ok(InfoZipUnicodeCommentExtraField::Unknown { version, data: data[1..(data_size as usize)].to_vec() }),
    }
}

fn info_zip_unicode_path_extra_field_from_bytes(
    _header_id: HeaderId,
    data_size: u16,
    data: &[u8],
) -> ZipResult<InfoZipUnicodePathExtraField> {
    if data.is_empty() {
        return Err(ZipError::InfoZipUnicodePathFieldIncomplete);
    }
    let version = data[0];
    match version {
        1 => {
            if data.len() < 5 {
                return Err(ZipError::InfoZipUnicodePathFieldIncomplete);
            }
            let crc32 = u32::from_le_bytes(data[1..5].try_into().unwrap());
            let unicode = data[5..(data_size as usize)].to_vec();
            Ok(InfoZipUnicodePathExtraField::V1 { crc32, unicode })
        }
        _ => Ok(InfoZipUnicodePathExtraField::Unknown { version, data: data[1..(data_size as usize)].to_vec() }),
    }
}

pub(crate) fn extra_field_from_bytes(
    header_id: HeaderId,
    data_size: u16,
    data: &[u8],
    uncompressed_size: u32,
    compressed_size: u32,
) -> ZipResult<ExtraField> {
    match header_id {
        HeaderId::ZIP64_EXTENDED_INFORMATION_EXTRA_FIELD => Ok(ExtraField::Zip64ExtendedInformation(
            zip64_extended_information_field_from_bytes(header_id, data, uncompressed_size, compressed_size)?,
        )),
        HeaderId::INFO_ZIP_UNICODE_COMMENT_EXTRA_FIELD => Ok(ExtraField::InfoZipUnicodeComment(
            info_zip_unicode_comment_extra_field_from_bytes(header_id, data_size, data)?,
        )),
        HeaderId::INFO_ZIP_UNICODE_PATH_EXTRA_FIELD => Ok(ExtraField::InfoZipUnicodePath(
            info_zip_unicode_path_extra_field_from_bytes(header_id, data_size, data)?,
        )),
        _ => Ok(ExtraField::Unknown(UnknownExtraField { header_id, data_size, content: data.to_vec() })),
    }
}

pub struct Zip64ExtendedInformationExtraFieldBuilder {
    field: Zip64ExtendedInformationExtraField,
}

impl Zip64ExtendedInformationExtraFieldBuilder {
    pub fn new() -> Self {
        Self {
            field: Zip64ExtendedInformationExtraField {
                header_id: HeaderId::ZIP64_EXTENDED_INFORMATION_EXTRA_FIELD,
                uncompressed_size: None,
                compressed_size: None,
                relative_header_offset: None,
                disk_start_number: None,
            },
        }
    }

    pub fn sizes(mut self, compressed_size: u64, uncompressed_size: u64) -> Self {
        self.field.compressed_size = Some(compressed_size);
        self.field.uncompressed_size = Some(uncompressed_size);
        self
    }

    pub fn relative_header_offset(mut self, relative_header_offset: u64) -> Self {
        self.field.relative_header_offset = Some(relative_header_offset);
        self
    }

    #[allow(dead_code)]
    pub fn disk_start_number(mut self, disk_start_number: u32) -> Self {
        self.field.disk_start_number = Some(disk_start_number);
        self
    }

    pub fn eof_only(&self) -> bool {
        (self.field.uncompressed_size.is_none() && self.field.compressed_size.is_none())
            && (self.field.relative_header_offset.is_some() || self.field.disk_start_number.is_some())
    }

    pub fn build(self) -> ZipResult<Zip64ExtendedInformationExtraField> {
        let field = self.field;

        if field.content_size() == 0 {
            return Err(ZipError::Zip64ExtendedFieldIncomplete);
        }
        Ok(field)
    }
}
