//! A small ASN.1 parsing library for Rust. In particular, this library is used
//! to translate the binary DER encoding of an ASN.1-formatted document into the
//! core primitives of ASN.1. It is assumed that you can do what you need to
//! from there.
//!
//! The critical items for this document are the traits `ToASN1` and `FromASN1`.
//! The first takes your data type and encodes it into a `Vec` of simple ASN.1
//! structures (`ASN1Block`s). The latter inverts the process.
//!
//! Items that implement `ToASN1` can be used with the function `der_encode`
//! to provide single-step encoding of a data type to binary DER encoding.
//! Similarly, items that are `FromASN` can be single-step decoded using
//! the helper function `der_decode`.
//!
//! You can implement one or both traits, depending on your needs. If you do
//! implement both, the obvious encode/decode quickcheck property is strongly
//! advised.
//!
//! For decoding schemes that require the actual bytes associated with the
//! binary representation, we also provide `FromASN1WithBody`. This can be
//! used with the offset information in the primitive `ASN1Block`s to, for
//! example, validate signatures in X509 documents.
//!
//! Finally, this library supports ASN.1 class information. I'm still not sure
//! why it's useful, but there it is.
//!
//! Please send any bug reports, patches, and curses to the GitHub repository
//! at <code>https://github.com/acw/simple_asn1</code>.
pub use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
#[cfg(test)]
use quickcheck::quickcheck;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::mem::size_of;
use std::str::Utf8Error;
use thiserror::Error;
use time::PrimitiveDateTime;

/// An ASN.1 block class.
///
/// I'm not sure if/when these are used, but here they are in case you want
/// to do something with them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASN1Class {
    Universal,
    Application,
    ContextSpecific,
    Private,
}

/// A primitive block from ASN.1.
///
/// Primitive blocks all contain the offset from the beginning of the parsed
/// document, followed by whatever data is associated with the block. The latter
/// should be fairly self-explanatory, so let's discuss the offset.
///
/// The offset is only valid during the reading process. It is ignored for
/// the purposes of encoding blocks into their binary form. It is also
/// ignored for the purpose of comparisons via `==`. It is included entirely
/// to support the parsing of things like X509 certificates, in which it is
/// necessary to know when particular blocks end.
///
/// The [`ASN1Class`] of explicitly tagged blocks is either `Application`,
/// `ContextSpecific` or `Private`. `Unknown` can have any class.
/// The class of all other variants is `Universal`.
///
/// [`ASN1Class`]: enum.ASN1Class.html
#[derive(Clone, Debug)]
pub enum ASN1Block {
    Boolean(usize, bool),
    Integer(usize, BigInt),
    BitString(usize, usize, Vec<u8>),
    OctetString(usize, Vec<u8>),
    Null(usize),
    ObjectIdentifier(usize, OID),
    UTF8String(usize, String),
    PrintableString(usize, String),
    TeletexString(usize, String),
    IA5String(usize, String),
    UTCTime(usize, PrimitiveDateTime),
    GeneralizedTime(usize, PrimitiveDateTime),
    UniversalString(usize, String),
    BMPString(usize, String),
    Sequence(usize, Vec<ASN1Block>),
    Set(usize, Vec<ASN1Block>),
    /// An explicitly tagged block.
    ///
    /// The class can be either `Application`, `ContextSpecific` or `Private`.
    /// The other parameters are `offset`, `tag` and `content`.
    ///
    /// This block is always `constructed`.
    Explicit(ASN1Class, usize, BigUint, Box<ASN1Block>),
    /// An unkown block.
    ///
    /// The parameters are `class`, `constructed`, `offset`, `tag` and
    /// `content`.
    Unknown(ASN1Class, bool, usize, BigUint, Vec<u8>),
}

impl ASN1Block {
    /// Get the class associated with the given ASN1Block, regardless of what
    /// kind of block it is.
    pub fn class(&self) -> ASN1Class {
        match *self {
            ASN1Block::Boolean(_, _) => ASN1Class::Universal,
            ASN1Block::Integer(_, _) => ASN1Class::Universal,
            ASN1Block::BitString(_, _, _) => ASN1Class::Universal,
            ASN1Block::OctetString(_, _) => ASN1Class::Universal,
            ASN1Block::Null(_) => ASN1Class::Universal,
            ASN1Block::ObjectIdentifier(_, _) => ASN1Class::Universal,
            ASN1Block::UTF8String(_, _) => ASN1Class::Universal,
            ASN1Block::PrintableString(_, _) => ASN1Class::Universal,
            ASN1Block::TeletexString(_, _) => ASN1Class::Universal,
            ASN1Block::IA5String(_, _) => ASN1Class::Universal,
            ASN1Block::UTCTime(_, _) => ASN1Class::Universal,
            ASN1Block::GeneralizedTime(_, _) => ASN1Class::Universal,
            ASN1Block::UniversalString(_, _) => ASN1Class::Universal,
            ASN1Block::BMPString(_, _) => ASN1Class::Universal,
            ASN1Block::Sequence(_, _) => ASN1Class::Universal,
            ASN1Block::Set(_, _) => ASN1Class::Universal,
            ASN1Block::Explicit(c, _, _, _) => c,
            ASN1Block::Unknown(c, _, _, _, _) => c,
        }
    }
    /// Get the starting offset associated with the given ASN1Block, regardless
    /// of what kind of block it is.
    pub fn offset(&self) -> usize {
        match *self {
            ASN1Block::Boolean(o, _) => o,
            ASN1Block::Integer(o, _) => o,
            ASN1Block::BitString(o, _, _) => o,
            ASN1Block::OctetString(o, _) => o,
            ASN1Block::Null(o) => o,
            ASN1Block::ObjectIdentifier(o, _) => o,
            ASN1Block::UTF8String(o, _) => o,
            ASN1Block::PrintableString(o, _) => o,
            ASN1Block::TeletexString(o, _) => o,
            ASN1Block::IA5String(o, _) => o,
            ASN1Block::UTCTime(o, _) => o,
            ASN1Block::GeneralizedTime(o, _) => o,
            ASN1Block::UniversalString(o, _) => o,
            ASN1Block::BMPString(o, _) => o,
            ASN1Block::Sequence(o, _) => o,
            ASN1Block::Set(o, _) => o,
            ASN1Block::Explicit(_, o, _, _) => o,
            ASN1Block::Unknown(_, _, o, _, _) => o,
        }
    }
}

