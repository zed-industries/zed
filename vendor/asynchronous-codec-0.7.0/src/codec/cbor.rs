use std::io::Error as IoError;
use std::marker::PhantomData;

use crate::{Decoder, Encoder};
use bytes::{Buf, BufMut, BytesMut};

use serde::{Deserialize, Serialize};
use serde_cbor::Error as CborError;

/// A codec for JSON encoding and decoding using serde_cbor
/// Enc is the type to encode, Dec is the type to decode
/// ```
/// # use futures::{executor, SinkExt, TryStreamExt};
/// # use futures::io::Cursor;
/// use serde::{Serialize, Deserialize};
/// use asynchronous_codec::{CborCodec, Framed};
///
/// #[derive(Serialize, Deserialize)]
/// struct Something {
///     pub data: u16,
/// }
///
/// async move {
///     # let mut buf = vec![];
///     # let stream = Cursor::new(&mut buf);
///     // let stream = ...
///     let codec = CborCodec::<Something, Something>::new();
///     let mut framed = Framed::new(stream, codec);
///
///     while let Some(s) = framed.try_next().await.unwrap() {
///         println!("{:?}", s.data);
///     }
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct CborCodec<Enc, Dec> {
    enc: PhantomData<Enc>,
    dec: PhantomData<Dec>,
}

/// JSON Codec error enumeration
#[derive(Debug)]
pub enum CborCodecError {
    /// IO error
    Io(IoError),
    /// JSON error
    Cbor(CborError),
}

impl std::fmt::Display for CborCodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CborCodecError::Io(e) => write!(f, "I/O error: {}", e),
            CborCodecError::Cbor(e) => write!(f, "CBOR error: {}", e),
        }
    }
}

impl std::error::Error for CborCodecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CborCodecError::Io(ref e) => Some(e),
            CborCodecError::Cbor(ref e) => Some(e),
        }
    }
}

impl From<IoError> for CborCodecError {
    fn from(e: IoError) -> CborCodecError {
        CborCodecError::Io(e)
    }
}

impl From<CborError> for CborCodecError {
    fn from(e: CborError) -> CborCodecError {
        CborCodecError::Cbor(e)
    }
}

impl<Enc, Dec> CborCodec<Enc, Dec>
where
    for<'de> Dec: Deserialize<'de> + 'static,
    for<'de> Enc: Serialize + 'static,
{
    /// Creates a new `CborCodec` with the associated types
    pub fn new() -> CborCodec<Enc, Dec> {
        CborCodec {
            enc: PhantomData,
            dec: PhantomData,
        }
    }
}

impl<Enc, Dec> Clone for CborCodec<Enc, Dec>
where
    for<'de> Dec: Deserialize<'de> + 'static,
    for<'de> Enc: Serialize + 'static,
{
    /// Clone creates a new instance of the `CborCodec`
    fn clone(&self) -> CborCodec<Enc, Dec> {
        CborCodec::new()
    }
}

/// Decoder impl parses cbor objects from bytes
impl<Enc, Dec> Decoder for CborCodec<Enc, Dec>
where
    for<'de> Dec: Deserialize<'de> + 'static,
    for<'de> Enc: Serialize + 'static,
{
    type Item = Dec;
    type Error = CborCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Build deserializer
        let mut de = serde_cbor::Deserializer::from_slice(&buf);

        // Attempt deserialization
        let res: Result<Dec, _> = serde::de::Deserialize::deserialize(&mut de);

        // If we ran out before parsing, return none and try again later
        let item = match res {
            Ok(item) => item,
            Err(e) if e.is_eof() => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        // Update offset from iterator
        let offset = de.byte_offset();

        // Advance buffer
        buf.advance(offset);

        Ok(Some(item))
    }
}

/// Encoder impl encodes object streams to bytes
impl<Enc, Dec> Encoder for CborCodec<Enc, Dec>
where
    for<'de> Dec: Deserialize<'de> + 'static,
    for<'de> Enc: Serialize + 'static,
{
    type Item<'a> = Enc;
    type Error = CborCodecError;

    fn encode(&mut self, data: Self::Item<'_>, buf: &mut BytesMut) -> Result<(), Self::Error> {
        // Encode cbor
        let j = serde_cbor::to_vec(&data)?;

        // Write to buffer
        buf.reserve(j.len());
        buf.put_slice(&j);

        Ok(())
    }
}

impl<Enc, Dec> Default for CborCodec<Enc, Dec>
where
    for<'de> Dec: Deserialize<'de> + 'static,
    for<'de> Enc: Serialize + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use bytes::BytesMut;
    use serde::{Deserialize, Serialize};

    use super::CborCodec;
    use crate::{Decoder, Encoder};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        pub name: String,
        pub data: u16,
    }

    #[test]
    fn cbor_codec_encode_decode() {
        let mut codec = CborCodec::<TestStruct, TestStruct>::new();
        let mut buff = BytesMut::new();

        let item1 = TestStruct {
            name: "Test name".to_owned(),
            data: 16,
        };
        codec.encode(item1.clone(), &mut buff).unwrap();

        let item2 = codec.decode(&mut buff).unwrap().unwrap();
        assert_eq!(item1, item2);

        assert_eq!(codec.decode(&mut buff).unwrap(), None);

        assert_eq!(buff.len(), 0);
    }

    #[test]
    fn cbor_codec_partial_decode() {
        let mut codec = CborCodec::<TestStruct, TestStruct>::new();
        let mut buff = BytesMut::new();

        let item1 = TestStruct {
            name: "Test name".to_owned(),
            data: 34,
        };
        codec.encode(item1, &mut buff).unwrap();

        let mut start = buff.clone().split_to(4);
        assert_eq!(codec.decode(&mut start).unwrap(), None);

        codec.decode(&mut buff).unwrap().unwrap();

        assert_eq!(buff.len(), 0);
    }

    #[test]
    fn cbor_codec_eof_reached() {
        let mut codec = CborCodec::<TestStruct, TestStruct>::new();
        let mut buff = BytesMut::new();

        let item1 = TestStruct {
            name: "Test name".to_owned(),
            data: 34,
        };
        codec.encode(item1.clone(), &mut buff).unwrap();

        // Split the buffer into two.
        let mut buff_start = buff.clone().split_to(4);
        let buff_end = buff.clone().split_off(4);

        // Attempt to decode the first half of the buffer. This should return `Ok(None)` and not
        // advance the buffer.
        assert_eq!(codec.decode(&mut buff_start).unwrap(), None);
        assert_eq!(buff_start.len(), 4);

        // Combine the buffer back together.
        buff_start.extend(buff_end.iter());

        // It should now decode successfully.
        let item2 = codec.decode(&mut buff).unwrap().unwrap();
        assert_eq!(item1, item2);
    }

    #[test]
    fn cbor_codec_decode_error() {
        let mut codec = CborCodec::<TestStruct, TestStruct>::new();
        let mut buff = BytesMut::new();

        let item1 = TestStruct {
            name: "Test name".to_owned(),
            data: 34,
        };
        codec.encode(item1.clone(), &mut buff).unwrap();

        // Split the end off the buffer.
        let mut buff_end = buff.clone().split_off(4);
        let buff_end_length = buff_end.len();

        // Attempting to decode should return an error.
        assert!(codec.decode(&mut buff_end).is_err());
        assert_eq!(buff_end.len(), buff_end_length);
    }
}
