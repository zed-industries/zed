// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aead::{Nonce, NonceSequence, NONCE_LEN};
use crate::error::Unspecified;
use crate::iv::FixedLength;

/// `Counter32` is an implementation of the `NonceSequence` trait.
///
/// The internal state of a `Counter32` is a 32-bit unsigned counter that
/// increments on each call to `advance` and an optional 8-byte identifier. Counter and identifier
/// values are used to construct each nonce.
/// A limit can be set on the number of nonces allowed to be generated, by default this limit is
/// `u32::MAX`.
///
/// See [Section 3.2 of RFC 5116](https://www.rfc-editor.org/rfc/rfc5116#section-3.2).
#[allow(clippy::module_name_repetitions)]
pub struct Counter32 {
    limit: u32,
    generated: u32,
    identifier: [u8; 8],
    counter: u32,
}

/// `NonceSequenceBuilder` facilitates the building of a `Counter32`.
#[allow(clippy::module_name_repetitions)]
pub struct Counter32Builder {
    limit: u32,
    identifier: [u8; 8],
    counter: u32,
}

impl Default for Counter32Builder {
    fn default() -> Self {
        Counter32Builder::new()
    }
}

impl Counter32Builder {
    /// Constructs a `Counter32Builder` with all default values.
    #[must_use]
    pub fn new() -> Counter32Builder {
        Counter32Builder {
            limit: u32::MAX,
            identifier: [0u8; 8],
            counter: 0,
        }
    }

    /// The identifier for the `Counter32` - this value helps differentiate nonce
    /// sequences.
    #[must_use]
    pub fn identifier<T: Into<[u8; 8]>>(mut self, identifier: T) -> Counter32Builder {
        self.identifier = identifier.into();
        self
    }

    /// The starting counter value for the `Counter32`.
    #[must_use]
    pub fn counter(mut self, counter: u32) -> Counter32Builder {
        self.counter = counter;
        self
    }

    /// The limit for the number of nonces the `Counter32` can produce.
    #[must_use]
    pub fn limit(mut self, limit: u32) -> Counter32Builder {
        self.limit = limit;
        self
    }

    /// Constructs a new `Counter32` with internal identifier and counter set to the
    /// values provided by this struct.
    #[must_use]
    pub fn build(self) -> Counter32 {
        Counter32 {
            limit: self.limit,
            generated: 0,
            identifier: self.identifier,
            counter: self.counter,
        }
    }
}

impl Counter32 {
    /// Provides the internal identifier.
    #[must_use]
    pub fn identifier(&self) -> [u8; 8] {
        self.identifier
    }

    /// Provides the current internal counter value.
    #[must_use]
    pub fn counter(&self) -> u32 {
        self.counter
    }

    /// Provides the current counter indicating how many nonces have been generated.
    #[must_use]
    pub fn generated(&self) -> u32 {
        self.generated
    }

    /// Provides the limit on the number of nonces allowed to be generate.
    #[must_use]
    pub fn limit(&self) -> u32 {
        self.limit
    }
}

impl NonceSequence for Counter32 {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.generated = self.generated.checked_add(1).ok_or(Unspecified)?;
        if self.generated > self.limit {
            return Err(Unspecified);
        }
        let counter_bytes: [u8; 4] = self.counter.to_be_bytes();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes[..8].copy_from_slice(&self.identifier);
        nonce_bytes[8..].copy_from_slice(&counter_bytes);
        self.counter = self.counter.wrapping_add(1);
        Ok(Nonce(FixedLength::from(nonce_bytes)))
    }
}

#[cfg(test)]
mod tests {
    use crate::aead::nonce_sequence::Counter32Builder;
    use crate::aead::NonceSequence;

    #[test]
    fn test_counter32_identifier() {
        let mut cns = Counter32Builder::default()
            .identifier([0xA1, 0xB2, 0xC3, 0xD4, 0xA2, 0xB3, 0xC4, 0xD5])
            .counter(7)
            .build();
        assert_eq!(0, cns.generated());
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(8, cns.counter());
        assert_eq!(
            [0xA1, 0xB2, 0xC3, 0xD4, 0xA2, 0xB3, 0xC4, 0xD5],
            cns.identifier()
        );
        assert_eq!(u32::MAX, cns.limit());
        assert_eq!(1, cns.generated());
        assert_eq!(
            nonce,
            &[0xA1, 0xB2, 0xC3, 0xD4, 0xA2, 0xB3, 0xC4, 0xD5, 0, 0, 0, 7]
        );
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(2, cns.generated());
        assert_eq!(9, cns.counter());
        assert_eq!(
            [0xA1, 0xB2, 0xC3, 0xD4, 0xA2, 0xB3, 0xC4, 0xD5],
            cns.identifier()
        );
        assert_eq!(
            nonce,
            &[0xA1, 0xB2, 0xC3, 0xD4, 0xA2, 0xB3, 0xC4, 0xD5, 0, 0, 0, 8]
        );
    }

    #[test]
    fn test_counter32() {
        let mut cns = Counter32Builder::new().counter(0x_4CB0_16EA_u32).build();
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0, 0x4C, 0xB0, 0x16, 0xEA]);
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0, 0x4C, 0xB0, 0x16, 0xEB]);
    }

    #[test]
    fn test_counter32_int_id() {
        let mut cns = Counter32Builder::new()
            .counter(0x_6A_u32)
            .identifier(0x_7B_u64.to_be_bytes())
            .build();
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0x7B, 0, 0, 0, 0x6A]);
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0x7B, 0, 0, 0, 0x6B]);
    }

    #[test]
    fn test_counter32_limit() {
        let mut cns = Counter32Builder::new().limit(1).build();
        assert_eq!(1, cns.limit());
        assert_eq!(0, cns.generated());
        let _nonce = cns.advance().unwrap();
        assert_eq!(1, cns.generated());
        assert!(cns.advance().is_err());
    }
}
