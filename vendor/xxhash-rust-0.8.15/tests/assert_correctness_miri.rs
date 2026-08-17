#[cfg(all(miri, feature = "xxh64"))]
#[test]
fn assert_xxh64_miri() {
    use getrandom::getrandom;
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

        let result = xxh64(&input, SEED_1);
        hasher_1.update(&input);
        assert_eq!(hasher_1.digest(), result);

        let result = xxh64(&input, SEED_2);
        hasher_2.update(&input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset(SEED_1);
        hasher_2.reset(SEED_2);
    }
}

#[cfg(all(miri, feature = "xxh32"))]
#[test]
fn assert_xxh32_miri() {
    use getrandom::getrandom;
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

        let result = xxh32(&input, SEED_1);
        hasher_1.update(&input);
        assert_eq!(hasher_1.digest(), result);

        let result = xxh32(&input, SEED_2);
        hasher_2.update(&input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset(SEED_1);
        hasher_2.reset(SEED_2);
    }
}

#[cfg(all(miri, feature = "xxh3"))]
#[test]
fn assert_xxh3_miri() {
    use getrandom::getrandom;
    use xxhash_rust::xxh3::{xxh3_64, xxh3_128, xxh3_64_with_seed, xxh3_128_with_seed, Xxh3};

    let mut hasher_1 = Xxh3::new();
    let mut hasher_2 = Xxh3::with_seed(1);

    let mut hasher_1_128 = Xxh3::new();
    let mut hasher_2_128 = Xxh3::with_seed(1);

    let mut input = Vec::with_capacity(2048);
    for num in 0..2048 {
        input.resize(num, 1);
        println!("input(len={})", input.len());
        getrandom(&mut input).expect("getrandom");
        let input = input.as_slice();

        let result = xxh3_64(input);
        hasher_1.update(input);
        hasher_1_128.update(input);
        assert_eq!(hasher_1.digest(), result);

        let result128 = xxh3_128(input);
        assert_eq!(hasher_1_128.digest128(), result128);

        let result = xxh3_64_with_seed(input, 1);
        hasher_2.update(input);
        hasher_2_128.update(input);
        assert_eq!(hasher_2.digest(), result);

        hasher_1.reset();
        hasher_2.reset();

        let result128 = xxh3_128_with_seed(input, 1);
        assert_eq!(hasher_2_128.digest128(), result128);

        hasher_1_128.reset();
        hasher_2_128.reset();
    }
}
