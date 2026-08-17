# Changelog

## 0.50.0

* Breaking change: [`winapi-rs`](https://crates.io/crates/winapi) is not maintained any more, so migrate to Microsofts [`windows-sys`](https://crates.io/crates/windows-sys) as a backend ([#48](https://github.com/gentoo90/winreg-rs/pull/48), [#51](https://github.com/gentoo90/winreg-rs/pull/51))
* Breaking change: Increase minimum supported Rust version to `1.46` since `windows-sys` doesn't compile with older versions
* Replace deprecated methods from `chrono` ([#48](https://github.com/gentoo90/winreg-rs/pull/48))

## 0.11.0

* Migrate to the 2018 edition of Rust
* Move the code from `lib.rs` to separate files
* Use [`cfg-if`](https://crates.io/crates/cfg-if) instead of `build.rs` to fail build on non-windows systems
* Reimplement deserialization logic, implement [de]serialization for byte arrays ([#49](https://github.com/gentoo90/winreg-rs/issues/49))
* Fix some typos and `clippy` warnings

## 0.10.1

* Bump the minimal required version of `winapi` to `0.3.9` (required for `load_app_key`)
* Reexport `REG_PROCESS_APPKEY` and use it in the `load_app_key` example

## 0.10.0

* Add `RegKey::load_app_key()` and `RegKey::load_app_key_with_flags()` ([#30](https://github.com/gentoo90/winreg-rs/issues/30))
* Update dev dependency `rand` to `0.8`
* Add Github actions
* Fix some clippy warnings

## 0.9.0

* Breaking change: `OsStr` and `OsString` registry values are not `NULL`-terminated any more ([#34](https://github.com/gentoo90/winreg-rs/issues/34), [#42](https://github.com/gentoo90/winreg-rs/issues/42))
* Refactoring: use macros for `ToRegValue` impls and tests for string values
* Fix `bare_trait_objects` warning in the doctests
* Add `impl ToRegValue for OsString`
* Add conversion between `REG_MULTI_SZ` and vectors of strings ([#16](https://github.com/gentoo90/winreg-rs/issues/16))
* Fix: set minimal `winapi` version to 0.3.7 (earlier versions don't have `impl-default` and `impl-debug` features which we use)
* Appveyor now checks the crate against `rust-1.31.1` too

## 0.8.0

* Implement serialization of `char` and maps
* Implement `std::fmt::Display` for `RegValue`
* Make `RegKey::{predef,raw_handle,enum_keys,enum_values}` functions `const`
* Give a better error message when compiling on platforms other than Windows ([#38](https://github.com/gentoo90/winreg-rs/pull/38))
* Tests are moved from `src/lib.rs` to `tests/reg_key.rs`

## 0.7.0

* Breaking change: remove deprecated `Error::description` ([#28](https://github.com/gentoo90/winreg-rs/pull/28))
* Optimize `Iterator::nth()` for the `Enum*` iterators ([#29](https://github.com/gentoo90/winreg-rs/pull/29))

## 0.6.2

* Add `RegKey::delete_subkey_with_flags()` ([#27](https://github.com/gentoo90/winreg-rs/pull/27))

## 0.6.1

* Add `last_write_time` field to `RegKeyMetadata` (returned by `RegKey::query_info()`) ([#25](https://github.com/gentoo90/winreg-rs/pull/25)).
* Add `get_last_write_time_system()` and `get_last_write_time_chrono()` (under `chrono` feature) methods to `RegKeyMetadata`.

## 0.6.0

* Breaking change: `create_subkey`, `create_subkey_with_flags`, `create_subkey_transacted` and
`create_subkey_transacted_with_flags` now return a tuple which contains the subkey and its disposition
which can be `REG_CREATED_NEW_KEY` or `REG_OPENED_EXISTING_KEY` ([#21](https://github.com/gentoo90/winreg-rs/issues/21)).
* Examples fixed to not use `unwrap` according to [Rust API guidelines](https://rust-lang-nursery.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark).

## 0.5.1

* Reexport `HKEY` ([#15](https://github.com/gentoo90/winreg-rs/issues/15)).
* Add `raw_handle` method ([#18](https://github.com/gentoo90/winreg-rs/pull/18)).

## 0.5.0

* Breaking change: `open_subkey` now opens a key with readonly permissions.
Use `create_subkey` or `open_subkey_with_flags` to open with read-write permissions.
* Breaking change: features `transactions` and `serialization-serde` are now disabled by default.
* Breaking change: serialization now uses `serde` instead of `rustc-serialize`.
* `winapi` updated to `0.3`.
* Documentation fixes ([#14](https://github.com/gentoo90/winreg-rs/pull/14))

## 0.4.0

* Make transactions and serialization otional features
* Update dependencies + minor fixes ([#12](https://github.com/gentoo90/winreg-rs/pull/12))

## 0.3.5

* Implement `FromRegValue` for `OsString` and `ToRegValue` for `OsStr` ([#8](https://github.com/gentoo90/winreg-rs/issues/8))
* Minor fixes

## 0.3.4

* Add `copy_tree` method to `RegKey`
* Now checked with [rust-clippy](https://github.com/Manishearth/rust-clippy)
    * no more `unwrap`s
    * replaced `to_string` with `to_owned`
* Fix: reading strings longer than 2048 characters ([#6](https://github.com/gentoo90/winreg-rs/pull/6))

## 0.3.3

* Fix: now able to read values longer than 2048 bytes ([#3](https://github.com/gentoo90/winreg-rs/pull/3))

## 0.3.2

* Fix: `FromRegValue` trait now requires `Sized` (fixes build with rust 1.4)

## 0.3.1

* Fix: bump `winapi` version to fix build

## 0.3.0

* Add transactions support and make serialization transacted
* Breaking change: use `std::io::{Error,Result}` instead of own `RegError` and `RegResult`
