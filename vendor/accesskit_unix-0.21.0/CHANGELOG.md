# Changelog

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.12.0 to 0.12.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.0 to 0.10.1
    * accesskit_consumer bumped from 0.14.0 to 0.14.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.1 to 0.11.0
    * accesskit_consumer bumped from 0.14.1 to 0.14.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.0 to 0.11.1
    * accesskit_consumer bumped from 0.15.0 to 0.15.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.1 to 0.11.2
    * accesskit_consumer bumped from 0.15.1 to 0.15.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.0 to 0.12.1
    * accesskit_consumer bumped from 0.16.0 to 0.16.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.1.0 to 0.1.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.2 to 0.12.3
    * accesskit_atspi_common bumped from 0.1.1 to 0.1.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.1.2 to 0.2.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.4.0 to 0.4.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.4.1 to 0.4.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.5.0 to 0.6.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.7.0 to 0.8.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.2 to 0.16.3
    * accesskit_atspi_common bumped from 0.9.2 to 0.9.3

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.0 to 0.17.1
    * accesskit_atspi_common bumped from 0.10.0 to 0.10.1

## [0.21.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.20.0...accesskit_unix-v0.21.0) (2026-03-04)


### Features

* Implement text attributes for AT-SPI ([#695](https://github.com/AccessKit/accesskit/issues/695)) ([33fc4f0](https://github.com/AccessKit/accesskit/commit/33fc4f06699da335f9b2de3ab381cc940314e9c1))


### Bug Fixes

* Correctly register interfaces on Unix after Hyperlink was added ([#694](https://github.com/AccessKit/accesskit/issues/694)) ([b5979a4](https://github.com/AccessKit/accesskit/commit/b5979a4c88f5d9084f70525cf24f3a37c0988f57))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.17.0 to 0.18.0

## [0.20.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.19.1...accesskit_unix-v0.20.0) (2026-02-01)


### ⚠ BREAKING CHANGES

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683))

### Bug Fixes

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683)) ([f8c22cb](https://github.com/AccessKit/accesskit/commit/f8c22cbb53ce6b1859adeff3c0b7154ad9e1f25e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.23.0 to 0.24.0
    * accesskit_atspi_common bumped from 0.16.1 to 0.17.0

## [0.19.1](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.19.0...accesskit_unix-v0.19.1) (2026-01-18)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.16.0 to 0.16.1

## [0.19.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.18.0...accesskit_unix-v0.19.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655))

### Features

* Implement the `url` property ([#669](https://github.com/AccessKit/accesskit/issues/669)) ([6202e63](https://github.com/AccessKit/accesskit/commit/6202e63c667e73216a306b3dd45ff25c364a4f45))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655)) ([c16eb80](https://github.com/AccessKit/accesskit/commit/c16eb80c49d14f39e8a7defb1694bf22b8ceed6f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.22.0 to 0.23.0
    * accesskit_atspi_common bumped from 0.15.0 to 0.16.0

## [0.18.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.17.2...accesskit_unix-v0.18.0) (2025-12-23)


### ⚠ BREAKING CHANGES

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626))

### Features

* Add GridCell role ([#643](https://github.com/AccessKit/accesskit/issues/643)) ([1e5abca](https://github.com/AccessKit/accesskit/commit/1e5abca737d1ee942c0804fec2c06d3cb08faa94))
* Implement BrailleLabel and BrailleRoleDescription roles ([#638](https://github.com/AccessKit/accesskit/issues/638)) ([0fdcebb](https://github.com/AccessKit/accesskit/commit/0fdcebb55e308e039ec99fbc31e94e8087a69f2d))


### Bug Fixes

* Activate when an assistive technology is actually running on Unix ([#628](https://github.com/AccessKit/accesskit/issues/628)) ([ec67fdf](https://github.com/AccessKit/accesskit/commit/ec67fdf1ec549913437a601fcd1fa142ec52c47d))
* Bump atspi crates ([dbf81c9](https://github.com/AccessKit/accesskit/commit/dbf81c968739187a9f4c3c105d0618c0f9db3135))


### Code Refactoring

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642)) ([4d46c27](https://github.com/AccessKit/accesskit/commit/4d46c2740631c5fe4f057707b949d12b26931d0b))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626)) ([ea23ec4](https://github.com/AccessKit/accesskit/commit/ea23ec424c7dbb8841e03d71b6a15b74264850a9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.1 to 0.22.0
    * accesskit_atspi_common bumped from 0.14.2 to 0.15.0

## [0.17.2](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.17.1...accesskit_unix-v0.17.2) (2025-10-20)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.14.1 to 0.14.2

## [0.17.1](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.17.0...accesskit_unix-v0.17.1) (2025-10-02)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.0 to 0.21.1
    * accesskit_atspi_common bumped from 0.14.0 to 0.14.1

## [0.17.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.16.0...accesskit_unix-v0.17.0) (2025-07-16)


### ⚠ BREAKING CHANGES

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594))

### Features

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594)) ([1d9b74c](https://github.com/AccessKit/accesskit/commit/1d9b74c057051509b3ffbf63afeacfb16f544ff8))
* Let parents declare actions supported on their children ([#593](https://github.com/AccessKit/accesskit/issues/593)) ([70b534b](https://github.com/AccessKit/accesskit/commit/70b534bed168a84b84cc35199588aa8ab784fb43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.20.0 to 0.21.0
    * accesskit_atspi_common bumped from 0.13.0 to 0.14.0

## [0.16.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.15.0...accesskit_unix-v0.16.0) (2025-06-26)


### ⚠ BREAKING CHANGES

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589))

### Bug Fixes

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589)) ([2887cdd](https://github.com/AccessKit/accesskit/commit/2887cddde817ba3851688068d8d10de5cef7c624))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.19.0 to 0.20.0
    * accesskit_atspi_common bumped from 0.12.0 to 0.13.0

## [0.15.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.14.0...accesskit_unix-v0.15.0) (2025-05-06)


### ⚠ BREAKING CHANGES

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550))

### Bug Fixes

* Expose tabs in the platform adapters ([341a11b](https://github.com/AccessKit/accesskit/commit/341a11bca2c8a29682c11ddcfe91fa58776ea11d))
* Mention caveats with window bounds under Wayland ([#559](https://github.com/AccessKit/accesskit/issues/559)) ([b0cf01a](https://github.com/AccessKit/accesskit/commit/b0cf01a26ded03d722818a193fa6902f69bbc102))


### Code Refactoring

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550)) ([56abf17](https://github.com/AccessKit/accesskit/commit/56abf17356e4c7f13f64aaeaca6a63c8f7ede553))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.18.0 to 0.19.0
    * accesskit_atspi_common bumped from 0.11.0 to 0.12.0

## [0.14.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.13.1...accesskit_unix-v0.14.0) (2025-03-06)


### ⚠ BREAKING CHANGES

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492))

### Bug Fixes

* Add list box support to the platform adapters ([6c622cf](https://github.com/AccessKit/accesskit/commit/6c622cff4c0c989d9a5f16b775adff9cdacf534c))
* Derive `Debug` for adapters ([#513](https://github.com/AccessKit/accesskit/issues/513)) ([753d904](https://github.com/AccessKit/accesskit/commit/753d90473cf57682568c7a17c82474c8e5d00b25))
* Don't enable the `zbus/tokio` feature on Unix ([#521](https://github.com/AccessKit/accesskit/issues/521)) ([9dfad97](https://github.com/AccessKit/accesskit/commit/9dfad9796f71563a78bc6a7d5f01b111d80e63e6))
* Fix some clippy warnings ([#509](https://github.com/AccessKit/accesskit/issues/509)) ([579b9c1](https://github.com/AccessKit/accesskit/commit/579b9c12dd8abc44ecab41fa3c326a1d8999871d))
* Update zbus to 5.0 ([#519](https://github.com/AccessKit/accesskit/issues/519)) ([4c10b80](https://github.com/AccessKit/accesskit/commit/4c10b801b6924c7010b83f4eb44c8c350c860cf6))


### Code Refactoring

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492)) ([089794c](https://github.com/AccessKit/accesskit/commit/089794c8f74957e91a19ae3df508e2a892f39ebc))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.1 to 0.18.0
    * accesskit_atspi_common bumped from 0.10.1 to 0.11.0

## [0.13.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.12.3...accesskit_unix-v0.13.0) (2024-10-31)


### ⚠ BREAKING CHANGES

* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468))

### Features

* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468)) ([2fa0d3f](https://github.com/AccessKit/accesskit/commit/2fa0d3f5b2b7ac11ef1751c133706f29e548bd6d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.3 to 0.17.0
    * accesskit_atspi_common bumped from 0.9.3 to 0.10.0

## [0.12.2](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.12.1...accesskit_unix-v0.12.2) (2024-10-07)


### Bug Fixes

* Update minimum supported Rust version to 1.75 ([#457](https://github.com/AccessKit/accesskit/issues/457)) ([fc622fe](https://github.com/AccessKit/accesskit/commit/fc622fe7657c80a4eedad6f6cded11d2538b54d5))
* Update to zbus v4 ([#456](https://github.com/AccessKit/accesskit/issues/456)) ([95db8f1](https://github.com/AccessKit/accesskit/commit/95db8f1c5f7f56598eab6910a990ccbf9d864dda))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.1 to 0.16.2
    * accesskit_atspi_common bumped from 0.9.1 to 0.9.2

## [0.12.1](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.12.0...accesskit_unix-v0.12.1) (2024-09-24)


### Bug Fixes

* Return a null object for AT-SPI application's parent ([#454](https://github.com/AccessKit/accesskit/issues/454)) ([8a84abf](https://github.com/AccessKit/accesskit/commit/8a84abf81eaf22dd3672813ca36f1a422b5c0f1e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.0 to 0.16.1
    * accesskit_atspi_common bumped from 0.9.0 to 0.9.1

## [0.12.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.11.1...accesskit_unix-v0.12.0) (2024-06-29)


### ⚠ BREAKING CHANGES

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434))

### Code Refactoring

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434)) ([7086bc0](https://github.com/AccessKit/accesskit/commit/7086bc0fad446d3ed4a0fd5eff641a1e75f6c599))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.15.0 to 0.16.0
    * accesskit_atspi_common bumped from 0.8.0 to 0.9.0

## [0.11.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.10.1...accesskit_unix-v0.11.0) (2024-06-09)


### Features

* Add `author_id` property ([#424](https://github.com/AccessKit/accesskit/issues/424)) ([0d1c56f](https://github.com/AccessKit/accesskit/commit/0d1c56f0bdde58715e1c69f6015df600cb7cb8c1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.14.0 to 0.15.0
    * accesskit_atspi_common bumped from 0.6.0 to 0.7.0

## [0.10.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.9.2...accesskit_unix-v0.10.0) (2024-05-26)


### Features

* Add basic text support on Unix ([#362](https://github.com/AccessKit/accesskit/issues/362)) ([52540f8](https://github.com/AccessKit/accesskit/commit/52540f82cf9fc148358351ed486bab3e7e91f1d6))
* Expose the `placeholder` property ([#417](https://github.com/AccessKit/accesskit/issues/417)) ([8f4a0a1](https://github.com/AccessKit/accesskit/commit/8f4a0a1c10f83fcc8580a37d8013fec2d110865b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_atspi_common bumped from 0.4.2 to 0.5.0

## [0.9.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.8.0...accesskit_unix-v0.9.0) (2024-04-30)


### ⚠ BREAKING CHANGES

* Force a semver-breaking release ([#398](https://github.com/AccessKit/accesskit/issues/398))

### Bug Fixes

* Force a semver-breaking release ([#398](https://github.com/AccessKit/accesskit/issues/398)) ([87b8b92](https://github.com/AccessKit/accesskit/commit/87b8b92b74a102c7cae48e013d2c2ae1cc2f9598))
* Increase minimum supported Rust version to `1.70` ([#396](https://github.com/AccessKit/accesskit/issues/396)) ([a8398b8](https://github.com/AccessKit/accesskit/commit/a8398b847aa003de91042ac45e33126fc2cae053))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.13.0 to 0.14.0
    * accesskit_atspi_common bumped from 0.3.0 to 0.4.0

## [0.8.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.7.5...accesskit_unix-v0.8.0) (2024-04-14)


### ⚠ BREAKING CHANGES

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375))

### Code Refactoring

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375)) ([9baebdc](https://github.com/AccessKit/accesskit/commit/9baebdceed7300389b6768815d7ae48f1ce401e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.3 to 0.13.0
    * accesskit_atspi_common bumped from 0.2.0 to 0.3.0

## [0.7.2](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.7.1...accesskit_unix-v0.7.2) (2024-02-24)


### Bug Fixes

* Don't emit focus event twice on Unix ([#354](https://github.com/AccessKit/accesskit/issues/354)) ([b39216c](https://github.com/AccessKit/accesskit/commit/b39216cb31df692fef377f9b3c3c718fd225cc3c))
* Use the new accesskit_atspi_common crate in the Unix adapter ([#356](https://github.com/AccessKit/accesskit/issues/356)) ([b2a468c](https://github.com/AccessKit/accesskit/commit/b2a468ccb91ee4e6d3435e73eb00c65cbe75060a))

## [0.7.1](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.7.0...accesskit_unix-v0.7.1) (2024-01-11)


### Bug Fixes

* Make full use of tokio ecosystem if the tokio feature is enabled on Unix ([#336](https://github.com/AccessKit/accesskit/issues/336)) ([c034802](https://github.com/AccessKit/accesskit/commit/c0348024665a615a30fd8fe2f02e8c93cf9c6332))
* Run our own async executor on Unix ([#337](https://github.com/AccessKit/accesskit/issues/337)) ([8f937ba](https://github.com/AccessKit/accesskit/commit/8f937baaa510dd96da196501822b82f75f05b595))

## [0.7.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.6.2...accesskit_unix-v0.7.0) (2024-01-03)


### ⚠ BREAKING CHANGES

* Lazily activate Unix adapters ([#324](https://github.com/AccessKit/accesskit/issues/324))

### Features

* Support custom role descriptions ([#316](https://github.com/AccessKit/accesskit/issues/316)) ([c8d1a56](https://github.com/AccessKit/accesskit/commit/c8d1a5638fa6c33adfa059815c04f7e043c56026))


### Bug Fixes

* Lazily activate Unix adapters ([#324](https://github.com/AccessKit/accesskit/issues/324)) ([54ed036](https://github.com/AccessKit/accesskit/commit/54ed036c99d87428a8eb5bb03fd77e9e31562d4c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.1 to 0.12.2
    * accesskit_consumer bumped from 0.16.1 to 0.17.0

## [0.6.2](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.6.1...accesskit_unix-v0.6.2) (2023-12-14)


### Bug Fixes

* Bump async-channel dependency to `2.1.1` ([#321](https://github.com/AccessKit/accesskit/issues/321)) ([99120b8](https://github.com/AccessKit/accesskit/commit/99120b828d65306ab71d41f71979dc67e8b0bf6b))

## [0.6.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.5.2...accesskit_unix-v0.6.0) (2023-09-27)


### ⚠ BREAKING CHANGES

* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296))
* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276))

### Features

* Add role for terminals ([#282](https://github.com/AccessKit/accesskit/issues/282)) ([ddbef37](https://github.com/AccessKit/accesskit/commit/ddbef37158b57f56217317b480e40d58f83a9c24))
* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291)) ([5313860](https://github.com/AccessKit/accesskit/commit/531386023257150f49b5e4be942f359855fb7cb6))
* Support live regions on Unix ([#299](https://github.com/AccessKit/accesskit/issues/299)) ([8d52a5f](https://github.com/AccessKit/accesskit/commit/8d52a5fc4271a3b5edcc602b23fd7b920446eab0))
* Support multiple top-level windows on Unix ([#292](https://github.com/AccessKit/accesskit/issues/292)) ([43ecf4b](https://github.com/AccessKit/accesskit/commit/43ecf4b3ab96d9e8f7d2c2222c7e664c4f4f4abf))


### Bug Fixes

* Don't require tokio rt-multi-thread feature ([#290](https://github.com/AccessKit/accesskit/issues/290)) ([cf61e47](https://github.com/AccessKit/accesskit/commit/cf61e477adff26b032fa0b24502c0ae0a96c1987))
* Use common filters across platform adapters ([#287](https://github.com/AccessKit/accesskit/issues/287)) ([09c1204](https://github.com/AccessKit/accesskit/commit/09c12045ff4ccdb22f0cf643077a27465013572d))


### Code Refactoring

* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289)) ([4fc9c55](https://github.com/AccessKit/accesskit/commit/4fc9c55c91812472593923d93ff89d75ff305ee4))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278)) ([d360d20](https://github.com/AccessKit/accesskit/commit/d360d20cf951e7643b81a5303006c9f7daa5bd56))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296)) ([4fc7846](https://github.com/AccessKit/accesskit/commit/4fc7846d732d61fb45c023060ebab96801a0053e))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276)) ([3eadd48](https://github.com/AccessKit/accesskit/commit/3eadd48ec47854faa94a94ebf910ec08f514642f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.2 to 0.12.0
    * accesskit_consumer bumped from 0.15.2 to 0.16.0

## [0.5.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.4.0...accesskit_unix-v0.5.0) (2023-05-21)


### Features

* Add features for async runtimes on Unix ([#248](https://github.com/AccessKit/accesskit/issues/248)) ([b56b4ea](https://github.com/AccessKit/accesskit/commit/b56b4ea7c967ee5a1dae21a2fa0dcd385346031e))

## [0.4.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.3.3...accesskit_unix-v0.4.0) (2023-03-30)


### ⚠ BREAKING CHANGES

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234))

### Bug Fixes

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234)) ([773389b](https://github.com/AccessKit/accesskit/commit/773389bff857fa18edf15de426e029251fc34591))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.14.2 to 0.15.0

## [0.3.1](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.3.0...accesskit_unix-v0.3.1) (2023-02-20)


### Bug Fixes

* Update atspi dependency ([#217](https://github.com/AccessKit/accesskit/issues/217)) ([93f2dc9](https://github.com/AccessKit/accesskit/commit/93f2dc9bf0a57a8b7592c3a4cf4aa3885a3356f2))

## [0.3.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.2.0...accesskit_unix-v0.3.0) (2023-02-12)


### ⚠ BREAKING CHANGES

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212))

### Code Refactoring

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212)) ([5df52e5](https://github.com/AccessKit/accesskit/commit/5df52e5545faddf6a51905409013c2f5be23981e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.9.0 to 0.10.0
    * accesskit_consumer bumped from 0.13.0 to 0.14.0

## [0.2.0](https://github.com/AccessKit/accesskit/compare/accesskit_unix-v0.1.1...accesskit_unix-v0.2.0) (2023-02-05)


### ⚠ BREAKING CHANGES

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205))

### Code Refactoring

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205)) ([4811152](https://github.com/AccessKit/accesskit/commit/48111521439b76c1a8687418a4b20f9b705eac6d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.8.1 to 0.9.0
    * accesskit_consumer bumped from 0.12.1 to 0.13.0

## 0.1.0 (2023-01-05)


### Features

* Basic Unix platform adapter ([#198](https://github.com/AccessKit/accesskit/issues/198)) ([1cea32e](https://github.com/AccessKit/accesskit/commit/1cea32e44ee743b778ac941ceff9087ae745cb37))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.11.0 to 0.12.0
