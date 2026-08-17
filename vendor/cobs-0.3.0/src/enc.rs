/// The [`CobsEncoder`] type is used to encode a stream of bytes to a
/// given mutable output slice. This is often useful when heap data
/// structures are not available, or when not all message bytes are
/// received at a single point in time.
#[derive(Debug)]
pub struct CobsEncoder<'a> {
    dest: &'a mut [u8],
    dest_idx: usize,
    state: EncoderState,
    might_be_done: bool,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("out of bounds error during encoding")]
pub struct DestBufTooSmallError;

/// The [`EncoderState`] is used to track the current state of a
/// streaming encoder. This struct does not contain the output buffer
/// (or a reference to one), and can be used when streaming the encoded
/// output to a custom data type
///
/// **IMPORTANT NOTE**: When implementing a custom streaming encoder,
/// the [`EncoderState`] state machine assumes that the output buffer
/// **ALREADY** contains a single placeholder byte, and no other bytes.
/// This placeholder byte will be later modified with the first distance
/// to the next header/zero byte.
#[derive(Clone, Debug)]
pub struct EncoderState {
    code_idx: usize,
    num_bt_sent: u8,
    offset_idx: u8,
}

/// [`PushResult`] is used to represent the changes to an (encoded)
/// output data buffer when an unencoded byte is pushed into [`EncoderState`].
pub enum PushResult {
    /// The returned byte should be placed at the current end of the data buffer
    AddSingle(u8),

    /// The byte at the given index should be replaced with the given byte.
    /// Additionally, a placeholder byte should be inserted at the current
    /// end of the output buffer to be later modified
    ModifyFromStartAndSkip((usize, u8)),

    /// The byte at the given index should be replaced with the given byte.
    /// Then, the last u8 in this tuple should be inserted at the end of the
    /// current output buffer. Finally, a placeholder byte should be inserted at
    /// the current end of the output buffer to be later modified if the encoding process is
    /// not done yet.
    ModifyFromStartAndPushAndSkip((usize, u8, u8)),
}

impl Default for EncoderState {
    /// Create a default initial state representation for a COBS encoder
    fn default() -> Self {
        Self {
            code_idx: 0,
            num_bt_sent: 1,
            offset_idx: 1,
        }
    }
}

impl EncoderState {
    /// Push a single unencoded byte into the encoder state machine
    pub fn push(&mut self, data: u8) -> PushResult {
        if data == 0 {
            let ret = PushResult::ModifyFromStartAndSkip((self.code_idx, self.num_bt_sent));
            self.code_idx += usize::from(self.offset_idx);
            self.num_bt_sent = 1;
            self.offset_idx = 1;
            ret
        } else {
            self.num_bt_sent += 1;
            self.offset_idx += 1;

            if 0xFF == self.num_bt_sent {
                let ret = PushResult::ModifyFromStartAndPushAndSkip((
                    self.code_idx,
                    self.num_bt_sent,
                    data,
                ));
                self.num_bt_sent = 1;
                self.code_idx += usize::from(self.offset_idx);
                self.offset_idx = 1;
                ret
            } else {
                PushResult::AddSingle(data)
            }
        }
    }

    /// Finalize the encoding process for a single message.
    /// The byte at the given index should be replaced with the given value,
    /// and the sentinel value (typically 0u8) must be inserted at the current
    /// end of the output buffer, serving as a framing byte.
    pub fn finalize(self) -> (usize, u8) {
        (self.code_idx, self.num_bt_sent)
    }
}

