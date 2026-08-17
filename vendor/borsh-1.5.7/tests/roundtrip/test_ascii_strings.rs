use borsh::{from_slice, to_vec};

use alloc::{
    string::String,
    vec::Vec,
};

/// Verifies serialisation and deserialisation of an ASCII string `value`.
fn check_ascii(value: &str) -> Vec<u8> {
    // Caller promises value is ASCII.
    let ascii_str = ascii::AsciiStr::from_ascii(&value).unwrap();
    let buf = to_vec(ascii_str).unwrap();
    // AsciiStr and AsciiString serialise the same way String does.
    assert_eq!(buf, to_vec(&ascii::AsciiString::from(ascii_str)).unwrap());
    // Check round trip.
    let got = from_slice::<ascii::AsciiString>(&buf).unwrap();
    assert_eq!(ascii_str, got);
    buf
}

macro_rules! test_ascii_string {
    ($test_name: ident, $str: expr, $snap:expr) => {
        #[test]
        fn $test_name() {
            let value = String::from($str);
            let _buf = check_ascii(&value);
            #[cfg(feature = "std")]
            if $snap {
                insta::assert_debug_snapshot!(_buf);
            }
        }
    };
}

test_ascii_string!(test_empty_string, "", true);
test_ascii_string!(test_a, "a", true);
test_ascii_string!(test_hello_world, "hello world", true);
test_ascii_string!(test_x_1024, "x".repeat(1024), true);
test_ascii_string!(test_x_4096, "x".repeat(4096), false);
test_ascii_string!(test_x_65535, "x".repeat(65535), false);
test_ascii_string!(test_hello_10, "hello world!".repeat(30), true);
test_ascii_string!(test_hello_1000, "hello Achilles!".repeat(1000), false);

#[test]
fn test_ascii_char() {
    use ascii::AsciiChar;

    let buf = to_vec(&AsciiChar::Dot).unwrap();
    assert_eq!(".".as_bytes(), buf);
    assert_eq!(AsciiChar::Dot, from_slice::<AsciiChar>(&buf).unwrap());

    from_slice::<AsciiChar>(&[b'\x80']).unwrap_err();
}
