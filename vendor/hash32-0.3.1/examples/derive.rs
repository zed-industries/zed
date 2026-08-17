extern crate hash32;

use std::hash::Hash;

use hash32::{FnvHasher, Hasher};

#[derive(Hash)]
struct Led {
    state: bool,
}

#[derive(Hash)]
struct Ipv4Addr([u8; 4]);

#[derive(Hash)]
struct Generic<T> {
    inner: T,
}

fn main() {
    let mut fnv = FnvHasher::default();
    Led { state: true }.hash(&mut fnv);
    Generic { inner: 0 }.hash(&mut fnv);
    Ipv4Addr([127, 0, 0, 1]).hash(&mut fnv);
    println!("{}", fnv.finish32())
}
