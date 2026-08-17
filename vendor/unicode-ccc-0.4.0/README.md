## unicode-ccc
[![Crates.io](https://img.shields.io/crates/v/unicode-ccc.svg)](https://crates.io/crates/unicode-ccc)
[![Documentation](https://docs.rs/unicode-ccc/badge.svg)](https://docs.rs/unicode-ccc)

This library implements
[Unicode Canonical Combining Class](https://unicode.org/reports/tr44/#Canonical_Combining_Class_Values) detection.

```rust
use unicode_ccc::*;

assert_eq!(get_canonical_combining_class('A'), CanonicalCombiningClass::NotReordered);
assert_eq!(get_canonical_combining_class('\u{0A3C}'), CanonicalCombiningClass::Nukta);
assert_eq!(get_canonical_combining_class('\u{18A9}'), CanonicalCombiningClass::AboveLeft);
```

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
