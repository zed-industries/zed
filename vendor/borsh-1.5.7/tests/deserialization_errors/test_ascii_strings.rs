use alloc::string::ToString;
use borsh::from_slice;

#[test]
fn test_non_ascii() {
    let buf = borsh::to_vec(&[0xbf, 0xf3, 0xb3, 0x77][..]).unwrap();
    assert_eq!(
        from_slice::<ascii::AsciiString>(&buf)
            .unwrap_err()
            .to_string(),
        "the byte at index 0 is not ASCII"
    );

    let buf = borsh::to_vec("żółw").unwrap();
    assert_eq!(
        from_slice::<ascii::AsciiString>(&buf)
            .unwrap_err()
            .to_string(),
        "the byte at index 0 is not ASCII"
    );

    assert_eq!(
        from_slice::<ascii::AsciiChar>(&[0xbf])
            .unwrap_err()
            .to_string(),
        "not an ASCII character"
    );
}
