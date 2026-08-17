use zvariant::{BE, LE};

#[test]
fn i64_value() {
    let encoded = basic_type_test!(BE, DBus, -0x0ABB_AABB_AABB_AAB0_i64, 8, i64, 8, I64, 16);
    assert_eq!(LE.read_i64(&encoded), 0x5055_4455_4455_44F5_i64);
    #[cfg(feature = "gvariant")]
    basic_type_test!(BE, GVariant, -0x0ABB_AABB_AABB_AAB0_i64, 8, i64, 8, I64, 10);
}
