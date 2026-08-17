use zvariant::LE;

#[test]
fn bool_value() {
    let encoded = basic_type_test!(LE, DBus, true, 4, bool, 4, Bool, 8);
    assert_eq!(encoded.len(), 4);

    #[cfg(feature = "gvariant")]
    {
        let gvariant = basic_type_test!(LE, GVariant, true, 1, bool, 1, Bool, 3);
        assert_eq!(*gvariant.bytes(), [1]);
    }
}
