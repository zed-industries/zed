use std::iter;

use criterion::*;

use fastrand::Rng;

use phf_generator::generate_hash;

fn gen_vec(len: usize) -> Vec<String> {
    let mut rng = Rng::with_seed(0xAAAAAAAAAAAAAAAA);
    let mut chars = iter::repeat_with(|| rng.alphanumeric());

    (0..len)
        .map(move |_| chars.by_ref().take(64).collect::<String>())
        .collect()
}

fn main() {
    let data = black_box(gen_vec(1_000_000));
    black_box(generate_hash(&data));
}
