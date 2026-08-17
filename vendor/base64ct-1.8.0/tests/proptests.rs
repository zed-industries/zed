//! Equivalence tests between `base64` crate and `base64ct`.

#![cfg(feature = "std")]
// TODO(tarcieri): fix `base64` crate deprecations
// warning: use of deprecated function `base64::encode`: Use Engine::encode
#![allow(deprecated)]

use base64ct::{Base64 as Base64ct, Encoding};
use proptest::{prelude::*, string::*};

/// Incremental Base64 decoder.
type Decoder<'a> = base64ct::Decoder<'a, Base64ct>;

/// Incremental Base64 encoder.
type Encoder<'a> = base64ct::Encoder<'a, Base64ct>;

proptest! {
    /// Ensure `base64ct` decodes data encoded by `base64` ref crate
    #[test]
    fn decode_equiv(bytes in bytes_regex(".{0,256}").unwrap()) {
        let encoded = base64::encode(&bytes);
        let decoded = Base64ct::decode_vec(&encoded);
        prop_assert_eq!(Ok(bytes), decoded);
    }

    /// Ensure that `base64ct`'s incremental decoder is able to decode randomly
    /// generated inputs encoded by the `base64` ref crate
    #[test]
    fn decode_incremental(bytes in bytes_regex(".{1,256}").unwrap(), chunk_size in 1..256usize) {
        let encoded = base64::encode(&bytes);
        let chunk_size = match chunk_size % bytes.len() {
            0 => 1,
            n => n
        };

        let mut buffer = [0u8; 384];
        let mut decoder = Decoder::new(encoded.as_bytes()).unwrap();
        let mut remaining_len = decoder.remaining_len();

        for chunk in bytes.chunks(chunk_size) {
            prop_assert!(!decoder.is_finished());

            let decoded = decoder.decode(&mut buffer[..chunk.len()]);
            prop_assert_eq!(Ok(chunk), decoded);

            remaining_len -= decoded.unwrap().len();
            prop_assert_eq!(remaining_len, decoder.remaining_len());
        }

        prop_assert!(decoder.is_finished());
        prop_assert_eq!(decoder.remaining_len(), 0);
    }

    #[test]
    fn decode_incremental_wrapped(
        bytes in bytes_regex(".{1,256}").unwrap(),
        line_width in 4..128usize,
        chunk_size in 1..256usize
    ) {
        for line_ending in ["\r", "\n", "\r\n"] {
            let encoded = base64::encode(&bytes);

            let mut encoded_wrapped = Vec::new();
            let mut lines = encoded.as_bytes().chunks_exact(line_width);

            for line in &mut lines {
                encoded_wrapped.extend_from_slice(line);
                encoded_wrapped.extend_from_slice(line_ending.as_bytes());
            }

            let last = lines.remainder();

            if last.is_empty() {
                encoded_wrapped.truncate(encoded_wrapped.len() - line_ending.len());
            } else {
                encoded_wrapped.extend_from_slice(last);
            }

            let chunk_size = match chunk_size % bytes.len() {
                0 => 1,
                n => n
            };

            let mut buffer = [0u8; 384];
            let mut decoder = Decoder::new_wrapped(&encoded_wrapped, line_width).unwrap();
            let mut remaining_len = decoder.remaining_len();

            for chunk in bytes.chunks(chunk_size) {
                prop_assert!(!decoder.is_finished());

                let decoded = decoder.decode(&mut buffer[..chunk.len()]);
                prop_assert_eq!(Ok(chunk), decoded);

                remaining_len -= decoded.unwrap().len();
                prop_assert_eq!(remaining_len, decoder.remaining_len());
            }

            prop_assert!(decoder.is_finished());
            prop_assert_eq!(decoder.remaining_len(), 0);
        }
    }

    /// Ensure `base64ct` and `base64` ref crate decode randomly generated
    /// inputs equivalently.
    ///
    /// Inputs are selected to be valid characters in the standard Base64
    /// padded alphabet, but are not necessarily valid Base64.
    #[test]
    fn decode_random(base64ish in string_regex("[A-Za-z0-9+/]{0,256}").unwrap()) {
        let base64ish_padded = match base64ish.len() % 4 {
            0 => base64ish,
            n => {
                let padding_len = 4 - n;
                base64ish + &"=".repeat(padding_len)
            }
        };

        let decoded_ct = Base64ct::decode_vec(&base64ish_padded).ok();
        let decoded_ref = base64::decode(&base64ish_padded).ok();
        prop_assert_eq!(decoded_ct, decoded_ref);
    }

    /// Ensure `base64ct` and the `base64` ref crate encode randomly generated
    /// inputs equivalently.
    #[test]
    fn encode_equiv(bytes in bytes_regex(".{0,256}").unwrap()) {
        let encoded_ct = Base64ct::encode_string(&bytes);
        let encoded_ref = base64::encode(&bytes);
        prop_assert_eq!(encoded_ct, encoded_ref);
    }

    /// Ensure that `base64ct`'s incremental encoder is able to encode randomly
    /// generated inputs which match what's encoded by the `base64` ref crate
    #[test]
    fn encode_incremental(bytes in bytes_regex(".{1,256}").unwrap(), chunk_size in 1..256usize) {
        let expected = base64::encode(&bytes);
        let chunk_size = match chunk_size % bytes.len() {
            0 => 1,
            n => n
        };

        let mut buffer = [0u8; 1024];
        let mut encoder = Encoder::new(&mut buffer).unwrap();

        for chunk in bytes.chunks(chunk_size) {
            encoder.encode(chunk).unwrap();
        }

        prop_assert_eq!(expected, encoder.finish().unwrap());
    }
}
