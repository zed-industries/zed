/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Event Stream message frame types and serialization/deserialization logic.

use crate::buf::count::CountBuf;
use crate::buf::crc::{CrcBuf, CrcBufMut};
use crate::error::{Error, ErrorKind};
use aws_smithy_types::config_bag::{Storable, StoreReplace};
use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
use aws_smithy_types::str_bytes::StrBytes;
use aws_smithy_types::DateTime;
use bytes::{Buf, BufMut};
use std::error::Error as StdError;
use std::fmt;
use std::mem::size_of;
use std::sync::{mpsc, Mutex};

const PRELUDE_LENGTH_BYTES: u32 = 3 * size_of::<u32>() as u32;
const PRELUDE_LENGTH_BYTES_USIZE: usize = PRELUDE_LENGTH_BYTES as usize;
const MESSAGE_CRC_LENGTH_BYTES: u32 = size_of::<u32>() as u32;
const MAX_HEADER_NAME_LEN: usize = 255;
const MIN_HEADER_LEN: usize = 2;

pub(crate) const TYPE_TRUE: u8 = 0;
pub(crate) const TYPE_FALSE: u8 = 1;
pub(crate) const TYPE_BYTE: u8 = 2;
pub(crate) const TYPE_INT16: u8 = 3;
pub(crate) const TYPE_INT32: u8 = 4;
pub(crate) const TYPE_INT64: u8 = 5;
pub(crate) const TYPE_BYTE_ARRAY: u8 = 6;
pub(crate) const TYPE_STRING: u8 = 7;
pub(crate) const TYPE_TIMESTAMP: u8 = 8;
pub(crate) const TYPE_UUID: u8 = 9;

pub type SignMessageError = Box<dyn StdError + Send + Sync + 'static>;

/// Signs an Event Stream message.
pub trait SignMessage: fmt::Debug {
    fn sign(&mut self, message: Message) -> Result<Message, SignMessageError>;

    /// SigV4 requires an empty last signed message to be sent.
    /// Other protocols do not require one.
    /// Return `Some(_)` to send a signed last empty message, before completing the stream.
    /// Return `None` to not send one and terminate the stream immediately.
    fn sign_empty(&mut self) -> Option<Result<Message, SignMessageError>>;
}

/// A sender that gets placed in the request config to wire up an event stream signer after signing.
#[derive(Debug)]
#[non_exhaustive]
pub struct DeferredSignerSender(Mutex<mpsc::Sender<Box<dyn SignMessage + Send + Sync>>>);

impl DeferredSignerSender {
    /// Creates a new `DeferredSignerSender`
    fn new(tx: mpsc::Sender<Box<dyn SignMessage + Send + Sync>>) -> Self {
        Self(Mutex::new(tx))
    }

    /// Sends a signer on the channel
    pub fn send(
        &self,
        signer: Box<dyn SignMessage + Send + Sync>,
    ) -> Result<(), mpsc::SendError<Box<dyn SignMessage + Send + Sync>>> {
        self.0.lock().unwrap().send(signer)
    }
}

impl Storable for DeferredSignerSender {
    type Storer = StoreReplace<Self>;
}

/// Deferred event stream signer to allow a signer to be wired up later.
///
/// HTTP request signing takes place after serialization, and the event stream
/// message stream body is established during serialization. Since event stream
/// signing may need context from the initial HTTP signing operation, this
/// [`DeferredSigner`] is needed to wire up the signer later in the request lifecycle.
///
/// This signer basically just establishes a MPSC channel so that the sender can
/// be placed in the request's config. Then the HTTP signer implementation can
/// retrieve the sender from that config and send an actual signing implementation
/// with all the context needed.
///
/// When an event stream implementation needs to sign a message, the first call to
/// sign will acquire a signing implementation off of the channel and cache it
/// for the remainder of the operation.
#[derive(Debug)]
pub struct DeferredSigner {
    rx: Option<Mutex<mpsc::Receiver<Box<dyn SignMessage + Send + Sync>>>>,
    signer: Option<Box<dyn SignMessage + Send + Sync>>,
}

