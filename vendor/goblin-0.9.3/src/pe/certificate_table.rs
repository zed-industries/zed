/// Implements parsing of pe32's Attribute Certificate Table
/// See reference:
/// https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#the-attribute-certificate-table-image-only
/// https://learn.microsoft.com/en-us/windows/win32/api/wintrust/ns-wintrust-win_certificate
use crate::error;
use scroll::{ctx, Pread, Pwrite};

use alloc::string::ToString;
use alloc::vec::Vec;

use super::utils::pad;

#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AttributeCertificateRevision {
    /// WIN_CERT_REVISION_1_0
    Revision1_0 = 0x0100,
    /// WIN_CERT_REVISION_2_0
    Revision2_0 = 0x0200,
}

impl TryFrom<u16> for AttributeCertificateRevision {
    type Error = error::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == AttributeCertificateRevision::Revision1_0 as u16 => {
                AttributeCertificateRevision::Revision1_0
            }
            x if x == AttributeCertificateRevision::Revision2_0 as u16 => {
                AttributeCertificateRevision::Revision2_0
            }
            _ => {
                return Err(error::Error::Malformed(
                    "Invalid certificate attribute revision".to_string(),
                ))
            }
        })
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AttributeCertificateType {
    /// WIN_CERT_TYPE_X509
    X509 = 0x0001,
    /// WIN_CERT_TYPE_PKCS_SIGNED_DATA
    PkcsSignedData = 0x0002,
    /// WIN_CERT_TYPE_RESERVED_1
    Reserved1 = 0x0003,
    /// WIN_CERT_TYPE_TS_STACK_SIGNED
    TsStackSigned = 0x0004,
}

impl TryFrom<u16> for AttributeCertificateType {
    type Error = error::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == AttributeCertificateType::X509 as u16 => AttributeCertificateType::X509,
            x if x == AttributeCertificateType::PkcsSignedData as u16 => {
                AttributeCertificateType::PkcsSignedData
            }
            x if x == AttributeCertificateType::Reserved1 as u16 => {
                AttributeCertificateType::Reserved1
            }
            x if x == AttributeCertificateType::TsStackSigned as u16 => {
                AttributeCertificateType::TsStackSigned
            }
            _ => {
                return Err(error::Error::Malformed(
                    "Invalid attribute certificate type".to_string(),
                ))
            }
        })
    }
}

#[derive(Clone, Pread)]
struct AttributeCertificateHeader {
    /// dwLength
    length: u32,
    revision: u16,
    certificate_type: u16,
}

const CERTIFICATE_DATA_OFFSET: u32 = 8;
#[derive(Debug)]
pub struct AttributeCertificate<'a> {
    pub length: u32,
    pub revision: AttributeCertificateRevision,
    pub certificate_type: AttributeCertificateType,
    pub certificate: &'a [u8],
}

impl<'a> AttributeCertificate<'a> {
    pub fn parse(
        bytes: &'a [u8],
        current_offset: &mut usize,
    ) -> Result<AttributeCertificate<'a>, error::Error> {
        // `current_offset` is moved sizeof(AttributeCertificateHeader) = 8 bytes further.
        let header: AttributeCertificateHeader = bytes.gread_with(current_offset, scroll::LE)?;
        let cert_size = usize::try_from(header.length.saturating_sub(CERTIFICATE_DATA_OFFSET))
            .map_err(|_err| {
                error::Error::Malformed(
                    "Attribute certificate size do not fit in usize".to_string(),
                )
            })?;

        if let Some(bytes) = bytes.get(*current_offset..(*current_offset + cert_size)) {
            let attr = Self {
                length: header.length,
                revision: header.revision.try_into()?,
                certificate_type: header.certificate_type.try_into()?,
                certificate: bytes,
            };
            // Moving past the certificate data.
            // Prevent the current_offset to wrap and ensure current_offset is strictly increasing.
            *current_offset = current_offset.saturating_add(cert_size);
            // Round to the next 8-bytes.
            *current_offset = (*current_offset + 7) & !7;
            Ok(attr)
        } else {
            Err(error::Error::Malformed(format!(
                "Unable to extract certificate. Probably cert_size:{} is malformed",
                cert_size
            )))
        }
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for &AttributeCertificate<'a> {
    type Error = error::Error;

    /// Writes an aligned attribute certificate in the buffer.
    fn try_into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        bytes.gwrite_with(self.length, offset, ctx)?;
        bytes.gwrite_with(self.revision as u16, offset, ctx)?;
        bytes.gwrite_with(self.certificate_type as u16, offset, ctx)?;
        // Extend by zero the buffer until it is aligned on a quadword (16 bytes).
        let maybe_certificate_padding = pad(self.certificate.len(), Some(16usize));
        bytes.gwrite(self.certificate, offset)?;
        if let Some(cert_padding) = maybe_certificate_padding {
            bytes.gwrite(&cert_padding[..], offset)?;
        }

        Ok(*offset)
    }
}

pub type CertificateDirectoryTable<'a> = Vec<AttributeCertificate<'a>>;

pub(crate) fn enumerate_certificates(
    bytes: &[u8],
    table_virtual_address: u32,
    table_size: u32,
) -> Result<CertificateDirectoryTable, error::Error> {
    let table_start_offset = usize::try_from(table_virtual_address).map_err(|_err| {
        error::Error::Malformed("Certificate table RVA do not fit in a usize".to_string())
    })?;
    // Here, we do not want wrapping semantics as it means that a too big table size or table start
    // offset will provide table_end_offset such that table_end_offset < table_start_offset, which
    // is not desirable at all.
    let table_end_offset =
        table_start_offset.saturating_add(usize::try_from(table_size).map_err(|_err| {
            error::Error::Malformed("Certificate table size do not fit in a usize".to_string())
        })?);
    let mut current_offset = table_start_offset;
    let mut attrs = vec![];

    // End offset cannot be further than the binary we have at hand.
    if table_end_offset > bytes.len() {
        return Err(error::Error::Malformed(
            "End of attribute certificates table is after the end of the PE binary".to_string(),
        ));
    }

    // This is guaranteed to terminate, either by a malformed error being returned
    // or because current_offset >= table_end_offset by virtue of current_offset being strictly
    // increasing through `AttributeCertificate::parse`.
    while current_offset < table_end_offset {
        attrs.push(AttributeCertificate::parse(bytes, &mut current_offset)?);
    }

    Ok(attrs)
}
