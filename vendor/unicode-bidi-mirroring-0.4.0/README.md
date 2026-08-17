## unicode-bidi-mirroring
[![Crates.io](https://img.shields.io/crates/v/unicode-bidi-mirroring.svg)](https://crates.io/crates/unicode-bidi-mirroring)
[![Documentation](https://docs.rs/unicode-bidi-mirroring/badge.svg)](https://docs.rs/unicode-bidi-mirroring)

This library implements
[Unicode Bidi Mirroring](https://unicode.org/reports/tr44/#BidiMirroring.txt) property detection.

```rust
use unicode_bidi_mirroring::*;

assert_eq!(get_mirrored('A'), None);
assert_eq!(get_mirrored('\u{2039}'), Some('\u{203A}'));
assert_eq!(get_mirrored('\u{203A}'), Some('\u{2039}'));

assert_eq!(is_mirroring('A'), false);
assert_eq!(is_mirroring('\u{29C4}'), true);
assert_eq!(is_mirroring('\u{22FF}'), true);
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
