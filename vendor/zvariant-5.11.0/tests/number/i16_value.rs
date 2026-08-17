use zvariant::{BE, LE};

#[test]
fn i16_value() {
    let encoded = basic_type_test!(BE, DBus, -0xAB0_i16, 2, i16, 2, I16, 6);
    assert_eq!(LE.read_i16(&encoded), 0x50F5_i16);
    #[cfg(feature = "gvariant")]
    basic_type_test!(BE, GVariant, -0xAB0_i16, 2, i16, 2, I16, 4);
}