impl PartialEq for ASN1Block {
    fn eq(&self, other: &ASN1Block) -> bool {
        match (self, other) {
            (&ASN1Block::Boolean(_, a1), &ASN1Block::Boolean(_, a2)) => a1 == a2,
            (&ASN1Block::Integer(_, ref a1), &ASN1Block::Integer(_, ref a2)) => a1 == a2,
            (&ASN1Block::BitString(_, a1, ref b1), &ASN1Block::BitString(_, a2, ref b2)) => {
                (a1 == a2) && (b1 == b2)
            }
            (&ASN1Block::OctetString(_, ref a1), &ASN1Block::OctetString(_, ref a2)) => a1 == a2,
            (&ASN1Block::Null(_), &ASN1Block::Null(_)) => true,
            (&ASN1Block::ObjectIdentifier(_, ref a1), &ASN1Block::ObjectIdentifier(_, ref a2)) => {
                a1 == a2
            }
            (&ASN1Block::UTF8String(_, ref a1), &ASN1Block::UTF8String(_, ref a2)) => a1 == a2,
            (&ASN1Block::PrintableString(_, ref a1), &ASN1Block::PrintableString(_, ref a2)) => {
                a1 == a2
            }
            (&ASN1Block::TeletexString(_, ref a1), &ASN1Block::TeletexString(_, ref a2)) => {
                a1 == a2
            }
            (&ASN1Block::IA5String(_, ref a1), &ASN1Block::IA5String(_, ref a2)) => a1 == a2,
            (&ASN1Block::UTCTime(_, ref a1), &ASN1Block::UTCTime(_, ref a2)) => a1 == a2,
            (&ASN1Block::GeneralizedTime(_, ref a1), &ASN1Block::GeneralizedTime(_, ref a2)) => {
                a1 == a2
            }
            (&ASN1Block::UniversalString(_, ref a1), &ASN1Block::UniversalString(_, ref a2)) => {
                a1 == a2
            }
            (&ASN1Block::BMPString(_, ref a1), &ASN1Block::BMPString(_, ref a2)) => a1 == a2,
            (&ASN1Block::Sequence(_, ref a1), &ASN1Block::Sequence(_, ref a2)) => a1 == a2,
            (&ASN1Block::Set(_, ref a1), &ASN1Block::Set(_, ref a2)) => a1 == a2,
            (
                &ASN1Block::Explicit(a1, _, ref b1, ref c1),
                &ASN1Block::Explicit(a2, _, ref b2, ref c2),
            ) => (a1 == a2) && (b1 == b2) && (c1 == c2),
            (
                &ASN1Block::Unknown(a1, b1, _, ref c1, ref d1),
                &ASN1Block::Unknown(a2, b2, _, ref c2, ref d2),
            ) => (a1 == a2) && (b1 == b2) && (c1 == c2) && (d1 == d2),
            _ => false,
        }
    }
}

/// An ASN.1 OID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OID(Vec<BigUint>);

impl OID {
    /// Generate an ASN.1. The vector should be in the obvious format,
    /// with each component going left-to-right.
    pub fn new(x: Vec<BigUint>) -> OID {
        OID(x)
    }

    /// converts the
    pub fn as_raw(&self) -> Result<Vec<u8>, ASN1EncodeErr> {
        match (self.0.get(0), self.0.get(1)) {
            (Some(v1), Some(v2)) => {
                let two = BigUint::from_u8(2).unwrap();

                // first, validate that the first two items meet spec
                if v1 > &two {
                    return Err(ASN1EncodeErr::ObjectIdentVal1TooLarge);
                }

                let u175 = BigUint::from_u8(175).unwrap();
                let u39 = BigUint::from_u8(39).unwrap();
                let bound = if v1 == &two { u175 } else { u39 };

                if v2 > &bound {
                    return Err(ASN1EncodeErr::ObjectIdentVal2TooLarge);
                }

                // the following unwraps must be safe, based on the
                // validation above.
                let value1 = v1.to_u8().unwrap();
                let value2 = v2.to_u8().unwrap();
                let byte1 = (value1 * 40) + value2;

                // now we can build all the rest of the body
                let mut body = vec![byte1];
                for num in self.0.iter().skip(2) {
                    let mut local = encode_base127(num);
                    body.append(&mut local);
                }

                Ok(body)
            }
            _ => Err(ASN1EncodeErr::ObjectIdentHasTooFewFields),
        }
    }

    pub fn as_vec<'a, T: TryFrom<&'a BigUint>>(&'a self) -> Result<Vec<T>, ASN1DecodeErr> {
        let mut vec = Vec::new();
        for val in self.0.iter() {
            let ul = match T::try_from(val) {
                Ok(a) => a,
                Err(_) => return Err(ASN1DecodeErr::Overflow),
            };
            vec.push(ul);
        }

        Ok(vec)
    }
}

impl<'a> PartialEq<OID> for &'a OID {
    fn eq(&self, v2: &OID) -> bool {
        let &&OID(ref vec1) = self;
        let &OID(ref vec2) = v2;

        if vec1.len() != vec2.len() {
            return false;
        }

        for i in 0..vec1.len() {
            if vec1[i] != vec2[i] {
                return false;
            }
        }

        true
    }
}

/// A handy macro for generating OIDs from a sequence of `u64`s.
///
/// Usage: oid!(1,2,840,113549,1,1,1) creates an OID that matches
/// 1.2.840.113549.1.1.1. (Coincidentally, this is RSA.)
#[macro_export]
macro_rules! oid {
    ( $( $e: expr ),* ) => {{
        $crate::OID::new(vec![$($crate::BigUint::from($e as u64)),*])
    }};
}

const PRINTABLE_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'()+,-./:=? ";

#[cfg(test)]
const KNOWN_TAGS: &[u8] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x0c, 0x10, 0x11, 0x13, 0x14, 0x16, 0x17, 0x18, 0x1c, 0x1e,
];

/// An error that can arise decoding ASN.1 primitive blocks.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum ASN1DecodeErr {
    #[error("Encountered an empty buffer decoding ASN1 block.")]
    EmptyBuffer,
    #[error("Bad length field in boolean block: {0}")]
    BadBooleanLength(usize),
    #[error("Length field too large for object type: {0}")]
    LengthTooLarge(usize),
    #[error("UTF8 string failed to properly decode: {0}")]
    UTF8DecodeFailure(Utf8Error),
    #[error("Printable string failed to properly decode.")]
    PrintableStringDecodeFailure,
    #[error("Invalid date value: {0}")]
    InvalidDateValue(String),
    #[error("Invalid length of bit string: {0}")]
    InvalidBitStringLength(isize),
    /// Not a valid ASN.1 class
    #[error("Invalid class value: {0}")]
    InvalidClass(u8),
    /// Expected more input
    ///
    /// Invalid ASN.1 input can lead to this error.
    #[error("Incomplete data or invalid ASN1")]
    Incomplete,
    #[error("Value overflow")]
    Overflow,
}

/// An error that can arise encoding ASN.1 primitive blocks.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum ASN1EncodeErr {
    #[error("ASN1 object identifier has too few fields.")]
    ObjectIdentHasTooFewFields,
    #[error("First value in ASN1 OID is too big.")]
    ObjectIdentVal1TooLarge,
    #[error("Second value in ASN1 OID is too big.")]
    ObjectIdentVal2TooLarge,
}

