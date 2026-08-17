# `bon-macros`

This is a proc-macro crate that is supposed to be a private implementation detail of the [`bon`] crate. Don't add it to your dependencies directly! The API surface of this crate is unstable, and your code may break if you use items from `bon-macros` bypassing the `bon` crate. Instead, use the proc macros from here via the reexports in the [`bon`] crate.

[`bon`]: https://docs.rs/bon/latest/bon/
