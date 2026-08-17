[![Build Status](https://github.com/Xudong-Huang/generator-rs/workflows/CI/badge.svg)](https://github.com/Xudong-Huang/generator-rs/actions?query=workflow%3ACI)
[![Current Crates.io Version](https://img.shields.io/crates/v/generator.svg)](https://crates.io/crates/generator)
[![Document](https://img.shields.io/badge/doc-generator-green.svg)](https://docs.rs/generator)


# Generator-rs

rust stackful generator library

```toml
[dependencies]
generator = "0.8"
```


## Usage
```rust
use generator::{done, Gn};

fn main() {
    let g = Gn::new_scoped(|mut s| {
        let (mut a, mut b) = (0, 1);
        while b < 200 {
            std::mem::swap(&mut a, &mut b);
            b = a + b;
            s.yield_(b);
        }
        done!();
    });

    for i in g {
        println!("{}", i);
    }
}
```

## Output
```
1
2
3
5
8
13
21
34
55
89
144
233
```

## Goals

- [x] basic send/yield with message support
- [x] generator cancel support
- [x] yield_from support
- [x] panic inside generator support
- [x] stack size tune support
- [x] scoped static type support
- [x] basic coroutine interface support
- [x] stable rust support


##  based on this basic library
- we can easily port python library based on generator into rust
- coroutine framework running on multi thread


## Notices

* This crate supports below platforms, welcome to contribute with other arch and platforms

    - x86_64 Linux
    - x86_64 macOS
    - x86_64 Windows
    - x86_64 Fuchsia
    - ~~x86_64 Android~~
    - aarch64 Linux
    - aarch64 macOS
    - aarch64 Fuchsia
    - aarch64 Android
    - loongarch64 Linux
    - armv7 Linux
    - riscv64 Linux
    - powerpc64le Linux

## License

This project is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