/// Translate a binary blob into a series of `ASN1Block`s, or provide an
/// error if it didn't work.
pub fn from_der(i: &[u8]) -> Result<Vec<ASN1Block>, ASN1DecodeErr> {
    from_der_(i, 0)
}

fn from_der_(i: &[u8], start_offset: usize) -> Result<Vec<ASN1Block>, ASN1DecodeErr> {
    let mut result: Vec<ASN1Block> = Vec::new();
    let mut index: usize = 0;
    let len = i.len();

    while index < len {
        let soff = start_offset + index;
        let (tag, constructed, class) = decode_tag(i, &mut index)?;
        let len = decode_length(i, &mut index)?;
        let checklen = index
            .checked_add(len)
            .ok_or(ASN1DecodeErr::LengthTooLarge(len))?;
        if checklen > i.len() {
            return Err(ASN1DecodeErr::Incomplete);
        }
        let body = &i[index..(index + len)];

        if class != ASN1Class::Universal {
            if constructed {
                // Try to read as explicitly tagged
                if let Ok(mut items) = from_der_(body, start_offset + index) {
                    if items.len() == 1 {
                        result.push(ASN1Block::Explicit(
                            class,
                            soff,
                            tag,
                            Box::new(items.remove(0)),
                        ));
                        index += len;
                        continue;
                    }
                }
            }
            result.push(ASN1Block::Unknown(
                class,
                constructed,
                soff,
                tag,
                body.to_vec(),
            ));
            index += len;
            continue;
        }

        // Universal class
        match tag.to_u8() {
            // BOOLEAN
            Some(0x01) => {
                if len != 1 {
                    return Err(ASN1DecodeErr::BadBooleanLength(len));
                }
                result.push(ASN1Block::Boolean(soff, body[0] != 0));
            }
            // INTEGER
            Some(0x02) => {
                let res = BigInt::from_signed_bytes_be(body);
                result.push(ASN1Block::Integer(soff, res));
            }
            // BIT STRING
            Some(0x03) if body.is_empty() => result.push(ASN1Block::BitString(soff, 0, Vec::new())),
            Some(0x03) => {
                let bits = (&body[1..]).to_vec();
                let bitcount = bits.len() * 8;
                let rest = body[0] as usize;
                if bitcount < rest {
                    return Err(ASN1DecodeErr::InvalidBitStringLength(
                        bitcount as isize - rest as isize,
                    ));
                }

                let nbits = bitcount - (body[0] as usize);
                result.push(ASN1Block::BitString(soff, nbits, bits))
            }
            // OCTET STRING
            Some(0x04) => result.push(ASN1Block::OctetString(soff, body.to_vec())),
            // NULL
            Some(0x05) => {
                result.push(ASN1Block::Null(soff));
            }
            // OBJECT IDENTIFIER
            Some(0x06) => {
                let mut value1 = BigUint::zero();
                if body.is_empty() {
                    return Err(ASN1DecodeErr::Incomplete);
                }
                let mut value2 = BigUint::from_u8(body[0]).unwrap();
                let mut oidres = Vec::new();
                let mut bindex = 1;

                if body[0] >= 40 {
                    if body[0] < 80 {
                        value1 = BigUint::one();
                        value2 -= BigUint::from_u8(40).unwrap();
                    } else {
                        value1 = BigUint::from_u8(2).unwrap();
                        value2 -= BigUint::from_u8(80).unwrap();
                    }
                }

                oidres.push(value1);
                oidres.push(value2);
                while bindex < body.len() {
                    oidres.push(decode_base127(body, &mut bindex)?);
                }
                let res = OID(oidres);

                result.push(ASN1Block::ObjectIdentifier(soff, res))
            }
            // UTF8STRING
            Some(0x0C) => match String::from_utf8(body.to_vec()) {
                Ok(v) => result.push(ASN1Block::UTF8String(soff, v)),
                Err(e) => return Err(ASN1DecodeErr::UTF8DecodeFailure(e.utf8_error())),
            },
            // SEQUENCE
            Some(0x10) => match from_der_(body, start_offset + index) {
                Ok(items) => result.push(ASN1Block::Sequence(soff, items)),
                Err(e) => return Err(e),
            },
            // SET
            Some(0x11) => match from_der_(body, start_offset + index) {
                Ok(items) => result.push(ASN1Block::Set(soff, items)),
                Err(e) => return Err(e),
            },
            // PRINTABLE STRING
            Some(0x13) => {
                let mut res = String::new();
                let val = body.iter().map(|x| *x as char);

                for c in val {
                    if PRINTABLE_CHARS.contains(c) {
                        res.push(c);
                    } else {
                        return Err(ASN1DecodeErr::PrintableStringDecodeFailure);
                    }
                }
                result.push(ASN1Block::PrintableString(soff, res));
            }
            // TELETEX STRINGS
            Some(0x14) => match String::from_utf8(body.to_vec()) {
                Ok(v) => result.push(ASN1Block::TeletexString(soff, v)),
                Err(e) => return Err(ASN1DecodeErr::UTF8DecodeFailure(e.utf8_error())),
            },
            // IA5 (ASCII) STRING
            Some(0x16) => {
                let val = body.iter().map(|x| *x as char);
                let res = String::from_iter(val);
                result.push(ASN1Block::IA5String(soff, res))
            }
            // UTCTime
            Some(0x17) => {
                if body.len() != 13 {
                    return Err(ASN1DecodeErr::InvalidDateValue(format!("{}", body.len())));
                }

                let v = String::from_iter(body.iter().map(|x| *x as char));

                let y = match v.get(0..2) {
                    Some(yy) => yy,
                    None => {
                        // This wasn't a valid character boundrary.
                        return Err(ASN1DecodeErr::InvalidDateValue(v));
                    }
                };

                let y_prefix = match y.parse::<u8>() {
                    Err(_) => return Err(ASN1DecodeErr::InvalidDateValue(v)),
                    Ok(y) => {
                        if y >= 50 {
                            "19"
                        } else {
                            "20"
                        }
                    }
                };

                let v = format!("{}{}", y_prefix, v);

                let format = time::format_description::parse(
                    "[year][month][day][hour repr:24][minute][second]Z",
                )
                .unwrap();

                match PrimitiveDateTime::parse(&v, &format) {
                    Err(_) => return Err(ASN1DecodeErr::InvalidDateValue(v)),
                    Ok(t) => result.push(ASN1Block::UTCTime(soff, t)),
                }
            }
            // GeneralizedTime
            Some(0x18) => {
                if body.len() < 15 {
                    return Err(ASN1DecodeErr::InvalidDateValue(format!("{}", body.len())));
                }

                let mut v: String = String::from_utf8(body.to_vec())
                    .map_err(|e| ASN1DecodeErr::UTF8DecodeFailure(e.utf8_error()))?;
                // Make sure the string is ascii, otherwise we cannot insert
                // chars at specific bytes.
                if !v.is_ascii() {
                    return Err(ASN1DecodeErr::InvalidDateValue(v));
                }

                // We need to add padding back to the string if it's not there.
                if !v.contains('.') {
                    v.insert(14, '.')
                }
                while v.len() < 25 {
                    let idx = v.len() - 1;
                    v.insert(idx, '0');
                }

                let format = time::format_description::parse(
                    "[year][month][day][hour repr:24][minute][second].[subsecond]Z",
                )
                .unwrap();

                match PrimitiveDateTime::parse(&v, &format) {
                    Err(_) => return Err(ASN1DecodeErr::InvalidDateValue(v)),
                    Ok(t) => result.push(ASN1Block::GeneralizedTime(soff, t)),
                }
            }
            // UNIVERSAL STRINGS
            Some(0x1C) => match String::from_utf8(body.to_vec()) {
                Ok(v) => result.push(ASN1Block::UniversalString(soff, v)),
                Err(e) => return Err(ASN1DecodeErr::UTF8DecodeFailure(e.utf8_error())),
            },
            // UNIVERSAL STRINGS
            Some(0x1E) => match String::from_utf8(body.to_vec()) {
                Ok(v) => result.push(ASN1Block::BMPString(soff, v)),
                Err(e) => return Err(ASN1DecodeErr::UTF8DecodeFailure(e.utf8_error())),
            },
            // Dunno.
            _ => {
                result.push(ASN1Block::Unknown(
                    class,
                    constructed,
                    soff,
                    tag,
                    body.to_vec(),
                ));
            }
        }
        index += len;
    }

    if result.is_empty() {
        Err(ASN1DecodeErr::EmptyBuffer)
    } else {
        Ok(result)
    }
}

