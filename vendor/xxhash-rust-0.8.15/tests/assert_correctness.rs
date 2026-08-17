#[cfg(feature = "xxh64")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_xxh64() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::xxh64::xxh64;

    const SEED_1: u64 = 0;
    const SEED_2: u64 = 1;

    let mut hasher_1 = xxhash_rust::xxh64::Xxh64::new(SEED_1);
    let mut hasher_2 = xxhash_rust::xxh64::Xxh64::new(SEED_2);

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        getrandom(&mut input).expect("getrandom");
        println!("input(len={})", input.len());

        let sys_result = unsafe {
            sys::XXH64(input.as_ptr() as _, input.len(), SEED_1)
        };
        let result = xxh64(&input, SEED_1);
        assert_eq!(result, sys_result);
        hasher_1.update(&input);
        assert_eq!(hasher_1.digest(), result);

        let sys_result = unsafe {
            sys::XXH64(input.as_ptr() as _, input.len(), SEED_2)
        };
        let result = xxh64(&input, SEED_2);
        assert_eq!(result, sys_result);
        hasher_2.update(&input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset(SEED_1);
        hasher_2.reset(SEED_2);
    }
}

#[cfg(feature = "xxh32")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_xxh32() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::xxh32::xxh32;

    const SEED_1: u32 = 0;
    const SEED_2: u32 = 1;

    let mut hasher_1 = xxhash_rust::xxh32::Xxh32::new(SEED_1);
    let mut hasher_2 = xxhash_rust::xxh32::Xxh32::new(SEED_2);

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        getrandom(&mut input).expect("getrandom");
        println!("input(len={})", input.len());

        let sys_result = unsafe {
            sys::XXH32(input.as_ptr() as _, input.len(), SEED_1)
        };
        let result = xxh32(&input, SEED_1);
        assert_eq!(result, sys_result);
        hasher_1.update(&input);
        assert_eq!(hasher_1.digest(), result);

        let sys_result = unsafe {
            sys::XXH32(input.as_ptr() as _, input.len(), SEED_2)
        };
        let result = xxh32(&input, SEED_2);
        assert_eq!(result, sys_result);
        hasher_2.update(&input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset(SEED_1);
        hasher_2.reset(SEED_2);
    }
}

#[cfg(feature = "const_xxh32")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_const_xxh32() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::const_xxh32::xxh32;

    const SEED_1: u32 = 0;
    const SEED_2: u32 = 1;

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        getrandom(&mut input).expect("getrandom");
        println!("input(len={})", input.len());
        let sys_result = unsafe {
            sys::XXH32(input.as_ptr() as _, input.len(), SEED_1)
        };
        let result = xxh32(&input, SEED_1);
        assert_eq!(result, sys_result);

        let sys_result = unsafe {
            sys::XXH32(input.as_ptr() as _, input.len(), SEED_2)
        };
        let result = xxh32(&input, SEED_2);
        assert_eq!(result, sys_result);
    }
}

#[cfg(feature = "const_xxh64")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_const_xxh64() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::const_xxh64::xxh64;

    const SEED_1: u64 = 0;
    const SEED_2: u64 = 1;

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        getrandom(&mut input).expect("getrandom");
        println!("input(len={})", input.len());
        let sys_result = unsafe {
            sys::XXH64(input.as_ptr() as _, input.len(), SEED_1)
        };
        let result = xxh64(&input, SEED_1);
        assert_eq!(result, sys_result);

        let sys_result = unsafe {
            sys::XXH64(input.as_ptr() as _, input.len(), SEED_2)
        };
        let result = xxh64(&input, SEED_2);
        assert_eq!(result, sys_result);
    }
}

#[cfg(feature = "const_xxh3")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_const_xxh3() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::const_xxh3::{xxh3_64, xxh3_128, xxh3_64_with_seed, xxh3_128_with_seed};

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        getrandom(&mut input).expect("getrandom");
        let input = input.as_slice();
        println!("input(len={})", input.len());

        let sys_result = unsafe {
            sys::XXH3_64bits(input.as_ptr() as _, input.len())
        };
        let result = xxh3_64(input);
        assert_eq!(result, sys_result);

        let sys_result = unsafe {
            sys::XXH3_64bits_withSeed(input.as_ptr() as _, input.len(), 1)
        };
        let result = xxh3_64_with_seed(input, 1);
        assert_eq!(result, sys_result);

        let sys_result128 = unsafe {
            sys::XXH3_128bits(input.as_ptr() as _, input.len())
        };
        let result128 = xxh3_128(input);
        assert_eq!(result128 as u64, sys_result128.low64);
        assert_eq!((result128 >> 64) as u64, sys_result128.high64);

        let sys_result128 = unsafe {
            sys::XXH3_128bits_withSeed(input.as_ptr() as _, input.len(), 1)
        };
        let result128 = xxh3_128_with_seed(input, 1);
        assert_eq!(result128 as u64, sys_result128.low64);
        assert_eq!((result128 >> 64) as u64, sys_result128.high64);
    }
}

#[cfg(feature = "xxh3")]
#[cfg_attr(miri, ignore)]
#[test]
fn assert_xxh3() {
    use getrandom::getrandom;
    use xxhash_c_sys as sys;
    use xxhash_rust::xxh3::{xxh3_64, xxh3_128, xxh3_64_with_seed, xxh3_128_with_seed, Xxh3, Xxh3Default};

    let mut hasher_default = Xxh3Default::new();
    let mut hasher_1 = Xxh3::new();
    let mut hasher_2 = Xxh3::with_seed(1);

    let mut hasher_default_128 = Xxh3Default::new();
    let mut hasher_1_128 = Xxh3::new();
    let mut hasher_2_128 = Xxh3::with_seed(1);

    let mut input = Vec::with_capacity(4096);
    for num in 0..input.capacity() {
        input.resize(num, 1);
        println!("input(len={})", input.len());
        getrandom(&mut input).expect("getrandom");
        let input = input.as_slice();

        let sys_result = unsafe {
            sys::XXH3_64bits(input.as_ptr() as _, input.len())
        };
        let result = xxh3_64(input);
        assert_eq!(result, sys_result);
        hasher_1.update(input);
        hasher_1_128.update(input);
        hasher_default.update(input);
        hasher_default_128.update(input);
        assert_eq!(hasher_1.digest(), result);
        assert_eq!(hasher_default.digest(), result);

        let sys_result128 = unsafe {
            sys::XXH3_128bits(input.as_ptr() as _, input.len())
        };
        let result128 = xxh3_128(input);
        assert_eq!(result128 as u64, sys_result128.low64);
        assert_eq!((result128 >> 64) as u64, sys_result128.high64);
        assert_eq!(hasher_1_128.digest128(), result128);
        assert_eq!(hasher_default_128.digest128(), result128);

        let sys_result = unsafe {
            sys::XXH3_64bits_withSeed(input.as_ptr() as _, input.len(), 1)
        };
        let result = xxh3_64_with_seed(input, 1);
        assert_eq!(result, sys_result);
        hasher_2.update(input);
        hasher_2_128.update(input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset();
        hasher_2.reset();
        hasher_default.reset();

        let sys_result128 = unsafe {
            sys::XXH3_128bits_withSeed(input.as_ptr() as _, input.len(), 1)
        };
        let result128 = xxh3_128_with_seed(input, 1);
        assert_eq!(result128 as u64, sys_result128.low64);
        assert_eq!((result128 >> 64) as u64, sys_result128.high64);
        assert_eq!(hasher_2_128.digest128(), result128);

        hasher_1_128.reset();
        hasher_2_128.reset();
        hasher_default_128.reset();
    }
}
