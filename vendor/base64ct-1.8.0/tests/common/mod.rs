//! Common testing functionality

/// Base64 test vector
pub struct TestVector {
    pub raw: &'static [u8],
    pub b64: &'static str,
}

/// Generate test suite for a particular Base64 flavor
#[macro_export]
macro_rules! impl_tests {
    ($encoding:ty) => {
        use base64ct::{Encoding, Error};

        #[test]
        fn encode_test_vectors() {
            let mut buf = [0u8; 1024];

            for vector in TEST_VECTORS {
                let out = <$encoding>::encode(vector.raw, &mut buf).unwrap();
                assert_eq!(<$encoding>::encoded_len(vector.raw), vector.b64.len());
                assert_eq!(vector.b64, &out[..]);

                #[cfg(feature = "alloc")]
                {
                    let out = <$encoding>::encode_string(vector.raw);
                    assert_eq!(vector.b64, &out[..]);
                }
            }
        }

        #[test]
        fn decode_test_vectors() {
            let mut buf = [0u8; 1024];

            for vector in TEST_VECTORS {
                let out = <$encoding>::decode(vector.b64, &mut buf).unwrap();
                assert_eq!(vector.raw, &out[..]);

                let n = vector.b64.len();
                buf[..n].copy_from_slice(vector.b64.as_bytes());
                let out = <$encoding>::decode_in_place(&mut buf[..n]).unwrap();
                assert_eq!(vector.raw, out);

                #[cfg(feature = "alloc")]
                {
                    let out = <$encoding>::decode_vec(vector.b64).unwrap();
                    assert_eq!(vector.raw, &out[..]);
                }
            }
        }

        #[test]
        fn encode_and_decode_various_lengths() {
            let data = [b'X'; 64];
            let mut inbuf = [0u8; 1024];
            let mut outbuf = [0u8; 1024];

            for i in 0..data.len() {
                let encoded = <$encoding>::encode(&data[..i], &mut inbuf).unwrap();

                // Make sure it round trips
                let decoded = <$encoding>::decode(encoded, &mut outbuf).unwrap();
                assert_eq!(decoded, &data[..i]);

                let elen = <$encoding>::encode(&data[..i], &mut inbuf).unwrap().len();
                let buf = &mut inbuf[..elen];
                let decoded = <$encoding>::decode_in_place(buf).unwrap();
                assert_eq!(decoded, &data[..i]);

                #[cfg(feature = "alloc")]
                {
                    let encoded = <$encoding>::encode_string(&data[..i]);
                    let decoded = <$encoding>::decode_vec(&encoded).unwrap();
                    assert_eq!(decoded, &data[..i]);
                }
            }
        }
    };
}
