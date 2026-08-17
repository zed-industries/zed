use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Pcg32};

#[test]
fn test_lcg64xsh32_advancing() {
    for seed in 0..20 {
        let mut rng1 = Lcg64Xsh32::seed_from_u64(seed);
        let mut rng2 = rng1.clone();
        for _ in 0..20 {
            rng1.next_u32();
        }
        rng2.advance(20);
        assert_eq!(rng1, rng2);
    }
}

#[test]
fn test_lcg64xsh32_construction() {
    // Test that various construction techniques produce a working RNG.
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut rng1 = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng1.next_u64(), 1204678643940597513);

    let mut rng2 = Lcg64Xsh32::from_rng(&mut rng1).unwrap();
    assert_eq!(rng2.next_u64(), 12384929573776311845);

    let mut rng3 = Lcg64Xsh32::seed_from_u64(0);
    assert_eq!(rng3.next_u64(), 18195738587432868099);

    // This is the same as Lcg64Xsh32, so we only have a single test:
    let mut rng4 = Pcg32::seed_from_u64(0);
    assert_eq!(rng4.next_u64(), 18195738587432868099);
}

#[test]
fn test_lcg64xsh32_true_values() {
    // Numbers copied from official test suite.
    let mut rng = Lcg64Xsh32::new(42, 54);

    let mut results = [0u32; 6];
    for i in results.iter_mut() {
        *i = rng.next_u32();
    }
    let expected: [u32; 6] = [
        0xa15c02b7, 0x7b47f409, 0xba1d3330, 0x83d2f293, 0xbfa4784b, 0xcbed606e,
    ];
    assert_eq!(results, expected);
}

#[cfg(feature = "serde1")]
#[test]
fn test_lcg64xsh32_serde() {
    use bincode;
    use std::io::{BufReader, BufWriter};

    let mut rng = Lcg64Xsh32::seed_from_u64(0);

    let buf: Vec<u8> = Vec::new();
    let mut buf = BufWriter::new(buf);
    bincode::serialize_into(&mut buf, &rng).expect("Could not serialize");

    let buf = buf.into_inner().unwrap();
    let mut read = BufReader::new(&buf[..]);
    let mut deserialized: Lcg64Xsh32 =
        bincode::deserialize_from(&mut read).expect("Could not deserialize");

    for _ in 0..16 {
        assert_eq!(rng.next_u64(), deserialized.next_u64());
    }
}
