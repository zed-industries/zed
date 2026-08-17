use std::io;
use std::ops::Range;

use super::base::Payload;
use super::codec::Codec;
use super::message::PlainMessage;
use crate::enums::{ContentType, ProtocolVersion};
use crate::error::{Error, InvalidMessage, PeerMisbehaved};
use crate::msgs::codec;
use crate::msgs::message::{MessageError, OpaqueMessage};
use crate::record_layer::{Decrypted, RecordLayer};

/// This deframer works to reconstruct TLS messages from a stream of arbitrary-sized reads.
///
/// It buffers incoming data into a `Vec` through `read()`, and returns messages through `pop()`.
/// QUIC connections will call `push()` to append handshake payload data directly.
#[derive(Default)]
pub struct MessageDeframer {
    /// Set if the peer is not talking TLS, but some other
    /// protocol.  The caller should abort the connection, because
    /// the deframer cannot recover.
    last_error: Option<Error>,

    /// Buffer of data read from the socket, in the process of being parsed into messages.
    ///
    /// For buffer size management, checkout out the `read()` method.
    buf: Vec<u8>,

    /// If we're in the middle of joining a handshake payload, this is the metadata.
    joining_hs: Option<HandshakePayloadMeta>,

    /// What size prefix of `buf` is used.
    used: usize,
}

