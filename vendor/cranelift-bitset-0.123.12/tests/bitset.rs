use cranelift_bitset::*;

#[test]
fn contains() {
    let s = ScalarBitSet::<u8>(255);
    for i in 0..7 {
        assert!(s.contains(i));
    }

    let s1 = ScalarBitSet::<u8>(0);
    for i in 0..7 {
        assert!(!s1.contains(i));
    }

    let s2 = ScalarBitSet::<u8>(127);
    for i in 0..6 {
        assert!(s2.contains(i));
    }
    assert!(!s2.contains(7));

    let s3 = ScalarBitSet::<u8>(2 | 4 | 64);
    assert!(!s3.contains(0) && !s3.contains(3) && !s3.contains(4));
    assert!(!s3.contains(5) && !s3.contains(7));
    assert!(s3.contains(1) && s3.contains(2) && s3.contains(6));

    let s4 = ScalarBitSet::<u16>(4 | 8 | 256 | 1024);
    assert!(
        !s4.contains(0)
            && !s4.contains(1)
            && !s4.contains(4)
            && !s4.contains(5)
            && !s4.contains(6)
            && !s4.contains(7)
            && !s4.contains(9)
            && !s4.contains(11)
    );
    assert!(s4.contains(2) && s4.contains(3) && s4.contains(8) && s4.contains(10));
}

#[test]
fn minmax() {
    let s = ScalarBitSet::<u8>(255);
    assert_eq!(s.min(), Some(0));
    assert_eq!(s.max(), Some(7));
    assert!(s.min() == Some(0) && s.max() == Some(7));
    let s1 = ScalarBitSet::<u8>(0);
    assert!(s1.min() == None && s1.max() == None);
    let s2 = ScalarBitSet::<u8>(127);
    assert!(s2.min() == Some(0) && s2.max() == Some(6));
    let s3 = ScalarBitSet::<u8>(2 | 4 | 64);
    assert!(s3.min() == Some(1) && s3.max() == Some(6));
    let s4 = ScalarBitSet::<u16>(4 | 8 | 256 | 1024);
    assert!(s4.min() == Some(2) && s4.max() == Some(10));
}

#[test]
fn from_range() {
    let s = ScalarBitSet::<u8>::from_range(5, 5);
    assert!(s.0 == 0);

    let s = ScalarBitSet::<u8>::from_range(0, 8);
    assert!(s.0 == 255);

    let s = ScalarBitSet::<u16>::from_range(0, 8);
    assert!(s.0 == 255u16);

    let s = ScalarBitSet::<u16>::from_range(0, 16);
    assert!(s.0 == 65535u16);

    let s = ScalarBitSet::<u8>::from_range(5, 6);
    assert!(s.0 == 32u8);

    let s = ScalarBitSet::<u8>::from_range(3, 7);
    assert!(s.0 == 8 | 16 | 32 | 64);

    let s = ScalarBitSet::<u16>::from_range(5, 11);
    assert!(s.0 == 32 | 64 | 128 | 256 | 512 | 1024);
}