impl DeferredSigner {
    pub fn new() -> (Self, DeferredSignerSender) {
        let (tx, rx) = mpsc::channel();
        (
            Self {
                rx: Some(Mutex::new(rx)),
                signer: None,
            },
            DeferredSignerSender::new(tx),
        )
    }

    fn acquire(&mut self) -> &mut (dyn SignMessage + Send + Sync) {
        // Can't use `if let Some(signer) = &mut self.signer` because the borrow checker isn't smart enough
        if self.signer.is_some() {
            self.signer.as_mut().unwrap().as_mut()
        } else {
            self.signer = Some(
                self.rx
                    .take()
                    .expect("only taken once")
                    .lock()
                    .unwrap()
                    .try_recv()
                    .ok()
                    // TODO(enableNewSmithyRuntimeCleanup): When the middleware implementation is removed,
                    // this should panic rather than default to the `NoOpSigner`. The reason it defaults
                    // is because middleware-based generic clients don't have any default middleware,
                    // so there is no way to send a `NoOpSigner` by default when there is no other
                    // auth scheme. The orchestrator auth setup is a lot more robust and will make
                    // this problem trivial.
                    .unwrap_or_else(|| Box::new(NoOpSigner {}) as _),
            );
            self.acquire()
        }
    }
}

impl SignMessage for DeferredSigner {
    fn sign(&mut self, message: Message) -> Result<Message, SignMessageError> {
        self.acquire().sign(message)
    }

    fn sign_empty(&mut self) -> Option<Result<Message, SignMessageError>> {
        self.acquire().sign_empty()
    }
}

#[derive(Debug)]
pub struct NoOpSigner {}
impl SignMessage for NoOpSigner {
    fn sign(&mut self, message: Message) -> Result<Message, SignMessageError> {
        Ok(message)
    }

    fn sign_empty(&mut self) -> Option<Result<Message, SignMessageError>> {
        None
    }
}

/// Converts a Smithy modeled Event Stream type into a [`Message`].
pub trait MarshallMessage: fmt::Debug {
    /// Smithy modeled input type to convert from.
    type Input;

    fn marshall(&self, input: Self::Input) -> Result<Message, Error>;
}

/// A successfully unmarshalled message that is either an `Event` or an `Error`.
#[derive(Debug)]
pub enum UnmarshalledMessage<T, E> {
    Event(T),
    Error(E),
}

/// Converts an Event Stream [`Message`] into a Smithy modeled type.
pub trait UnmarshallMessage: fmt::Debug {
    /// Smithy modeled type to convert into.
    type Output;
    /// Smithy modeled error to convert into.
    type Error;

    fn unmarshall(
        &self,
        message: &Message,
    ) -> Result<UnmarshalledMessage<Self::Output, Self::Error>, Error>;
}

macro_rules! read_value {
    ($buf:ident, $typ:ident, $size_typ:ident, $read_fn:ident) => {
        if $buf.remaining() >= size_of::<$size_typ>() {
            Ok(HeaderValue::$typ($buf.$read_fn()))
        } else {
            Err(ErrorKind::InvalidHeaderValue.into())
        }
    };
}

