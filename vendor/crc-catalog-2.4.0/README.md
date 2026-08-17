# crc-catalog

[![rust](https://github.com/akhilles/crc-catalog/actions/workflows/rust.yaml/badge.svg)](https://github.com/akhilles/crc-catalog/actions/workflows/rust.yaml)
[![Crate](https://img.shields.io/crates/v/crc-catalog.svg)](https://crates.io/crates/crc-catalog)
[![Docs](https://docs.rs/crc-catalog/badge.svg)](https://docs.rs/crc-catalog)

MSRV is 1.46.

Catalog of CRC algorithms expressed as simple Rust structs. The Rust code is generated from the "[Catalogue of parametrised CRC algorithms](http://reveng.sourceforge.net/crc-catalogue)" using the `generate_catalog.sh` script:

```
$ ./generate_catalog.sh > src/algorithm.rs
```

## License

Licensed under either of

 * [Apache License, Version 2.0](LICENSES/Apache-2.0.txt)
 * [MIT License](LICENSES/MIT.txt)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
