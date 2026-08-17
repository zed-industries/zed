use alloc::string::String;
use borsh::{from_slice, to_vec};

/// Verifies serialisation and deserialisation of the given string.
///
/// Returns serialised representation of the string.
fn check_string(value: &str) -> alloc::vec::Vec<u8> {
    // Encoding is the same as Vec<u8> with UTF-8 encoded string.
    let buf = to_vec(value.as_bytes()).unwrap();
    assert_eq!(buf, to_vec(value).unwrap());
    assert_eq!(buf, to_vec(&String::from(value)).unwrap());
    // Check round trip.
    assert_eq!(value, from_slice::<String>(&buf).unwrap());
    buf
}

macro_rules! test_string {
    ($test_name: ident, $str: expr, $snap:expr) => {
        #[test]
        fn $test_name() {
            let value = String::from($str);
            let _buf = check_string(&value);
            #[cfg(feature = "std")]
            if $snap {
                insta::assert_debug_snapshot!(_buf);
            }
        }
    };
}

test_string!(test_empty_string, "", true);
test_string!(test_a, "a", true);
test_string!(test_hello_world, "hello world", true);
test_string!(test_x_1024, "x".repeat(1024), true);
test_string!(test_x_4096, "x".repeat(4096), false);
test_string!(test_x_65535, "x".repeat(65535), false);
test_string!(test_hello_10, "hello world!".repeat(30), true);
test_string!(test_hello_1000, "hello world!".repeat(1000), false);

test_string!(test_non_ascii, "💩", true);
