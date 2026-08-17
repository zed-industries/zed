# crc

Rust implementation of CRC.

[![ci](https://github.com/mrhooray/crc-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/mrhooray/crc-rs/actions/workflows/ci.yaml)
[![Crate](https://img.shields.io/crates/v/crc.svg)](https://crates.io/crates/crc)
[![Docs](https://docs.rs/crc/badge.svg)](https://docs.rs/crc)
[![License](https://img.shields.io/crates/l/crc.svg?maxAge=2592000)](https://github.com/mrhooray/crc-rs#license)

### Usage

Add `crc` to `Cargo.toml`
```toml
[dependencies]
crc = "3.3.0"
```

### Examples

Using a well-known algorithm:
```rust
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
assert_eq!(X25.checksum(b"123456789"), 0x906e);
```

Using a custom algorithm:
```rust
const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
```

### Minimum supported Rust version (MSRV)

This crate's MSRV is 1.65.

At a minimum, the MSRV will be <= the oldest stable release in the last 12 months. MSRV may be bumped in minor version releases.

### Implementations

This crate has several pluggable implementations:

1. `NoTable` doesn't use a lookup table, and thus minimizes binary size and memory usage.
2. `Table<1>` uses a lookup table with 256 entries (e.g. for u32 thats 256 * 4 bytes).
3. `Table<16>` uses a lookup table with 16 * 256 entries (e.g. for u32 thats 16 * 256 * 4 bytes).

`Table<1>` is the default implementation, but this can be overridden by specifying `I` in `Crc<W, I>`. E.g.: `Crc<u32, NoTable>`, `Crc<u64, Table<16>>`, ...

NOTE: Lookup tables will increase binary size if they're generated at compile-time. Wrapping `Crc` initialization in a `std::cell::OnceCell` may be preferable if binary size is a concern.

### Benchmark

`cargo bench` with AMD Ryzen 7 3800X ([comparison](http://create.stephan-brumme.com/crc32/)).

#### Throughput (GiB/s)

| Width | NoTable | Bytewise | Slice16 |
|-------|---------|----------|---------|
| 8     | 0.113   | 0.585    | 3.11    |
| 16    | 0.105   | 0.483    | 3.23    |
| 32    | 0.111   | 0.516    | 3.30    |
| 64    | 0.139   | 0.517    | 2.92    |
| 82    | 0.091   | 0.438    | 0.623   |

### License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
