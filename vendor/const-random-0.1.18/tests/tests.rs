use const_random::const_random;

#[test]
fn u32() {
    const VALUE1: u32 = const_random!(u32);
    const VALUE2: u32 = const_random!(u32);
    assert_ne!(0, VALUE1, "A random generated constant was zero. (This can randomly occur one time in 2^32) If this reproduces, it is a bug.");
    assert_ne!(0, VALUE2, "A random generated constant was zero. (This can randomly occur one time in 2^32) If this reproduces, it is a bug.");
    assert_ne!(VALUE1, VALUE2, "A random generated constant was the same as another. (This can randomly occur one time in 2^32) If this reproduces, it is a bug.");
}

#[test]
fn i64() {
    const VALUE1: i64 = const_random!(i64);
    const VALUE2: i64 = const_random!(i64);
    assert_ne!(0, VALUE1, "A random generated constant was zero. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
    assert_ne!(0, VALUE2, "A random generated constant was zero. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
    assert_ne!(VALUE1, VALUE2, "A random generated constant was the same as another. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
}

#[test]
fn usize() {
    const VALUE1: usize = const_random!(usize);
    const VALUE2: usize = const_random!(usize);
    assert_ne!(0, VALUE1, "A random generated constant was zero. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
    assert_ne!(0, VALUE2, "A random generated constant was zero. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
    assert_ne!(VALUE1, VALUE2, "A random generated constant was the same as another. (This can randomly occur one time in 2^64) If this reproduces, it is a bug.");
}

#[test]
fn u128() {
    const VALUE1: u128 = const_random!(u128);
    const VALUE2: u128 = const_random!(u128);
    assert_ne!(0, VALUE1);
    assert_ne!(0, VALUE2);
    assert_ne!(VALUE1, VALUE2);
}

#[test]
fn suffixed() {
    fn f<T>(_: T) {
        // If const_random! emits an unsuffixed integer literal, this assertion
        // would fail because T would be inferred as the default unsuffixed
        // integer literal type i32.
        assert_eq!("u8", std::any::type_name::<T>());
    }
    f(const_random!(u8));
}

#[test]
fn array() {
    const VALUE1: &[u8] = &const_random!([u8; 30]);
    const VALUE2: [u8; 30] = const_random!([u8; 30]);
    assert_ne!([0u8; 30], VALUE1);
    assert_ne!([0u8; 30], VALUE2);
    assert_ne!(VALUE1, VALUE2);
}