/// Returns the tag, if the type is constructed and the class.
fn decode_tag(i: &[u8], index: &mut usize) -> Result<(BigUint, bool, ASN1Class), ASN1DecodeErr> {
    if *index >= i.len() {
        return Err(ASN1DecodeErr::Incomplete);
    }
    let tagbyte = i[*index];
    let constructed = (tagbyte & 0b0010_0000) != 0;
    let class = decode_class(tagbyte)?;
    let basetag = tagbyte & 0b1_1111;

    *index += 1;

    if basetag == 0b1_1111 {
        let res = decode_base127(i, index)?;
        Ok((res, constructed, class))
    } else {
        Ok((BigUint::from(basetag), constructed, class))
    }
}

fn decode_base127(i: &[u8], index: &mut usize) -> Result<BigUint, ASN1DecodeErr> {
    let mut res = BigUint::zero();

    loop {
        if *index >= i.len() {
            return Err(ASN1DecodeErr::Incomplete);
        }

        let nextbyte = i[*index];

        *index += 1;
        res = (res << 7) + BigUint::from(nextbyte & 0x7f);
        if (nextbyte & 0x80) == 0 {
            return Ok(res);
        }
    }
}

fn decode_class(i: u8) -> Result<ASN1Class, ASN1DecodeErr> {
    match i >> 6 {
        0b00 => Ok(ASN1Class::Universal),
        0b01 => Ok(ASN1Class::Application),
        0b10 => Ok(ASN1Class::ContextSpecific),
        0b11 => Ok(ASN1Class::Private),
        _ => Err(ASN1DecodeErr::InvalidClass(i)),
    }
}

fn decode_length(i: &[u8], index: &mut usize) -> Result<usize, ASN1DecodeErr> {
    if *index >= i.len() {
        return Err(ASN1DecodeErr::Incomplete);
    }
    let startbyte = i[*index];

    // NOTE: Technically, this size can be much larger than a usize.
    // However, our whole universe starts to break down if we get
    // things that big. So we're boring, and only accept lengths
    // that fit within a usize.
    *index += 1;
    if startbyte >= 0x80 {
        let mut lenlen = (startbyte & 0x7f) as usize;
        let mut res = 0;

        if lenlen > size_of::<usize>() {
            return Err(ASN1DecodeErr::LengthTooLarge(lenlen));
        }

        while lenlen > 0 {
            if *index >= i.len() {
                return Err(ASN1DecodeErr::Incomplete);
            }

            res = (res << 8) + (i[*index] as usize);

            *index += 1;
            lenlen -= 1;
        }

        Ok(res)
    } else {
        Ok(startbyte as usize)
    }
}