impl<'a> CobsEncoder<'a> {
    /// Create a new streaming Cobs Encoder
    pub fn new(out_buf: &'a mut [u8]) -> CobsEncoder<'a> {
        CobsEncoder {
            dest: out_buf,
            dest_idx: 1,
            state: EncoderState::default(),
            might_be_done: false,
        }
    }

    /// Push a slice of data to be encoded
    pub fn push(&mut self, data: &[u8]) -> Result<(), DestBufTooSmallError> {
        // TODO: could probably check if this would fit without
        // iterating through all data

        // There was the possibility that the encoding process is done, but more data is pushed
        // instead of a `finalize` call, so the destination index needs to be incremented.
        if self.might_be_done {
            self.dest_idx += 1;
            self.might_be_done = false;
        }
        for (slice_idx, val) in data.iter().enumerate() {
            use PushResult::*;
            match self.state.push(*val) {
                AddSingle(y) => {
                    *self
                        .dest
                        .get_mut(self.dest_idx)
                        .ok_or(DestBufTooSmallError)? = y;
                }
                ModifyFromStartAndSkip((idx, mval)) => {
                    *self.dest.get_mut(idx).ok_or(DestBufTooSmallError)? = mval;
                }
                ModifyFromStartAndPushAndSkip((idx, mval, nval1)) => {
                    *self.dest.get_mut(idx).ok_or(DestBufTooSmallError)? = mval;
                    *self
                        .dest
                        .get_mut(self.dest_idx)
                        .ok_or(DestBufTooSmallError)? = nval1;
                    // Do not increase index if these is the possibility that we are finished.
                    if slice_idx == data.len() - 1 {
                        // If push is called again, the index will be incremented. If finalize
                        // is called, there is no need to increment the index.
                        self.might_be_done = true;
                    } else {
                        self.dest_idx += 1;
                    }
                }
            }

            // All branches above require advancing the pointer at least once
            self.dest_idx += 1;
        }

        Ok(())
    }

    /// Complete encoding of the output message. Does NOT terminate
    /// the message with the sentinel value
    pub fn finalize(self) -> usize {
        if self.dest_idx == 1 {
            return 0;
        }

        // Get the last index that needs to be fixed
        let (idx, mval) = self.state.finalize();

        // If the current code index is outside of the destination slice,
        // we do not need to write it out
        if let Some(i) = self.dest.get_mut(idx) {
            *i = mval;
        }

        self.dest_idx
    }
}

/// Encodes the `source` buffer into the `dest` buffer.
///
/// This function assumes the typical sentinel value of 0, but does not terminate the encoded
/// message with the sentinel value. This should be done by the caller to ensure proper framing.
///
/// # Returns
///
/// The number of bytes written to in the `dest` buffer.
///
/// # Panics
///
/// This function will panic if the `dest` buffer is not large enough for the
/// encoded message. You can calculate the size the `dest` buffer needs to be with
/// the [crate::max_encoding_length] function.
pub fn encode(source: &[u8], dest: &mut [u8]) -> usize {
    let mut enc = CobsEncoder::new(dest);
    enc.push(source).unwrap();
    enc.finalize()
}

/// Attempts to encode the `source` buffer into the `dest` buffer.
///
/// This function assumes the typical sentinel value of 0, but does not terminate the encoded
/// message with the sentinel value. This should be done by the caller to ensure proper framing.
///
/// # Returns
///
/// The number of bytes written to in the `dest` buffer.
///
/// If the destination buffer does not have enough room, an error will be returned.
pub fn try_encode(source: &[u8], dest: &mut [u8]) -> Result<usize, DestBufTooSmallError> {
    let mut enc = CobsEncoder::new(dest);
    enc.push(source)?;
    Ok(enc.finalize())
}

/// Encodes the `source` buffer into the `dest` buffer using an
/// arbitrary sentinel value.
///
/// This is done by first encoding the message with the typical sentinel value
/// of 0, then XOR-ing each byte of the encoded message with the chosen sentinel
/// value. This will ensure that the sentinel value doesn't show up in the encoded
/// message. See the paper "Consistent Overhead Byte Stuffing" for details.
///
/// This function does not terminate the encoded message with the sentinel value. This should be
/// done by the caller to ensure proper framing.
///
/// # Returns
///
/// The number of bytes written to in the `dest` buffer.
pub fn encode_with_sentinel(source: &[u8], dest: &mut [u8], sentinel: u8) -> usize {
    let encoded_size = encode(source, dest);
    for x in &mut dest[..encoded_size] {
        *x ^= sentinel;
    }
    encoded_size
}

#[cfg(feature = "alloc")]
/// Encodes the `source` buffer into a vector, using the [encode] function.
pub fn encode_vec(source: &[u8]) -> alloc::vec::Vec<u8> {
    let mut encoded = alloc::vec![0; crate::max_encoding_length(source.len())];
    let encoded_len = encode(source, &mut encoded[..]);
    encoded.truncate(encoded_len);
    encoded
}

#[cfg(feature = "alloc")]
/// Encodes the `source` buffer into a vector with an arbitrary sentinel value, using the
/// [encode_with_sentinel] function.
pub fn encode_vec_with_sentinel(source: &[u8], sentinel: u8) -> alloc::vec::Vec<u8> {
    let mut encoded = alloc::vec![0; crate::max_encoding_length(source.len())];
    let encoded_len = encode_with_sentinel(source, &mut encoded[..], sentinel);
    encoded.truncate(encoded_len);
    encoded
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn encode_target_buf_too_small() {
        let source = &[10, 11, 0, 12];
        let expected = &[3, 10, 11, 2, 12];
        for len in 0..expected.len() {
            let mut dest = alloc::vec![0; len];
            matches!(
                try_encode(source, &mut dest).unwrap_err(),
                DestBufTooSmallError
            );
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    #[should_panic]
    fn encode_target_buf_too_small_panicking() {
        let source = &[10, 11, 0, 12];
        let expected = &[3, 10, 11, 2, 12];
        encode(source, &mut alloc::vec![0; expected.len() - 1]);
    }
}
