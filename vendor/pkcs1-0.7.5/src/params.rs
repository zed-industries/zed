//! PKCS#1 RSA parameters.

use crate::{Error, Result};
use der::{
    asn1::{AnyRef, ContextSpecificRef, ObjectIdentifier},
    oid::AssociatedOid,
    Decode, DecodeValue, Encode, EncodeValue, FixedTag, Length, Reader, Sequence, Tag, TagMode,
    TagNumber, Writer,
};
use spki::{AlgorithmIdentifier, AlgorithmIdentifierRef};

const OID_SHA_1: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
const OID_MGF_1: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.8");
const OID_PSPECIFIED: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.9");

const SHA_1_AI: AlgorithmIdentifierRef<'_> = AlgorithmIdentifierRef {
    oid: OID_SHA_1,
    parameters: Some(AnyRef::NULL),
};

/// `TrailerField` as defined in [RFC 8017 Appendix 2.3].
/// ```text
/// TrailerField ::= INTEGER { trailerFieldBC(1) }
/// ```
/// [RFC 8017 Appendix 2.3]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.3
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TrailerField {
    /// the only supported value (0xbc, default)
    BC = 1,
}

impl Default for TrailerField {
    fn default() -> Self {
        Self::BC
    }
}

impl<'a> DecodeValue<'a> for TrailerField {
    fn decode_value<R: Reader<'a>>(decoder: &mut R, header: der::Header) -> der::Result<Self> {
        match u8::decode_value(decoder, header)? {
            1 => Ok(TrailerField::BC),
            _ => Err(Self::TAG.value_error()),
        }
    }
}

impl EncodeValue for TrailerField {
    fn value_len(&self) -> der::Result<Length> {
        Ok(Length::ONE)
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        (*self as u8).encode_value(writer)
    }
}

impl FixedTag for TrailerField {
    const TAG: Tag = Tag::Integer;
}

/// PKCS#1 RSASSA-PSS parameters as defined in [RFC 8017 Appendix 2.3]
///
/// ASN.1 structure containing a serialized RSASSA-PSS parameters:
/// ```text
/// RSASSA-PSS-params ::= SEQUENCE {
///     hashAlgorithm      [0] HashAlgorithm      DEFAULT sha1,
///     maskGenAlgorithm   [1] MaskGenAlgorithm   DEFAULT mgf1SHA1,
///     saltLength         [2] INTEGER            DEFAULT 20,
///     trailerField       [3] TrailerField       DEFAULT trailerFieldBC
/// }
/// HashAlgorithm ::= AlgorithmIdentifier
/// MaskGenAlgorithm ::= AlgorithmIdentifier
/// ```
///
/// [RFC 8017 Appendix 2.3]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.3
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaPssParams<'a> {
    /// Hash Algorithm
    pub hash: AlgorithmIdentifierRef<'a>,

    /// Mask Generation Function (MGF)
    pub mask_gen: AlgorithmIdentifier<AlgorithmIdentifierRef<'a>>,

    /// Salt length
    pub salt_len: u8,

    /// Trailer field (i.e. [`TrailerField::BC`])
    pub trailer_field: TrailerField,
}

impl<'a> RsaPssParams<'a> {
    /// Default RSA PSS Salt length in RsaPssParams
    pub const SALT_LEN_DEFAULT: u8 = 20;

    /// Create new RsaPssParams for the provided digest and salt len
    pub fn new<D>(salt_len: u8) -> Self
    where
        D: AssociatedOid,
    {
        Self {
            hash: AlgorithmIdentifierRef {
                oid: D::OID,
                parameters: Some(AnyRef::NULL),
            },
            mask_gen: AlgorithmIdentifier {
                oid: OID_MGF_1,
                parameters: Some(AlgorithmIdentifierRef {
                    oid: D::OID,
                    parameters: Some(AnyRef::NULL),
                }),
            },
            salt_len,
            trailer_field: Default::default(),
        }
    }

