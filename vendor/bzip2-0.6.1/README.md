# bzip2

[Documentation](https://docs.rs/bzip2)

A streaming bzip2 compression/decompression library for rust.

## Features

*`bzip2-sys`*

Attempt to use the system `libbz2`. When `libbz2` is not available, built from source.
A from-source build requires a functional C toolchain for your target, and may not
work for all targets (in particular webassembly).

```sh
bzip2 = { version = "0.5.1", default-features = false, features = ["bzip2-sys"] }
```

*`static`*

Always build `libbz2` from C source, and statically link it. This flag is only meaningful when `bzip2-sys` is used,
and has no effect when the default `libbz2-rs-sys` is used as the bzip2 implementation.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
