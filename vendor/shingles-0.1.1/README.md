shingles.rs
====

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/dimack/shingles.rs/master/LICENSE)

>  Shingles implementation in rust

See docs [(0.1 / master)](https://dimack.github.io/shingles.rs/0.1/shingles/)

## Overview
**Shingles** is a crate for constructing shingles ("tokenizing") from slices and utf-8 strings.
It was primary created to use in fuzzy matching algorithms like minhash or similar.

## Examples
```rust
extern crate shingles;

use shingles::AsShingles;

fn main() {
    let v = [1, 2, 3, 4];
    let mut num_sh = v.as_shingles(3);
    let mut str_sh = "привет!".as_shingles_with_step(4, 2);
    
    assert_eq!(Some(&v[0..3]), num_sh.next());
    assert_eq!(Some(&v[1..4]), num_sh.next());
    
    assert_eq!(Some("прив"), str_sh.next());
    assert_eq!(Some("ивет"), str_sh.next());
    
    for h in "привет!".as_shingles(4).hashes() {
        // prints hash for each shingle
        println!("{}", h);
    }
}
```

## 2D shingle examples
```rust
extern crate shingles;

use shingles::AsShingles2D;

fn main() {
    let v: Vec<_> = "abcd\n\
                     efgh\n\
                     ijkl"
        .split_terminator("\n")
        .collect();

    let mut sh_2d = v.as_shingles_2d([3, 3]);

    assert_eq!(
        Some(vec![&v[0][0..3], &v[1][0..3], &v[2][0..3]]),
        sh_2d.next()
    );

    // You can easily get hashes from 2D-shingles
    for h in v.as_shingles_2d([3, 3]).hashes() {
        // print u64 hash value for each 2D-shingle
        println!("{}", h);
    }
}
```