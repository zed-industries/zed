/// The [`CobsDecoder`] type is used to decode a stream of bytes to a
/// given mutable output slice. This is often useful when heap data
/// structures are not available, or when not all message bytes are
/// received at a single point in time.
#[derive(Debug)]
pub struct CobsDecoder<'a> {
    /// Destination slice for decoded message
    dest: &'a mut [u8],

    /// Index of next byte to write in `dest`
    dest_idx: usize,

    /// Decoder state as an enum
    state: DecoderState,
}

/// The [`DecoderState`] is used to track the current state of a
/// streaming decoder. This struct does not contain the output buffer
/// (or a reference to one), and can be used when streaming the decoded
/// output to a custom data type.
#[derive(Debug)]
pub enum DecoderState {
    /// State machine has not received any non-zero bytes
    Idle,

    /// 1-254 bytes, can be header or 00
    Grab(u8),

    /// 255 bytes, will be a header next
    GrabChain(u8),
}

fn add(to: &mut [u8], idx: usize, data: u8) -> Result<(), DecodeError> {
    *to.get_mut(idx).ok_or(DecodeError::TargetBufTooSmall)? = data;
    Ok(())
}

/// [`DecodeResult`] represents the possible non-error outcomes of
/// pushing an encoded data byte into the [`DecoderState`] state machine
pub enum DecodeResult {
    /// The given input byte did not prompt an output byte, either because the
    /// state machine is still idle, or we have just processed a header byte.
    /// More data is needed to complete the message.
    NoData,

    /// We have received a complete and well-encoded COBS message. The
    /// contents of the associated output buffer may now be used
    DataComplete,

