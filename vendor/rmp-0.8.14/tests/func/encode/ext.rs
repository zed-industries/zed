use rmp::encode::*;
use rmp::Marker;

#[test]
fn pass_pack_meta_fix1() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::FixExt1, write_ext_meta(&mut &mut buf[..], 1, 16).unwrap());

    assert_eq!([0xd4, 0x10], buf);
}

#[test]
fn pass_pack_meta_fix2() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::FixExt2, write_ext_meta(&mut &mut buf[..], 2, 16).unwrap());

    assert_eq!([0xd5, 0x10], buf);
}

#[test]
fn pass_pack_meta_fix4() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::FixExt4, write_ext_meta(&mut &mut buf[..], 4, 16).unwrap());

    assert_eq!([0xd6, 0x10], buf);
}

#[test]
fn pass_pack_meta_fix4_timesamp() {
    let mut buf = [0x00, 0x00];
    assert_eq!(Marker::FixExt4, write_ext_meta(&mut &mut buf[..], 4, -1).unwrap());
    assert_eq!([0xd6, 0xff], buf);
}

#[test]
fn pass_pack_meta_fix8() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::FixExt8, write_ext_meta(&mut &mut buf[..], 8, 16).unwrap());

    assert_eq!([0xd7, 0x10], buf);
}

#[test]
fn pass_pack_meta_fix16() {
    let mut buf = [0x00, 0x00];

    assert_eq!(Marker::FixExt16, write_ext_meta(&mut &mut buf[..], 16, 16).unwrap());

    assert_eq!([0xd8, 0x10], buf);
}

#[test]
fn pass_pack_meta_8() {
    let mut buf = [0x00, 0x00, 0x00];

    assert_eq!(Marker::Ext8, write_ext_meta(&mut &mut buf[..], 255, 16).unwrap());

    assert_eq!([0xc7, 0xff, 0x10], buf);
}

#[test]
fn pass_pack_meta_16() {
    let mut buf = [0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::Ext16, write_ext_meta(&mut &mut buf[..], 65535, 16).unwrap());

    assert_eq!([0xc8, 0xff, 0xff, 0x10], buf);
}

#[test]
fn pass_pack_meta_32() {
    let mut buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Marker::Ext32, write_ext_meta(&mut &mut buf[..], 4294967295, 16).unwrap());

    assert_eq!([0xc9, 0xff, 0xff, 0xff, 0xff, 0x10], buf);
}