fn read_header_value_from<B: Buf>(mut buffer: B) -> Result<HeaderValue, Error> {
    let value_type = buffer.get_u8();
    match value_type {
        TYPE_TRUE => Ok(HeaderValue::Bool(true)),
        TYPE_FALSE => Ok(HeaderValue::Bool(false)),
        TYPE_BYTE => read_value!(buffer, Byte, i8, get_i8),
        TYPE_INT16 => read_value!(buffer, Int16, i16, get_i16),
        TYPE_INT32 => read_value!(buffer, Int32, i32, get_i32),
        TYPE_INT64 => read_value!(buffer, Int64, i64, get_i64),
        TYPE_BYTE_ARRAY | TYPE_STRING => {
            if buffer.remaining() > size_of::<u16>() {
                let len = buffer.get_u16() as usize;
                if buffer.remaining() < len {
                    return Err(ErrorKind::InvalidHeaderValue.into());
                }
                let bytes = buffer.copy_to_bytes(len);
                if value_type == TYPE_STRING {
                    Ok(HeaderValue::String(
                        bytes.try_into().map_err(|_| ErrorKind::InvalidUtf8String)?,
                    ))
                } else {
                    Ok(HeaderValue::ByteArray(bytes))
                }
            } else {
                Err(ErrorKind::InvalidHeaderValue.into())
            }
        }
        TYPE_TIMESTAMP => {
            if buffer.remaining() >= size_of::<i64>() {
                let epoch_millis = buffer.get_i64();
                Ok(HeaderValue::Timestamp(DateTime::from_millis(epoch_millis)))
            } else {
                Err(ErrorKind::InvalidHeaderValue.into())
            }
        }
        TYPE_UUID => read_value!(buffer, Uuid, u128, get_u128),
        _ => Err(ErrorKind::InvalidHeaderValueType(value_type).into()),
    }
}

fn write_header_value_to<B: BufMut>(value: &HeaderValue, mut buffer: B) -> Result<(), Error> {
    use HeaderValue::*;
    match value {
        Bool(val) => buffer.put_u8(if *val { TYPE_TRUE } else { TYPE_FALSE }),
        Byte(val) => {
            buffer.put_u8(TYPE_BYTE);
            buffer.put_i8(*val);
        }
        Int16(val) => {
            buffer.put_u8(TYPE_INT16);
            buffer.put_i16(*val);
        }
        Int32(val) => {
            buffer.put_u8(TYPE_INT32);
            buffer.put_i32(*val);
        }
        Int64(val) => {
            buffer.put_u8(TYPE_INT64);
            buffer.put_i64(*val);
        }
        ByteArray(val) => {
            buffer.put_u8(TYPE_BYTE_ARRAY);
            buffer.put_u16(checked(val.len(), ErrorKind::HeaderValueTooLong.into())?);
            buffer.put_slice(&val[..]);
        }
        String(val) => {
            buffer.put_u8(TYPE_STRING);
            buffer.put_u16(checked(
                val.as_bytes().len(),
                ErrorKind::HeaderValueTooLong.into(),
            )?);
            buffer.put_slice(&val.as_bytes()[..]);
        }
        Timestamp(time) => {
            buffer.put_u8(TYPE_TIMESTAMP);
            buffer.put_i64(
                time.to_millis()
                    .map_err(|_| ErrorKind::TimestampValueTooLarge(*time))?,
            );
        }
        Uuid(val) => {
            buffer.put_u8(TYPE_UUID);
            buffer.put_u128(*val);
        }
        _ => {
            panic!("matched on unexpected variant in `aws_smithy_types::event_stream::HeaderValue`")
        }
    }
    Ok(())
}

/// Reads a header from the given `buffer`.
fn read_header_from<B: Buf>(mut buffer: B) -> Result<(Header, usize), Error> {
    if buffer.remaining() < MIN_HEADER_LEN {
        return Err(ErrorKind::InvalidHeadersLength.into());
    }

    let mut counting_buf = CountBuf::new(&mut buffer);
    let name_len = counting_buf.get_u8();
    if name_len as usize >= counting_buf.remaining() {
        return Err(ErrorKind::InvalidHeaderNameLength.into());
    }

    let name: StrBytes = counting_buf
        .copy_to_bytes(name_len as usize)
        .try_into()
        .map_err(|_| ErrorKind::InvalidUtf8String)?;
    let value = read_header_value_from(&mut counting_buf)?;
    Ok((Header::new(name, value), counting_buf.into_count()))
}

