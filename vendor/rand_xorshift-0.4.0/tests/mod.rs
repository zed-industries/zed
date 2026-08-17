use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

#[test]
fn test_xorshift_construction() {
    // Test that various construction techniques produce a working RNG.
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut rng1 = XorShiftRng::from_seed(seed);
    assert_eq!(rng1.next_u64(), 4325440999699518727);

    let mut rng2 = XorShiftRng::from_rng(&mut rng1);
    // Yes, this makes rng2 a clone of rng1!
    assert_eq!(rng1.next_u64(), 15614385950550801700);
    assert_eq!(rng2.next_u64(), 15614385950550801700);
}

#[test]
fn test_xorshift_true_values() {
    let seed = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut rng = XorShiftRng::from_seed(seed);

    let mut results = [0u32; 9];
    for i in results.iter_mut() {
        *i = rng.next_u32();
    }
    let expected: [u32; 9] = [
        2081028795, 620940381, 269070770, 16943764, 854422573, 29242889, 1550291885, 1227154591,
        271695242,
    ];
    assert_eq!(results, expected);

    let mut results = [0u64; 9];
    for i in results.iter_mut() {
        *i = rng.next_u64();
    }
    let expected: [u64; 9] = [
        9247529084182843387,
        8321512596129439293,
        14104136531997710878,
        6848554330849612046,
        343577296533772213,
        17828467390962600268,
        9847333257685787782,
        7717352744383350108,
        1133407547287910111,
    ];
    assert_eq!(results, expected);

    let mut results = [0u8; 32];
    rng.fill_bytes(&mut results);
    let expected = [
        102, 57, 212, 16, 233, 130, 49, 183, 158, 187, 44, 203, 63, 149, 45, 17, 117, 129, 131,
        160, 70, 121, 158, 155, 224, 209, 192, 53, 10, 62, 57, 72,
    ];
    assert_eq!(results, expected);
}

#[test]
fn test_xorshift_zero_seed() {
    // Xorshift does not work with an all zero seed.
    // Assert it does not panic.
    let seed = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut rng = XorShiftRng::from_seed(seed);
    let a = rng.next_u64();
    let b = rng.next_u64();
    assert!(a != 0);
    assert!(b != a);
}

#[test]
fn test_xorshift_clone() {
    let seed = [1, 2, 3, 4, 5, 5, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut rng1 = XorShiftRng::from_seed(seed);
    let mut rng2 = rng1.clone();
    for _ in 0..16 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[cfg(feature = "serde")]
#[test]
fn test_xorshift_serde() {
    use bincode;
    use std::io::{BufReader, BufWriter};

    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut rng = XorShiftRng::from_seed(seed);

    let buf: Vec<u8> = Vec::new();
    let mut buf = BufWriter::new(buf);
    bincode::serialize_into(&mut buf, &rng).expect("Could not serialize");

    let buf = buf.into_inner().unwrap();
    let mut read = BufReader::new(&buf[..]);
    let mut deserialized: XorShiftRng =
        bincode::deserialize_from(&mut read).expect("Could not deserialize");

    for _ in 0..16 {
        assert_eq!(rng.next_u64(), deserialized.next_u64());
    }
}
