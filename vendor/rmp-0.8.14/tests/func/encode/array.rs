use rmp::encode::*;
use rmp::Marker;

#[test]
fn pass_pack_len_fix() {
    let mut buf = [0x00];

    assert_eq!(Marker::FixArray(15), write_array_len(&mut &mut buf[..], 15).unwrap());

    assert_eq!([0x9f], buf);
}

#[test]
fn pass_pack_len_u16() {
    let mut buf = [0x00, 0x00, 0x00];

    assert_eq!(Marker::Array16, write_array_len(&mut &mut buf[..], 65535).unwrap());

    assert_eq!([0xdc, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_len_u32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::Array32, write_array_len(&mut &mut buf[..], 4294967295).unwrap());

    assert_eq!([0xdd, 0xff, 0xff, 0xff, 0xff], buf);
}
