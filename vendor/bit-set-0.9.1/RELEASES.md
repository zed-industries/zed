Version 0.10.0 (not yet released)
========================================================

<a id="v0.10.0"></a>

Version 0.9.1
========================================================

<a id="v0.9.1"></a>

- Fixed serde feature

Version 0.9.0
========================================================

<a id="v0.9.0"></a>

- Minimal Supported Rust Version is 1.82
- Rust edition 2021 is used
- implemented `fn make_empty`
- implemented `fn reset`
- added general initialization functions: `fn new_general`, `fn from_bit_vec_general`, `fn with_capacity_general`, `fn from_bytes_general`

Version 0.8.0
========================================================

<a id="v0.8.0"></a>

Version 0.7.0 (ZERO BREAKING CHANGES)
========================================================

<a id="v0.7.0"></a>

- `serde::Serialize`, `Deserialize` is derived under the `serde` optional feature
- `impl Display` is implemented
- `impl Debug` has different output (we do not promise stable `Debug` output)
- `fn truncate` is implemented
- `fn get_mut` is implemented