/// Writes the header to the given `buffer`.
fn write_header_to<B: BufMut>(header: &Header, mut buffer: B) -> Result<(), Error> {
    if header.name().as_bytes().len() > MAX_HEADER_NAME_LEN {
        return Err(ErrorKind::InvalidHeaderNameLength.into());
    }

    buffer.put_u8(u8::try_from(header.name().as_bytes().len()).expect("bounds check above"));
    buffer.put_slice(&header.name().as_bytes()[..]);
    write_header_value_to(header.value(), buffer)
}

/// Writes the given `headers` to a `buffer`.
pub fn write_headers_to<B: BufMut>(headers: &[Header], mut buffer: B) -> Result<(), Error> {
    for header in headers {
        write_header_to(header, &mut buffer)?;
    }
    Ok(())
}

// Returns (total_len, header_len)
fn read_prelude_from<B: Buf>(mut buffer: B) -> Result<(u32, u32), Error> {
    let mut crc_buffer = CrcBuf::new(&mut buffer);

    // If the buffer doesn't have the entire, then error
    let total_len = crc_buffer.get_u32();
    if crc_buffer.remaining() + size_of::<u32>() < total_len as usize {
        return Err(ErrorKind::InvalidMessageLength.into());
    }

    // Validate the prelude
    let header_len = crc_buffer.get_u32();
    let (expected_crc, prelude_crc) = (crc_buffer.into_crc(), buffer.get_u32());
    if expected_crc != prelude_crc {
        return Err(ErrorKind::PreludeChecksumMismatch(expected_crc, prelude_crc).into());
    }
    // The header length can be 0 or >= 2, but must fit within the frame size
    if header_len == 1 || header_len > max_header_len(total_len)? {
        return Err(ErrorKind::InvalidHeadersLength.into());
    }
    Ok((total_len, header_len))
}

/// Reads a message from the given `buffer`. For streaming use cases, use
/// the [`MessageFrameDecoder`] instead of this.
pub fn read_message_from<B: Buf>(mut buffer: B) -> Result<Message, Error> {
    if buffer.remaining() < PRELUDE_LENGTH_BYTES_USIZE {
        return Err(ErrorKind::InvalidMessageLength.into());
    }

    // Calculate a CRC as we go and read the prelude
    let mut crc_buffer = CrcBuf::new(&mut buffer);
    let (total_len, header_len) = read_prelude_from(&mut crc_buffer)?;

    // Verify we have the full frame before continuing
    let remaining_len = total_len
        .checked_sub(PRELUDE_LENGTH_BYTES)
        .ok_or_else(|| Error::from(ErrorKind::InvalidMessageLength))?;
    if crc_buffer.remaining() < remaining_len as usize {
        return Err(ErrorKind::InvalidMessageLength.into());
    }

    // Read headers
    let mut header_bytes_read = 0;
    let mut headers = Vec::new();
    while header_bytes_read < header_len as usize {
        let (header, bytes_read) = read_header_from(&mut crc_buffer)?;
        header_bytes_read += bytes_read;
        if header_bytes_read > header_len as usize {
            return Err(ErrorKind::InvalidHeaderValue.into());
        }
        headers.push(header);
    }

    // Read payload
    let payload_len = payload_len(total_len, header_len)?;
    let payload = crc_buffer.copy_to_bytes(payload_len as usize);

    let expected_crc = crc_buffer.into_crc();
    let message_crc = buffer.get_u32();
    if expected_crc != message_crc {
        return Err(ErrorKind::MessageChecksumMismatch(expected_crc, message_crc).into());
    }

    Ok(Message::new_from_parts(headers, payload))
}

