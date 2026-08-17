// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

use crate::aead::{Nonce, NonceSequence, NONCE_LEN};
use crate::error::Unspecified;
use crate::iv::FixedLength;

/// `Counter64` is an implementation of the `NonceSequence` trait.
///
/// The internal state of a `Counter64` is a 64-bit unsigned counter that
/// increments on each call to `advance` and an optional 4-byte identifier. Counter and identifier
/// values are used to construct each nonce.
/// A limit can be set on the number of nonces allowed to be generated, by default this limit is
/// `u64::MAX`.
/// See [Section 3.2 of RFC 5116](https://www.rfc-editor.org/rfc/rfc5116#section-3.2).
#[allow(clippy::module_name_repetitions)]
pub struct Counter64 {
    limit: u64,
    generated: u64,
    identifier: [u8; 4],
    counter: u64,
}

/// `NonceSequenceBuilder` facilitates the building of a `Counter64`.
#[allow(clippy::module_name_repetitions)]
pub struct Counter64Builder {
    limit: u64,
    identifier: [u8; 4],
    counter: u64,
}

impl Default for Counter64Builder {
    fn default() -> Self {
        Counter64Builder::new()
    }
}

impl Counter64Builder {
    /// Constructs a `Counter64Builder` with all default values.
    #[must_use]
    pub fn new() -> Counter64Builder {
        Counter64Builder {
            limit: u64::MAX,
            identifier: [0u8; 4],
            counter: 0,
        }
    }

    /// The identifier for the `Counter64` - this value helps differentiate nonce
    /// sequences.
    #[must_use]
    pub fn identifier<T: Into<[u8; 4]>>(mut self, identifier: T) -> Counter64Builder {
        self.identifier = identifier.into();
        self
    }

    /// The starting counter value for the `Counter64`.
    #[must_use]
    pub fn counter(mut self, counter: u64) -> Counter64Builder {
        self.counter = counter;
        self
    }

    /// The limit for the number of nonces the `Counter64` can produce.
    #[must_use]
    pub fn limit(mut self, limit: u64) -> Counter64Builder {
        self.limit = limit;
        self
    }

    /// Constructs a new `Counter64` with internal identifier and counter set to the
    /// values provided by this struct.
    #[must_use]
    pub fn build(self) -> Counter64 {
        Counter64 {
            limit: self.limit,
            generated: 0,
            identifier: self.identifier,
            counter: self.counter,
        }
    }
}

impl Counter64 {
    /// Provides the internal identifier.
    #[must_use]
    pub fn identifier(&self) -> [u8; 4] {
        self.identifier
    }

    /// Provides the current internal counter value.
    #[must_use]
    pub fn counter(&self) -> u64 {
        self.counter
    }

    /// Provides the current counter indicating how many nonces have been generated.
    #[must_use]
    pub fn generated(&self) -> u64 {
        self.generated
    }

    /// Provides the limit on the number of nonces allowed to be generate.
    #[must_use]
    pub fn limit(&self) -> u64 {
        self.limit
    }
}

impl NonceSequence for Counter64 {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        self.generated = self.generated.checked_add(1).ok_or(Unspecified)?;
        if self.generated > self.limit {
            return Err(Unspecified);
        }
        let bytes: [u8; 8] = self.counter.to_be_bytes();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes[..4].copy_from_slice(&self.identifier);
        nonce_bytes[4..].copy_from_slice(&bytes);
        self.counter = self.counter.wrapping_add(1);
        Ok(Nonce(FixedLength::from(nonce_bytes)))
    }
}

#[cfg(test)]
mod tests {
    use crate::aead::nonce_sequence::Counter64Builder;
    use crate::aead::NonceSequence;

    #[test]
    fn test_counter64_identifier() {
        let mut cns = Counter64Builder::default()
            .identifier([0xA1, 0xB2, 0xC3, 0xD4])
            .counter(7)
            .build();
        assert_eq!(0, cns.generated());
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(8, cns.counter());
        assert_eq!([0xA1, 0xB2, 0xC3, 0xD4], cns.identifier());
        assert_eq!(u64::MAX, cns.limit());
        assert_eq!(1, cns.generated());
        assert_eq!(nonce, &[0xA1, 0xB2, 0xC3, 0xD4, 0, 0, 0, 0, 0, 0, 0, 7]);
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(2, cns.generated());
        assert_eq!(9, cns.counter());
        assert_eq!([0xA1, 0xB2, 0xC3, 0xD4], cns.identifier());
        assert_eq!(nonce, &[0xA1, 0xB2, 0xC3, 0xD4, 0, 0, 0, 0, 0, 0, 0, 8]);
    }

    #[test]
    fn test_counter64() {
        let mut cns = Counter64Builder::new()
            .counter(0x0002_4CB0_16EA_u64)
            .build();
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0x02, 0x4C, 0xB0, 0x16, 0xEA]);
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0, 0, 0, 0, 0x02, 0x4C, 0xB0, 0x16, 0xEB]);
    }

    #[test]
    fn test_counter64_id() {
        let mut cns = Counter64Builder::new()
            .counter(0x_6A_u64)
            .identifier(0x_7B_u32.to_be_bytes())
            .build();
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0x7B, 0, 0, 0, 0, 0, 0, 0, 0x6A]);
        let nonce = cns.advance().unwrap();
        let nonce = nonce.as_ref();
        assert_eq!(nonce, &[0, 0, 0, 0x7B, 0, 0, 0, 0, 0, 0, 0, 0x6B]);
    }

    #[test]
    fn test_counter64_limit() {
        let mut cns = Counter64Builder::new().limit(1).build();
        assert_eq!(1, cns.limit());
        assert_eq!(0, cns.generated());
        let _nonce = cns.advance().unwrap();
        assert_eq!(1, cns.generated());
        assert!(cns.advance().is_err());
    }
}
