# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2021-06-15
- Add `advance` methods to RNGs (#1111)
- Document dependencies between streams (#1122)

## [0.3.0] - 2020-12-08
- Bump `rand_core` version to 0.6.0
- Bump MSRV to 1.36 (#1011)
- Derive PartialEq+Eq for Lcg64Xsh32, Lcg128Xsl64, and Mcg128Xsl64 (#979)

## [0.2.1] - 2019-10-22
- Bump `bincode` version to 1.1.4 to fix minimal-dependency builds
- Removed unused `autocfg` build dependency.

## [0.2.0] - 2019-06-12
- Add `Lcg128Xsl64` aka `Pcg64`
- Bump minor crate version since rand_core bump is a breaking change
- Switch to Edition 2018

## [0.1.2] - 2019-02-23
- require `bincode` 1.1.2 for i128 auto-detection
- make `bincode` a dev-dependency again #663
- clean up tests and Serde support

## [0.1.1] - 2018-10-04
- make `bincode` an explicit dependency when using Serde

## [0.1.0] - 2018-10-04
Initial release, including:

- `Lcg64Xsh32` aka `Pcg32`
- `Mcg128Xsl64` aka `Pcg64Mcg`