/// Writes the `message` to the given `buffer`.
pub fn write_message_to(message: &Message, buffer: &mut dyn BufMut) -> Result<(), Error> {
    let mut headers = Vec::new();
    for header in message.headers() {
        write_header_to(header, &mut headers)?;
    }

    let headers_len = checked(headers.len(), ErrorKind::HeadersTooLong.into())?;
    let payload_len = checked(message.payload().len(), ErrorKind::PayloadTooLong.into())?;
    let message_len = [
        PRELUDE_LENGTH_BYTES,
        headers_len,
        payload_len,
        MESSAGE_CRC_LENGTH_BYTES,
    ]
    .iter()
    .try_fold(0u32, |acc, v| {
        acc.checked_add(*v)
            .ok_or_else(|| Error::from(ErrorKind::MessageTooLong))
    })?;

    let mut crc_buffer = CrcBufMut::new(buffer);
    crc_buffer.put_u32(message_len);
    crc_buffer.put_u32(headers_len);
    crc_buffer.put_crc();
    crc_buffer.put(&headers[..]);
    crc_buffer.put(&message.payload()[..]);
    crc_buffer.put_crc();
    Ok(())
}

fn checked<T: TryFrom<U>, U>(from: U, err: Error) -> Result<T, Error> {
    T::try_from(from).map_err(|_| err)
}

fn max_header_len(total_len: u32) -> Result<u32, Error> {
    total_len
        .checked_sub(PRELUDE_LENGTH_BYTES + MESSAGE_CRC_LENGTH_BYTES)
        .ok_or_else(|| Error::from(ErrorKind::InvalidMessageLength))
}

fn payload_len(total_len: u32, header_len: u32) -> Result<u32, Error> {
    total_len
        .checked_sub(
            header_len
                .checked_add(PRELUDE_LENGTH_BYTES + MESSAGE_CRC_LENGTH_BYTES)
                .ok_or_else(|| Error::from(ErrorKind::InvalidHeadersLength))?,
        )
        .ok_or_else(|| Error::from(ErrorKind::InvalidMessageLength))
}

#[cfg(test)]
mod message_tests {
    use super::read_message_from;
    use crate::error::ErrorKind;
    use crate::frame::{write_message_to, Header, HeaderValue, Message};
    use aws_smithy_types::DateTime;
    use bytes::Bytes;

    macro_rules! read_message_expect_err {
        ($bytes:expr, $err:pat) => {
            let result = read_message_from(&mut Bytes::from_static($bytes));
            let result = result.as_ref();
            assert!(result.is_err(), "Expected error, got {:?}", result);
            assert!(
                matches!(result.err().unwrap().kind(), $err),
                "Expected {}, got {:?}",
                stringify!($err),
                result
            );
        };
    }

