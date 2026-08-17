use zvariant::BE;

#[test]
fn u16_value() {
    let encoded = basic_type_test!(BE, DBus, 0xABBA_u16, 2, u16, 2, U16, 6);
    assert_eq!(encoded.len(), 2);
    #[cfg(feature = "gvariant")]
    basic_type_test!(BE, GVariant, 0xABBA_u16, 2, u16, 2, U16, 4);
}
