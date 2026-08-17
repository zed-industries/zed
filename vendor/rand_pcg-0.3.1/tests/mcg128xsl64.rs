use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};

#[test]
fn test_mcg128xsl64_advancing() {
    for seed in 0..20 {
        let mut rng1 = Mcg128Xsl64::seed_from_u64(seed);
        let mut rng2 = rng1.clone();
        for _ in 0..20 {
            rng1.next_u64();
        }
        rng2.advance(20);
        assert_eq!(rng1, rng2);
    }
}

#[test]
fn test_mcg128xsl64_construction() {
    // Test that various construction techniques produce a working RNG.
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut rng1 = Mcg128Xsl64::from_seed(seed);
    assert_eq!(rng1.next_u64(), 7071994460355047496);

    let mut rng2 = Mcg128Xsl64::from_rng(&mut rng1).unwrap();
    assert_eq!(rng2.next_u64(), 12300796107712034932);

    let mut rng3 = Mcg128Xsl64::seed_from_u64(0);
    assert_eq!(rng3.next_u64(), 6198063878555692194);

    // This is the same as Mcg128Xsl64, so we only have a single test:
    let mut rng4 = Pcg64Mcg::seed_from_u64(0);
    assert_eq!(rng4.next_u64(), 6198063878555692194);
}

#[test]
fn test_mcg128xsl64_true_values() {
    // Numbers copied from official test suite (C version).
    let mut rng = Mcg128Xsl64::new(42);

    let mut results = [0u64; 6];
    for i in results.iter_mut() {
        *i = rng.next_u64();
    }
    let expected: [u64; 6] = [
        0x63b4a3a813ce700a,
        0x382954200617ab24,
        0xa7fd85ae3fe950ce,
        0xd715286aa2887737,
        0x60c92fee2e59f32c,
        0x84c4e96beff30017,
    ];
    assert_eq!(results, expected);
}

#[cfg(feature = "serde1")]
#[test]
fn test_mcg128xsl64_serde() {
    use bincode;
    use std::io::{BufReader, BufWriter};

    let mut rng = Mcg128Xsl64::seed_from_u64(0);

    let buf: Vec<u8> = Vec::new();
    let mut buf = BufWriter::new(buf);
    bincode::serialize_into(&mut buf, &rng).expect("Could not serialize");

    let buf = buf.into_inner().unwrap();
    let mut read = BufReader::new(&buf[..]);
    let mut deserialized: Mcg128Xsl64 =
        bincode::deserialize_from(&mut read).expect("Could not deserialize");

    for _ in 0..16 {
        assert_eq!(rng.next_u64(), deserialized.next_u64());
    }
}
