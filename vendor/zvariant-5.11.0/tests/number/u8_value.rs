use zvariant::LE;

#[test]
fn u8_value() {
    let encoded = basic_type_test!(LE, DBus, 77_u8, 1, u8, 1, U8, 4);
    assert_eq!(encoded.len(), 1);
    #[cfg(feature = "gvariant")]
    basic_type_test!(LE, GVariant, 77_u8, 1, u8, 1, U8, 3);
}
