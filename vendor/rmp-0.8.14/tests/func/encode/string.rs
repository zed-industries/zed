use rmp::encode::*;
use rmp::Marker;

#[test]
fn pass_pack_len_fix() {
    let mut buf = [0x00];

    assert_eq!(Marker::FixStr(31), write_str_len(&mut &mut buf[..], 31).unwrap());

    assert_eq!([0xbf], buf);
}

#[test]
fn pass_pack_len_u8() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::Str8, write_str_len(&mut &mut buf[..], 255).unwrap());

    assert_eq!([0xd9, 0xff], buf);
}

#[test]
fn pass_pack_len_u16() {
    let mut buf = [0x00, 0x00, 0x00];

    assert_eq!(Marker::Str16, write_str_len(&mut &mut buf[..], 65535).unwrap());

    assert_eq!([0xda, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_len_u32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::Str32, write_str_len(&mut &mut buf[..], 4294967295).unwrap());

    assert_eq!([0xdb, 0xff, 0xff, 0xff, 0xff], buf);
}