impl MessageDeframer {
    /// Return any decrypted messages that the deframer has been able to parse.
    ///
    /// Returns an `Error` if the deframer failed to parse some message contents or if decryption
    /// failed, `Ok(None)` if no full message is buffered or if trial decryption failed, and
    /// `Ok(Some(_))` if a valid message was found and decrypted successfully.
    pub fn pop(&mut self, record_layer: &mut RecordLayer) -> Result<Option<Deframed>, Error> {
        if let Some(last_err) = self.last_error.clone() {
            return Err(last_err);
        } else if self.used == 0 {
            return Ok(None);
        }

        // We loop over records we've received but not processed yet.
        // For records that decrypt as `Handshake`, we keep the current state of the joined
        // handshake message payload in `self.joining_hs`, appending to it as we see records.
        let expected_len = loop {
            let start = match &self.joining_hs {
                Some(meta) => {
                    match meta.expected_len {
                        // We're joining a handshake payload, and we've seen the full payload.
                        Some(len) if len <= meta.payload.len() => break len,
                        // Not enough data, and we can't parse any more out of the buffer (QUIC).
                        _ if meta.quic => return Ok(None),
                        // Try parsing some more of the encrypted buffered data.
                        _ => meta.message.end,
                    }
                }
                None => 0,
            };

            // Does our `buf` contain a full message?  It does if it is big enough to
            // contain a header, and that header has a length which falls within `buf`.
            // If so, deframe it and place the message onto the frames output queue.
            let mut rd = codec::Reader::init(&self.buf[start..self.used]);
            let m = match OpaqueMessage::read(&mut rd) {
                Ok(m) => m,
                Err(msg_err) => {
                    let err_kind = match msg_err {
                        MessageError::TooShortForHeader | MessageError::TooShortForLength => {
                            return Ok(None)
                        }
                        MessageError::InvalidEmptyPayload => InvalidMessage::InvalidEmptyPayload,
                        MessageError::MessageTooLarge => InvalidMessage::MessageTooLarge,
                        MessageError::InvalidContentType => InvalidMessage::InvalidContentType,
                        MessageError::UnknownProtocolVersion => {
                            InvalidMessage::UnknownProtocolVersion
                        }
                    };

                    return Err(self.set_err(err_kind));
                }
            };

            // If we're in the middle of joining a handshake payload and the next message is not of
            // type handshake, yield an error. Return CCS messages immediately without decrypting.
            let end = start + rd.used();
            if m.typ == ContentType::ChangeCipherSpec && self.joining_hs.is_none() {
                // This is unencrypted. We check the contents later.
                self.discard(end);
                return Ok(Some(Deframed {
                    want_close_before_decrypt: false,
                    aligned: true,
                    trial_decryption_finished: false,
                    message: m.into_plain_message(),
                }));
            }

            // Decrypt the encrypted message (if necessary).
            let msg = match record_layer.decrypt_incoming(m) {
                Ok(Some(decrypted)) => {
                    let Decrypted {
                        want_close_before_decrypt,
                        plaintext,
                    } = decrypted;
                    debug_assert!(!want_close_before_decrypt);
                    plaintext
                }
                // This was rejected early data, discard it. If we currently have a handshake
                // payload in progress, this counts as interleaved, so we error out.
                Ok(None) if self.joining_hs.is_some() => {
                    return Err(self.set_err(
                        PeerMisbehaved::RejectedEarlyDataInterleavedWithHandshakeMessage,
                    ));
                }
                Ok(None) => {
                    self.discard(end);
                    continue;
                }
                Err(e) => return Err(e),
            };

            if self.joining_hs.is_some() && msg.typ != ContentType::Handshake {
                // "Handshake messages MUST NOT be interleaved with other record
                // types.  That is, if a handshake message is split over two or more
                // records, there MUST NOT be any other records between them."
                // https://www.rfc-editor.org/rfc/rfc8446#section-5.1
                return Err(self.set_err(PeerMisbehaved::MessageInterleavedWithHandshakeMessage));
            }

            // If it's not a handshake message, just return it -- no joining necessary.
            if msg.typ != ContentType::Handshake {
                let end = start + rd.used();
                self.discard(end);
                return Ok(Some(Deframed {
                    want_close_before_decrypt: false,
                    aligned: true,
                    trial_decryption_finished: false,
                    message: msg,
                }));
            }

            // If we don't know the payload size yet or if the payload size is larger
            // than the currently buffered payload, we need to wait for more data.
            match self.append_hs(msg.version, &msg.payload.0, end, false)? {
                HandshakePayloadState::Blocked => return Ok(None),
                HandshakePayloadState::Complete(len) => break len,
                HandshakePayloadState::Continue => continue,
            }
        };

        let meta = self.joining_hs.as_mut().unwrap(); // safe after calling `append_hs()`

        // We can now wrap the complete handshake payload in a `PlainMessage`, to be returned.
        let message = PlainMessage {
            typ: ContentType::Handshake,
            version: meta.version,
            payload: Payload::new(&self.buf[meta.payload.start..meta.payload.start + expected_len]),
        };

        // But before we return, update the `joining_hs` state to skip past this payload.
        if meta.payload.len() > expected_len {
            // If we have another (beginning of) a handshake payload left in the buffer, update
            // the payload start to point past the payload we're about to yield, and update the
            // `expected_len` to match the state of that remaining payload.
            meta.payload.start += expected_len;
            meta.expected_len = payload_size(&self.buf[meta.payload.start..meta.payload.end])?;
        } else {
            // Otherwise, we've yielded the last handshake payload in the buffer, so we can
            // discard all of the bytes that we're previously buffered as handshake data.
            let end = meta.message.end;
            self.joining_hs = None;
            self.discard(end);
        }

        Ok(Some(Deframed {
            want_close_before_decrypt: false,
            aligned: self.joining_hs.is_none(),
            trial_decryption_finished: true,
            message,
        }))
    }

    /// Fuses this deframer's error and returns the set value.
    ///
    /// Any future calls to `pop` will return `err` again.
    fn set_err(&mut self, err: impl Into<Error>) -> Error {
        let err = err.into();
        self.last_error = Some(err.clone());
        err
    }

    /// Allow pushing handshake messages directly into the buffer.
    #[cfg(feature = "quic")]
    pub fn push(&mut self, version: ProtocolVersion, payload: &[u8]) -> Result<(), Error> {
        if self.used > 0 && self.joining_hs.is_none() {
            return Err(Error::General(
                "cannot push QUIC messages into unrelated connection".into(),
            ));
        } else if let Err(err) = self.prepare_read() {
            return Err(Error::General(err.into()));
        }

        let end = self.used + payload.len();
        self.append_hs(version, payload, end, true)?;
        self.used = end;
        Ok(())
    }