    #[test]
    fn invalid_messages() {
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_header_string_value_length"),
            ErrorKind::InvalidHeaderValue
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_header_string_length_cut_off"),
            ErrorKind::InvalidHeaderValue
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_header_value_type"),
            ErrorKind::InvalidHeaderValueType(0x60)
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_header_name_length"),
            ErrorKind::InvalidHeaderNameLength
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_headers_length"),
            ErrorKind::InvalidHeadersLength
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_prelude_checksum"),
            ErrorKind::PreludeChecksumMismatch(0x8BB495FB, 0xDEADBEEF)
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_message_checksum"),
            ErrorKind::MessageChecksumMismatch(0x01a05860, 0xDEADBEEF)
        );
        read_message_expect_err!(
            include_bytes!("../test_data/invalid_header_name_length_too_long"),
            ErrorKind::InvalidUtf8String
        );
    }

    #[test]
    fn read_message_no_headers() {
        // Test message taken from the CRT:
        // https://github.com/awslabs/aws-c-event-stream/blob/main/tests/message_deserializer_test.c
        let data: &'static [u8] = &[
            0x00, 0x00, 0x00, 0x1D, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x52, 0x8c, 0x5a, 0x7b, 0x27,
            0x66, 0x6f, 0x6f, 0x27, 0x3a, 0x27, 0x62, 0x61, 0x72, 0x27, 0x7d, 0xc3, 0x65, 0x39,
            0x36,
        ];

        let result = read_message_from(&mut Bytes::from_static(data)).unwrap();
        assert_eq!(result.headers(), Vec::new());

        let expected_payload = b"{'foo':'bar'}";
        assert_eq!(expected_payload, result.payload().as_ref());
    }

    #[test]
    fn read_message_one_header() {
        // Test message taken from the CRT:
        // https://github.com/awslabs/aws-c-event-stream/blob/main/tests/message_deserializer_test.c
        let data: &'static [u8] = &[
            0x00, 0x00, 0x00, 0x3D, 0x00, 0x00, 0x00, 0x20, 0x07, 0xFD, 0x83, 0x96, 0x0C, b'c',
            b'o', b'n', b't', b'e', b'n', b't', b'-', b't', b'y', b'p', b'e', 0x07, 0x00, 0x10,
            b'a', b'p', b'p', b'l', b'i', b'c', b'a', b't', b'i', b'o', b'n', b'/', b'j', b's',
            b'o', b'n', 0x7b, 0x27, 0x66, 0x6f, 0x6f, 0x27, 0x3a, 0x27, 0x62, 0x61, 0x72, 0x27,
            0x7d, 0x8D, 0x9C, 0x08, 0xB1,
        ];

        let result = read_message_from(&mut Bytes::from_static(data)).unwrap();
        assert_eq!(
            result.headers(),
            vec![Header::new(
                "content-type",
                HeaderValue::String("application/json".into())
            )]
        );

        let expected_payload = b"{'foo':'bar'}";
        assert_eq!(expected_payload, result.payload().as_ref());
    }

    #[test]
    fn read_all_headers_and_payload() {
        let message = include_bytes!("../test_data/valid_with_all_headers_and_payload");
        let result = read_message_from(&mut Bytes::from_static(message)).unwrap();
        assert_eq!(
            result.headers(),
            vec![
                Header::new("true", HeaderValue::Bool(true)),
                Header::new("false", HeaderValue::Bool(false)),
                Header::new("byte", HeaderValue::Byte(50)),
                Header::new("short", HeaderValue::Int16(20_000)),
                Header::new("int", HeaderValue::Int32(500_000)),
                Header::new("long", HeaderValue::Int64(50_000_000_000)),
                Header::new(
                    "bytes",
                    HeaderValue::ByteArray(Bytes::from(&b"some bytes"[..]))
                ),
                Header::new("str", HeaderValue::String("some str".into())),
                Header::new(
                    "time",
                    HeaderValue::Timestamp(DateTime::from_secs(5_000_000))
                ),
                Header::new(
                    "uuid",
                    HeaderValue::Uuid(0xb79bc914_de21_4e13_b8b2_bc47e85b7f0b)
                ),
            ]
        );

        assert_eq!(b"some payload", result.payload().as_ref());
    }

    #[test]
    fn round_trip_all_headers_payload() {
        let message = Message::new(&b"some payload"[..])
            .add_header(Header::new("true", HeaderValue::Bool(true)))
            .add_header(Header::new("false", HeaderValue::Bool(false)))
            .add_header(Header::new("byte", HeaderValue::Byte(50)))
            .add_header(Header::new("short", HeaderValue::Int16(20_000)))
            .add_header(Header::new("int", HeaderValue::Int32(500_000)))
            .add_header(Header::new("long", HeaderValue::Int64(50_000_000_000)))
            .add_header(Header::new(
                "bytes",
                HeaderValue::ByteArray((&b"some bytes"[..]).into()),
            ))
            .add_header(Header::new("str", HeaderValue::String("some str".into())))
            .add_header(Header::new(
                "time",
                HeaderValue::Timestamp(DateTime::from_secs(5_000_000)),
            ))
            .add_header(Header::new(
                "uuid",
                HeaderValue::Uuid(0xb79bc914_de21_4e13_b8b2_bc47e85b7f0b),
            ));

        let mut actual = Vec::new();
        write_message_to(&message, &mut actual).unwrap();

        let expected = include_bytes!("../test_data/valid_with_all_headers_and_payload").to_vec();
        assert_eq!(expected, actual);

        let result = read_message_from(&mut Bytes::from(actual)).unwrap();
        assert_eq!(message.headers(), result.headers());
        assert_eq!(message.payload().as_ref(), result.payload().as_ref());
    }
}

