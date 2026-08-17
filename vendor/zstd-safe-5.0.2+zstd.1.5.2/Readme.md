# zstd-safe

This is a thin, no-std, safe abstraction built on top of the bindings from [zstd-sys].

It is close to a 1-for-1 mapping to the C functions, but uses rust types like slices instead of pointers and lengths.

For a more comfortable higher-level library (with `Read`/`Write` implementations), see [zstd-rs].

[zstd-rs]: https://github.com/gyscos/zstd-rs/tree/master/zstd-safe/zstd-sys
[zstd-rs]: https://github.com/gyscos/zstd-rs
