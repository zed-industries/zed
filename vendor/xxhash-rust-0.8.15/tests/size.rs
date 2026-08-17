#[cfg(feature = "xxh32")]
#[test]
fn size_xxh32() {
    assert_eq!(core::mem::size_of::<xxhash_rust::xxh32::Xxh32>(), 44);
}

#[cfg(feature = "xxh64")]
#[test]
fn size_xxh64() {
    assert_eq!(core::mem::size_of::<xxhash_rust::xxh64::Xxh64>(), 80);
}

#[cfg(feature = "xxh3")]
#[test]
fn size_xxh3() {
    assert_eq!(core::mem::size_of::<xxhash_rust::xxh3::Xxh3>(), 576);
}
