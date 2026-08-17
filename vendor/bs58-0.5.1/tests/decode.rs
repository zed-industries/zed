mod cases;

#[cfg(feature = "check")]
use assert_matches::assert_matches;

#[test]
fn test_decode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(val.to_vec(), bs58::decode(s).into_vec().unwrap());

        const PREFIX: &[u8] = &[0, 1, 2];

        {
            let mut vec = PREFIX.to_vec();
            assert_eq!(Ok(val.len()), bs58::decode(s).onto(&mut vec));
            assert_eq!((PREFIX, val), vec.split_at(3));
        }

        {
            let vec = bs58::decode(s.as_bytes()).into_array_const_unwrap::<128>();
            let mut check = [0; 128];
            check[..val.len()].copy_from_slice(val);
            assert_eq!(vec, check);
        }

        #[cfg(feature = "smallvec")]
        {
            let mut vec = smallvec::SmallVec::<[u8; 36]>::from(PREFIX);
            assert_eq!(Ok(val.len()), bs58::decode(s).onto(&mut vec));
            assert_eq!((PREFIX, val), vec.split_at(3));
        }

        #[cfg(feature = "tinyvec")]
        {
            {
                let mut vec = tinyvec::ArrayVec::<[u8; 36]>::from_iter(PREFIX.iter().copied());
                let res = bs58::decode(s).onto(&mut vec);
                if PREFIX.len() + val.len() <= vec.capacity() {
                    assert_eq!(Ok(val.len()), res);
                    assert_eq!((PREFIX, val), vec.split_at(3));
                } else {
                    assert_eq!(Err(bs58::decode::Error::BufferTooSmall), res);
                }
            }

            {
                let mut array = [0; 36];
                array[..PREFIX.len()].copy_from_slice(PREFIX);
                let mut vec = tinyvec::SliceVec::from_slice_len(&mut array, PREFIX.len());
                let res = bs58::decode(s).onto(&mut vec);
                if PREFIX.len() + val.len() <= vec.capacity() {
                    assert_eq!(Ok(val.len()), res);
                    assert_eq!((PREFIX, val), vec.split_at(3));
                } else {
                    assert_eq!(Err(bs58::decode::Error::BufferTooSmall), res);
                }
            }

            {
                let mut vec = tinyvec::TinyVec::<[u8; 36]>::from(PREFIX);
                assert_eq!(Ok(val.len()), bs58::decode(s).onto(&mut vec));
                assert_eq!((PREFIX, val), vec.split_at(3));
            }
        }
    }
}

#[test]
fn test_decode_small_buffer_err() {
    let mut output = [0; 2];
    assert_eq!(
        bs58::decode("a3gV").onto(&mut output),
        Err(bs58::decode::Error::BufferTooSmall)
    );
}

#[test]
#[should_panic]
fn test_decode_const_small_buffer_panic() {
    bs58::decode(&b"a3gV"[..]).into_array_const_unwrap::<2>();
}

#[test]
#[should_panic]
fn test_decode_const_invalid_char_panic() {
    let sample = "123456789abcd!efghij";
    let _ = bs58::decode(sample.as_bytes()).into_array_const_unwrap::<32>();
}

#[test]
fn test_decode_invalid_char() {
    let sample = "123456789abcd!efghij";
    assert_eq!(
        bs58::decode(sample).into_vec().unwrap_err(),
        bs58::decode::Error::InvalidCharacter {
            character: '!',
            index: 13
        }
    );
}

#[test]
#[cfg(feature = "check")]
fn test_decode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(
            val.to_vec(),
            bs58::decode(s).with_check(None).into_vec().unwrap()
        );
    }

    for &(val, s) in cases::CHECK_TEST_CASES[1..].iter() {
        assert_eq!(
            val.to_vec(),
            bs58::decode(s).with_check(Some(val[0])).into_vec().unwrap()
        );
    }
}

#[test]
#[cfg(feature = "check")]
fn test_check_ver_failed() {
    let d = bs58::decode("K5zqBMZZTzUbAZQgrt4")
        .with_check(Some(0x01))
        .into_vec();

    assert!(d.is_err());
    assert_matches!(d.unwrap_err(), bs58::decode::Error::InvalidVersion { .. });
}

#[test]
fn append() {
    let mut buf = b"hello world".to_vec();
    bs58::decode("a").onto(&mut buf).unwrap();
    assert_eq!(b"hello world!", buf.as_slice());
}

#[test]
fn no_append() {
    let mut buf = b"hello world".to_owned();
    bs58::decode("a").onto(buf.as_mut()).unwrap();
    assert_eq!(b"!ello world", buf.as_ref());
}