    /// Write the handshake message contents into the buffer and update the metadata.
    ///
    /// Returns true if a complete message is found.
    fn append_hs(
        &mut self,
        version: ProtocolVersion,
        payload: &[u8],
        end: usize,
        quic: bool,
    ) -> Result<HandshakePayloadState, Error> {
        let meta = match &mut self.joining_hs {
            Some(meta) => {
                debug_assert_eq!(meta.quic, quic);

                // We're joining a handshake message to the previous one here.
                // Write it into the buffer and update the metadata.

                let dst = &mut self.buf[meta.payload.end..meta.payload.end + payload.len()];
                dst.copy_from_slice(payload);
                meta.message.end = end;
                meta.payload.end += payload.len();

                // If we haven't parsed the payload size yet, try to do so now.
                if meta.expected_len.is_none() {
                    meta.expected_len =
                        payload_size(&self.buf[meta.payload.start..meta.payload.end])?;
                }

                meta
            }
            None => {
                // We've found a new handshake message here.
                // Write it into the buffer and create the metadata.

                let expected_len = payload_size(payload)?;
                let dst = &mut self.buf[..payload.len()];
                dst.copy_from_slice(payload);
                self.joining_hs
                    .insert(HandshakePayloadMeta {
                        message: Range { start: 0, end },
                        payload: Range {
                            start: 0,
                            end: payload.len(),
                        },
                        version,
                        expected_len,
                        quic,
                    })
            }
        };

        Ok(match meta.expected_len {
            Some(len) if len <= meta.payload.len() => HandshakePayloadState::Complete(len),
            _ => match self.used > meta.message.end {
                true => HandshakePayloadState::Continue,
                false => HandshakePayloadState::Blocked,
            },
        })
    }

    /// Read some bytes from `rd`, and add them to our internal buffer.
    #[allow(clippy::comparison_chain)]
    pub fn read(&mut self, rd: &mut dyn io::Read) -> io::Result<usize> {
        if let Err(err) = self.prepare_read() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, err));
        }

        // Try to do the largest reads possible. Note that if
        // we get a message with a length field out of range here,
        // we do a zero length read.  That looks like an EOF to
        // the next layer up, which is fine.
        let new_bytes = rd.read(&mut self.buf[self.used..])?;
        self.used += new_bytes;
        Ok(new_bytes)
    }

    /// Resize the internal `buf` if necessary for reading more bytes.
    fn prepare_read(&mut self) -> Result<(), &'static str> {
        // We allow a maximum of 64k of buffered data for handshake messages only. Enforce this
        // by varying the maximum allowed buffer size here based on whether a prefix of a
        // handshake payload is currently being buffered. Given that the first read of such a
        // payload will only ever be 4k bytes, the next time we come around here we allow a
        // larger buffer size. Once the large message and any following handshake messages in
        // the same flight have been consumed, `pop()` will call `discard()` to reset `used`.
        // At this point, the buffer resizing logic below should reduce the buffer size.
        let allow_max = match self.joining_hs {
            Some(_) => MAX_HANDSHAKE_SIZE as usize,
            None => OpaqueMessage::MAX_WIRE_SIZE,
        };

        if self.used >= allow_max {
            return Err("message buffer full");
        }

        // If we can and need to increase the buffer size to allow a 4k read, do so. After
        // dealing with a large handshake message (exceeding `OpaqueMessage::MAX_WIRE_SIZE`),
        // make sure to reduce the buffer size again (large messages should be rare).
        // Also, reduce the buffer size if there are neither full nor partial messages in it,
        // which usually means that the other side suspended sending data.
        let need_capacity = Ord::min(allow_max, self.used + READ_SIZE);
        if need_capacity > self.buf.len() {
            self.buf.resize(need_capacity, 0);
        } else if self.used == 0 || self.buf.len() > allow_max {
            self.buf.resize(need_capacity, 0);
            self.buf.shrink_to(need_capacity);
        }

        Ok(())
    }

    /// Returns true if we have messages for the caller
    /// to process, either whole messages in our output
    /// queue or partial messages in our buffer.
    pub fn has_pending(&self) -> bool {
        self.used > 0
    }

    /// Discard `taken` bytes from the start of our buffer.
    fn discard(&mut self, taken: usize) {
        #[allow(clippy::comparison_chain)]
        if taken < self.used {
            /* Before:
             * +----------+----------+----------+
             * | taken    | pending  |xxxxxxxxxx|
             * +----------+----------+----------+
             * 0          ^ taken    ^ self.used
             *
             * After:
             * +----------+----------+----------+
             * | pending  |xxxxxxxxxxxxxxxxxxxxx|
             * +----------+----------+----------+
             * 0          ^ self.used
             */

            self.buf
                .copy_within(taken..self.used, 0);
            self.used -= taken;
        } else if taken == self.used {
            self.used = 0;
        }
    }
}

