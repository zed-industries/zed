use zvariant::BE;

#[test]
fn u32_value() {
    let encoded = basic_type_test!(BE, DBus, 0xABBA_ABBA_u32, 4, u32, 4, U32, 8);
    assert_eq!(encoded.len(), 4);
    #[cfg(feature = "gvariant")]
    basic_type_test!(BE, GVariant, 0xABBA_ABBA_u32, 4, u32, 4, U32, 6);
}
