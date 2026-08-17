# Changelog

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.2 to 0.12.3
    * accesskit_consumer bumped from 0.17.0 to 0.17.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.19.0 to 0.19.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.0 to 0.16.1
    * accesskit_consumer bumped from 0.24.0 to 0.24.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.2 to 0.16.3
    * accesskit_consumer bumped from 0.24.2 to 0.24.3

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.0 to 0.17.1
    * accesskit_consumer bumped from 0.25.0 to 0.26.0

## [0.18.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.17.0...accesskit_atspi_common-v0.18.0) (2026-03-04)


### Features

* Implement text attributes for AT-SPI ([#695](https://github.com/AccessKit/accesskit/issues/695)) ([33fc4f0](https://github.com/AccessKit/accesskit/commit/33fc4f06699da335f9b2de3ab381cc940314e9c1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.34.0 to 0.35.0

## [0.17.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.16.1...accesskit_atspi_common-v0.17.0) (2026-02-01)


### ⚠ BREAKING CHANGES

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683))

### Bug Fixes

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683)) ([f8c22cb](https://github.com/AccessKit/accesskit/commit/f8c22cbb53ce6b1859adeff3c0b7154ad9e1f25e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.23.0 to 0.24.0
    * accesskit_consumer bumped from 0.33.1 to 0.34.0

## [0.16.1](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.16.0...accesskit_atspi_common-v0.16.1) (2026-01-18)


### Bug Fixes

* Emit initial focus event for active descendant node ([#681](https://github.com/AccessKit/accesskit/issues/681)) ([13a37d1](https://github.com/AccessKit/accesskit/commit/13a37d144e2fafceaf26e7645f6acc679ccb2f34))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.33.0 to 0.33.1

## [0.16.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.15.0...accesskit_atspi_common-v0.16.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655))

### Features

* Implement the `url` property ([#669](https://github.com/AccessKit/accesskit/issues/669)) ([6202e63](https://github.com/AccessKit/accesskit/commit/6202e63c667e73216a306b3dd45ff25c364a4f45))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655)) ([c16eb80](https://github.com/AccessKit/accesskit/commit/c16eb80c49d14f39e8a7defb1694bf22b8ceed6f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.22.0 to 0.23.0
    * accesskit_consumer bumped from 0.32.0 to 0.33.0

## [0.15.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.14.2...accesskit_atspi_common-v0.15.0) (2025-12-23)


### ⚠ BREAKING CHANGES

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626))

### Features

* Add GridCell role ([#643](https://github.com/AccessKit/accesskit/issues/643)) ([1e5abca](https://github.com/AccessKit/accesskit/commit/1e5abca737d1ee942c0804fec2c06d3cb08faa94))
* Implement BrailleLabel and BrailleRoleDescription roles ([#638](https://github.com/AccessKit/accesskit/issues/638)) ([0fdcebb](https://github.com/AccessKit/accesskit/commit/0fdcebb55e308e039ec99fbc31e94e8087a69f2d))
* Rename HeaderAsNonLandmark to SectionHeader, FooterAsNonLandmark to SectionFooter ([#635](https://github.com/AccessKit/accesskit/issues/635)) ([dd69526](https://github.com/AccessKit/accesskit/commit/dd695260877cb862ece4c2b7d8607308f5fed608))


### Bug Fixes

* Bump atspi crates ([dbf81c9](https://github.com/AccessKit/accesskit/commit/dbf81c968739187a9f4c3c105d0618c0f9db3135))
* Expose dialogs in the platform adapters ([1576a94](https://github.com/AccessKit/accesskit/commit/1576a940ec3e9066fc5e181419dae6899bf2db46))
* Replace thiserror with manual trait implementations ([#652](https://github.com/AccessKit/accesskit/issues/652)) ([b42c5e2](https://github.com/AccessKit/accesskit/commit/b42c5e25659aeb74d7f534ef2a8745d80227c1e3))


### Code Refactoring

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642)) ([4d46c27](https://github.com/AccessKit/accesskit/commit/4d46c2740631c5fe4f057707b949d12b26931d0b))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626)) ([ea23ec4](https://github.com/AccessKit/accesskit/commit/ea23ec424c7dbb8841e03d71b6a15b74264850a9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.1 to 0.22.0
    * accesskit_consumer bumped from 0.31.0 to 0.32.0

## [0.14.2](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.14.1...accesskit_atspi_common-v0.14.2) (2025-10-20)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.30.1 to 0.31.0

## [0.14.1](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.14.0...accesskit_atspi_common-v0.14.1) (2025-10-02)


### Bug Fixes

* Fix clippy warnings introduced in 1.89 ([#606](https://github.com/AccessKit/accesskit/issues/606)) ([b2c07d6](https://github.com/AccessKit/accesskit/commit/b2c07d654a8ce6f01e61a79c91f2f9d5a96afdc9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.0 to 0.21.1
    * accesskit_consumer bumped from 0.30.0 to 0.30.1

## [0.14.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.13.0...accesskit_atspi_common-v0.14.0) (2025-07-16)


### ⚠ BREAKING CHANGES

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594))

### Features

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594)) ([1d9b74c](https://github.com/AccessKit/accesskit/commit/1d9b74c057051509b3ffbf63afeacfb16f544ff8))
* Let parents declare actions supported on their children ([#593](https://github.com/AccessKit/accesskit/issues/593)) ([70b534b](https://github.com/AccessKit/accesskit/commit/70b534bed168a84b84cc35199588aa8ab784fb43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.20.0 to 0.21.0
    * accesskit_consumer bumped from 0.29.0 to 0.30.0

## [0.13.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.12.0...accesskit_atspi_common-v0.13.0) (2025-06-26)


### ⚠ BREAKING CHANGES

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589))

### Bug Fixes

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589)) ([2887cdd](https://github.com/AccessKit/accesskit/commit/2887cddde817ba3851688068d8d10de5cef7c624))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.19.0 to 0.20.0
    * accesskit_consumer bumped from 0.28.0 to 0.29.0

## [0.12.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.11.0...accesskit_atspi_common-v0.12.0) (2025-05-06)


### ⚠ BREAKING CHANGES

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550))

### Features

* Expose tabs in consumer and atspi-common ([b1fb5b3](https://github.com/AccessKit/accesskit/commit/b1fb5b3de12c001e34021263038b66a6e3a7dd1e))


### Bug Fixes

* Fix a compilation error in atspi-common `Event::new` ([#537](https://github.com/AccessKit/accesskit/issues/537)) ([23b4d8d](https://github.com/AccessKit/accesskit/commit/23b4d8d49fed378899855a40e63aff10e829f6e8))


### Code Refactoring

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550)) ([56abf17](https://github.com/AccessKit/accesskit/commit/56abf17356e4c7f13f64aaeaca6a63c8f7ede553))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.18.0 to 0.19.0
    * accesskit_consumer bumped from 0.27.0 to 0.28.0

## [0.11.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.10.1...accesskit_atspi_common-v0.11.0) (2025-03-06)


### ⚠ BREAKING CHANGES

* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493))
* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492))

### Features

* Add list box support to the `consumer` and `atspi-common` crates ([d6dca15](https://github.com/AccessKit/accesskit/commit/d6dca15d5c298c797ab7a702f0186043eac33c5c))
* Expose the `is_required` property ([#497](https://github.com/AccessKit/accesskit/issues/497)) ([46ed99b](https://github.com/AccessKit/accesskit/commit/46ed99bb958ddb32cbf1bee2fcfb7b328bcbe0ab))


### Bug Fixes

* Derive `Debug` for adapters ([#513](https://github.com/AccessKit/accesskit/issues/513)) ([753d904](https://github.com/AccessKit/accesskit/commit/753d90473cf57682568c7a17c82474c8e5d00b25))
* Remove unnecessary explicit lifetimes ([#488](https://github.com/AccessKit/accesskit/issues/488)) ([d2bcd6d](https://github.com/AccessKit/accesskit/commit/d2bcd6d3048d23df4e132bee6171eb247b2dc2c8))
* Update zbus to 5.0 ([#519](https://github.com/AccessKit/accesskit/issues/519)) ([4c10b80](https://github.com/AccessKit/accesskit/commit/4c10b801b6924c7010b83f4eb44c8c350c860cf6))


### Code Refactoring

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492)) ([089794c](https://github.com/AccessKit/accesskit/commit/089794c8f74957e91a19ae3df508e2a892f39ebc))
* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493)) ([484fd7c](https://github.com/AccessKit/accesskit/commit/484fd7cbfb778222369d3f57d31dd998f6fa80d8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.1 to 0.18.0
    * accesskit_consumer bumped from 0.26.0 to 0.27.0

## [0.10.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.9.3...accesskit_atspi_common-v0.10.0) (2024-10-31)


### ⚠ BREAKING CHANGES

* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473))
* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472))
* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468))

### Features

* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468)) ([2fa0d3f](https://github.com/AccessKit/accesskit/commit/2fa0d3f5b2b7ac11ef1751c133706f29e548bd6d))


### Code Refactoring

* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472)) ([ef3b003](https://github.com/AccessKit/accesskit/commit/ef3b0038224459094f650368412650bc3b69526b))
* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475)) ([e0053a5](https://github.com/AccessKit/accesskit/commit/e0053a5399929e8e0d4f07aa18de604ed8766ace))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473)) ([29fa341](https://github.com/AccessKit/accesskit/commit/29fa34125a811bd3a0f9da579a9f35c9da90bf29))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.3 to 0.17.0
    * accesskit_consumer bumped from 0.24.3 to 0.25.0

## [0.9.2](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.9.1...accesskit_atspi_common-v0.9.2) (2024-10-07)


### Bug Fixes

* Update minimum supported Rust version to 1.75 ([#457](https://github.com/AccessKit/accesskit/issues/457)) ([fc622fe](https://github.com/AccessKit/accesskit/commit/fc622fe7657c80a4eedad6f6cded11d2538b54d5))
* Update to zbus v4 ([#456](https://github.com/AccessKit/accesskit/issues/456)) ([95db8f1](https://github.com/AccessKit/accesskit/commit/95db8f1c5f7f56598eab6910a990ccbf9d864dda))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.1 to 0.16.2
    * accesskit_consumer bumped from 0.24.1 to 0.24.2

## [0.9.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.8.0...accesskit_atspi_common-v0.9.0) (2024-06-29)


### ⚠ BREAKING CHANGES

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434))

### Bug Fixes

* Correctly handle recursive filtering ([#438](https://github.com/AccessKit/accesskit/issues/438)) ([72f9b42](https://github.com/AccessKit/accesskit/commit/72f9b424a5c6e7914df8bf31eeb2fc61be35f47b))
* Smarter calculation of AT-SPI extents for the window ([#435](https://github.com/AccessKit/accesskit/issues/435)) ([7f40fbc](https://github.com/AccessKit/accesskit/commit/7f40fbc811f863e99088cd7eb725994d0f79263f))


### Code Refactoring

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434)) ([7086bc0](https://github.com/AccessKit/accesskit/commit/7086bc0fad446d3ed4a0fd5eff641a1e75f6c599))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.15.0 to 0.16.0
    * accesskit_consumer bumped from 0.23.0 to 0.24.0

## [0.8.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.7.0...accesskit_atspi_common-v0.8.0) (2024-06-15)


### Features

* Feature-gate the `accesskit_atspi_common::simplified` module ([#430](https://github.com/AccessKit/accesskit/issues/430)) ([50341f1](https://github.com/AccessKit/accesskit/commit/50341f10cf32ef16c904d54725f717a585d21043))


### Bug Fixes

* Avoid unnecessary repeated checks for text changes ([#432](https://github.com/AccessKit/accesskit/issues/432)) ([0e89671](https://github.com/AccessKit/accesskit/commit/0e89671fc0cf831d2c86a2cacf0195e247560753))

## [0.7.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.6.0...accesskit_atspi_common-v0.7.0) (2024-06-09)


### Features

* Add `author_id` property ([#424](https://github.com/AccessKit/accesskit/issues/424)) ([0d1c56f](https://github.com/AccessKit/accesskit/commit/0d1c56f0bdde58715e1c69f6015df600cb7cb8c1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.14.0 to 0.15.0
    * accesskit_consumer bumped from 0.22.0 to 0.23.0

## [0.6.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.5.0...accesskit_atspi_common-v0.6.0) (2024-05-27)


### Features

* Expose the `orientation` property ([#421](https://github.com/AccessKit/accesskit/issues/421)) ([590aada](https://github.com/AccessKit/accesskit/commit/590aada070dc812f9b8f171fb9e43ac984fad2a1))


### Bug Fixes

* Fix a logic error in suffix calculation for text changes ([#423](https://github.com/AccessKit/accesskit/issues/423)) ([1121723](https://github.com/AccessKit/accesskit/commit/1121723983cb2fa64b5053626ec64afb851ff6c4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.21.0 to 0.22.0

## [0.5.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.4.2...accesskit_atspi_common-v0.5.0) (2024-05-26)


### Features

* Add basic text support on Unix ([#362](https://github.com/AccessKit/accesskit/issues/362)) ([52540f8](https://github.com/AccessKit/accesskit/commit/52540f82cf9fc148358351ed486bab3e7e91f1d6))
* Expose the `placeholder` property ([#417](https://github.com/AccessKit/accesskit/issues/417)) ([8f4a0a1](https://github.com/AccessKit/accesskit/commit/8f4a0a1c10f83fcc8580a37d8013fec2d110865b))


### Bug Fixes

* Don't fire events for filtered children on Unix ([#414](https://github.com/AccessKit/accesskit/issues/414)) ([2bcb1b6](https://github.com/AccessKit/accesskit/commit/2bcb1b63e88b801b194a4db50059fa063efbee64))
* Improve how coordinates are computed on Unix ([#420](https://github.com/AccessKit/accesskit/issues/420)) ([fc5125e](https://github.com/AccessKit/accesskit/commit/fc5125e27f8f4f655e1de5049d0d53536284d9a0))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.20.0 to 0.21.0

## [0.4.2](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.4.1...accesskit_atspi_common-v0.4.2) (2024-05-13)


### Bug Fixes

* Fix platform adapters to support copy-on-write tree snapshots again ([#411](https://github.com/AccessKit/accesskit/issues/411)) ([d3a130a](https://github.com/AccessKit/accesskit/commit/d3a130a5ec8ae1d9edf0bf85a44f35f0e365242c))
* Return to handling focus events directly, after generic node changes ([#409](https://github.com/AccessKit/accesskit/issues/409)) ([cd2e35e](https://github.com/AccessKit/accesskit/commit/cd2e35e43817405199ae6acd64ef90aee445be0b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.19.1 to 0.20.0

## [0.4.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.3.0...accesskit_atspi_common-v0.4.0) (2024-04-30)


### ⚠ BREAKING CHANGES

* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388))

### Features

* Implement the `description` property ([#382](https://github.com/AccessKit/accesskit/issues/382)) ([d49f406](https://github.com/AccessKit/accesskit/commit/d49f40660b5dc23ed074cd72a91e511b130756ae))


### Bug Fixes

* Increase minimum supported Rust version to `1.70` ([#396](https://github.com/AccessKit/accesskit/issues/396)) ([a8398b8](https://github.com/AccessKit/accesskit/commit/a8398b847aa003de91042ac45e33126fc2cae053))


### Code Refactoring

* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393)) ([e34dad9](https://github.com/AccessKit/accesskit/commit/e34dad94448a5321ece9def3f2e054aa5f62dd79))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388)) ([6bc040b](https://github.com/AccessKit/accesskit/commit/6bc040b7cf75cdbd6a019cc380d8dbce804b3c81))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.13.0 to 0.14.0
    * accesskit_consumer bumped from 0.18.0 to 0.19.0

## [0.3.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.2.0...accesskit_atspi_common-v0.3.0) (2024-04-14)


### ⚠ BREAKING CHANGES

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375))

### Code Refactoring

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375)) ([9baebdc](https://github.com/AccessKit/accesskit/commit/9baebdceed7300389b6768815d7ae48f1ce401e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.3 to 0.13.0
    * accesskit_consumer bumped from 0.17.1 to 0.18.0

## [0.2.0](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.1.2...accesskit_atspi_common-v0.2.0) (2024-03-11)


### Features

* Expose root node ID in `accesskit_atspi_common::Adapter` ([#370](https://github.com/AccessKit/accesskit/issues/370)) ([a43b497](https://github.com/AccessKit/accesskit/commit/a43b497afbbbcf90e9d15259635a329164d6a791))

## [0.1.1](https://github.com/AccessKit/accesskit/compare/accesskit_atspi_common-v0.1.0...accesskit_atspi_common-v0.1.1) (2024-02-24)


### Bug Fixes

* Add missing README ([#357](https://github.com/AccessKit/accesskit/issues/357)) ([e8cf48e](https://github.com/AccessKit/accesskit/commit/e8cf48e21be0146768b2d14289164d192823fd1f))

## 0.1.0 (2024-02-24)


### Features

* Factor out core AT-SPI translation layer ([#352](https://github.com/AccessKit/accesskit/issues/352)) ([8c0ab58](https://github.com/AccessKit/accesskit/commit/8c0ab58d441c0d4484e0bc31a554bdfb3f088cd6))
