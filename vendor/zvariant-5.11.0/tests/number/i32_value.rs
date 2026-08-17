use zvariant::{BE, LE};

#[test]
fn i32_value() {
    let encoded = basic_type_test!(BE, DBus, -0x0ABB_AAB0_i32, 4, i32, 4, I32, 8);
    assert_eq!(LE.read_i32(&encoded), 0x5055_44F5_i32);
    #[cfg(feature = "gvariant")]
    basic_type_test!(BE, GVariant, -0x0ABB_AAB0_i32, 4, i32, 4, I32, 6);
}
