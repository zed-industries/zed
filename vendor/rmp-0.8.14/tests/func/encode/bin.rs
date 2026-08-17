use rmp::encode::*;
use rmp::Marker;

#[test]
fn pass_pack_len_u8() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::Bin8, write_bin_len(&mut &mut buf[..], 255).unwrap());

    assert_eq!([0xc4, 0xff], buf);
}

#[test]
fn pass_pack_len_u16() {
    let mut buf = [0x00, 0x00, 0x00];

    assert_eq!(Marker::Bin16, write_bin_len(&mut &mut buf[..], 65535).unwrap());

    assert_eq!([0xc5, 0xff, 0xff], buf);
}

#[test]
fn pass_pack_len_u32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::Bin32, write_bin_len(&mut &mut buf[..], 4294967295).unwrap());

    assert_eq!([0xc6, 0xff, 0xff, 0xff, 0xff], buf);
}
