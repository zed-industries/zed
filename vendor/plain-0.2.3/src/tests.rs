

#![allow(dead_code)]

use ::*;
use core::mem;

#[repr(C)]
#[derive(Debug, Default, Copy, Eq, Clone, PartialEq)]
struct Dummy1 {
    field1: u64,
    field2: u32,
    field3: u16,
    field4: u8,
    field5: u8,
}

unsafe impl Plain for Dummy1 {}

#[repr(C)]
#[derive(Debug, Default, Copy, Eq, Clone, PartialEq)]
struct Dummy2 {
    field1: u8,
    field2: u8,
    field3: u16,
    field4: u32,
    field5: u64,
}

unsafe impl Plain for Dummy2 {}

fn as_bytes<T: ?Sized>(r: &T) -> &[u8] {
    unsafe { methods::as_bytes(r) }
}

fn as_mut_bytes<T: Plain + ?Sized>(r: &mut T) -> &mut [u8] {
    unsafe { methods::as_mut_bytes(r) }
}

#[test]
fn one_too_short() {
    let b = vec![0u8; mem::size_of::<Dummy1>() - 1];

    let r = Dummy1::from_bytes(&b);
    assert!(r == Err(Error::TooShort));
}

#[test]
fn unaligned() {
    let b = vec![0u8; mem::size_of::<Dummy1>() + 1];
    let b = &b[1..];

    let r = Dummy1::from_bytes(&b);
    assert!(r == Err(Error::BadAlignment));
}

#[test]
fn copy_test() {
    let t1 = Dummy1 {
        field1: 0xaaaaaaaaaaaaaaaau64,
        field2: 0xbbbbbbbbu32,
        field3: 0xccccu16,
        field4: 0xddu8,
        field5: 0xeeu8,
    };

    let mut t2 = Dummy2::default();

    assert!(t2.copy_from_bytes(as_bytes(&t1)) == Ok(()));

    assert!(t2.field1 == 0xaau8);
    assert!(t2.field2 == 0xaau8);
    assert!(t2.field3 == 0xaaaau16);
    assert!(t2.field4 == 0xaaaaaaaau32);
    assert!(t2.field5 == 0xbbbbbbbbccccddeeu64 || t2.field5 == 0xeeddccccbbbbbbbbu64);

    let sz = mem::size_of::<Dummy2>();
    assert!(t2.copy_from_bytes(&as_bytes(&t1)[..sz - 1]) == Err(Error::TooShort));
}

#[test]
fn basic_function() {
    let t1 = Dummy1 {
        field1: 0xaaaaaaaaaaaaaaaau64,
        field2: 0xbbbbbbbbu32,
        field3: 0xccccu16,
        field4: 0xddu8,
        field5: 0xeeu8,
    };

    let r1: &Dummy2 = from_bytes(as_bytes(&t1)).unwrap();

    assert!(r1.field1 == 0xaau8);
    assert!(r1.field2 == 0xaau8);
    assert!(r1.field3 == 0xaaaau16);
    assert!(r1.field4 == 0xaaaaaaaau32);
    assert!(r1.field5 == 0xbbbbbbbbccccddeeu64 || r1.field5 == 0xeeddccccbbbbbbbbu64);

    let r2 = as_bytes(r1);
    assert!(r2.len() == mem::size_of::<Dummy1>());
    assert!(r2[5] == 0xaa);

    let size = r2.len();
    let r3 = as_bytes(r2);
    assert!(r3.len() == size);

    let r4 = Dummy1::from_bytes(r3).unwrap();

    let r5 = from_bytes::<Dummy2>(as_bytes(r4)).unwrap();

    {
        let r6 = slice_from_bytes::<Dummy1>(as_bytes(r5)).unwrap();

        assert!(r6.len() == 1);
        assert!(t1 == r6[0]);
    }

    let r7 = slice_from_bytes::<u64>(as_bytes(r5)).unwrap();
    assert!(r7.len() == 2);

    assert!(r7[0] == 0xaaaaaaaaaaaaaaaau64);
    assert!(r7[1] == 0xbbbbbbbbccccddeeu64 || r7[1] == 0xeeddccccbbbbbbbbu64);
}