enum HandshakePayloadState {
    /// Waiting for more data.
    Blocked,
    /// We have a complete handshake message.
    Complete(usize),
    /// More records available for processing.
    Continue,
}

struct HandshakePayloadMeta {
    /// The range of bytes from the deframer buffer that contains data processed so far.
    ///
    /// This will need to be discarded as the last of the handshake message is `pop()`ped.
    message: Range<usize>,
    /// The range of bytes from the deframer buffer that contains payload.
    payload: Range<usize>,
    /// The protocol version as found in the decrypted handshake message.
    version: ProtocolVersion,
    /// The expected size of the handshake payload, if available.
    ///
    /// If the received payload exceeds 4 bytes (the handshake payload header), we update
    /// `expected_len` to contain the payload length as advertised (at most 16_777_215 bytes).
    expected_len: Option<usize>,
    /// True if this is a QUIC handshake message.
    ///
    /// In the case of QUIC, we get a plaintext handshake data directly from the CRYPTO stream,
    /// so there's no need to unwrap and decrypt the outer TLS record. This is implemented
    /// by directly calling `MessageDeframer::push()` from the connection.
    quic: bool,
}

/// Determine the expected length of the payload as advertised in the header.
///
/// Returns `Err` if the advertised length is larger than what we want to accept
/// (`MAX_HANDSHAKE_SIZE`), `Ok(None)` if the buffer is too small to contain a complete header,
/// and `Ok(Some(len))` otherwise.
fn payload_size(buf: &[u8]) -> Result<Option<usize>, Error> {
    if buf.len() < HEADER_SIZE {
        return Ok(None);
    }

    let (header, _) = buf.split_at(HEADER_SIZE);
    match codec::u24::read_bytes(&header[1..]) {
        Ok(len) if len.0 > MAX_HANDSHAKE_SIZE => Err(Error::InvalidMessage(
            InvalidMessage::HandshakePayloadTooLarge,
        )),
        Ok(len) => Ok(Some(HEADER_SIZE + usize::from(len))),
        _ => Ok(None),
    }
}

#[derive(Debug)]
pub struct Deframed {
    pub want_close_before_decrypt: bool,
    pub aligned: bool,
    pub trial_decryption_finished: bool,
    pub message: PlainMessage,
}

#[derive(Debug)]
pub enum DeframerError {
    HandshakePayloadSizeTooLarge,
}

const HEADER_SIZE: usize = 1 + 3;

/// TLS allows for handshake messages of up to 16MB.  We
/// restrict that to 64KB to limit potential for denial-of-
/// service.
const MAX_HANDSHAKE_SIZE: u32 = 0xffff;

const READ_SIZE: usize = 4096;

#[cfg(test)]
mod tests {
    use super::MessageDeframer;
    use crate::msgs::message::{Message, OpaqueMessage};
    use crate::record_layer::RecordLayer;
    use crate::{ContentType, Error, InvalidMessage};

    use std::io;

    const FIRST_MESSAGE: &[u8] = include_bytes!("../testdata/deframer-test.1.bin");
    const SECOND_MESSAGE: &[u8] = include_bytes!("../testdata/deframer-test.2.bin");

    const EMPTY_APPLICATIONDATA_MESSAGE: &[u8] =
        include_bytes!("../testdata/deframer-empty-applicationdata.bin");

