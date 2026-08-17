## unicode-vo
[![Crates.io](https://img.shields.io/crates/v/unicode-vo.svg)](https://crates.io/crates/unicode-vo)
[![Documentation](https://docs.rs/unicode-vo/badge.svg)](https://docs.rs/unicode-vo)

This library implements
[Unicode Vertical_Orientation Property](https://www.unicode.org/reports/tr50/tr50-19.html)
(annex #50).

```rust
use unicode_vo::*;

assert_eq!(char_orientation('A'), Orientation::Rotated);
assert_eq!(char_orientation('本'), Orientation::Upright);
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
