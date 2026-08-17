use rmpv::encode::write_value_ref;
use rmpv::ValueRef;

#[test]
fn pack_nil() {
    let mut buf = [0x00];

    let val = ValueRef::Nil;

    write_value_ref(&mut &mut buf[..], &val).unwrap();

    assert_eq!([0xc0], buf);
}

#[test]
fn pack_nil_when_buffer_is_tool_small() {
    let mut buf = [];

    let val = ValueRef::Nil;

    match write_value_ref(&mut &mut buf[..], &val) {
        Err(..) => (),
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn pass_pack_true() {
    let mut buf = [0x00];

    let val = ValueRef::Boolean(true);

    write_value_ref(&mut &mut buf[..], &val).unwrap();

    assert_eq!([0xc3], buf);
}

#[test]
fn pass_pack_uint_u16() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    let val = ValueRef::from(65535);

    write_value_ref(&mut &mut buf[..], &val).unwrap();

    assert_eq!([0xcd, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

#[test]
fn pass_pack_i64() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    let val = ValueRef::from(-9223372036854775808i64);

    write_value_ref(&mut &mut buf[..], &val).unwrap();

    assert_eq!([0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], buf);
}

fn check_packed_eq(expected: &Vec<u8>, actual: &ValueRef<'_>) {
    let mut buf = Vec::new();

    write_value_ref(&mut buf, actual).unwrap();

    assert_eq!(*expected, buf);
}

#[test]
fn pass_pack_f32() {
    check_packed_eq(
        &vec![0xca, 0x7f, 0x7f, 0xff, 0xff],
        &ValueRef::F32(3.4028234e38_f32),
    );
}

#[test]
fn pass_pack_f64() {
    use std::f64;
    check_packed_eq(
        &vec![0xcb, 0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        &ValueRef::F64(f64::INFINITY),
    );
}

#[test]
fn pass_pack_string() {
    check_packed_eq(
        &vec![0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65],
        &ValueRef::from("le message")
    );
}

#[test]
fn pass_pack_bin() {
    check_packed_eq(
        &vec![0xc4, 0x0a, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65],
        &ValueRef::Binary(&[0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65])
    );
}

#[test]
fn pass_pack_array() {
    check_packed_eq(
        &vec![0x93, 0x01, 0x02, 0x03],
        &ValueRef::Array(
            vec![
                ValueRef::from(1),
                ValueRef::from(2),
                ValueRef::from(3)
            ]
        )
    );
}

#[test]
fn pass_pack_map() {
    check_packed_eq(
        &vec![0x81, 0x01, 0x02],
        &ValueRef::Map(
            vec![(ValueRef::from(1), ValueRef::from(2))]
        )
    );
}

#[test]
fn pass_pack_ext() {
    check_packed_eq(
        &vec![0xc7, 0x03, 0x10, 0x01, 0x02, 0x03],
        &ValueRef::Ext(16, &[0x01, 0x02, 0x03]),
    );
}
