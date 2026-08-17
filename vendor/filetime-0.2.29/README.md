# filetime

[Documentation](https://docs.rs/filetime)

> **Note**: this library is quite old and superseded by functionality in the
> standard library. Nowadays you shouldn't use this crate and you should use
> [`FileTimes`] and [`File::set_times`] instead.

[`FileTimes`]: https://doc.rust-lang.org/stable/std/fs/struct.FileTimes.html
[`File::set_times`]: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_times

A helper library for inspecting and setting the various timestamps of files in
Rust. This library takes into account cross-platform differences in terms of
where the timestamps are located, what they are called, and how to convert them
into a platform-independent representation.

```toml
# Cargo.toml
[dependencies]
filetime = "0.2"
```

# Advantages over using `std::fs::Metadata`

This library, when built with `RUSTFLAGS=--cfg emulate_second_only_system` set,
will return all times rounded down to the second. This emulates the behavior of
some file systems, mostly [HFS](https://en.wikipedia.org/wiki/HFS_Plus),
allowing debugging on other hardware.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Filetime by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
