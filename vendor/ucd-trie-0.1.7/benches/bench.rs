#![feature(test)]

extern crate test;

use once_cell::sync::Lazy;
use ucd_trie::TrieSetOwned;

#[bench]
fn bench_trie_set(b: &mut test::Bencher) {
    const CHARS: &'static [char] = &['a', 'β', '☃', '😼'];
    // const CHARS: &'static [char] = &['a'];
    static SET: Lazy<TrieSetOwned> =
        Lazy::new(|| TrieSetOwned::from_scalars(CHARS).unwrap());

    let set = Lazy::force(&SET);
    let mut i = 0;
    b.iter(|| {
        let c = CHARS[i];
        i = (i + 1) % CHARS.len();

        for _ in 0..10000 {
            assert!(set.contains_char(c));
        }
    });
}