/// Return value from [`MessageFrameDecoder`].
#[derive(Debug)]
pub enum DecodedFrame {
    /// There wasn't enough data in the buffer to decode a full message.
    Incomplete,
    /// There was enough data in the buffer to decode.
    Complete(Message),
}

/// Streaming decoder for decoding a [`Message`] from a stream.
#[non_exhaustive]
#[derive(Default, Debug)]
pub struct MessageFrameDecoder {
    prelude: [u8; PRELUDE_LENGTH_BYTES_USIZE],
    prelude_read: bool,
}

impl MessageFrameDecoder {
    /// Returns a new `MessageFrameDecoder`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines if the `buffer` has enough data in it to read a full frame.
    /// Returns `Ok(None)` if there's not enough data, or `Some(remaining)` where
    /// `remaining` is the number of bytes after the prelude that belong to the
    /// message that's in the buffer.
    fn remaining_bytes_if_frame_available<B: Buf>(
        &self,
        buffer: &B,
    ) -> Result<Option<usize>, Error> {
        if self.prelude_read {
            let remaining_len = (&self.prelude[..])
                .get_u32()
                .checked_sub(PRELUDE_LENGTH_BYTES)
                .ok_or_else(|| Error::from(ErrorKind::InvalidMessageLength))?;
            if buffer.remaining() >= remaining_len as usize {
                return Ok(Some(remaining_len as usize));
            }
        }
        Ok(None)
    }

    /// Resets the decoder.
    fn reset(&mut self) {
        self.prelude_read = false;
        self.prelude = [0u8; PRELUDE_LENGTH_BYTES_USIZE];
    }

    /// Attempts to decode a [`Message`] from the given `buffer`. This function expects
    /// to be called over and over again with more data in the buffer each time its called.
    /// When there's not enough data to decode a message, it returns `Ok(None)`.
    ///
    /// Once there is enough data to read a message prelude, then it will mutate the `Buf`
    /// position. The state from the reading of the prelude is stored in the decoder so that
    /// the next call will be able to decode the entire message, even though the prelude
    /// is no longer available in the `Buf`.
    pub fn decode_frame<B: Buf>(&mut self, mut buffer: B) -> Result<DecodedFrame, Error> {
        if !self.prelude_read && buffer.remaining() >= PRELUDE_LENGTH_BYTES_USIZE {
            buffer.copy_to_slice(&mut self.prelude);
            self.prelude_read = true;
        }

        if let Some(remaining_len) = self.remaining_bytes_if_frame_available(&buffer)? {
            let mut message_buf = (&self.prelude[..]).chain(buffer.take(remaining_len));
            let result = read_message_from(&mut message_buf).map(DecodedFrame::Complete);
            self.reset();
            return result;
        }

        Ok(DecodedFrame::Incomplete)
    }
}

#[cfg(test)]
mod message_frame_decoder_tests {
    use super::{DecodedFrame, MessageFrameDecoder};
    use crate::frame::read_message_from;
    use bytes::Bytes;
    use bytes_utils::SegmentedBuf;

