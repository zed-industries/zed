# human_bytes

> A Rust crate & cli to convert bytes into human-readable values.

[![License](https://img.shields.io/crates/l/human_bytes?style=flat-square)](./LICENSE)
[![Latest version](https://img.shields.io/crates/v/human_bytes?style=flat-square)](https://crates.io/crates/human_bytes)
[![Build status](https://img.shields.io/gitlab/pipeline/nkeor/human_bytes-rs?style=flat-square)]()

It can return either KiB/MiB/GiB/TiB or KB/MB/GB/TB by disabling the `si-units` feature.

> 1 KiB = 1024 B, 1 KB = 1000 B

It supports from 0 bytes to several yottabytes (I cannot tell how many because I have to use `u128`s
to fit a single YB)

## Usage

### As a CLI

* (Optional) Install [Just](https://just.systems/)
* Build:
  - With just: `just build-binary`
  - Plain cargo: `cargo build --release --features 'build-binary fast' --bin hb`
* Copy `target/release/hb` to somewhere in your `$PATH`
* Run `hb <bytes>` or `echo <bytes> | hb`

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
human_bytes = "0.4"
# or, to disable the SI Units:
human_bytes = { version = "0.4", default-features = false }
```

And then

```rust
use human_bytes::human_bytes;

assert_eq!(human_bytes(563_200_u32), "550 KiB".to_string());
// or
assert_eq!(human_bytes(563_200_u64 as f64), "550 KiB".to_string());
// ________________________________/
// |
// | Needed only when you're using `u64` values,
// | because `f64` doesn't implement `std::convert::From<u64>`

// With the `si-units` feature disabled:
assert_eq!(human_bytes(550_000_u32), "550 KB".to_string());
```

The crate is dependency-free, but you can boost the speed by enabling the `fast` feature,
which switches from using `std::format!` to [ryu](https://github.com/dtolnay/ryu)
to convert floats to strings.

```toml
[dependencies]
human_bytes = { version = "0.4", features = ["fast"] }
```

## About
The code is based on a PHP function I found [here](https://math.stackexchange.com/questions/247444/explain-convertion-algorithm-from-bytes-to-kb-mb-gb).

It is useful because you don't have to provide a prefix, it does it on its own.
It'll always return `1 MiB` instead of `1024 KiB`

It has some tests I wrote to check that the conversion is correct, and it returns decimals (e.g. `16.5 GiB`)

## Changelog
Check the [CHANGELOG.md](./CHANGELOG.md)

## License
[BSD 2-clause](./LICENSE) (c) 2020-2022 Namkhai B.
