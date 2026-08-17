extern crate seahash;
use seahash::SeaHasher as H;

use std::hash::Hasher;

#[test]
fn hash_chunking_vs_not() {
    // originally from https://gitlab.redox-os.org/redox-os/seahash/issues/5
    let c1: &[u8] = b"This hashing algorithm was extracted from the Rustc compiler.";
    let c2: &[u8] =
        b" This is the same hashing algoirthm used for some internal operations in FireFox.";
    let c3: &[u8] = b" The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one byte at a time.";

    let mut h1 = H::default();
    h1.write(c1);
    h1.write(c2);
    h1.write(c3);
    let hash1 = h1.finish();

    let mut c4 = Vec::<u8>::new();
    c4.extend_from_slice(c1);
    c4.extend_from_slice(c2);
    c4.extend_from_slice(c3);

    let mut h2 = H::default();
    h2.write(&c4);
    let hash2 = h2.finish();

    let reference = seahash::reference::hash(&c4);
    let buffer = seahash::hash(&c4);

    println!("hash1: {:016x}", hash1);
    println!("hash2: {:016x}", hash2);
    println!("ref  : {:016x}", reference);
    println!("buf  : {:016x}", buffer);

    assert_eq!(hash1, hash2);
    assert_eq!(hash1, reference);
    assert_eq!(hash1, buffer);
    assert_eq!(hash1, 0xa06e72e1b06144a0);
}

#[test]
fn test_different_chunk_sizes() {
    let v = {
        let c1: &[u8] = b"This hashing algorithm was extracted from the Rustc compiler.";
        let c2: &[u8] =
            b" This is the same hashing algoirthm used for some internal operations in FireFox.";
        let c3: &[u8] = b" The strength of this algorithm is in hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one byte at a time.";

        [c1, c2, c3].concat()
    };

    let mut h1 = H::default();
    h1.write(&v);
    let h1 = h1.finish();

    for chunk_len in 1..v.len() {
        let mut h2 = H::default();
        for w in v.chunks(chunk_len) {
            h2.write(w);
        }
        let h2 = h2.finish();

        assert_eq!(h1, h2, "failed with chunk_len={}", chunk_len);
    }
}
