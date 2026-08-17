# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1]

### Testing
- Added building the crate to CI

### Fixes
- Fix missing import for `no_std` builds

## [0.5.0] - 2025-01-27

### Dependencies and features
- Bump the MSRV to 1.61.0 (#1207, #1246, #1269, #1341, #1416); note that 1.60.0 may work for dependents when using `--ignore-rust-version`
- Update to `rand` v0.9.0 (#1558)
- Rename feature `serde1` to `serde` (#1477)

### API changes
- Make distributions comparable with `PartialEq` (#1218)
- `Dirichlet` now uses `const` generics, which means that its size is required at compile time (#1292)
- The `Dirichlet::new_with_size` constructor was removed (#1292)
- Add `WeightedIndexTree` (#1372, #1444)
- Add `PertBuilder` to allow specification of `mean` or `mode` (#1452)
- Rename `Zeta`'s parameter `a` to `s` (#1466)
- Mark `WeightError`, `PoissonError`, `BinomialError` as `#[non_exhaustive]` (#1480)
- Remove support for usage of `isize` as a `WeightedAliasIndex` weight (#1487)
- Change parameter type of `Zipf::new`: `n` is now floating-point (#1518)

### API changes: renames
- Move `Slice` -> `slice::Choose`, `EmptySlice` -> `slice::Empty` (#1548)
- Rename trait `DistString` -> `SampleString` (#1548)
- Rename `DistIter` -> `Iter`, `DistMap` -> `Map` (#1548)
- Move `{Weight, WeightError, WeightedIndex}` -> `weighted::{Weight, Error, WeightedIndex}` (#1548)
- Move `weighted_alias::{AliasableWeight, WeightedAliasIndex}` -> `weighted::{..}` (#1548)
- Move `weighted_tree::WeightedTreeIndex` -> `weighted::WeightedTreeIndex` (#1548)

### Testing
- Add Kolmogorov Smirnov tests for distributions (#1494, #1504, #1525, #1530)

### Fixes
- Fix Knuth's method so `Poisson` doesn't return -1.0 for small lambda (#1284)
- Fix `Poisson` distribution instantiation so it return an error if lambda is infinite (#1291)
- Fix Dirichlet sample for small alpha values to avoid NaN samples (#1209)
- Fix infinite loop in `Binomial` distribution (#1325)
- Fix `Pert` distribution where `mode` is close to `(min + max) / 2` (#1452)
- Fix panic in Binomial (#1484)
- Limit the maximal acceptable lambda for `Poisson` to solve (#1312) (#1498)
- Fix bug in `Hypergeometric`, this is a Value-breaking change (#1510)

### Other changes
- Remove unused fields from `Gamma`, `NormalInverseGaussian` and `Zipf` distributions (#1184)
  This breaks serialization compatibility with older versions.
- Add plots for `rand_distr` distributions to documentation (#1434)
- Move some of the computations in Binomial from `sample` to `new` (#1484)
- Reimplement `Poisson`'s rejection method to improve performance and correct sampling inaccuracies for large lambda values, this is a Value-breaking change (#1560)

## [0.4.3] - 2021-12-30
- Fix `no_std` build (#1208)

## [0.4.2] - 2021-09-18
- New `Zeta` and `Zipf` distributions (#1136)
- New `SkewNormal` distribution (#1149)
- New `Gumbel` and `Frechet` distributions (#1168, #1171)

## [0.4.1] - 2021-06-15
- Empirically test PDF of normal distribution (#1121)
- Correctly document `no_std` support (#1100)
- Add `std_math` feature to prefer `std` over `libm` for floating point math (#1100)
- Add mean and std_dev accessors to Normal (#1114)
- Make sure all distributions and their error types implement `Error`, `Display`, `Clone`,
 `Copy`, `PartialEq` and `Eq` as appropriate (#1126)
- Port benchmarks to use Criterion crate (#1116)
- Support serde for distributions (#1141)

## [0.4.0] - 2020-12-18
- Bump `rand` to v0.8.0
- New `Geometric`, `StandardGeometric` and `Hypergeometric` distributions (#1062)
- New `Beta` sampling algorithm for improved performance and accuracy (#1000)
- `Normal` and `LogNormal` now support `from_mean_cv` and `from_zscore` (#1044)
- Variants of `NormalError` changed (#1044)

## [0.3.0] - 2020-08-25
- Move alias method for `WeightedIndex` from `rand` (#945)
- Rename `WeightedIndex` to `WeightedAliasIndex` (#1008)
- Replace custom `Float` trait with `num-traits::Float` (#987)
- Enable `no_std` support via `num-traits` math functions (#987)
- Remove `Distribution<u64>` impl for `Poisson` (#987)
- Tweak `Dirichlet` and `alias_method` to use boxed slice instead of `Vec` (#987)
- Use whitelist for package contents, reducing size by 5kb (#983)
- Add case `lambda = 0` in the parametrization of `Exp` (#972)
- Implement inverse Gaussian distribution (#954)
- Reformatting and use of `rustfmt::skip` (#926)
- All error types now implement `std::error::Error` (#919)
- Re-exported `rand::distributions::BernoulliError` (#919)
- Add value stability tests for distributions (#891)

## [0.2.2] - 2019-09-10
- Fix version requirement on rand lib (#847)
- Clippy fixes & suppression (#840)

## [0.2.1] - 2019-06-29
- Update dependency to support Rand 0.7
- Doc link fixes

## [0.2.0] - 2019-06-06
- Remove `new` constructors for zero-sized types
- Add Pert distribution
- Fix undefined behavior in `Poisson`
- Make all distributions return `Result`s instead of panicking
- Implement `f32` support for most distributions
- Rename `UnitSphereSurface` to `UnitSphere`
- Implement `UnitBall` and `UnitDisc`

## [0.1.0] - 2019-06-06
Initial release. This is equivalent to the code in `rand` 0.6.5.
