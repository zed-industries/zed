use borsh::from_slice;

#[cfg(feature = "derive")]
use borsh::BorshDeserialize;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(feature = "derive")]
#[derive(BorshDeserialize, Debug)]
#[borsh(use_discriminant = true)]
enum A {
    X,
    Y,
}

#[cfg(feature = "derive")]
#[derive(BorshDeserialize, Debug)]
#[borsh(use_discriminant = false)]
enum AWithUseDiscriminantFalse {
    X,
    Y,
}

#[cfg(feature = "derive")]
#[derive(BorshDeserialize, Debug)]
struct B {
    #[allow(unused)]
    x: u64,
    #[allow(unused)]
    y: u32,
}

const ERROR_UNEXPECTED_LENGTH_OF_INPUT: &str = "Unexpected length of input";
const ERROR_INVALID_ZERO_VALUE: &str = "Expected a non-zero value";

#[cfg(feature = "derive")]
#[test]
fn test_missing_bytes() {
    let bytes = vec![1, 0];
    assert_eq!(
        from_slice::<B>(&bytes).unwrap_err().to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[cfg(feature = "derive")]
#[test]
fn test_invalid_enum_variant() {
    let bytes = vec![123];
    assert_eq!(
        from_slice::<A>(&bytes).unwrap_err().to_string(),
        "Unexpected variant tag: 123"
    );
}

#[cfg(feature = "derive")]
#[test]
fn test_invalid_enum_variant_old() {
    let bytes = vec![123];
    assert_eq!(
        from_slice::<AWithUseDiscriminantFalse>(&bytes)
            .unwrap_err()
            .to_string(),
        "Unexpected variant tag: 123"
    );
}

#[test]
fn test_extra_bytes() {
    let bytes = vec![1, 0, 0, 0, 32, 32];
    assert_eq!(
        from_slice::<Vec<u8>>(&bytes).unwrap_err().to_string(),
        "Not all bytes read"
    );
}

#[test]
fn test_invalid_bool() {
    for i in 2u8..=255 {
        let bytes = [i];
        assert_eq!(
            from_slice::<bool>(&bytes).unwrap_err().to_string(),
            format!("Invalid bool representation: {}", i)
        );
    }
}

#[test]
fn test_invalid_option() {
    for i in 2u8..=255 {
        let bytes = [i, 32];
        assert_eq!(
            from_slice::<Option<u8>>(&bytes).unwrap_err().to_string(),
            format!(
                "Invalid Option representation: {}. The first byte must be 0 or 1",
                i
            )
        );
    }
}

#[test]
fn test_invalid_result() {
    for i in 2u8..=255 {
        let bytes = [i, 0];
        assert_eq!(
            from_slice::<Result<u64, String>>(&bytes)
                .unwrap_err()
                .to_string(),
            format!(
                "Invalid Result representation: {}. The first byte must be 0 or 1",
                i
            )
        );
    }
}

#[test]
fn test_invalid_length() {
    let bytes = vec![255u8; 4];
    assert_eq!(
        from_slice::<Vec<u64>>(&bytes).unwrap_err().to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[test]
fn test_invalid_length_string() {
    let bytes = vec![255u8; 4];
    assert_eq!(
        from_slice::<String>(&bytes).unwrap_err().to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[test]
fn test_non_utf_string() {
    let bytes = vec![1, 0, 0, 0, 0xC0];
    assert_eq!(
        from_slice::<String>(&bytes).unwrap_err().to_string(),
        "invalid utf-8 sequence of 1 bytes from index 0"
    );
}

#[test]
fn test_nan_float() {
    let bytes = vec![0, 0, 192, 127];
    assert_eq!(
        from_slice::<f32>(&bytes).unwrap_err().to_string(),
        "For portability reasons we do not allow to deserialize NaNs."
    );
}

#[test]
fn test_evil_bytes_vec_with_extra() {
    // Should fail to allocate given length
    // test takes a really long time if read() is used instead of read_exact()
    let bytes = vec![255, 255, 255, 255, 32, 32];
    assert_eq!(
        from_slice::<Vec<[u8; 32]>>(&bytes).unwrap_err().to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[test]
fn test_evil_bytes_string_extra() {
    // Might fail if reading too much
    let bytes = vec![255, 255, 255, 255, 32, 32];
    assert_eq!(
        from_slice::<String>(&bytes).unwrap_err().to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}

#[test]
fn test_zero_on_nonzero_integer_u8() {
    let bytes = &[0];
    assert_eq!(
        from_slice::<core::num::NonZeroU8>(bytes)
            .unwrap_err()
            .to_string(),
        ERROR_INVALID_ZERO_VALUE
    );
}

#[test]
fn test_zero_on_nonzero_integer_u32() {
    let bytes = &[0; 4];
    assert_eq!(
        from_slice::<core::num::NonZeroU32>(bytes)
            .unwrap_err()
            .to_string(),
        ERROR_INVALID_ZERO_VALUE
    );
}

#[test]
fn test_zero_on_nonzero_integer_i64() {
    let bytes = &[0; 8];
    assert_eq!(
        from_slice::<core::num::NonZeroI64>(bytes)
            .unwrap_err()
            .to_string(),
        ERROR_INVALID_ZERO_VALUE
    );
}

#[test]
fn test_zero_on_nonzero_integer_usize() {
    let bytes = &[0; 8];
    assert_eq!(
        from_slice::<core::num::NonZeroUsize>(bytes)
            .unwrap_err()
            .to_string(),
        ERROR_INVALID_ZERO_VALUE
    );
}

#[test]
fn test_zero_on_nonzero_integer_missing_byte() {
    let bytes = &[0; 7];
    assert_eq!(
        from_slice::<core::num::NonZeroUsize>(bytes)
            .unwrap_err()
            .to_string(),
        ERROR_UNEXPECTED_LENGTH_OF_INPUT
    );
}