/// Given an `ASN1Block`, covert it to its DER encoding, or return an error
/// if something broke along the way.
pub fn to_der(i: &ASN1Block) -> Result<Vec<u8>, ASN1EncodeErr> {
    match *i {
        // BOOLEAN
        ASN1Block::Boolean(_, val) => {
            let inttag = BigUint::one();
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);
            tagbytes.push(1);
            tagbytes.push(if val { 0xFF } else { 0x00 });
            Ok(tagbytes)
        }
        // INTEGER
        ASN1Block::Integer(_, ref int) => {
            let mut base = int.to_signed_bytes_be();
            let mut lenbytes = encode_len(base.len());
            let inttag = BigUint::from_u8(0x02).unwrap();
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);

            let mut result = Vec::new();
            result.append(&mut tagbytes);
            result.append(&mut lenbytes);
            result.append(&mut base);
            Ok(result)
        }
        // BIT STRING
        ASN1Block::BitString(_, bits, ref vs) => {
            let inttag = BigUint::from_u8(0x03).unwrap();
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);

            if bits == 0 {
                tagbytes.push(0);
                Ok(tagbytes)
            } else {
                let mut lenbytes = encode_len(vs.len() + 1);
                let nbits = (vs.len() * 8) - bits;

                let mut result = Vec::new();
                result.append(&mut tagbytes);
                result.append(&mut lenbytes);
                result.push(nbits as u8);
                result.extend_from_slice(vs);
                Ok(result)
            }
        }
        // OCTET STRING
        ASN1Block::OctetString(_, ref bytes) => {
            let inttag = BigUint::from_u8(0x04).unwrap();
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);
            let mut lenbytes = encode_len(bytes.len());

            let mut result = Vec::new();
            result.append(&mut tagbytes);
            result.append(&mut lenbytes);
            result.extend_from_slice(bytes);
            Ok(result)
        }
        // NULL
        ASN1Block::Null(_) => {
            let inttag = BigUint::from_u8(0x05).unwrap();
            let mut result = encode_tag(ASN1Class::Universal, false, &inttag);
            result.push(0);
            Ok(result)
        }
        // OBJECT IDENTIFIER
        ASN1Block::ObjectIdentifier(_, OID(ref nums)) => {
            match (nums.get(0), nums.get(1)) {
                (Some(v1), Some(v2)) => {
                    let two = BigUint::from_u8(2).unwrap();

                    // first, validate that the first two items meet spec
                    if v1 > &two {
                        return Err(ASN1EncodeErr::ObjectIdentVal1TooLarge);
                    }

                    let u175 = BigUint::from_u8(175).unwrap();
                    let u39 = BigUint::from_u8(39).unwrap();
                    let bound = if v1 == &two { u175 } else { u39 };

                    if v2 > &bound {
                        return Err(ASN1EncodeErr::ObjectIdentVal2TooLarge);
                    }

                    // the following unwraps must be safe, based on the
                    // validation above.
                    let value1 = v1.to_u8().unwrap();
                    let value2 = v2.to_u8().unwrap();
                    let byte1 = (value1 * 40) + value2;

                    // now we can build all the rest of the body
                    let mut body = vec![byte1];
                    for num in nums.iter().skip(2) {
                        let mut local = encode_base127(num);
                        body.append(&mut local);
                    }

                    // now that we have the body, we can build the header
                    let inttag = BigUint::from_u8(0x06).unwrap();
                    let mut result = encode_tag(ASN1Class::Universal, false, &inttag);
                    let mut lenbytes = encode_len(body.len());

                    result.append(&mut lenbytes);
                    result.append(&mut body);

                    Ok(result)
                }
                _ => Err(ASN1EncodeErr::ObjectIdentHasTooFewFields),
            }
        }
        // SEQUENCE
        ASN1Block::Sequence(_, ref items) => {
            let mut body = Vec::new();

            // put all the subsequences into a block
            for x in items.iter() {
                let mut bytes = to_der(x)?;
                body.append(&mut bytes);
            }

            let inttag = BigUint::from_u8(0x10).unwrap();
            let mut lenbytes = encode_len(body.len());
            // SEQUENCE and SET mut have the constructed encoding form (bit 5) set
            // See: https://docs.microsoft.com/en-us/windows/desktop/seccertenroll/about-encoded-tag-bytes
            let mut tagbytes = encode_tag(ASN1Class::Universal, true, &inttag);

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.append(&mut body);
            Ok(res)
        }
        // SET
        ASN1Block::Set(_, ref items) => {
            let mut body = Vec::new();

            // put all the subsequences into a block
            for x in items.iter() {
                let mut bytes = to_der(x)?;
                body.append(&mut bytes);
            }

            let inttag = BigUint::from_u8(0x11).unwrap();
            let mut lenbytes = encode_len(body.len());
            // SEQUENCE and SET mut have the constructed encoding form (bit 5) set
            // See: https://docs.microsoft.com/en-us/windows/desktop/seccertenroll/about-encoded-tag-bytes
            let mut tagbytes = encode_tag(ASN1Class::Universal, true, &inttag);

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.append(&mut body);
            Ok(res)
        }
        ASN1Block::UTCTime(_, ref time) => {
            let format = time::format_description::parse(
                "[year][month][day][hour repr:24][minute][second]Z",
            )
            .unwrap();
            let mut body = time.format(&format).unwrap().into_bytes();
            body.drain(0..2);
            let inttag = BigUint::from_u8(0x17).unwrap();
            let mut lenbytes = encode_len(body.len());
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.append(&mut body);
            Ok(res)
        }
        ASN1Block::GeneralizedTime(_, ref time) => {
            let format = time::format_description::parse(
                "[year][month][day][hour repr:24][minute][second].[subsecond]",
            )
            .unwrap();
            let base = time.format(&format).unwrap();
            let zclear = base.trim_end_matches('0');
            let dclear = zclear.trim_end_matches('.');
            let mut body = format!("{}Z", dclear).into_bytes();

            let inttag = BigUint::from_u8(0x18).unwrap();
            let mut lenbytes = encode_len(body.len());
            let mut tagbytes = encode_tag(ASN1Class::Universal, false, &inttag);

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.append(&mut body);
            Ok(res)
        }
        ASN1Block::UTF8String(_, ref str) => {
            encode_asn1_string(0x0c, false, ASN1Class::Universal, str)
        }
        ASN1Block::PrintableString(_, ref str) => {
            encode_asn1_string(0x13, true, ASN1Class::Universal, str)
        }
        ASN1Block::TeletexString(_, ref str) => {
            encode_asn1_string(0x14, false, ASN1Class::Universal, str)
        }
        ASN1Block::UniversalString(_, ref str) => {
            encode_asn1_string(0x1c, false, ASN1Class::Universal, str)
        }
        ASN1Block::IA5String(_, ref str) => {
            encode_asn1_string(0x16, true, ASN1Class::Universal, str)
        }
        ASN1Block::BMPString(_, ref str) => {
            encode_asn1_string(0x1e, false, ASN1Class::Universal, str)
        }
        ASN1Block::Explicit(class, _, ref tag, ref item) => {
            let mut tagbytes = encode_tag(class, true, tag);
            let mut bytes = to_der(item)?;
            let mut lenbytes = encode_len(bytes.len());

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.append(&mut bytes);
            Ok(res)
        }
        // Unknown blocks
        ASN1Block::Unknown(class, c, _, ref tag, ref bytes) => {
            let mut tagbytes = encode_tag(class, c, tag);
            let mut lenbytes = encode_len(bytes.len());

            let mut res = Vec::new();
            res.append(&mut tagbytes);
            res.append(&mut lenbytes);
            res.extend_from_slice(bytes);
            Ok(res)
        }
    }
}

fn encode_asn1_string(
    tag: u8,
    force_chars: bool,
    c: ASN1Class,
    s: &str,
) -> Result<Vec<u8>, ASN1EncodeErr> {
    let mut body = {
        if force_chars {
            let mut out = Vec::new();

            for c in s.chars() {
                out.push(c as u8);
            }
            out
        } else {
            s.to_string().into_bytes()
        }
    };
    let inttag = BigUint::from_u8(tag).unwrap();
    let mut lenbytes = encode_len(body.len());
    let mut tagbytes = encode_tag(c, false, &inttag);

    let mut res = Vec::new();
    res.append(&mut tagbytes);
    res.append(&mut lenbytes);
    res.append(&mut body);
    Ok(res)
}

fn encode_tag(c: ASN1Class, constructed: bool, t: &BigUint) -> Vec<u8> {
    let cbyte = encode_class(c);

    match t.to_u8() {
        Some(mut x) if x < 31 => {
            if constructed {
                x |= 0b0010_0000;
            }
            vec![cbyte | x]
        }
        _ => {
            let mut res = encode_base127(t);
            let mut x = cbyte | 0b0001_1111;
            if constructed {
                x |= 0b0010_0000;
            }
            res.insert(0, x);
            res
        }
    }
}

fn encode_base127(v: &BigUint) -> Vec<u8> {
    let mut acc = v.clone();
    let mut res = Vec::new();
    let u128 = BigUint::from_u8(128).unwrap();
    let zero = BigUint::zero();

    if acc == zero {
        res.push(0);
        return res;
    }

    while acc > zero {
        // we build this vector backwards
        let digit = &acc % &u128;
        acc >>= 7;

        match digit.to_u8() {
            None => panic!("7 bits don't fit into 8, cause ..."),
            Some(x) if res.is_empty() => res.push(x),
            Some(x) => res.push(x | 0x80),
        }
    }

    res.reverse();
    res
}