    fn context_specific_hash(&self) -> Option<ContextSpecificRef<'_, AlgorithmIdentifierRef<'a>>> {
        if self.hash == SHA_1_AI {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N0,
                tag_mode: TagMode::Explicit,
                value: &self.hash,
            })
        }
    }

    fn context_specific_mask_gen(
        &self,
    ) -> Option<ContextSpecificRef<'_, AlgorithmIdentifier<AlgorithmIdentifierRef<'a>>>> {
        if self.mask_gen == default_mgf1_sha1() {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N1,
                tag_mode: TagMode::Explicit,
                value: &self.mask_gen,
            })
        }
    }

    fn context_specific_salt_len(&self) -> Option<ContextSpecificRef<'_, u8>> {
        if self.salt_len == RsaPssParams::SALT_LEN_DEFAULT {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N2,
                tag_mode: TagMode::Explicit,
                value: &self.salt_len,
            })
        }
    }

    fn context_specific_trailer_field(&self) -> Option<ContextSpecificRef<'_, TrailerField>> {
        if self.trailer_field == TrailerField::default() {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N3,
                tag_mode: TagMode::Explicit,
                value: &self.trailer_field,
            })
        }
    }
}

impl<'a> Default for RsaPssParams<'a> {
    fn default() -> Self {
        Self {
            hash: SHA_1_AI,
            mask_gen: default_mgf1_sha1(),
            salt_len: RsaPssParams::SALT_LEN_DEFAULT,
            trailer_field: Default::default(),
        }
    }
}

impl<'a> DecodeValue<'a> for RsaPssParams<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: der::Header) -> der::Result<Self> {
        reader.read_nested(header.length, |reader| {
            Ok(Self {
                hash: reader
                    .context_specific(TagNumber::N0, TagMode::Explicit)?
                    .unwrap_or(SHA_1_AI),
                mask_gen: reader
                    .context_specific(TagNumber::N1, TagMode::Explicit)?
                    .unwrap_or_else(default_mgf1_sha1),
                salt_len: reader
                    .context_specific(TagNumber::N2, TagMode::Explicit)?
                    .unwrap_or(RsaPssParams::SALT_LEN_DEFAULT),
                trailer_field: reader
                    .context_specific(TagNumber::N3, TagMode::Explicit)?
                    .unwrap_or_default(),
            })
        })
    }
}

impl EncodeValue for RsaPssParams<'_> {
    fn value_len(&self) -> der::Result<Length> {
        self.context_specific_hash().encoded_len()?
            + self.context_specific_mask_gen().encoded_len()?
            + self.context_specific_salt_len().encoded_len()?
            + self.context_specific_trailer_field().encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.context_specific_hash().encode(writer)?;
        self.context_specific_mask_gen().encode(writer)?;
        self.context_specific_salt_len().encode(writer)?;
        self.context_specific_trailer_field().encode(writer)?;
        Ok(())
    }
}

impl<'a> Sequence<'a> for RsaPssParams<'a> {}

impl<'a> TryFrom<&'a [u8]> for RsaPssParams<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_der(bytes)?)
    }
}

/// Default Mask Generation Function (MGF): SHA-1.
fn default_mgf1_sha1<'a>() -> AlgorithmIdentifier<AlgorithmIdentifierRef<'a>> {
    AlgorithmIdentifier::<AlgorithmIdentifierRef<'a>> {
        oid: OID_MGF_1,
        parameters: Some(SHA_1_AI),
    }
}

/// PKCS#1 RSAES-OAEP parameters as defined in [RFC 8017 Appendix 2.1]
///
/// ASN.1 structure containing a serialized RSAES-OAEP parameters:
/// ```text
/// RSAES-OAEP-params ::= SEQUENCE {
///     hashAlgorithm      [0] HashAlgorithm     DEFAULT sha1,
///     maskGenAlgorithm   [1] MaskGenAlgorithm  DEFAULT mgf1SHA1,
///     pSourceAlgorithm   [2] PSourceAlgorithm  DEFAULT pSpecifiedEmpty
/// }
/// HashAlgorithm ::= AlgorithmIdentifier
/// MaskGenAlgorithm ::= AlgorithmIdentifier
/// PSourceAlgorithm ::= AlgorithmIdentifier
/// ```
///
/// [RFC 8017 Appendix 2.1]: https://datatracker.ietf.org/doc/html/rfc8017#appendix-A.2.1
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RsaOaepParams<'a> {
    /// Hash Algorithm
    pub hash: AlgorithmIdentifierRef<'a>,

    /// Mask Generation Function (MGF)
    pub mask_gen: AlgorithmIdentifier<AlgorithmIdentifierRef<'a>>,

    /// The source (and possibly the value) of the label L
    pub p_source: AlgorithmIdentifierRef<'a>,
}