    const INVALID_EMPTY_MESSAGE: &[u8] = include_bytes!("../testdata/deframer-invalid-empty.bin");
    const INVALID_CONTENTTYPE_MESSAGE: &[u8] =
        include_bytes!("../testdata/deframer-invalid-contenttype.bin");
    const INVALID_VERSION_MESSAGE: &[u8] =
        include_bytes!("../testdata/deframer-invalid-version.bin");
    const INVALID_LENGTH_MESSAGE: &[u8] = include_bytes!("../testdata/deframer-invalid-length.bin");

    fn input_bytes(d: &mut MessageDeframer, bytes: &[u8]) -> io::Result<usize> {
        let mut rd = io::Cursor::new(bytes);
        d.read(&mut rd)
    }

    fn input_bytes_concat(
        d: &mut MessageDeframer,
        bytes1: &[u8],
        bytes2: &[u8],
    ) -> io::Result<usize> {
        let mut bytes = vec![0u8; bytes1.len() + bytes2.len()];
        bytes[..bytes1.len()].clone_from_slice(bytes1);
        bytes[bytes1.len()..].clone_from_slice(bytes2);
        let mut rd = io::Cursor::new(&bytes);
        d.read(&mut rd)
    }

    struct ErrorRead {
        error: Option<io::Error>,
    }

    impl ErrorRead {
        fn new(error: io::Error) -> Self {
            Self { error: Some(error) }
        }
    }

    impl io::Read for ErrorRead {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for (i, b) in buf.iter_mut().enumerate() {
                *b = i as u8;
            }