    /// The following byte should be appended to the current end of the decoded
    /// output buffer.
    /// More data is needed to complete the message.
    DataContinue(u8),
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodeError {
    #[error("empty input frame")]
    EmptyFrame,
    #[error("frame with invalid format, written {decoded_bytes:?} to decoded buffer")]
    InvalidFrame {
        /// Number of bytes written to the decoded buffer.
        decoded_bytes: usize,
    },
    #[error("target buffer too small")]
    TargetBufTooSmall,
}

impl DecoderState {
    /// Push a single encoded byte into the state machine. If the input was
    /// unexpected, such as an early end of a framed message segment, an Error will
    /// be returned, and the current associated output buffer contents should be discarded.
    ///
    /// If a complete message is indicated, the decoding state machine will automatically
    /// reset itself to the Idle state, and may be used to begin decoding another message.
    ///
    /// NOTE: Sentinel value must be included in the input to this function for the
    /// decoding to complete
    pub fn feed(&mut self, data: u8) -> Result<DecodeResult, DecodeError> {
        use DecodeResult::*;
        use DecoderState::*;
        let (ret, state) = match (&self, data) {
            // Currently Idle, received a terminator, ignore, stay idle
            (Idle, 0x00) => (Ok(NoData), Idle),

            // Currently Idle, received a byte indicating the
            // next 255 bytes have no zeroes, so we will have 254 unmodified
            // data bytes, then an overhead byte
            (Idle, 0xFF) => (Ok(NoData), GrabChain(0xFE)),

            // Currently Idle, received a byte indicating there will be a
            // zero that must be modified in the next 1..=254 bytes
            (Idle, n) => (Ok(NoData), Grab(n - 1)),

            // We have reached the end of a data run indicated by an overhead
            // byte, AND we have recieved the message terminator. This was a
            // well framed message!
            (Grab(0), 0x00) => (Ok(DataComplete), Idle),

            // We have reached the end of a data run indicated by an overhead
            // byte, and the next segment of 254 bytes will have no modified
            // sentinel bytes
            (Grab(0), 0xFF) => (Ok(DataContinue(0)), GrabChain(0xFE)),

            // We have reached the end of a data run indicated by an overhead
            // byte, and we will treat this byte as a modified sentinel byte.
            // place the sentinel byte in the output, and begin processing the
            // next non-sentinel sequence
            (Grab(0), n) => (Ok(DataContinue(0)), Grab(n - 1)),

            // We were not expecting the sequence to terminate, but here we are.
            // Report an error due to early terminated message
            (Grab(_), 0) => (Err(DecodeError::InvalidFrame { decoded_bytes: 0 }), Idle),

            // We have not yet reached the end of a data run, decrement the run
            // counter, and place the byte into the decoded output
            (Grab(i), n) => (Ok(DataContinue(n)), Grab(*i - 1)),

            // We have reached the end of a data run indicated by an overhead
            // byte, AND we have recieved the message terminator. This was a
            // well framed message!
            (GrabChain(0), 0x00) => (Ok(DataComplete), Idle),

            // We have reached the end of a data run, and we will begin another
            // data run with an overhead byte expected at the end
            (GrabChain(0), 0xFF) => (Ok(NoData), GrabChain(0xFE)),

            // We have reached the end of a data run, and we will expect `n` data
            // bytes unmodified, followed by a sentinel byte that must be modified
            (GrabChain(0), n) => (Ok(NoData), Grab(n - 1)),

            // We were not expecting the sequence to terminate, but here we are.
            // Report an error due to early terminated message
            (GrabChain(_), 0) => (Err(DecodeError::InvalidFrame { decoded_bytes: 0 }), Idle),

            // We have not yet reached the end of a data run, decrement the run
            // counter, and place the byte into the decoded output
            (GrabChain(i), n) => (Ok(DataContinue(n)), GrabChain(*i - 1)),
        };

        *self = state;
        ret
    }
}

impl<'a> CobsDecoder<'a> {
    /// Create a new streaming Cobs Decoder. Provide the output buffer
    /// for the decoded message to be placed in
    pub fn new(dest: &'a mut [u8]) -> CobsDecoder<'a> {
        CobsDecoder {
            dest,
            dest_idx: 0,
            state: DecoderState::Idle,
        }
    }

    /// Push a single byte into the streaming CobsDecoder. Return values mean:
    ///
    /// * Ok(None) - State machine okay, more data needed
    /// * Ok(Some(N)) - A message of N bytes was successfully decoded
    /// * Err([DecodeError]) - Message decoding failed
    ///
    /// NOTE: Sentinel value must be included in the input to this function for the
    /// decoding to complete
    pub fn feed(&mut self, data: u8) -> Result<Option<usize>, DecodeError> {
        match self.state.feed(data) {
            Err(_) => Err(DecodeError::InvalidFrame {
                decoded_bytes: self.dest_idx,
            }),
            Ok(DecodeResult::NoData) => Ok(None),
            Ok(DecodeResult::DataContinue(n)) => {
                add(self.dest, self.dest_idx, n)?;
                self.dest_idx += 1;
                Ok(None)
            }
            Ok(DecodeResult::DataComplete) => Ok(Some(self.dest_idx)),
        }
    }

    /// Push a slice of bytes into the streaming CobsDecoder. Return values mean:
    ///
    /// * Ok(None) - State machine okay, more data needed
    /// * Ok(Some((N, M))) - A message of N bytes was successfully decoded,
    ///     using M bytes from `data` (and earlier data)
    /// * Err([DecodeError]) - Message decoding failed
    ///
    /// NOTE: Sentinel value must be included in the input to this function for the
    /// decoding to complete
    pub fn push(&mut self, data: &[u8]) -> Result<Option<(usize, usize)>, DecodeError> {
        for (consumed_idx, d) in data.iter().enumerate() {
            let opt_decoded_bytes = self.feed(*d)?;
            if let Some(decoded_bytes_ct) = opt_decoded_bytes {
                // convert from index to number of bytes consumed
                return Ok(Some((decoded_bytes_ct, consumed_idx + 1)));
            }
        }

        Ok(None)
    }
}

/// Decodes the `source` buffer into the `dest` buffer.
///
/// This function uses the typical sentinel value of 0.
///
/// # Failures
///
/// This will return `Err(())` if there was a decoding error. Otherwise,
/// it will return `Ok(n)` where `n` is the length of the decoded message.
pub fn decode(source: &[u8], dest: &mut [u8]) -> Result<usize, DecodeError> {
    if source.is_empty() {
        return Err(DecodeError::EmptyFrame);
    }

    let mut dec = CobsDecoder::new(dest);

    // Did we decode a message, using some or all of the buffer?
    if let Some((d_used, _s_used)) = dec.push(source)? {
        return Ok(d_used);
    }

    // If we consumed the entire buffer, but did NOT get a message,
    // AND the message did not end with a zero, try providing one to
    // complete the decoding.
    if source.last() != Some(&0) {
        // Explicitly push sentinel of zero
        if let Some((d_used, _s_used)) = dec.push(&[0])? {
            return Ok(d_used);
        }
    }

    // Nope, no early message, no missing terminator, just failed to decode
    Err(DecodeError::InvalidFrame {
        decoded_bytes: dec.dest_idx,
    })
}

/// A report of the source and destination bytes used during in-place decoding
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DecodeReport {
    // The number of source bytes used, NOT INCLUDING the sentinel byte,
    // if there was one.
    pub src_used: usize,

    // The number of bytes of the source buffer that now include the
    // decoded result
    pub dst_used: usize,
}

/// Decodes a message in-place.
///
/// This is the same function as [decode_in_place], but provides a report
/// of both the number of source bytes consumed as well as the size of the
/// destination used.
pub fn decode_in_place_report(buf: &mut [u8]) -> Result<DecodeReport, DecodeError> {
    let mut source_index = 0;
    let mut dest_index = 0;

    if buf.is_empty() {
        return Err(DecodeError::EmptyFrame);
    }

    // Stop at the first terminator, if any
    let src_end = if let Some(end) = buf.iter().position(|b| *b == 0) {
        end
    } else {
        buf.len()
    };

    while source_index < src_end {
        let code = buf[source_index];

        if source_index + code as usize > src_end && code != 1 {
            return Err(DecodeError::InvalidFrame {
                decoded_bytes: dest_index,
            });
        }

        source_index += 1;

        for _ in 1..code {
            *buf.get_mut(dest_index)
                .ok_or(DecodeError::TargetBufTooSmall)? = buf[source_index];
            source_index += 1;
            dest_index += 1;
        }

        if 0xFF != code && source_index < src_end {
            *buf.get_mut(dest_index)
                .ok_or(DecodeError::TargetBufTooSmall)? = 0;
            dest_index += 1;
        }
    }

    Ok(DecodeReport {
        dst_used: dest_index,
        src_used: source_index,
    })
}

/// Decodes a message in-place.
///
/// This is the same function as [decode], but replaces the encoded message
/// with the decoded message instead of writing to another buffer.
///
/// The returned `usize` is the number of bytes used for the DECODED value,
/// NOT the number of source bytes consumed during decoding.
pub fn decode_in_place(buff: &mut [u8]) -> Result<usize, DecodeError> {
    decode_in_place_report(buff).map(|r| r.dst_used)
}

/// Decodes the `source` buffer into the `dest` buffer using an arbitrary sentinel value.
///
/// This is done by XOR-ing each byte of the source message with the chosen sentinel value,
/// which transforms the message into the same message encoded with a sentinel value of 0.
/// Then the regular decoding transformation is performed.
///
/// The returned `usize` is the number of bytes used for the DECODED value,
/// NOT the number of source bytes consumed during decoding.
pub fn decode_with_sentinel(
    source: &[u8],
    dest: &mut [u8],
    sentinel: u8,
) -> Result<usize, DecodeError> {
    for (x, y) in source.iter().zip(dest.iter_mut()) {
        *y = *x ^ sentinel;
    }
    decode_in_place(dest)
}

/// Decodes a message in-place using an arbitrary sentinel value.
///
/// The returned `usize` is the number of bytes used for the DECODED value,
/// NOT the number of source bytes consumed during decoding.
pub fn decode_in_place_with_sentinel(buff: &mut [u8], sentinel: u8) -> Result<usize, DecodeError> {
    for x in buff.iter_mut() {
        *x ^= sentinel;
    }
    decode_in_place(buff)
}

#[cfg(feature = "alloc")]
/// Decodes the `source` buffer into a vector.
pub fn decode_vec(source: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    let mut decoded = alloc::vec![0; source.len()];
    let n = decode(source, &mut decoded[..])?;
    decoded.truncate(n);
    Ok(decoded)
}

#[cfg(feature = "alloc")]
/// Decodes the `source` buffer into a vector with an arbitrary sentinel value.
pub fn decode_vec_with_sentinel(
    source: &[u8],
    sentinel: u8,
) -> Result<alloc::vec::Vec<u8>, DecodeError> {
    let mut decoded = alloc::vec![0; source.len()];
    let n = decode_with_sentinel(source, &mut decoded[..], sentinel)?;
    decoded.truncate(n);
    Ok(decoded)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decode_malforemd() {
        let malformed_buf: [u8; 32] = [
            68, 69, 65, 68, 66, 69, 69, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let mut dest_buf: [u8; 32] = [0; 32];
        if let Err(DecodeError::InvalidFrame { decoded_bytes }) =
            decode(&malformed_buf, &mut dest_buf)
        {
            assert_eq!(decoded_bytes, 7);
        } else {
            panic!("decoding worked when it should not have");
        }
    }

    #[test]
    fn decode_empty() {
        #[cfg(feature = "alloc")]
        matches!(decode_vec(&[]).unwrap_err(), DecodeError::EmptyFrame);
        matches!(
            decode_in_place(&mut []).unwrap_err(),
            DecodeError::EmptyFrame
        );
        matches!(
            decode(&[], &mut [0; 256]).unwrap_err(),
            DecodeError::EmptyFrame
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn decode_target_buf_too_small() {
        let encoded = &[3, 10, 11, 2, 12];
        let expected_decoded_len = 4;
        for i in 0..expected_decoded_len - 1 {
            let mut dest = alloc::vec![0; i];
            let result = decode(encoded, &mut dest);
            assert_eq!(result, Err(DecodeError::TargetBufTooSmall));
        }
    }
}