impl<'a> RsaOaepParams<'a> {
    /// Create new RsaPssParams for the provided digest and default (empty) label
    pub fn new<D>() -> Self
    where
        D: AssociatedOid,
    {
        Self::new_with_label::<D>(&[])
    }

    /// Create new RsaPssParams for the provided digest and specified label
    pub fn new_with_label<D>(label: &'a impl AsRef<[u8]>) -> Self
    where
        D: AssociatedOid,
    {
        Self {
            hash: AlgorithmIdentifierRef {
                oid: D::OID,
                parameters: Some(AnyRef::NULL),
            },
            mask_gen: AlgorithmIdentifier {
                oid: OID_MGF_1,
                parameters: Some(AlgorithmIdentifierRef {
                    oid: D::OID,
                    parameters: Some(AnyRef::NULL),
                }),
            },
            p_source: pspecicied_algorithm_identifier(label),
        }
    }

    fn context_specific_hash(&self) -> Option<ContextSpecificRef<'_, AlgorithmIdentifierRef<'a>>> {
        if self.hash == SHA_1_AI {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N0,
                tag_mode: TagMode::Explicit,
                value: &self.hash,
            })
        }
    }

    fn context_specific_mask_gen(
        &self,
    ) -> Option<ContextSpecificRef<'_, AlgorithmIdentifier<AlgorithmIdentifierRef<'a>>>> {
        if self.mask_gen == default_mgf1_sha1() {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N1,
                tag_mode: TagMode::Explicit,
                value: &self.mask_gen,
            })
        }
    }

    fn context_specific_p_source(
        &self,
    ) -> Option<ContextSpecificRef<'_, AlgorithmIdentifierRef<'a>>> {
        if self.p_source == default_pempty_string() {
            None
        } else {
            Some(ContextSpecificRef {
                tag_number: TagNumber::N2,
                tag_mode: TagMode::Explicit,
                value: &self.p_source,
            })
        }
    }
}

impl<'a> Default for RsaOaepParams<'a> {
    fn default() -> Self {
        Self {
            hash: SHA_1_AI,
            mask_gen: default_mgf1_sha1(),
            p_source: default_pempty_string(),
        }
    }
}

impl<'a> DecodeValue<'a> for RsaOaepParams<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, header: der::Header) -> der::Result<Self> {
        reader.read_nested(header.length, |reader| {
            Ok(Self {
                hash: reader
                    .context_specific(TagNumber::N0, TagMode::Explicit)?
                    .unwrap_or(SHA_1_AI),
                mask_gen: reader
                    .context_specific(TagNumber::N1, TagMode::Explicit)?
                    .unwrap_or_else(default_mgf1_sha1),
                p_source: reader
                    .context_specific(TagNumber::N2, TagMode::Explicit)?
                    .unwrap_or_else(default_pempty_string),
            })
        })
    }
}

impl EncodeValue for RsaOaepParams<'_> {
    fn value_len(&self) -> der::Result<Length> {
        self.context_specific_hash().encoded_len()?
            + self.context_specific_mask_gen().encoded_len()?
            + self.context_specific_p_source().encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl Writer) -> der::Result<()> {
        self.context_specific_hash().encode(writer)?;
        self.context_specific_mask_gen().encode(writer)?;
        self.context_specific_p_source().encode(writer)?;
        Ok(())
    }
}

impl<'a> Sequence<'a> for RsaOaepParams<'a> {}

impl<'a> TryFrom<&'a [u8]> for RsaOaepParams<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_der(bytes)?)
    }
}

fn pspecicied_algorithm_identifier(label: &impl AsRef<[u8]>) -> AlgorithmIdentifierRef<'_> {
    AlgorithmIdentifierRef {
        oid: OID_PSPECIFIED,
        parameters: Some(
            AnyRef::new(Tag::OctetString, label.as_ref()).expect("error creating OAEP params"),
        ),
    }
}

/// Default Source Algorithm, empty string
fn default_pempty_string<'a>() -> AlgorithmIdentifierRef<'a> {
    pspecicied_algorithm_identifier(&[])
}
