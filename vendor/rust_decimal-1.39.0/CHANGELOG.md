# Version History

**Please see [Github Releases](https://github.com/paupino/rust-decimal/releases) for version history going forward.**

# 1.36.0

### Fixed

* Fixes a bug where `from_sql` was converting `Numeric::NaN` to `0` rather than returning an
  error. ([#656](https://github.com/paupino/rust-decimal/pull/656)).
* Fixes an issue where `i128::MIN` would cause overflow when calling
  `from_i128` ([#657](https://github.com/paupino/rust-decimal/pull/657)).
* Fixes issue with `abs_sub` whereby it was not returning the
  difference ([#666](https://github.com/paupino/rust-decimal/pull/666)).
* Fixes issue with `serde_str` after downstream dependency
  update ([#670](https://github.com/paupino/rust-decimal/pull/670)).

### Changed

* Github workflow improvements ([#658](https://github.com/paupino/rust-decimal/pull/658)).
* Diesel 1 feature has been deprecated and Diesel 2 was upgraded to
  v2.2 ([#663](https://github.com/paupino/rust-decimal/pull/663)).
* Replace `proptest` strategy to improve generation of random
  values ([#659](https://github.com/paupino/rust-decimal/pull/659)).

# 1.35.0

### Fixed

* Fixes an issue where reading from Postgres may cause a panic instead of raising an
  Error ([#653](https://github.com/paupino/rust-decimal/pull/653))

### Changed

* Reduces PostgreSQL import dependencies by only requiring the common `postgres-types`
  crate. ([#651](https://github.com/paupino/rust-decimal/pull/651))

### Credit

Thank you to [@the-kenny](https://github.com/the-kenny) for your contribution!

## 1.34.3

### Fixed

* Fixes an issue where when hydrating an underflowing `NUMERIC` value from PostgreSQL it would sometimes generate an
  invalid
  Decimal ([#647](https://github.com/paupino/rust-decimal/pull/647))

## 1.34.2

### Fixed

* Deprecate `macros` feature until circular dependency issue is resolved properly.

## 1.34.1

### Fixed

* Hotfix for circular dependency when using the `macros` feature.

## 1.34.0

### Added

* `rust_decimal_macros` can now be utilized using the `macros` feature
  flag. ([#628](https://github.com/paupino/rust-decimal/pull/628))

### Fixed

* Reimplement `pow` function to more effectively handle larger
  exponents ([#638](https://github.com/paupino/rust-decimal/pull/638))
* Fixes error handling when parsing a `Decimal` from a string when a decimal point or placeholder is at the rounding
  position ([#636](https://github.com/paupino/rust-decimal/pull/636))

### Changed

* Added documentation for serde features as well as a few
  examples. ([#630](https://github.com/paupino/rust-decimal/pull/630))
* Documentation and rustdoc generation
  improvements ([#633](https://github.com/paupino/rust-decimal/pull/633), [#634](https://github.com/paupino/rust-decimal/pull/634))

### Credit

Thank you to [@robjtede](https://github.com/robjtede) and [@schungx](https://github.com/schungx) for your contributions
this release.

Last but not least, a special thank you to [@Tony-Samuels](https://github.com/Tony-Samuels) for your help with
managing `rust_decimal`!

## 1.33.1

### Fixed

* Fixes an issue when converting from a `Decimal` to a float whereby the float would be inaccurate after
  rounding. ([#625](https://github.com/paupino/rust-decimal/pull/625))

## 1.33.0

### Fixed

* Fixes an issue when adding/subtracting two `Decimal`s when one has a scale of
    29. ([#619](https://github.com/paupino/rust-decimal/pull/619))
* An empty string will be parsed as `None` during JSON deserialization instead of
  panicking. ([#607](https://github.com/paupino/rust-decimal/pull/607))

### Changed

* Upgrades `borsh` to version `1.1` as a result of
  a [security advisory](https://rustsec.org/advisories/RUSTSEC-2023-0033.html). ([#621](https://github.com/paupino/rust-decimal/pull/621))

### Credit

Thank you to [@gai6948](https://github.com/gai6948) for their contribution! Also thank you to all of those that pushed
for the security advisory changes.

## 1.32.0

### Fixed

* Fixes an issue with `is_integer` returning incorrect results for mantissa's 10^n where n >=
    10. ([#605](https://github.com/paupino/rust-decimal/pull/605))

### Changed

* `byteorder` is no longer required as a dependency for the postgres
  feature. ([#603](https://github.com/paupino/rust-decimal/pull/603))

### Credit

Thank you to [@psychon](https://github.com/psychon) for your contribution!

## 1.31.0

### Fixed

* Fixes an issue with `trunc_with_scale` implicitly rounding in some
  scenarios ([#600](https://github.com/paupino/rust-decimal/pull/600))

### Changed

* Various dependency features were updated.

### Credit

Thank you to [@mkatychev](https://github.com/mkatychev) for your contribution this release.

## 1.30.0

As the minor releases for Rust Decimal are getting smaller, I'll be looking at formally starting version 2 of the
Decimal library. This will be unstable as we find experiment with a new API and address some of the constraints the
current
implementation of the library has (e.g. revised error types for better const support).

### Added

* Add `proptest` feature support for `Decimal` ([#582](https://github.com/paupino/rust-decimal/pull/582)).
* Implement an `is_integer` function for an efficient means to check whether a `Decimal` is an integer with no
  fractional portion. ([#591](https://github.com/paupino/rust-decimal/pull/591)).

### Changed

* Improved GitHub Actions Workflow ([#592](https://github.com/paupino/rust-decimal/pull/592)).

### Credit

Thank you to [@cardoe](https://github.com/cardoe/) for your contribution!

## 1.29.1

### Fixed

* Fixes an issue with `checked_cos` and `checked_sin` where `QUARTER_PI` would send the calculation into an infinite
  loop. (#[585](https://github.com/paupino/rust-decimal/pull/585))

## 1.29.0

### Added

* New `trunc_with_scale` function which allows you to truncate a decimal number whilst retaining a specified
  precision. ([#576](https://github.com/paupino/rust-decimal/pull/576)), ([#578](https://github.com/paupino/rust-decimal/pull/578))

### Fixed

* Fix issue when rounding using a high precision decimal place on zero values. This would previously allow the `Decimal`
  number to enter into an invalid state. ([#575](https://github.com/paupino/rust-decimal/pull/575))

### Changed

There were also a couple of housekeeping tasks, to help pave the way for v2 of Rust Decimal.

* Clean up features so that they are explicit in the `cargo.toml` and leverage the new `dep:` syntax where
  required. ([#579](https://github.com/paupino/rust-decimal/pull/579))
* Restructure and compartmentalize Makefile. ([#580](https://github.com/paupino/rust-decimal/pull/580))

### Credit

Thank you [@c410-f3r](https://github.com/c410-f3r) and [@jon-chuang](https://github.com/jon-chuang) for your
contributions to this release!

## 1.28.1

### Fixed

* Bumped `borsh` and `criterion` dependencies to the latest
  versions. ([#568](https://github.com/paupino/rust-decimal/pull/568))
* Removed erroneous debug statements in `serde-with-str`
  feature. ([#571](https://github.com/paupino/rust-decimal/pull/571))

Thanks [@attila-lin](https://github.com/attila-lin) for your help bumping dependencies.

## 1.28.0

### Added

* Implement `TryFrom<&str>` for `Decimal` ([#560](https://github.com/paupino/rust-decimal/pull/560))

### Fixed

* Explicit string deserialize for `Option<Decimal>` when
  using `serde-with-str` ([#565](https://github.com/paupino/rust-decimal/pull/565))
* Fix for `rescale` preventing `Decimal::ZERO` to be rescaled to an invalid
  precision ([#566](https://github.com/paupino/rust-decimal/pull/566))

### Credit

Thank you to [@c410-f3r](https://github.com/c410-f3r) for your diligent help adding features to this library!

## 1.27.0

### Added

* Added `Copy`/`Clone` traits for `rkvy::Archive` ([#539](https://github.com/paupino/rust-decimal/pull/539))
* Support precision for scientific notation
  formatting ([#555](https://github.com/paupino/rust-decimal/pull/555), [#557](https://github.com/paupino/rust-decimal/pull/557))

### Fixed

* Fixed edge case parsing of strings whereby it would incorrectly error instead of
  round ([#541](https://github.com/paupino/rust-decimal/pull/541))

### Changed

* Update diesel feature to use release version ([#546](https://github.com/paupino/rust-decimal/pull/546))
* Updated documentation to include `cargo-edit` instructions ([#538](https://github.com/paupino/rust-decimal/pull/538))
* Start using new feature resolver for cargo dependencies ([#551](https://github.com/paupino/rust-decimal/pull/551))
* Optional serde deserialization avoids using `deserialize_any` except in explicit use
  cases ([#558](https://github.com/paupino/rust-decimal/pull/558))

### Credit

This release wouldn't have been possible without the help of the community. Special call out and thank you
to [@icy-ux](https://github.com/icy-ux), [@CAD97](https://github.com/CAD97), [@nicksenger](https://github.com/nicksenger),
[@c410-f3r](https://github.com/c410-f3r) and [@yongqli](https://github.com/yongqli).

## 1.26.1

### Fixed

* Fixes `docs.rs` [failing to compile](https://docs.rs/crate/rust_decimal/1.26.0/builds/605522) due to mutually
  exclusive `diesel1` and `diesel2` features. The previous `compile_error!`
  was in fact an [anti-pattern](https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features).
  Consequently,
  if both `diesel1` and `diesel2` features are specified then it will favor `diesel2`. This helps prevent any conflict
  scenarios
  and ultimately resolves the downstream `docs.rs` issue.

## 1.26.0

### Added

* `serde-with-*` features now support `Option<T>` variants ([#524](https://github.com/paupino/rust-decimal/pull/524)).
* Implementation support for `core::iter::Product` ([#525](https://github.com/paupino/rust-decimal/pull/525)).
* Diesel `2.0.0-rc.1` support via `db-diesel2-mysql` and `db-diesel2-postgres`
  features. ([#533](https://github.com/paupino/rust-decimal/pull/533)
  and ([#532](https://github.com/paupino/rust-decimal/pull/532))).

### Fixed

* Fixes a silent overflow in `from_str` when parsing `Decimal::MAX` with a fractional
  portion. ([#530](https://github.com/paupino/rust-decimal/pull/530)).

### Credit

Thank you to [@Guara92](https://github.com/Guara92) who made the initial `diesel2` PR which made parallel implementation
a breeze! Also, once again, thank you
to [@c410-f3r](https://github.com/c410-f3r) for implementing the missing `Product` trait!

## 1.25.0

### Added

* Expands `rand` support to implement `SampleUniform` to allow for `rng.gen_range`
  calls ([#519](https://github.com/paupino/rust-decimal/pull/519))
* Adds [`rkyv` framework](https://github.com/rkyv/rkyv) support for zero-cost (de)
  serialization ([#520](https://github.com/paupino/rust-decimal/pull/520))

A big thank you to [@lukesneeringer](https://github.com/lukesneeringer) and [@dovahcrow](https://github.com/dovahcrow)
for your help building
features for this release.

## 1.24.0

* Introduces new feature `rand` which allows for random number generation
  within `rust_decimal` ([#517](https://github.com/paupino/rust-decimal/pull/517)).
* Upgrade to Rust 2021 and remove unnecessary dependencies to improve build
  speed ([#513](https://github.com/paupino/rust-decimal/pull/513), [#516](https://github.com/paupino/rust-decimal/pull/516))
* Some test scoping improvements to ensure we capture as many test combinations as
  possible ([#503](https://github.com/paupino/rust-decimal/pull/503), [#504](https://github.com/paupino/rust-decimal/pull/504)).

A big thank you to [@c410-f3r](https://github.com/c410-f3r) for their contributions to this release!

## 1.23.1

Fixes issue with `no_std` compatability introduced
in `1.23.0` ([#501](https://github.com/paupino/rust-decimal/pull/501)).

## 1.23.0

* Add `BorschSchema` support for the `borsh` feature ([#498](https://github.com/paupino/rust-decimal/pull/498)).
* Implement `TryFrom` for Decimal primitive types ([#493](https://github.com/paupino/rust-decimal/pull/493)).
* Fix `to_i64` to support `i64::MIN` ([#496](https://github.com/paupino/rust-decimal/pull/496)).
* Implement `Inv` from `num_traits` ([#495](https://github.com/paupino/rust-decimal/pull/495)).
* Some minor housekeeping
  tasks ([#487](https://github.com/paupino/rust-decimal/pull/487), [#490](https://github.com/paupino/rust-decimal/pull/490)).

Thank you [@turion](https://github.com/turion), [@arthurprs](https://github.com/arthurprs)
and [@jnitard](https://github.com/jnitard) for your help with this release.

## 1.22.0

* Add support for `borsh` serialization/deserialization ([#478](https://github.com/paupino/rust-decimal/pull/478)).
* Fixes an issue with `serde-with-str` where it wasn't behaving the same as `serde-str` when
  using `bincode` ([#484](https://github.com/paupino/rust-decimal/pull/484)).
* Added `must_use` to `is_sign_negative` and `unpack` ([#482](https://github.com/paupino/rust-decimal/pull/482)).
* Minor documentation fixes.

Thank you [@jkbpvsc](https://github.com/jkbpvsc) for your help in this release.

## 1.21.0

* Saturating op variants have been added: `saturating_add`, `saturating_sub`
  and `saturating_mul` ([#464](https://github.com/paupino/rust-decimal/pull/464))
* Fixes issue with `log10` values `0 < x < 1` ([#469](https://github.com/paupino/rust-decimal/pull/469))
* Various documentation fixes/cleanup.

Thank you [@c410-f3r](https://github.com/c410-f3r) for your work in this release!

## 1.20.0

* Additional fuzz testing for deserialize ([#452](https://github.com/paupino/rust-decimal/pull/452))
* Documentation fix for rounding strategy ([#458](https://github.com/paupino/rust-decimal/pull/458))
* `from_str` is now over 4x faster, utilizing const generics and
  TCO ([#456](https://github.com/paupino/rust-decimal/pull/456))
* Fixed `from_str` issue with rounding issues when too many digits in source
  string. ([#453](https://github.com/paupino/rust-decimal/issues/453))
* New `serde-with` functionality for greater configurability when
  using `serde` ([#459](https://github.com/paupino/rust-decimal/pull/459))
* Various maintenance tasks ([#460](https://github.com/paupino/rust-decimal/pull/460))

This is truly a collaborative release and has some significant contributions from the community. A huge thank
you to everyone involved:

* [@chris-cantor](https://github.com/chris-cantor)
* [@TradingTomatillo](https://github.com/TradingTomatillo)
* [@c410-f3r](https://github.com/c410-f3r)
* [@ShigotoMitame](https://github.com/ShigotoMitame)
* [@luke-brown](https://github.com/luke-brown)

## 1.19.0

This is a minor dot release and contains library updates ([#334](https://github.com/paupino/rust-decimal/pull/334)) and
expanded scope in `prelude` to include `num_traits::Signed` ([#450](https://github.com/paupino/rust-decimal/pull/450)).
While subtle, it also expands the way towards some bigger improvements including variable precision.

Of a larger note, the minimum Rust compiler version has been updated so we can start utilizing various compiler features
(such as const generics). The miminum compiler version is now `1.51.0`.

Thank you to [@c410-f3r](https://github.com/c410-f3r) and [@jracollins](https://github.com/jracollins) for your
contributions
to this release.

## 1.18.0

* Fixes integer handling for `to_f64` when scale is set
  to `0`. [#443](https://github.com/paupino/rust-decimal/pull/443).
  Thank you [@anatols](https://github.com/anatols) for creating this fix.
* Add support for Rocket Forms using the feature
  flag `rocket-traits`. [#445](https://github.com/paupino/rust-decimal/pull/445).
  Thank you [@Misterio77](https://github.com/Misterio77) for putting this together.
* Add Diesel MySQL support, enabled by the feature
  flag `db-diesel-mysq`. [#446](https://github.com/paupino/rust-decimal/pull/446).

## 1.17.0

* Fixes panic when display precision was specified greater than
    30. [#428](https://github.com/paupino/rust-decimal/issues/428).
* Binds `deserialize` so that invalid scale values cannot be set. This may be a breaking change for some users
  leveraging
  this in a `const` function. [#428](https://github.com/paupino/rust-decimal/issues/428).
* Fixes an issue in `round_sf` whereby integral values that started with a 0 or 1 would output incorrect rounded
  numbers. [#430](https://github.com/paupino/rust-decimal/issues/430), [#437](https://github.com/paupino/rust-decimal/issues/437).
* Adds `from_f32_retain` and `from_f64_retain` functions which allow parsing a floating point number and retaining any
  excess precision. [#438](https://github.com/paupino/rust-decimal/issues/438).

## 1.16.0

* Implement `sin`/`cos`/`tan` functions which are enabled within the `maths`
  feature. [#413](https://github.com/paupino/rust-decimal/pull/413).
* Implement `round_sf` and `round_sf_with_strategy` to enable significant figure
  rounding. [#421](https://github.com/paupino/rust-decimal/pull/421)
* Remove unnecessary `std` feature from `arrayvec` usage [#417](https://github.com/paupino/rust-decimal/pull/417).
* Adhoc benchmarking and fuzz tooling improvements [#412](https://github.com/paupino/rust-decimal/pull/412),
  [#415](https://github.com/paupino/rust-decimal/pull/415), [#416](https://github.com/paupino/rust-decimal/pull/416),
  [#418](https://github.com/paupino/rust-decimal/pull/418).

Thank you to [@c410-f3r](https://github.com/c410-f3r) for all your help with fuzz and benchmarking improvements!

## 1.15.0

A minor bug and feature release which adds a couple of new functions as well as cleans up some documentation:

* Support for serializing to float without converting to float via the `serde-arbitrary-precision` feature.
  [#402](https://github.com/paupino/rust-decimal/issues/402).
  Thanks [@JamesHinshelwood](https://github.com/JamesHinshelwood)!
  for finding and fixing this!
* Add `log10` support to the `maths` feature. Please note that `ln` and `log10` will now panic on invalid input since
  both
  functions have a `checked_*` equivalent. This is the preferred approach going forward, however if you would like to
  re-enable
  the previous behavior please use the `maths-nopanic`
  feature. [#397](https://github.com/paupino/rust-decimal/issues/397).
* Added further constants to the `Decimal` library including `TWO`, `TEN`, `ONE_HUNDRED`, `ONE_THOUSAND`,
  and `NEGATIVE_ONE`.
  [#400](https://github.com/paupino/rust-decimal/issues/400).
* Fixes serialization issue for `-0` whereby `-` would be
  output [#406](https://github.com/paupino/rust-decimal/pull/406). Thanks
  [@edwardycl](https://github.com/edwardycl)!
* Fixes float rounding before return in `to_f64`. [#401](https://github.com/paupino/rust-decimal/issues/401).
* Added `BUILD.md` file to help people get set up with Rust Decimal locally and cleaned up some
  documentation examples.

## 1.14.3

Fixes an issue [#398](https://github.com/paupino/rust-decimal/issues/398) where `Decimal::ZERO.ln()` would panic rather
than returning `Decimal::ZERO`. This
aligns the behavior with calling `ln` on negative decimals.

Thank you to [@SebRollen](https://github.com/SebRollen) for finding and fixing this.

## 1.14.2

Fixes an overflow issue during division under some specific
circumstances. ([#392](https://github.com/paupino/rust-decimal/issues/392))

## 1.14.1

A bug fix release following on from `1.14.0`:

* Fixes an issue whereby in some cases when subtracting a 64 bit `Decimal` a negating overflow would occur during
  underflow.
  [#384](https://github.com/paupino/rust-decimal/issues/384). Thank you to [@c410-f3r](https://github.com/c410-f3r) for
  finding
  this as part of fuzz testing.
* Fixes an issue with `exp` whereby negative values lost accuracy due to inability to converge.
  [#378](https://github.com/paupino/rust-decimal/issues/378). Thank you to [@schungx](https://github.com/schungx) for
  finding this and proposing a fix.
* Fixes some documentation issues.

## 1.14.0

* Added `checked_exp` and `checked_norm_pdf` functions [#375](https://github.com/paupino/rust-decimal/pull/375).
* Fixes bug in division under certain circumstances whereby overflow would occur during
  rounding. [#377](https://github.com/paupino/rust-decimal/pull/377)
* Documentation improvements

Thank you to [@falsetru](https://github.com/falsetru), [@schungx](https://github.com/schungx)
and [@blasrodri](https://github.com/blasrodri) for your
help with this release!

## 1.13.0

This is a minor update to the library providing a few new features and one breaking change (I'm not using semver
properly here
sorry).

* `#[must_use]` added to functions to provide additional compiler hints.
* `try_from_i128_with_scale` function added to safely handle `i128` overflow errors.
* New `c-repr` feature added which will ensure that `#[repr(C)]` is used on the `Decimal` type.
  Thanks [@jean-airoldie](https://github.com/jean-airoldie).
* Small improvements to `from_scientific`. It now supports a wider range of values as well has slightly faster
  performance.
* Support for negative and decimal `pow` functions. This is *breaking* since `powi(u64)` has been renamed
  to `powi(i64)`. If you want to
  continue using `u64` arguments then please use `powu(u64)`. The fractional functions should be considered experimental
  for the time being
  and may have subtle issues that still need ironing out. Functions are now:
    * `powi`, `checked_powi` - When the exponent is a signed integer.
    * `powu`, `checked_powu` - When the exponent is an unsigned integer.
    * `powf`, `checked_powf` - When the exponent is a floating point number. Please note, numbers with a fractional
      component
      will use an approximation function.
    * `powd`, `checked_powd` - When the exponent is a `Decimal`. Please note, numbers with a fractional component will
      use
      an approximation function.

## 1.12.4

Adds `num_traits::One` back to `rust_decimal::prelude` to prevent unnecessary downstream dependency breakages.
Thanks [@spearman](https://github.com/spearman).

## 1.12.3

Fixes an issue [#361](https://github.com/paupino/rust-decimal/issues/361) when rounding a small number towards zero.

## 1.12.2

Fixes small regression whereby `0 - 0` was producing `-0`. Thank
you [@KonishchevDmitry](https://github.com/KonishchevDmitry) for
providing a swift fix ([#356](https://github.com/paupino/rust-decimal/pull/356)).

## 1.12.1

Added `num_traits::Zero` back to `rust_decimal::prelude` to prevent unnecessary downstream dependency breakages.

## 1.12.0

This version releases faster operation support for `add`, `sub`, `cmp`, `rem` and `mul` to match the renewed `div`
strategy.
It does this by leveraging 64 bit support when it makes sense, while attempting to still keep 32 bit optimizations in
place.
To ensure correct functionality, thousands more tests were included to cover a wide variety of different scenarios
and bit combinations. Compared to previous operations, we get the following speed improvements:

* `add` - up to 2.2x faster
* `div` - up to 428x faster
* `mul` - up to 1.8x faster
* `rem` - up to 1.08x faster
* `sub` - up to 2.5x faster

Of course, if old functionality is desired, it can be re-enabled by using the `legacy-ops` feature.

Other improvements include:

* Remove unnecessary `String` allocation when parsing a scientific number format.
  Thanks [@thomcc](https://github.com/thomcc) for the fix [#350](https://github.com/paupino/rust-decimal/pull/350).
* Fixes overflow bug with `sqrt` when using the smallest possible representable
  number. [#349](https://github.com/paupino/rust-decimal/pull/349).
* Some minor optimizations in the `maths` feature. Future work will involve speeding up this feature by keeping
  operations
  in an internal format until required.
* Added associated constants for `MIN`, `MAX` and `ZERO`. Deprecated `min_value()` and `max_value()` in favor of these
  new
  constants.
* `-0` now gets corrected to `0`. During operation rewrite I needed to consider operations such as `-0 * 2` - in cases
  like
  this I opted towards `0` always being the right number and `-0` being superfluous (since `+0 == -0`). Consequently,
  parsing
  `-0` etc _in general_ will automatically be parsed as `0`. Of course, this _may_ be a breaking change so if this
  functionality is required then please create an issue with the use case described.
* Small breaking change by renaming `is_negative` to `negative` in `UnpackedDecimal`.
* Some internal housekeeping was made to help make way for version 2.0 improvements.

## 1.11.1

This is a documentation only release and has no new functionality included. Thank
you [@c410-f3r](https://github.com/c410-f3r) for the documentation fix.

## 1.11.0

This release includes a number of bug fixes and ergonomic improvements.

* Mathematical functionality is now behind a feature flag. This should help optimize library size when functions such as
  `log` and `pow` are not required (e.g. simple financial applications). Mathematical functionality is now behind
  the `maths`
  feature flag. [#321](https://github.com/paupino/rust-decimal/pull/321).
* Numerous test coverage improvements to ensure broader
  coverage. [#322](https://github.com/paupino/rust-decimal/pull/322),
  [#323](https://github.com/paupino/rust-decimal/pull/323)
* Various documentation
  improvements. [#324](https://github.com/paupino/rust-decimal/pull/324), [#342](https://github.com/paupino/rust-decimal/pull/342)
* Fixes `u128` and `i128` parsing. [#332](https://github.com/paupino/rust-decimal/pull/332)
* Implemented `Checked*` traits from `num_traits`. [#333](https://github.com/paupino/rust-decimal/pull/333). Thank you
  [@teoxoy](https://github.com/teoxoy)
* Added `checked_powi` function to `maths` feature. [#336](https://github.com/paupino/rust-decimal/pull/336)
* Updated `from_parts` to avoid accidental scale clobbering. [#337](https://github.com/paupino/rust-decimal/pull/337)
* Added support for the `Arbitrary` trait for `rust-fuzz` support. This is behind the feature flag `rust-fuzz`.
  [#338](https://github.com/paupino/rust-decimal/pull/338)
* Fixes `e^-1` returning an incorrect approximation. [#339](https://github.com/paupino/rust-decimal/pull/339)
* Revamp of `RoundingStrategy` naming and documentation ([#340](https://github.com/paupino/rust-decimal/pull/340)).
  The old naming was ambiguous in interpretation - the new naming
  convention follows guidance from other libraries to ensure an easy to follow scheme. The `RoundingStrategy` enum now
  includes:
    * `MidpointNearestEven` (previously `BankersRounding`)
    * `MidpointAwayFromZero` (previously `RoundHalfUp`)
    * `MidpointTowardZero` (previously `RoundHalfDown`)
    * `ToZero` (previously `RoundDown`)
    * `AwayFromZero` (previously `RoundUp`)
    * `ToNegativeInfinity` - new rounding strategy
    * `ToPositiveInfinity` - new rounding strategy
* Added function to access `mantissa` directly. [#341](https://github.com/paupino/rust-decimal/pull/341)
* Added a feature to `rust_decimal_macros` to make re-exporting the macro from a downstream crate more approachable.
  Enabling the `reexportable` feature will ensure that the generated code doesn't require `rust_decimal` to be exposed
  at
  the root level. [#343](https://github.com/paupino/rust-decimal/pull/343)

## 1.10.3

* Fixes bug in bincode serialization where a negative symbol causes a buffer overflow (#317).

## 1.10.2

* Fixes a bug introduced in division whereby certain values when using a large remainder cause an incorrect results (
  #314).

## 1.10.1

* Fixes bug introduced in `neg` whereby sign would always be turned negative as opposed to being correctly negated.

Thank you [KonishchevDmitry](https://github.com/KonishchevDmitry) for finding and fixing this.

## 1.10.0

* Upgrade `postgres` to `0.19` and `tokio-postgres` to `0.7`.
* Faster `serde` serialization by preventing heap allocation.
* Alternative division algorithm which provides significant speed improvements. The new algorithms are enabled by
  default,
  but can be disabled with the feature: `legacy-ops`. Further work to improve other operations will
  be made available in future versions.
* Add `TryFrom` for `f32`/`f64` to/from Decimal

Thank you for the the community help and support for making this release happen, in particular:
[jean-airoldie](https://github.com/jean-airoldie), [gakonst](https://github.com/gakonst), [okaneco](https://github.com/okaneco)
and
[c410-f3r](https://github.com/c410-f3r).

## 1.9.0

* Added arbitrary precision support for `serde_json` deserialization (#283)
* Add `u128` and `i128` `FromPrimitive` overrides to prevent default implementation kicking in. Also adds default `From`
  interceptors to avoid having to use trait directly. (#282)
* Alias `serde-bincode` as `serde-str` to make usage clearer (#279)
* Adds scientific notation to format strings via `UpperExp` and `LowerExp` traits. (#271)
* Upgrade `tokio-postgres` and `postgres` libraries.
* Add statistical function support for `powi`, `sqrt`, `exp`, `norm_cdf`, `norm_pdf`, `ln` & `erf` (#281, #287)
* Allow `sum` across immutable references (#280)

Thank you for all the community help and support with this release, in particular [xilec](https://github.com/xilec),
[remkade](https://github.com/remkade) and [Anders429](https://github.com/Anders429).

## 1.8.1

Make `std` support the default to prevent breaking downstream library dependencies. To enable `no_std` support please
set
default features to false and opt-in to any required components. e.g.

```
rust_decimal = { default-features = false, version = "1.8.0" }
```

## 1.8.0

* `no_std` support added to Rust Decimal by default. `std` isn't required to use Rust Decimal, however can be enabled by
  using the `std` feature. [#190](https://github.com/paupino/rust-decimal/issues/190)
* Fixes issue with Decimal sometimes losing precision
  through `to_f64`. [#267](https://github.com/paupino/rust-decimal/issues/267).
* Add `Clone`, `Copy`, `PartialEq` and `Eq` derives to `RoundingStrategy`.
* Remove Proc Macro hack due to procedural macros as expressions being stabilized.
* Minor optimizations

Thank you to [@c410-f3r](https://github.com/c410-f3r), [@smessmer](https://github.com/smessmer)
and [@KiChjang](https://github.com/KiChjang).

## 1.7.0

* Enables `bincode` support via the feature `serde-bincode`. This provides a long term fix for a regression
  that was introduced in version `0.6.5` (tests now cover this
  case!). [Issue 43](https://github.com/paupino/rust-decimal/issues/43).
* Fixes issue where `rescale` on zero would not have an affect. This was due to an early exit condition which failed to
  set the new scale. [Issue 253](https://github.com/paupino/rust-decimal/issues/253).
* Add `min` and `max` functions, similar to what `f32` and `f64` provide. Thank
  you [@michalsieron](https://github.com/michalsieron).
* Updates documentation for `is_sign_positive` and `is_sign_negative` to specify that the sign bit is being checked.

Please note: feature naming conventions have been modified, however backwards compatible aliases have been created where
necessary. It's highly recommended that you move over to the new naming conventions as these aliases may be removed at a
later date.

## 1.6.0

* Fixes issue with PostgreSQL conversions whereby certain inputs would cause unexpected
  outputs. [Issue 241](https://github.com/paupino/rust-decimal/issues/241).
* Fixes issue with `from_str_radix` whereby rounding logic would kick in too early,
  especially with radix less than 10. [Issue 242](https://github.com/paupino/rust-decimal/issues/242).
* Fixes issue whereby `from_str` (implicity `from_str_radix`) would panic when there was overflow
  and overflow significant digit was < 5. [Issue 246](https://github.com/paupino/rust-decimal/issues/246).
* Make `bytes` and `byteorder` optional since they're only used in the `postgres` feature and tests.
* Fix edge case in `from_i128_with_scale` when `i128::MIN` was provided.

Thank you
to [@serejkaaa512](https://github.com/serejkaaa512), [@AbsurdlySuspicious](https://github.com/AbsurdlySuspicious)
and [@0e4ef622]((https://github.com/0e4ef622)) for your contributions!

## 1.5.0

* Added additional `RoundStrategy` abilities: `RoundUp` to always round up and `RoundDown` to always round down.
* Updated prelude to include expected structs and traits by default.

Special thank you to [@jean-airoldie](https://github.com/jean-airoldie) for adding the additional rounding strategies
and to [@pfrenssen](https://github.com/pfrenssen) for fixing an
issue in the README.

## 1.4.1

* Performance improvements for `to_f64` when using a scale > 0.

Special thank you to [@hengchu](https://github.com/hengchu) who discovered and resolved the issue!

## 1.4.0

* Allow uppercase "E" in scientific notation.
* Allow scientific notation in `dec!` macro.
* Deprecate `set_sign` and replace with `set_sign_positive` and `set_sign_negative`. This is intended
  to improve the readability of the API.
* Fixes precision issue when parsing `f64` values. The base 2 mantissa of the float was assuming guaranteed accuracy
  of 53 bit precision, however 52 bit precision is more accurate (`f64` only).
* Removes deprecated usage of `Error::description`.

## 1.3.0

* Replace `num` dependency with `num_trait` - implemented `Signed` and `Num` traits.

## 1.2.1

* Fixes issue whereby overflow would occur reading from PostgreSQL with high precision. The library now
  handles this by rounding high precision numbers as they're read as opposed to crashing (similar to other
  underflow situations e.g. 1/3).

## 1.2.0

* Retain trailing zeros from PostgreSQL. This ensures that the scale is maintained when serialized into the Decimal
  type.
* Fixes issue where -0 != 0 (these are now equivalent - thank you @hengchu for discovering).
* Improve hashing function so that the following property is true: `k1 == k2 -> hash(k1) == hash(k2)`
* Update normalize function so that -0 normalizes to 0.

Special thanks to @hathawsh for their help in this release!

## 1.1.0

* Update to Postgres 0.17 and add postgres async/await support via `tokio-pg`
* Added option for serializing decimals as float via `serde-float`

Special thanks to @pimeys and @kaibyao!

## 1.0.3

Updates dependencies to prevent build issues.

## 1.0.2

Bug fix release:

* Fixes issue where scaling logic produced incorrect results when one arm was a high precision zero. Thank you
  @KonishchevDmitry!

## 1.0.1

Bug fix release:

* Fixes issue where `ToSql` was incorrectly calculating weight when whole portion = numeric portion.
* Fixes issue where `Decimal::new` incorrectly handled `i64::max_value()` and `i64::min_value()`.
* Fixes issue where `rem` operation incorrectly returned results when `scale` was required.

## 1.0.0

This release represents the start of semantic versioning and allows the library to start making fundamental improvements
under
the guise of V2.0. Leading up to that I expect to release 1.x versions which will include adding
various mathematical functions such as `pow`, `ln`, `log10` etc.

Version `1.0.0` does come with some new features:

* Checked Operations! This implements `checked_add`, `checked_sub`, `checked_mul`, `checked_div` and `checked_rem`.
* Fixes overflow from `max_value()` and `min_value()` for `i32` and `i64`.
* Minor documentation improvements and test coverage.

Special thanks to @0e4ef622 for their help with this release!

## 0.11.3

* Add prelude to help num trait inclusion (`use rust_decimal::prelude::*`)
* Add `Default` trait to the library. This is equivalent to using `Decimal::zero()`
* Added assignment operators for references.

Special thanks to @jean-airoldie for his help with this release!

## 0.11.2

* Fall back to `from_scientific` when `from_str` fails during deserialization. Thanks @mattjbray!
* Added basic `Sum` trait implementation

## 0.11.1

* Fixes a bug in `floor` and `ceil` where negative numbers were incorrectly handled.

## 0.11.0

* Macros are now supported on stable. This does use a [hack](https://github.com/dtolnay/proc-macro-hack) for the
  meantime
  so due diligence is required before usage.
* Fixes issue when parsing strings where an underscore preceded a decimal point.
* `const_fn` support via a feature flag. In the future this will be the default option however in order to support older
  compiler versions is behind a feature flag.

## 0.10.2

* Macros (nightly) now output structural data as opposed to serialized data. This is fully backwards compatible and
  results in some minor performance improvements. Also, removed feature gate so that it can be compiled in stable.
* Fixes a string parsing bug when given highly significant numbers that require rounding.

## 0.10.1

* Bumped dependencies to remove some legacy serialization requirements.

## 0.10.0

Special thanks to @xilec, @snd and @AndrewSpeed for their help with this release.

* New rounding strategies introduced via `round_dp_with_strategy`. Previously default rounding support used bankers
  rounding by default whereas now you can choose to round the half way point either up or down.
* PostgreSQL write performance improved so that it is at least 3 times faster than the previous implementation.
* `Debug` trait now outputs the actual decimal number by default to make it more useful within consuming libraries (
  e.g. `criterion.rs`). To get something similar to the previous functionality you can use the `unpack` argument - this
  is likely for core `rust-decimal` library maintainers.
* Various other performance improvements for common operations such as `rescale`, `sub` and `div`.

## 0.9.1

* Performance optimization for `add`.

## 0.9.0

* Introduces the `Neg` trait to support the ability to use `-decimal_variable`.
* Fixes bug with underflow on addition.

## 0.8.1

This release updates the published documentation only and is a no-op for functionality.

## 0.8.0

* Introduces `from_scientific` allowing parsing of scientific notation into the Decimal type.
* Fixes a bug when formatting a number with a leading zero's.

## 0.7.2

* Fixes bug in `rescale` whereby scaling which invoked rounding incorrectly set the new scale for the left/right sides.

## 0.7.1

* Fixes bug in `cmp` whereby two negatives would return an incorrect result.
* Further documentation examples
* Small improvements in division logic
* New `abs`, `floor` and `ceil` functions.

## 0.7.0

This is a minor version bump as we slowly build our way towards 1.0. Thank you for everyone's support and help as we get
there! This has a few notable changes - also introducing a few new interfaces which is the reason for the version bump:

* `from_parts` function to allow effective creation of `Decimal`'s without requiring binary serialization. An example of
  this benefit is with the lazy static group initializers for Postgres.
* `normalize` function to allow stripping trailing zero's easily.
* `trunc` function allows truncation of a number without any rounding. This effectively "truncates" the fractional part
  of the number.
* `fract` function returns the fractional part of the number without the integral.
* Minor improvements in some iterator logic, utilizing the compiler for further optimizations.
* Fixes issue in string parsing logic whereby `_` would cause numbers to be incorrectly identified.
* Many improvements to `mul`. Numbers utilizing the `lo` portion of the decimal only will now be shortcut and bigger
  numbers will now correctly overflow. True overflows will still panic, however large underflows will now be rounded as
  necessary as opposed to panicing.
* `Hash` was implemented by convention in `0.6.5` however is reimplemented explicitly in `0.7.0` for effectiveness.
* PostgreSQL read performance improved by pre-caching groups and leveraging `normalize` (i.e. avoiding strings). Further
  optimizations can be made in write however require some `div` optimizations first.
* Added short circuit write improvement for zero in PostgreSQL writes.
* Benchmarks are now recorded per build so we can start tracking where slow downs have occurred. This does mean there is
  a performance hit on Travis builds however hopefully the pay off will make it worthwhile.

## 0.6.5

Fixes issue with rescale sometimes causing a silent overflow which led to incorrect results during addition, subtraction
and compare. Consequently Decimal now rounds the most significant number so that these operations work successfully.

In addition, Decimal now derive's the `Hash` trait so that it can be used for indexing.

## 0.6.4

Fixes silent overflow errors when parsing highly significant strings. `from_str` will now round in these scenario's,
similar to oleaut32 behavior.

## 0.6.3

Fixes a regression in ordering where by different scales would be rescaled towards losing precision instead of
increasing precision. Have added numerous test suites to help cover more issues like this in the future.
Also fixes an issue in parsing invalid strings whereby the precision exceeded our maximum precision. Previously, this
would work with unintended results however this now returns an Error returned from `FromStr`.

## 0.6.2

Fixes an issue with division of rational numbers allowing results greater than `MAX_PRECISION`. This would ultimately
cause issues for future operations on this number.
In addition, in some cases transitive operations would not be equal due to overflow being lost.

## 0.6.1

This minor release is purely to expose `rust_decimal_macros` for use on the nightly channel. Documentation has been
updated accordingly.

## 0.6.0

This release has a few major changes to the internal workings of the `Decimal` implementation and consequently comes
with a number of performance improvements.

* Floats can now be parsed into a `Decimal` type using `from_f32` and `from_f64`.
* `add`, `sub`, `mul` run roughly 1500% faster than before.
* `div` run's roughly 1000% faster than before with room for future improvement.
* Also get significant speed improvements with `cmp`, `rescale`, `round_dp` and some string manipulations.
* Implemented `*Assign` traits for simpler usage.
* Removed `BigInt` and `BigUint` as being intermediary data types.

## 0.5.2

Minor bug fix to prevent a `panic` from overflow during comparison of high significant digit decimals.

## 0.5.1

Minor bux fix to prevent `panic` upon parsing an empty string.

## 0.5.0

* Removes postgres from default feature set.
* `bincode` support for serde
* Better support for format strings
* Benchmarks added to tests

## 0.4.2

Fixes bug in `cmp` whereby negative's were not being compared correctly.

## 0.4.1

Minor bug fix to support creating negative numbers using the default constructor.

## 0.4.0

This release is a stylistic cleanup however does include some minor changes that may break existing builds.

### Changed

* Serde is now optional. You can enable Serde support within `features` using the keyword `serde`.
* Serde now returns errors on invalid input as opposed to `0`.
* `f64` conversion support has been added.
* Update Postgres dependency to use v0.15.

## 0.3.1

This is a documentation release that should help with discoverability and usage.

## 0.3.0

### Changed

* Removed trait `ToDecimal` and replaced with builtin [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
  trait ([`#12`](https://github.com/paupino/rust-decimal/pull/12))
