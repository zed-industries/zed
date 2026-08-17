# num-cmp

The **[`NumCmp`](./trait.NumCmp.html)** trait for comparison between differently typed numbers.

```rust
use std::f32;
use std::cmp::Ordering;
use num_cmp::NumCmp;

assert!(NumCmp::num_eq(3u64, 3.0f32));
assert!(NumCmp::num_lt(-4.7f64, -4i8));
assert!(!NumCmp::num_ge(-3i8, 1u16));

// 40_000_000 can be exactly represented in f32, 40_000_001 cannot
assert_eq!(NumCmp::num_cmp(40_000_000.0f32, 40_000_000u32), Some(Ordering::Equal));
assert_ne!(NumCmp::num_cmp(40_000_001.0f32, 40_000_001u32), Some(Ordering::Equal));
assert_eq!(NumCmp::num_cmp(f32::NAN,        40_000_002u32), None);
```

The `i128` Cargo feature can be enabled in nightly
to get support for `i128` and `u128` types as well,
which is being implemented in [Rust issue #35118][issue-35118].

[issue-35118]: https://github.com/rust-lang/rust/issues/35118

