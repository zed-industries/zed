use rmp::encode::*;

#[test]
fn pass_pack_true() {
    let mut buf = [0x00];

    write_bool(&mut &mut buf[..], true).unwrap();

    assert_eq!([0xc3], buf);
}

#[test]
fn pass_pack_false() {
    let mut buf = [0x00];

    write_bool(&mut &mut buf[..], false).unwrap();

    assert_eq!([0xc2], buf);
}
