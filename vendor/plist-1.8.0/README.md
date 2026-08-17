# Plist

A rusty plist parser.

Many features from previous versions are now hidden behind the `enable_unstable_features_that_may_break_with_minor_version_bumps` feature. These will break in minor version releases after the 1.0 release. If you really really must use them you should specify a tilde requirement e.g. `plist = "~1.0.3"` in you `Cargo.toml` so that the plist crate is not automatically updated to version 1.1.

[Documentation](https://docs.rs/plist/)
