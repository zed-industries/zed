# Stability and SemVer

`async-std` follows https://semver.org/.

In short: we are versioning our software as `MAJOR.MINOR.PATCH`. We increase the:

* MAJOR version when there are incompatible API changes,
* MINOR version when we introduce functionality in a backwards-compatible manner
* PATCH version when we make backwards-compatible bug fixes

We will provide migration documentation between major versions.

## Future expectations

`async-std` uses its own implementations of the following concepts:

* `Read`
* `Write`
* `Seek`
* `BufRead`
* `Stream`

For integration with the ecosystem, all types implementing these traits also have an implementation of the corresponding interfaces in the `futures-rs` library.
Please note that our SemVer guarantees don't extend to usage of those interfaces. We expect those to be conservatively updated and in lockstep.

## Minimum version policy

The current tentative policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if `async-std` 1.0 requires Rust 1.37.0, then `async-std` 1.0.z for all values of z will also require Rust 1.37.0 or newer. However, `async-std` 1.y for y > 0 may require a newer minimum version of Rust.

In general, this crate will be conservative with respect to the minimum supported version of Rust. With `async/await` being a new feature though, we will track changes in a measured pace initially.

## Security fixes

Security fixes will be applied to _all_ minor branches of this library in all _supported_ major revisions. This policy might change in the future, in which case we give a notice at least _3 months_ ahead.

## Credits

This policy is based on [BurntSushi's regex crate][regex-policy].

[regex-policy]: https://github.com/rust-lang/regex#minimum-rust-version-policy
