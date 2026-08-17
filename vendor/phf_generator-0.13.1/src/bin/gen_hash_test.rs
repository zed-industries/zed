use std::hint::black_box;
use std::iter;

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
    let data_refs: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    black_box(generate_hash(&data_refs));
}