fn encode_class(c: ASN1Class) -> u8 {
    match c {
        ASN1Class::Universal => 0b0000_0000,
        ASN1Class::Application => 0b0100_0000,
        ASN1Class::ContextSpecific => 0b1000_0000,
        ASN1Class::Private => 0b1100_0000,
    }
}

fn encode_len(x: usize) -> Vec<u8> {
    if x < 128 {
        vec![x as u8]
    } else {
        let mut bstr = Vec::new();
        let mut work = x;

        // convert this into bytes, backwards
        while work > 0 {
            bstr.push(work as u8);
            work >>= 8;
        }

        // encode the front of the length
        let len = bstr.len() as u8;
        bstr.push(len | 0x80);

        // and then reverse it into the right order
        bstr.reverse();
        bstr
    }
}

// ----------------------------------------------------------------------------

/// A trait defining types that can be decoded from an `ASN1Block` stream,
/// assuming they also have access to the underlying bytes making up the
/// stream.
pub trait FromASN1WithBody: Sized {
    type Error: From<ASN1DecodeErr>;

    fn from_asn1_with_body<'a>(
        v: &'a [ASN1Block],
        _b: &[u8],
    ) -> Result<(Self, &'a [ASN1Block]), Self::Error>;
}

/// A trait defining types that can be decoded from an `ASN1Block` stream.
/// Any member of this trait is also automatically a member of
/// `FromASN1WithBody`, as it can obviously just ignore the body.
pub trait FromASN1: Sized {
    type Error: From<ASN1DecodeErr>;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error>;
}

impl<T: FromASN1> FromASN1WithBody for T {
    type Error = T::Error;

    fn from_asn1_with_body<'a>(
        v: &'a [ASN1Block],
        _b: &[u8],
    ) -> Result<(T, &'a [ASN1Block]), T::Error> {
        T::from_asn1(v)
    }
}

/// Automatically decode a type via DER encoding, assuming that the type
/// is a member of `FromASN1` or `FromASN1WithBody`.
pub fn der_decode<T: FromASN1WithBody>(v: &[u8]) -> Result<T, T::Error> {
    let vs = from_der(v)?;
    T::from_asn1_with_body(&vs, v).map(|(a, _)| a)
}

/// The set of types that can automatically converted into a sequence
/// of `ASN1Block`s. You should probably use to_asn1() but implement
/// to_asn1_class(). The former has a default implementation that passes
/// `ASN1Class::Universal` as the tag to use, which should be good for
/// most people.
pub trait ToASN1 {
    type Error: From<ASN1EncodeErr>;