            let error = self.error.take().unwrap();
            Err(error)
        }
    }

    fn input_error(d: &mut MessageDeframer) {
        let error = io::Error::from(io::ErrorKind::TimedOut);
        let mut rd = ErrorRead::new(error);
        d.read(&mut rd)
            .expect_err("error not propagated");
    }

    fn input_whole_incremental(d: &mut MessageDeframer, bytes: &[u8]) {
        let before = d.used;

        for i in 0..bytes.len() {
            assert_len(1, input_bytes(d, &bytes[i..i + 1]));
            assert!(d.has_pending());
        }

        assert_eq!(before + bytes.len(), d.used);
    }

    fn assert_len(want: usize, got: io::Result<usize>) {
        if let Ok(gotval) = got {
            assert_eq!(gotval, want);
        } else {
            panic!("read failed, expected {:?} bytes", want);
        }
    }

    fn pop_first(d: &mut MessageDeframer, rl: &mut RecordLayer) {
        let m = d.pop(rl).unwrap().unwrap().message;
        assert_eq!(m.typ, ContentType::Handshake);
        Message::try_from(m).unwrap();
    }

    fn pop_second(d: &mut MessageDeframer, rl: &mut RecordLayer) {
        let m = d.pop(rl).unwrap().unwrap().message;
        assert_eq!(m.typ, ContentType::Alert);
        Message::try_from(m).unwrap();
    }

    #[test]
    fn check_incremental() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        input_whole_incremental(&mut d, FIRST_MESSAGE);
        assert!(d.has_pending());

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn check_incremental_2() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        input_whole_incremental(&mut d, FIRST_MESSAGE);
        assert!(d.has_pending());
        input_whole_incremental(&mut d, SECOND_MESSAGE);
        assert!(d.has_pending());

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        assert!(d.has_pending());
        pop_second(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn check_whole() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        assert_len(FIRST_MESSAGE.len(), input_bytes(&mut d, FIRST_MESSAGE));
        assert!(d.has_pending());

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn check_whole_2() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        assert_len(FIRST_MESSAGE.len(), input_bytes(&mut d, FIRST_MESSAGE));
        assert_len(SECOND_MESSAGE.len(), input_bytes(&mut d, SECOND_MESSAGE));

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        pop_second(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn test_two_in_one_read() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        assert_len(
            FIRST_MESSAGE.len() + SECOND_MESSAGE.len(),
            input_bytes_concat(&mut d, FIRST_MESSAGE, SECOND_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        pop_second(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn test_two_in_one_read_shortest_first() {
        let mut d = MessageDeframer::default();
        assert!(!d.has_pending());
        assert_len(
            FIRST_MESSAGE.len() + SECOND_MESSAGE.len(),
            input_bytes_concat(&mut d, SECOND_MESSAGE, FIRST_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        pop_second(&mut d, &mut rl);
        pop_first(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn test_incremental_with_nonfatal_read_error() {
        let mut d = MessageDeframer::default();
        assert_len(3, input_bytes(&mut d, &FIRST_MESSAGE[..3]));
        input_error(&mut d);
        assert_len(
            FIRST_MESSAGE.len() - 3,
            input_bytes(&mut d, &FIRST_MESSAGE[3..]),
        );

        let mut rl = RecordLayer::new();
        pop_first(&mut d, &mut rl);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn test_invalid_contenttype_errors() {
        let mut d = MessageDeframer::default();
        assert_len(
            INVALID_CONTENTTYPE_MESSAGE.len(),
            input_bytes(&mut d, INVALID_CONTENTTYPE_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        assert_eq!(
            d.pop(&mut rl).unwrap_err(),
            Error::InvalidMessage(InvalidMessage::InvalidContentType)
        );
    }

    #[test]
    fn test_invalid_version_errors() {
        let mut d = MessageDeframer::default();
        assert_len(
            INVALID_VERSION_MESSAGE.len(),
            input_bytes(&mut d, INVALID_VERSION_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        assert_eq!(
            d.pop(&mut rl).unwrap_err(),
            Error::InvalidMessage(InvalidMessage::UnknownProtocolVersion)
        );
    }

    #[test]
    fn test_invalid_length_errors() {
        let mut d = MessageDeframer::default();
        assert_len(
            INVALID_LENGTH_MESSAGE.len(),
            input_bytes(&mut d, INVALID_LENGTH_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        assert_eq!(
            d.pop(&mut rl).unwrap_err(),
            Error::InvalidMessage(InvalidMessage::MessageTooLarge)
        );
    }

    #[test]
    fn test_empty_applicationdata() {
        let mut d = MessageDeframer::default();
        assert_len(
            EMPTY_APPLICATIONDATA_MESSAGE.len(),
            input_bytes(&mut d, EMPTY_APPLICATIONDATA_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        let m = d.pop(&mut rl).unwrap().unwrap().message;
        assert_eq!(m.typ, ContentType::ApplicationData);
        assert_eq!(m.payload.0.len(), 0);
        assert!(!d.has_pending());
        assert!(d.last_error.is_none());
    }

    #[test]
    fn test_invalid_empty_errors() {
        let mut d = MessageDeframer::default();
        assert_len(
            INVALID_EMPTY_MESSAGE.len(),
            input_bytes(&mut d, INVALID_EMPTY_MESSAGE),
        );

        let mut rl = RecordLayer::new();
        assert_eq!(
            d.pop(&mut rl).unwrap_err(),
            Error::InvalidMessage(InvalidMessage::InvalidEmptyPayload)
        );
        // CorruptMessage has been fused
        assert_eq!(
            d.pop(&mut rl).unwrap_err(),
            Error::InvalidMessage(InvalidMessage::InvalidEmptyPayload)
        );
    }

    #[test]
    fn test_limited_buffer() {
        const PAYLOAD_LEN: usize = 16_384;
        let mut message = Vec::with_capacity(16_389);
        message.push(0x17); // ApplicationData
        message.extend(&[0x03, 0x04]); // ProtocolVersion
        message.extend((PAYLOAD_LEN as u16).to_be_bytes()); // payload length
        message.extend(&[0; PAYLOAD_LEN]);

        let mut d = MessageDeframer::default();
        assert_len(4096, input_bytes(&mut d, &message));
        assert_len(4096, input_bytes(&mut d, &message));
        assert_len(4096, input_bytes(&mut d, &message));
        assert_len(4096, input_bytes(&mut d, &message));
        assert_len(
            OpaqueMessage::MAX_WIRE_SIZE - 16_384,
            input_bytes(&mut d, &message),
        );
        assert!(input_bytes(&mut d, &message).is_err());
    }
}