    #[test]
    fn single_streaming_message() {
        let message = include_bytes!("../test_data/valid_with_all_headers_and_payload");

        let mut decoder = MessageFrameDecoder::new();
        let mut segmented = SegmentedBuf::new();
        for i in 0..(message.len() - 1) {
            segmented.push(&message[i..(i + 1)]);
            if let DecodedFrame::Complete(_) = decoder.decode_frame(&mut segmented).unwrap() {
                panic!("incomplete frame shouldn't result in message");
            }
        }

        segmented.push(&message[(message.len() - 1)..]);
        match decoder.decode_frame(&mut segmented).unwrap() {
            DecodedFrame::Incomplete => panic!("frame should be complete now"),
            DecodedFrame::Complete(actual) => {
                let expected = read_message_from(&mut Bytes::from_static(message)).unwrap();
                assert_eq!(expected, actual);
            }
        }
    }

    fn multiple_streaming_messages_chunk_size(chunk_size: usize) {
        let message1 = include_bytes!("../test_data/valid_with_all_headers_and_payload");
        let message2 = include_bytes!("../test_data/valid_empty_payload");
        let message3 = include_bytes!("../test_data/valid_no_headers");
        let mut repeated = message1.to_vec();
        repeated.extend_from_slice(message2);
        repeated.extend_from_slice(message3);

        let mut decoder = MessageFrameDecoder::new();
        let mut segmented = SegmentedBuf::new();
        let mut decoded = Vec::new();
        for window in repeated.chunks(chunk_size) {
            segmented.push(window);
            match dbg!(decoder.decode_frame(&mut segmented)).unwrap() {
                DecodedFrame::Incomplete => {}
                DecodedFrame::Complete(message) => {
                    decoded.push(message);
                }
            }
        }

        let expected1 = read_message_from(&mut Bytes::from_static(message1)).unwrap();
        let expected2 = read_message_from(&mut Bytes::from_static(message2)).unwrap();
        let expected3 = read_message_from(&mut Bytes::from_static(message3)).unwrap();
        assert_eq!(3, decoded.len());
        assert_eq!(expected1, decoded[0]);
        assert_eq!(expected2, decoded[1]);
        assert_eq!(expected3, decoded[2]);
    }

    #[test]
    fn multiple_streaming_messages() {
        for chunk_size in 1..=11 {
            println!("chunk size: {chunk_size}");
            multiple_streaming_messages_chunk_size(chunk_size);
        }
    }
}

#[cfg(test)]
mod deferred_signer_tests {
    use crate::frame::{DeferredSigner, Header, HeaderValue, Message, SignMessage};
    use bytes::Bytes;

    fn check_send_sync<T: Send + Sync>(value: T) -> T {
        value
    }

    #[test]
    fn deferred_signer() {
        #[derive(Default, Debug)]
        struct TestSigner {
            call_num: i32,
        }
        impl SignMessage for TestSigner {
            fn sign(
                &mut self,
                message: Message,
            ) -> Result<Message, crate::frame::SignMessageError> {
                self.call_num += 1;
                Ok(message.add_header(Header::new("call_num", HeaderValue::Int32(self.call_num))))
            }

            fn sign_empty(&mut self) -> Option<Result<Message, crate::frame::SignMessageError>> {
                None
            }
        }

        let (mut signer, sender) = check_send_sync(DeferredSigner::new());

        sender.send(Box::<TestSigner>::default()).expect("success");

        let message = signer.sign(Message::new(Bytes::new())).expect("success");
        assert_eq!(1, message.headers()[0].value().as_int32().unwrap());

        let message = signer.sign(Message::new(Bytes::new())).expect("success");
        assert_eq!(2, message.headers()[0].value().as_int32().unwrap());

        assert!(signer.sign_empty().is_none());
    }

    #[test]
    fn deferred_signer_defaults_to_noop_signer() {
        let (mut signer, _sender) = DeferredSigner::new();
        assert_eq!(
            Message::new(Bytes::new()),
            signer.sign(Message::new(Bytes::new())).unwrap()
        );
        assert!(signer.sign_empty().is_none());
    }
}