    fn to_asn1(&self) -> Result<Vec<ASN1Block>, Self::Error> {
        self.to_asn1_class(ASN1Class::Universal)
    }
    fn to_asn1_class(&self, c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error>;
}

/// Automatically encode a type into binary via DER encoding, assuming
/// that the type is a member of `ToASN1`.
pub fn der_encode<T: ToASN1>(v: &T) -> Result<Vec<u8>, T::Error> {
    let blocks = T::to_asn1(v)?;
    let mut res = Vec::new();

    for block in blocks {
        let mut x = to_der(&block)?;
        res.append(&mut x);
    }

    Ok(res)
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use std::fs::File;
    use std::io::Read;
    use time::{Date, Month, Time};

    impl Arbitrary for ASN1Class {
        fn arbitrary(g: &mut Gen) -> ASN1Class {
            match u8::arbitrary(g) % 4 {
                0 => ASN1Class::Private,
                1 => ASN1Class::ContextSpecific,
                2 => ASN1Class::Universal,
                3 => ASN1Class::Application,
                _ => panic!("I weep for a broken life."),
            }
        }
    }

    quickcheck! {
        fn class_encdec_roundtrips(c: ASN1Class) -> bool {
            c == decode_class(encode_class(c.clone())).unwrap()
        }

        fn class_decenc_roundtrips(v: u8) -> bool {
            (v & 0b11000000) == encode_class(decode_class(v).unwrap())
        }
    }

    #[derive(Clone, Debug)]
    struct RandomUint {
        x: BigUint,
    }

    impl Arbitrary for RandomUint {
        fn arbitrary(g: &mut Gen) -> RandomUint {
            let v = BigUint::from_u32(u32::arbitrary(g)).unwrap();
            RandomUint { x: v }
        }
    }

    quickcheck! {
        fn tags_encdec_roundtrips(c: ASN1Class, con: bool, t: RandomUint) -> bool {
            let bytes = encode_tag(c, con, &t.x);
            let mut zero = 0;
            let (t2, con2, c2) = decode_tag(&bytes[..], &mut zero).unwrap();
            (c == c2) && (con == con2) && (t.x == t2)
        }

        fn len_encdec_roundtrips(l: usize) -> bool {
            let bytes = encode_len(l);
            let mut zero = 0;
            match decode_length(&bytes[..], &mut zero) {
                Err(_) => false,
                Ok(l2) => l == l2
            }
        }
    }

    #[derive(Clone, Debug)]
    struct RandomInt {
        x: BigInt,
    }

    impl Arbitrary for RandomInt {
        fn arbitrary(g: &mut Gen) -> RandomInt {
            let v = BigInt::from_i64(i64::arbitrary(g)).unwrap();
            RandomInt { x: v }
        }
    }

    #[allow(type_alias_bounds)]
    type ASN1BlockGen = fn(&mut Gen, usize) -> ASN1Block;

    fn arb_boolean(g: &mut Gen, _d: usize) -> ASN1Block {
        let v = bool::arbitrary(g);
        ASN1Block::Boolean(0, v)
    }

    fn arb_integer(g: &mut Gen, _d: usize) -> ASN1Block {
        let d = RandomInt::arbitrary(g);
        ASN1Block::Integer(0, d.x)
    }

    fn arb_bitstr(g: &mut Gen, _d: usize) -> ASN1Block {
        let size = u16::arbitrary(g) as usize % 16;
        let maxbits = (size as usize) * 8;
        let modbits = u8::arbitrary(g) as usize % 8;
        let nbits = if modbits > maxbits {
            maxbits
        } else {
            maxbits - modbits
        };

        let mut bytes = Vec::with_capacity(size);
        while bytes.len() < size {
            bytes.push(u8::arbitrary(g));
        }

        ASN1Block::BitString(0, nbits, bytes)
    }

    fn arb_octstr(g: &mut Gen, _d: usize) -> ASN1Block {
        let size = usize::arbitrary(g) % 16;
        let mut bytes = Vec::with_capacity(size);

        while bytes.len() < size {
            bytes.push(u8::arbitrary(g));
        }

        ASN1Block::OctetString(0, bytes)
    }

    fn arb_null(_g: &mut Gen, _d: usize) -> ASN1Block {
        ASN1Block::Null(0)
    }

    impl Arbitrary for OID {
        fn arbitrary(g: &mut Gen) -> OID {
            let count = usize::arbitrary(g) % 40;
            let val1 = u8::arbitrary(g) % 3;
            let v2mod = if val1 == 2 { 176 } else { 40 };
            let val2 = u8::arbitrary(g) % v2mod;
            let v1 = BigUint::from_u8(val1).unwrap();
            let v2 = BigUint::from_u8(val2).unwrap();
            let mut nums = vec![v1, v2];

            for _ in 0..count {
                let num = RandomUint::arbitrary(g);
                nums.push(num.x);
            }

            OID(nums)
        }
    }

    fn arb_objid(g: &mut Gen, _d: usize) -> ASN1Block {
        let oid = OID::arbitrary(g);
        ASN1Block::ObjectIdentifier(0, oid)
    }

    fn arb_seq(g: &mut Gen, d: usize) -> ASN1Block {
        let count = usize::arbitrary(g) % 63 + 1;
        let mut items = Vec::new();

        for _ in 0..count {
            items.push(limited_arbitrary(g, d - 1));
        }

        ASN1Block::Sequence(0, items)
    }

    fn arb_set(g: &mut Gen, d: usize) -> ASN1Block {
        let count = usize::arbitrary(g) % 63 + 1;
        let mut items = Vec::new();

        for _ in 0..count {
            items.push(limited_arbitrary(g, d - 1));
        }

        ASN1Block::Set(0, items)
    }

    fn arb_print(g: &mut Gen, _d: usize) -> ASN1Block {
        let count = usize::arbitrary(g) % 384;
        let mut items = Vec::new();

        for _ in 0..count {
            let v = g.choose(PRINTABLE_CHARS.as_bytes());
            items.push(*v.unwrap() as char);
        }

        ASN1Block::PrintableString(0, String::from_iter(items.iter()))
    }

    fn arb_ia5(g: &mut Gen, _d: usize) -> ASN1Block {
        let count = usize::arbitrary(g) % 384;
        let mut items = Vec::new();

        for _ in 0..count {
            items.push(u8::arbitrary(g) as char);
        }

        ASN1Block::IA5String(0, String::from_iter(items.iter()))
    }

    fn arb_utf8(g: &mut Gen, _d: usize) -> ASN1Block {
        let val = String::arbitrary(g);
        ASN1Block::UTF8String(0, val)
    }

    fn arb_tele(g: &mut Gen, _d: usize) -> ASN1Block {
        let val = String::arbitrary(g);
        ASN1Block::TeletexString(0, val)
    }

    fn arb_uni(g: &mut Gen, _d: usize) -> ASN1Block {
        let val = String::arbitrary(g);
        ASN1Block::UniversalString(0, val)
    }

    fn arb_bmp(g: &mut Gen, _d: usize) -> ASN1Block {
        let val = String::arbitrary(g);
        ASN1Block::BMPString(0, val)
    }

    fn arb_utc(g: &mut Gen, _d: usize) -> ASN1Block {
        let min = Date::from_calendar_date(1950, Month::January, 01)
            .unwrap()
            .to_julian_day();
        let max = Date::from_calendar_date(2049, Month::December, 31)
            .unwrap()
            .to_julian_day();
        let date =
            Date::from_julian_day(i32::arbitrary(g).rem_euclid(max - min + 1) + min).unwrap();

        let h = u8::arbitrary(g).rem_euclid(24);
        let m = u8::arbitrary(g).rem_euclid(60);
        let s = u8::arbitrary(g).rem_euclid(60);
        let time = Time::from_hms(h, m, s).unwrap();

        let t = PrimitiveDateTime::new(date, time);
        ASN1Block::UTCTime(0, t)
    }

    fn arb_time(g: &mut Gen, _d: usize) -> ASN1Block {
        let min = Date::from_calendar_date(0, Month::January, 01)
            .unwrap()
            .to_julian_day();
        let max = Date::from_calendar_date(9999, Month::December, 31)
            .unwrap()
            .to_julian_day();
        let date =
            Date::from_julian_day(i32::arbitrary(g).rem_euclid(max - min + 1) + min).unwrap();

        let time = Time::arbitrary(g);

        let t = PrimitiveDateTime::new(date, time);
        ASN1Block::GeneralizedTime(0, t)
    }

    fn arb_explicit(g: &mut Gen, d: usize) -> ASN1Block {
        let mut class = ASN1Class::arbitrary(g);
        if class == ASN1Class::Universal {
            // Universal is invalid for an explicitly tagged block
            class = ASN1Class::ContextSpecific;
        }
        let tag = RandomUint::arbitrary(g);
        let item = limited_arbitrary(g, d - 1);

        ASN1Block::Explicit(class, 0, tag.x, Box::new(item))
    }

    fn arb_unknown(g: &mut Gen, _d: usize) -> ASN1Block {
        let class = ASN1Class::arbitrary(g);
        let tag = loop {
            let potential = RandomUint::arbitrary(g);
            match potential.x.to_u8() {
                None => break potential,
                Some(x) if KNOWN_TAGS.contains(&x) => {}
                Some(_) => break potential,
            }
        };
        let size = usize::arbitrary(g) % 128;
        let mut items = Vec::with_capacity(size);

        while items.len() < size {
            items.push(u8::arbitrary(g));
        }

        ASN1Block::Unknown(class, false, 0, tag.x, items)
    }

    fn limited_arbitrary(g: &mut Gen, d: usize) -> ASN1Block {
        let mut possibles: Vec<ASN1BlockGen> = vec![
            arb_boolean,
            arb_integer,
            arb_bitstr,
            arb_octstr,
            arb_null,
            arb_objid,
            arb_utf8,
            arb_print,
            arb_tele,
            arb_uni,
            arb_ia5,
            arb_utc,
            arb_time,
            arb_bmp,
            arb_unknown,
        ];

        if d > 0 {
            possibles.push(arb_seq);
            possibles.push(arb_set);
            possibles.push(arb_explicit);
        }

        match g.choose(&possibles[..]) {
            Some(f) => f(g, d),
            None => panic!("Couldn't generate arbitrary value."),
        }
    }

    impl Arbitrary for ASN1Block {
        fn arbitrary(g: &mut Gen) -> ASN1Block {
            limited_arbitrary(g, 2)
        }
    }

    quickcheck! {
        fn encode_decode_roundtrips(v: ASN1Block) -> bool {
            match to_der(&v) {
                Err(e) => {
                    println!("Serialization error: {:?}", e);
                    false
                }
                Ok(bytes) =>
                    match from_der(&bytes[..]) {
                        Err(e) => {
                            println!("Parse error: {:?}", e);
                            false
                        }
                        Ok(ref rvec) if rvec.len() == 1 => {
                            let v2 = rvec.get(0).unwrap();
                            if &v != v2 {
                                println!("Original: {:?}", v);
                                println!("Constructed: {:?}", v2);
                            }
                            &v == v2
                        }
                        Ok(_) => {
                            println!("Too many results returned.");
                            false
                        }
                    }
            }
        }
    }

    fn result_int(v: i16) -> Result<Vec<ASN1Block>, ASN1DecodeErr> {
        let val = BigInt::from(v);
        Ok(vec![ASN1Block::Integer(0, val)])
    }

    #[test]
    fn utc_time_tests() {
        // Check for a regression against issue #27, in which this would
        // cause a panic.
        let input = [
            55, 13, 13, 133, 13, 13, 50, 13, 13, 133, 13, 13, 50, 13, 133,
        ];
        let output = from_der(&input);
        assert!(output.is_err());
    }

    #[test]
    fn generalized_time_tests() {
        check_spec(
            &PrimitiveDateTime::new(
                Date::from_calendar_date(1992, Month::May, 21).unwrap(),
                Time::from_hms(0, 0, 0).unwrap(),
            ),
            "19920521000000Z".to_string(),
        );
        check_spec(
            &PrimitiveDateTime::new(
                Date::from_calendar_date(1992, Month::June, 22).unwrap(),
                Time::from_hms(12, 34, 21).unwrap(),
            ),
            "19920622123421Z".to_string(),
        );
        check_spec(
            &PrimitiveDateTime::new(
                Date::from_calendar_date(1992, Month::July, 22).unwrap(),
                Time::from_hms_milli(13, 21, 00, 300).unwrap(),
            ),
            "19920722132100.3Z".to_string(),
        );
    }

    fn check_spec(d: &PrimitiveDateTime, s: String) {
        let b = ASN1Block::GeneralizedTime(0, d.clone());
        match to_der(&b) {
            Err(_) => assert_eq!(format!("Broken: {}", d), s),
            Ok(ref vec) => {
                let mut resvec = vec.clone();
                resvec.remove(0);
                resvec.remove(0);
                assert_eq!(String::from_utf8(resvec).unwrap(), s);
                match from_der_(vec, 0) {
                    Err(_) => assert_eq!(format!("Broken [reparse]: {}", d), s),
                    Ok(mut vec) => {
                        assert_eq!(vec.len(), 1);
                        match vec.pop() {
                            None => assert!(false, "The world's gone mad again."),
                            Some(ASN1Block::GeneralizedTime(_, d2)) => assert_eq!(&d2, d),
                            Some(_) => assert!(false, "Bad reparse of GeneralizedTime."),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn base_integer_tests() {
        assert_eq!(from_der(&vec![0x02, 0x01, 0x00]), result_int(0));
        assert_eq!(from_der(&vec![0x02, 0x01, 0x7F]), result_int(127));
        assert_eq!(from_der(&vec![0x02, 0x02, 0x00, 0x80]), result_int(128));
        assert_eq!(from_der(&vec![0x02, 0x02, 0x01, 0x00]), result_int(256));
        assert_eq!(from_der(&vec![0x02, 0x01, 0x80]), result_int(-128));
        assert_eq!(from_der(&vec![0x02, 0x02, 0xFF, 0x7F]), result_int(-129));
    }

    fn can_parse(f: &str) -> Result<Vec<ASN1Block>, ASN1DecodeErr> {
        let mut fd = File::open(f).unwrap();
        let mut buffer = Vec::new();
        let _amt = fd.read_to_end(&mut buffer);
        from_der(&buffer[..])
    }

    #[test]
    fn x509_tests() {
        can_parse("test/server.bin").unwrap();
        can_parse("test/key.bin").unwrap();
    }

    #[test]
    fn encode_base127_zero() {
        let zero = BigUint::from(0 as u64);
        let encoded = encode_base127(&zero);
        let expected: Vec<u8> = vec![0x0];
        assert_eq!(expected, encoded);
    }

    #[test]
    fn raw_oid_eq() {
        // data taken from https://tools.ietf.org/html/rfc4880
        // ( OID as vector of unsigned integers , asn1 encoded block)

        // comparision is not done against the full length, but only for
        // the actually encoded OID part (see the expect statement further down)
        let md5 = (
            oid!(1, 2, 840, 113549, 2, 5),
            vec![
                0x30, 0x20, 0x30, 0x0C, 0x06, 0x08, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x05,
                0x05, 0x00, 0x04, 0x10,
            ],
        );

        let ripmed160 = (
            oid!(1, 3, 36, 3, 2, 1),
            vec![
                0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2B, 0x24, 0x03, 0x02, 0x01, 0x05, 0x00, 0x04,
                0x14,
            ],
        );

        let sha1 = (
            oid!(1, 3, 14, 3, 2, 26),
            vec![
                0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0E, 0x03, 0x02, 0x1A, 0x05, 0x00, 0x04,
                0x14,
            ],
        );

        let sha224 = (
            oid!(2, 16, 840, 1, 101, 3, 4, 2, 4),
            vec![
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x04, 0x05, 0x00, 0x04, 0x1C,
            ],
        );

        let sha256 = (
            oid!(2, 16, 840, 1, 101, 3, 4, 2, 1),
            vec![
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x01, 0x05, 0x00, 0x04, 0x20,
            ],
        );

        let sha384 = (
            oid!(2, 16, 840, 1, 101, 3, 4, 2, 2),
            vec![
                0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x02, 0x05, 0x00, 0x04, 0x30,
            ],
        );

        let sha512 = (
            oid!(2, 16, 840, 1, 101, 3, 4, 2, 3),
            vec![
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x03, 0x05, 0x00, 0x04, 0x40,
            ],
        );

        let tests: Vec<(OID, Vec<u8>)> = vec![md5, ripmed160, sha1, sha224, sha256, sha384, sha512];

        for test in tests {
            let expected = test.1;
            let raw_oid = test.0.as_raw().expect("Failed to convert OID to raw");
            assert_eq!(raw_oid, &expected[6..(expected.len() - 4)]);
        }
    }

    #[test]
    fn vec_oid() {
        let vec_u64: Vec<u64> = vec![1, 2, 840, 10045, 4, 3, 2];
        let vec_i64: Vec<i64> = vec![1, 2, 840, 10045, 4, 3, 2];
        let vec_usize: Vec<usize> = vec![1, 2, 840, 10045, 4, 3, 2];

        let mut o = Vec::new();
        for val in vec_u64.iter() {
            o.push(BigUint::from(*val));
        }

        let oid = OID::new(o);

        assert_eq!(Ok(vec_u64), oid.as_vec());
        assert_eq!(Ok(vec_i64), oid.as_vec());
        assert_eq!(Ok(vec_usize), oid.as_vec());
        assert_eq!(Err(ASN1DecodeErr::Overflow), oid.as_vec::<u8>());
    }
}
