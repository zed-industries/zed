//! OID encoder with `const` support.

use crate::{
    arcs::{ARC_MAX_FIRST, ARC_MAX_SECOND},
    Arc, Error, ObjectIdentifier, Result,
};

/// BER/DER encoder
#[derive(Debug)]
pub(crate) struct Encoder {
    /// Current state
    state: State,

    /// Bytes of the OID being encoded in-progress
    bytes: [u8; ObjectIdentifier::MAX_SIZE],

    /// Current position within the byte buffer
    cursor: usize,
}

/// Current state of the encoder
#[derive(Debug)]
enum State {
    /// Initial state - no arcs yet encoded
    Initial,

    /// First arc parsed
    FirstArc(Arc),

    /// Encoding base 128 body of the OID
    Body,
}

impl Encoder {
    /// Create a new encoder initialized to an empty default state.
    pub(crate) const fn new() -> Self {
        Self {
            state: State::Initial,
            bytes: [0u8; ObjectIdentifier::MAX_SIZE],
            cursor: 0,
        }
    }

    /// Extend an existing OID.
    pub(crate) const fn extend(oid: ObjectIdentifier) -> Self {
        Self {
            state: State::Body,
            bytes: oid.bytes,
            cursor: oid.length as usize,
        }
    }

    /// Encode an [`Arc`] as base 128 into the internal buffer.
    pub(crate) const fn arc(mut self, arc: Arc) -> Result<Self> {
        match self.state {
            State::Initial => {
                if arc > ARC_MAX_FIRST {
                    return Err(Error::ArcInvalid { arc });
                }

                self.state = State::FirstArc(arc);
                Ok(self)
            }
            // Ensured not to overflow by `ARC_MAX_SECOND` check
            #[allow(clippy::integer_arithmetic)]
            State::FirstArc(first_arc) => {
                if arc > ARC_MAX_SECOND {
                    return Err(Error::ArcInvalid { arc });
                }

                self.state = State::Body;
                self.bytes[0] = (first_arc * (ARC_MAX_SECOND + 1)) as u8 + arc as u8;
                self.cursor = 1;
                Ok(self)
            }
            // TODO(tarcieri): finer-grained overflow safety / checked arithmetic
            #[allow(clippy::integer_arithmetic)]
            State::Body => {
                // Total number of bytes in encoded arc - 1
                let nbytes = base128_len(arc);

                // Shouldn't overflow on any 16-bit+ architectures
                if self.cursor + nbytes + 1 >= ObjectIdentifier::MAX_SIZE {
                    return Err(Error::Length);
                }

                let new_cursor = self.cursor + nbytes + 1;

                // TODO(tarcieri): use `?` when stable in `const fn`
                match self.encode_base128_byte(arc, nbytes, false) {
                    Ok(mut encoder) => {
                        encoder.cursor = new_cursor;
                        Ok(encoder)
                    }
                    Err(err) => Err(err),
                }
            }
        }
    }

    /// Finish encoding an OID.
    pub(crate) const fn finish(self) -> Result<ObjectIdentifier> {
        if self.cursor >= 2 {
            Ok(ObjectIdentifier {
                bytes: self.bytes,
                length: self.cursor as u8,
            })
        } else {
            Err(Error::NotEnoughArcs)
        }
    }

    /// Encode a single byte of a Base 128 value.
    const fn encode_base128_byte(mut self, mut n: u32, i: usize, continued: bool) -> Result<Self> {
        let mask = if continued { 0b10000000 } else { 0 };

        // Underflow checked by branch
        #[allow(clippy::integer_arithmetic)]
        if n > 0x80 {
            self.bytes[checked_add!(self.cursor, i)] = (n & 0b1111111) as u8 | mask;
            n >>= 7;

            if i > 0 {
                self.encode_base128_byte(n, i.saturating_sub(1), true)
            } else {
                Err(Error::Base128)
            }
        } else {
            self.bytes[self.cursor] = n as u8 | mask;
            Ok(self)
        }
    }
}

/// Compute the length - 1 of an arc when encoded in base 128.
const fn base128_len(arc: Arc) -> usize {
    match arc {
        0..=0x7f => 0,
        0x80..=0x3fff => 1,
        0x4000..=0x1fffff => 2,
        0x200000..=0x1fffffff => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder;
    use hex_literal::hex;

    /// OID `1.2.840.10045.2.1` encoded as ASN.1 BER/DER
    const EXAMPLE_OID_BER: &[u8] = &hex!("2A8648CE3D0201");

    #[test]
    fn encode() {
        let encoder = Encoder::new();
        let encoder = encoder.arc(1).unwrap();
        let encoder = encoder.arc(2).unwrap();
        let encoder = encoder.arc(840).unwrap();
        let encoder = encoder.arc(10045).unwrap();
        let encoder = encoder.arc(2).unwrap();
        let encoder = encoder.arc(1).unwrap();
        assert_eq!(&encoder.bytes[..encoder.cursor], EXAMPLE_OID_BER);
    }
}
